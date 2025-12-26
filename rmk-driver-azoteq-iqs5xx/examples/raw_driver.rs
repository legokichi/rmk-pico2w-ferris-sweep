#![no_std]
#![no_main]
#![allow(unused)]

use embassy_time::{Duration, Timer};
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal_async::i2c::I2c;
use rmk_driver_azoteq_iqs5xx::{Event, Iqs5xx, Iqs5xxConfig};

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

async fn sample<I2C, RDY, RST>(mut i2c: I2C, rdy: RDY, rst: RST)
where
    I2C: I2c,
    RDY: InputPin,
    RST: OutputPin,
{
    let config = Iqs5xxConfig::default();
    let mut dev = Iqs5xx::new(i2c, Some(rdy), Some(rst), config);

    let _ = dev.init().await;

    loop {
        if let Ok(Some(report)) = dev.try_read_report().await {
            let _event = Event::from_report(&report);
            // handle event
        }
        Timer::after(Duration::from_millis(5)).await;
    }
}
