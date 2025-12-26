#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_rp::i2c::{Config as I2cConfig, I2c, InterruptHandler};
use embassy_rp::peripherals::I2C0;
use embassy_time::{Duration, Timer};
use rmk_driver_azoteq_iqs5xx::{Event, Iqs5xx, Iqs5xxConfig};

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

bind_interrupts!(struct Irqs {
    I2C0_IRQ => InterruptHandler<I2C0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut i2c_config = I2cConfig::default();
    i2c_config.frequency = 400_000;

    let i2c = I2c::new_async(p.I2C0, p.PIN_1, p.PIN_0, Irqs, i2c_config);
    let rdy = Input::new(p.PIN_3, Pull::Down);
    let rst = Output::new(p.PIN_2, Level::High);

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
