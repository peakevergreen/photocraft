# LensFlow browser integration

This fork adds a browser-only `lensflow_request(action, paramsJson, bytes)` promise interface for a capability-scoped parent workspace. It supports opening an original, restoring/snapshotting a native project, exporting a PNG, reading revision state, acknowledging an exactly saved revision, and dispatching existing engine commands. Project/export saves require one open document; oversized document input is rejected at 256 MiB.

When launched with `?lensflow=1`, the web shell does not load or save local browser preferences or shared IndexedDB brush presets. The host owns tenant-scoped persistence and decides when to acknowledge a saved revision. Standalone browser behavior remains unchanged.

The editor receives document bytes and command data, never portal tokens or storage credentials. The host must verify the exact iframe window, origin, protocol and per-workspace channel before accepting messages, and enforce its own upload limits and revision concurrency. The Rust engine does not acquire network or backend storage authority.

This fork does not add model weights or a deployment workflow. Upstream licenses and notices remain applicable. LensFlow pins the reviewed fork commit and packages its own host and third-party notices separately.
