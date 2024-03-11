mod server;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let mut server = server::create_server()?;
    server::add_static_handlers(&mut server)?;
    server::add_websocket(&mut server, |cmd| {
        log::info!("Cmd: \"{}\"", cmd);
        buzzing(cmd == "start");
    })?;

    core::mem::forget(server);
    Ok(())
}

fn buzzing(active: bool) {
    if active {
        log::info!("Start buzzing!")
    } else {
        log::info!("Stop buzzing!")
    }
}
