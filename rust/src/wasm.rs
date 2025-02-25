#[cfg(target_arch = "wasm32")]
mod wasm {
    use super::*;
    use wasm_bindgen_futures::spawn_local;
    use std::sync::Arc;

    pub fn init_web() {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        let rt = Arc::new(rt);

        std::panic::set_hook(Box::new(|panic_info| {
            eprintln!("Web panic: {:?}", panic_info);
        }));

        rt.spawn(async move {
            if let Err(e) = rt.shutdown_timeout(std::time::Duration::from_secs(60)) {
                eprintln!("Error shutting down runtime: {:?}", e);
            }
        });
    }
}