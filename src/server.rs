use anyhow;
use esp_idf_svc::{
    eventloop, hal::prelude, http::server, http::Method, io::Write, nvs, sys, wifi, ws,
};
use log;
use std::str;

const SSID: &str = env!("WIFI_SSID");
const PASSWORD: &str = env!("WIFI_PASS");
const STACK_SIZE: usize = 10240;
const MAX_LEN: usize = 8;

static INDEX_HTML: &str = include_str!("../buzzer_frontend/dist/index.html");
static INDEX_CSS: &str = include_str!("../buzzer_frontend/dist/assets/index.css");
static INDEX_JS: &str = include_str!("../buzzer_frontend/dist/assets/index.js");

pub fn create_server() -> anyhow::Result<server::EspHttpServer<'static>> {
    let peripherals = prelude::Peripherals::take()?;
    let sys_loop = eventloop::EspSystemEventLoop::take()?;
    let nvs = nvs::EspDefaultNvsPartition::take()?;

    let wifi_driver = wifi::EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs))?;
    let mut wifi = wifi::BlockingWifi::wrap(wifi_driver, sys_loop)?;

    let wifi_configuration = wifi::Configuration::Client(wifi::ClientConfiguration {
        ssid: SSID.try_into().unwrap(),
        password: PASSWORD.try_into().unwrap(),
        ..Default::default()
    });
    wifi.set_configuration(&wifi_configuration)?;
    wifi.start()?;
    wifi.connect()?;
    wifi.wait_netif_up()?;

    log::info!("Logged in to WiFi.");

    log::info!("IP info: {:?}", wifi.wifi().sta_netif().get_ip_info()?);

    let server_configuration = server::Configuration {
        stack_size: STACK_SIZE,
        ..Default::default()
    };

    // Keep wifi running beyond when this function returns (forever)
    // Do not call this if you ever want to stop or access it later.
    // Otherwise it should be returned from this function and kept somewhere
    // so it does not go out of scope.
    // https://doc.rust-lang.org/stable/core/mem/fn.forget.html
    core::mem::forget(wifi);

    Ok(server::EspHttpServer::new(&server_configuration)?)
}

pub fn add_static_handlers(server: &mut server::EspHttpServer) -> anyhow::Result<()> {
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

    server.fn_handler(
        "/assets/index.js",
        Method::Get,
        |req: server::Request<&mut server::EspHttpConnection<'_>>| {
            req.into_response(200, Some("OK"), &[("Content-Type", "text/javascript")])?
                .write_all(INDEX_JS.as_bytes())
                .map(|_| ())
        },
    )?;

    Ok(())
}

pub fn add_websocket<'a, C>(server: &mut server::EspHttpServer<'a>, callback: C) -> anyhow::Result<()>
where
    C: Fn(&str) + Send + Sync + 'a,
{
    server.ws_handler("/ws", move |ws_connection| {
        match ws_connection {
            server::ws::EspHttpWsConnection::New(_, _) => {
                log::info!("New WebSocket session ({})", ws_connection.session());
                return Ok::<(), sys::EspError>(());
            }
            server::ws::EspHttpWsConnection::Closed(_) => {
                log::info!("Closed WebSocket session ({})", ws_connection.session());
                return Ok::<(), sys::EspError>(());
            }
            server::ws::EspHttpWsConnection::Receiving(_, _, _) => {
                log::info!(
                    "Receiving at WebSocket session ({})",
                    ws_connection.session()
                );
                let (_frame_type, len) = ws_connection.recv(&mut [])?;
                if len > MAX_LEN {
                    ws_connection.send(ws::FrameType::Text(false), "Request too big".as_bytes())?;
                    ws_connection.send(ws::FrameType::Close, &[])?;
                    return Err(sys::EspError::from_infallible::<
                        { sys::ESP_ERR_INVALID_SIZE },
                    >());
                }
                log::info!("WebSocket frame received with length {}.", len);

                let mut buf = [0; MAX_LEN]; // Small digit buffer can go on the stack
                ws_connection.recv(buf.as_mut())?;
                let Ok(recived_message) = str::from_utf8(&buf[..len]) else {
                    ws_connection.send(ws::FrameType::Text(false), "[UTF-8 Error]".as_bytes())?;
                    return Ok(());
                };
                log::info!(
                    "Received WebSocket text frame with content \"{}\".",
                    recived_message
                );

                callback(&recived_message);

                return Ok(());
            }
        }
    })?;

    Ok(())
}
