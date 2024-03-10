use core::cmp::Ordering;
use esp_idf_svc::{
    http::Method,
    io::Write,
    sys::{EspError, ESP_ERR_INVALID_SIZE},
    systime::EspSystemTime,
    ws::FrameType,
};
use std::str;
mod server;

static INDEX_HTML: &str = include_str!("../buzzer_frontend/dist/index.html");
static INDEX_CSS: &str = include_str!("../buzzer_frontend/dist/assets/index.css");
static INDEX_JS: &str = include_str!("../buzzer_frontend/dist/assets/index.js");

// Max payload length
const MAX_LEN: usize = 8;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let mut server = server::create_server()?;

    server.fn_handler("/", Method::Get, |req| {
        req.into_ok_response()?
            .write_all(INDEX_HTML.as_bytes())
            .map(|_| ())
    })?;

    server.fn_handler("/assets/index.css", Method::Get, |req| {
        req.into_response(200, Some("OK"), &[("Content-Type", "text/css")])?
            .write_all(INDEX_CSS.as_bytes())
            .map(|_| ())
    })?;

    server.fn_handler("/assets/index.js", Method::Get, |req| {
        req.into_response(200, Some("OK"), &[("Content-Type", "text/javascript")])?
            .write_all(INDEX_JS.as_bytes())
            .map(|_| ())
    })?;

    core::mem::forget(server);
    Ok(())
}
