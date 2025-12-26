#![no_std]
#![no_main]
#![allow(unused)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(feature = "rmk")]
mod rmk_example {
    use core::cell::RefCell;

    use embedded_hal::digital::{InputPin, OutputPin};
    use embedded_hal_async::i2c::I2c;
    use rmk::keymap::KeyMap;
    use rmk_driver_azoteq_iqs5xx::rmk_support::{
        Iqs5xxDevice, Iqs5xxProcessor, Iqs5xxProcessorConfig,
    };
    use rmk_driver_azoteq_iqs5xx::Iqs5xxConfig;

    async fn sample<I2C, RDY, RST, const ROW: usize, const COL: usize, const NUM_LAYER: usize, const NUM_ENCODER: usize>(
        i2c: I2C,
        rdy: RDY,
        rst: RST,
        keymap: &RefCell<KeyMap<'_, ROW, COL, NUM_LAYER, NUM_ENCODER>>,
    ) where
        I2C: I2c,
        RDY: InputPin,
        RST: OutputPin,
    {
        let config = Iqs5xxConfig::default();
        let mut tp = Iqs5xxDevice::new(i2c, rdy, rst, config);
        let mut tp_proc = Iqs5xxProcessor::new(keymap, Iqs5xxProcessorConfig::default());

        let _ = (&mut tp, &mut tp_proc);
        // Wire with RMK run loop, e.g.:
        // rmk::run_devices!((tp) => rmk::EVENT_CHANNEL);
    }
}
