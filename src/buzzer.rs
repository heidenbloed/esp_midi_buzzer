pub use core::time::Duration;
use esp_idf_svc::hal::{
    gpio,
    rmt::{self, config::TransmitConfig, TxRmtDriver},
    units::Hertz,
};

pub struct Buzzer {
    tx_rmt_driver: TxRmtDriver<'static>,
}

impl Buzzer {
    pub fn new(pins: gpio::Pins, rmt: rmt::RMT) -> anyhow::Result<Self> {
        let buzzer_pin = pins.gpio17;
        let channel = rmt.channel0;
        let config = TransmitConfig::new();
        let tx_rmt_driver: TxRmtDriver<'static> = TxRmtDriver::new(channel, buzzer_pin, &config)?;

        Ok(Self { tx_rmt_driver })
    }

    pub fn play_note(&mut self, freq: u16, duration: Duration) -> anyhow::Result<()> {
        let ticks_hz: Hertz = self.tx_rmt_driver.counter_clock()?;
        let dur_ms = duration.as_millis();
        let cycles_per_sec = freq; // pitch
        let ticks_per_cycle = ticks_hz.0 as u128 / cycles_per_sec as u128;
        let ticks_per_half = (ticks_per_cycle / 2_u128) as u16;
        let ticks = rmt::PulseTicks::new(ticks_per_half)?;
        let cycles = (cycles_per_sec as u128 * dur_ms / 1000_u128) as u32;

        let p1 = rmt::Pulse::new(rmt::PinState::High, ticks);
        let p2 = rmt::Pulse::new(rmt::PinState::Low, ticks);

        let mut signal = rmt::VariableLengthSignal::with_capacity(cycles as usize);
        for _ in 0..cycles {
            signal.push([&p1, &p2])?;
        }

        self.tx_rmt_driver.start_blocking(&signal)?;
        Ok(())
    }
}
