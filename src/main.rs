use esp_idf_svc::{hal::prelude, timer};
use std::sync::{Arc, Mutex};
use core::time;

mod buzzer;
mod server;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = prelude::Peripherals::take()?;
    let buzzing_active = Arc::new(Mutex::new(false));

    let mut server = server::create_server(peripherals.modem)?;
    server::add_static_handlers(&mut server)?;
    let buzzing_active_ptr = Arc::clone(&buzzing_active);
    server::add_websocket(&mut server, move |cmd| {
        *buzzing_active_ptr
            .lock()
            .expect("Could not lock buzzing_active_ptr.") = cmd == "start\0";
    })?;
    core::mem::forget(server);

    let mut buzzer = buzzer::Buzzer::new(peripherals.pins, peripherals.rmt)?;
    let timer_service = timer::EspTaskTimerService::new()?;
    let timer = timer_service.timer(move || {
        log::info!("Timer triggered");
        if *buzzing_active
            .lock()
            .expect("Could not lock buzzing_active.")
        {
            buzzer
                .play_note(440, buzzer::Duration::from_millis(10))
                .expect("Could not play note.");
        }
    })?;
    timer.every(time::Duration::from_secs(1))?;

    Ok(())
}
