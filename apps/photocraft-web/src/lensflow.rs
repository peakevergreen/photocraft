//! Browser-only integration with a capability-scoped LensFlow parent workspace.
//! No network credentials or storage authority enter the editing engine.

use js_sys::{Function, Object, Promise, Reflect, Uint8Array};
use photocraft_ui_egui::{ExportSettings, PhotocraftApp};
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

type Request = (String, String, Vec<u8>, Function, Function);
thread_local! {
    static QUEUE: RefCell<Vec<Request>> = const { RefCell::new(Vec::new()) };
    static CTX: RefCell<Option<egui::Context>> = const { RefCell::new(None) };
}

#[wasm_bindgen]
pub fn lensflow_request(action: String, params: String, bytes: Vec<u8>) -> Promise {
    Promise::new(&mut |resolve, reject| {
        QUEUE.with(|q| q.borrow_mut().push((action.clone(), params.clone(), bytes.clone(), resolve, reject)));
        CTX.with(|ctx| {
            if let Some(ctx) = ctx.borrow().as_ref() {
                ctx.request_repaint();
            }
        });
    })
}

fn perform(app: &mut PhotocraftApp, action: &str, params: &str, bytes: &[u8]) -> Result<JsValue, String> {
    let params: serde_json::Value = serde_json::from_str(params).map_err(|e| e.to_string())?;
    match action {
        "open" | "restore" => {
            if bytes.len() > 256 * 1024 * 1024 {
                return Err("This document exceeds the 256 MB workspace limit".into());
            }
            app.open_bytes(params["name"].as_str().unwrap_or("photo.png"), bytes)?;
            if action == "restore" {
                while app.session.documents().len() > 1 {
                    app.session.close(0);
                }
            }
            Ok(JsValue::NULL)
        }
        "status" => {
            let state = app.session.active().ok_or("No document is open")?;
            Ok(serde_json::json!({"revision": state.revision, "name": state.doc.name}).to_string().into())
        }
        "acknowledge" => {
            let state = app.session.active_mut().ok_or("No document is open")?;
            if params["revision"].as_u64() == Some(state.revision) {
                state.saved_revision = state.revision;
            }
            Ok(JsValue::NULL)
        }
        "snapshot" | "export" => {
            if app.session.documents().len() != 1 {
                return Err("This LensFlow workspace saves one document. Close extra document tabs before saving.".into());
            }
            let state = app.session.active().ok_or("No document is open")?;
            let name = if action == "snapshot" { "project.pcraft" } else { "export.png" };
            let encoded = photocraft_io::export(&state.doc, name, &ExportSettings::default().io_options()).map_err(|e| e.to_string())?;
            let result = Object::new();
            Reflect::set(&result, &"bytes".into(), &Uint8Array::from(encoded.bytes.as_slice())).map_err(|e| format!("{e:?}"))?;
            Reflect::set(&result, &"revision".into(), &JsValue::from_f64(state.revision as f64)).map_err(|e| format!("{e:?}"))?;
            Reflect::set(&result, &"warnings".into(), &JsValue::from_str(&encoded.warnings.join("\n"))).map_err(|e| format!("{e:?}"))?;
            Ok(result.into())
        }
        // Same command dispatch as the UI; no control-server or filesystem authority.
        "command" => app.run(params["id"].as_str().ok_or("Missing command")?, params["params"].clone()).map(|v| v.to_string().into()),
        _ => Err("Unknown LensFlow operation".into()),
    }
}

pub fn drain(app: &mut PhotocraftApp, ctx: &egui::Context) {
    CTX.with(|value| *value.borrow_mut() = Some(ctx.clone()));
    for (action, params, bytes, resolve, reject) in QUEUE.with(|q| std::mem::take(&mut *q.borrow_mut())) {
        match perform(app, &action, &params, &bytes) {
            Ok(value) => {
                let _ = resolve.call1(&JsValue::NULL, &value);
            }
            Err(error) => {
                let _ = reject.call1(&JsValue::NULL, &error.into());
            }
        }
    }
}
