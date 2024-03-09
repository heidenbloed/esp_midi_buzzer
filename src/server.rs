use anyhow;
use esp_idf_svc::{eventloop, hal::prelude, http::server, nvs, wifi};
use log;

const SSID: &str = env!("WIFI_SSID");
const PASSWORD: &str = env!("WIFI_PASS");
const STACK_SIZE: usize = 10240;

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
