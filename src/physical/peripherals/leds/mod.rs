use std::thread;
use std::time::Duration;

use esp_idf_hal::gpio::{Output, PinDriver};

pub struct LED {}

impl LED {
    pub fn blink_led_ms(
        led: &mut PinDriver<Output>,
        milliseconds: u64,
        times: u8,
    ) -> anyhow::Result<()> {
        for __i in 0..times {
            led.set_high()?;
            thread::sleep(Duration::from_millis(milliseconds));
            led.set_low()?;
            thread::sleep(Duration::from_millis(milliseconds));
        }

        return Ok(());
    }
}
