pub mod app;
pub mod models;
pub mod storage;
pub mod sync;

/// Re-exported from `app` (defined there so it resolves in both the lib and the
/// binary crate). The version is surfaced in the UI and baked into the WASM, so
/// a version bump changes the content-hashed asset filename and busts the cache.
pub use app::VERSION;

#[cfg(test)]
mod version_tests {
    use super::VERSION;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn version_is_present_and_dotted() {
        // Proves the compile-time version wiring: non-empty and semver-shaped,
        // so the UI always has a real build identifier to show and to embed in
        // the WASM (the cache-bust guarantee).
        assert!(!VERSION.is_empty(), "CARGO_PKG_VERSION must be set");
        assert!(VERSION.contains('.'), "expected a dotted version, got {VERSION:?}");
    }
}
