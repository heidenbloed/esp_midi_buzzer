use anyhow::{self, Ok};
use esp_idf_svc::{http, log, sys, io::Write};
mod server;

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    log::EspLogger::initialize_default();

    let mut server = server::create_server()?;
    server.fn_handler("/", http::Method::Get, |req| {
        req.into_ok_response()?.write_all("Moin".as_bytes())?;
        Ok(())
    })?;

    core::mem::forget(server);
    Ok(())
}
