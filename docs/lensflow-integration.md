# LensFlow browser integration

This fork adds a browser-only `lensflow_request(action, paramsJson, bytes)` promise interface for a capability-scoped parent workspace. It supports opening an original, restoring/snapshotting a native project, exporting a PNG, reading revision state, acknowledging an exactly saved revision, and dispatching existing engine commands. Project/export saves require one open document; oversized document input is rejected at 256 MiB.

When launched with `?lensflow=1`, the web shell does not load or save local browser preferences or shared IndexedDB brush presets. The host owns tenant-scoped persistence and decides when to acknowledge a saved revision. Standalone browser behavior remains unchanged.

The editor receives document bytes and command data, never portal tokens or storage credentials. The host must verify the exact iframe window, origin, protocol and per-workspace channel before accepting messages, and enforce its own upload limits and revision concurrency. The Rust engine does not acquire network or backend storage authority.

This fork does not add model weights or a deployment workflow. Upstream licenses and notices remain applicable. LensFlow pins the reviewed fork commit and packages its own host and third-party notices separately.

## Fork CI/CD scope

The only active workflow in this fork is `lensflow-web.yml`. It checks Rust
formatting, Clippy against the browser target, browser-safe crates and dependency
layers, then builds optimized WebAssembly and validates compressed download parts.
The uploaded artifact is a CI intermediate; LensFlow separately supplies its host,
wasm-bindgen glue, and complete license notices. This workflow never deploys.

Inherited desktop, platform, documentation, performance, and release workflow
files are retained. Their GitHub workflows are disabled, and each inherited job
also requires the repository variable `ENABLE_NATIVE_EDITOR_WORKFLOWS=true`.
It is currently false. To restore an inherited pipeline later, review its purpose,
set that variable to true, and enable only the desired workflow through GitHub.
Native application code and packaging sources remain available.
