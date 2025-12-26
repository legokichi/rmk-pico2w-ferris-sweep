# rmk-driver-azoteq-iqs5xx

Azoteq IQS5xx (TPS65) trackpad driver for RMK. This crate is `no_std` and provides
an async I2C driver plus optional RMK integration.

## Usage (raw driver)

```rust
#![no_std]
#![no_main]

use embassy_time::{Duration, Timer};
use embedded_hal_async::i2c::I2c;
use rmk_driver_azoteq_iqs5xx::{Iqs5xx, Iqs5xxConfig};

async fn poll<I2C, RDY, RST>(mut i2c: I2C, mut rdy: RDY, mut rst: RST)
where
    I2C: I2c,
    RDY: embedded_hal::digital::InputPin,
    RST: embedded_hal::digital::OutputPin,
{
    let config = Iqs5xxConfig::default();
    let mut dev = Iqs5xx::new(i2c, Some(rdy), Some(rst), config);

    dev.init().await.ok();

    loop {
        if let Ok(Some(report)) = dev.try_read_report().await {
            let event = rmk_driver_azoteq_iqs5xx::Event::from_report(&report);
            // handle event
            let _ = event;
        }
        Timer::after(Duration::from_millis(5)).await;
    }
}
```

## Usage (RMK integration)

Enable the `rmk` feature and wire the device + processor manually in your
`central.rs` / `peripheral.rs` (this crate does not yet integrate with
`keyboard.toml`). Example sketch using embassy-rp types:

```rust
use rmk_driver_azoteq_iqs5xx::rmk_support::{
    Iqs5xxDevice, Iqs5xxProcessor, Iqs5xxProcessorConfig,
};
use rmk_driver_azoteq_iqs5xx::Iqs5xxConfig;

// ... create I2C + RDY/RST pins via your HAL ...

let config = Iqs5xxConfig {
    enable_scroll: true,
    ..Iqs5xxConfig::default()
};

let mut tp = Iqs5xxDevice::new(i2c, rdy_pin, rst_pin, config);
let mut tp_proc = Iqs5xxProcessor::new(&keymap, Iqs5xxProcessorConfig::default());

// Run alongside RMK. Example:
// embassy_futures::join::join(
//     rmk::run_devices!((tp) => rmk::EVENT_CHANNEL),
//     rmk::run_rmk!(/* ... */),
// ).await;
```

Notes:
- `Iqs5xxDevice` emits `Event::Touchpad` for motion/scroll and `Event::Custom` for taps/hold.
- `Iqs5xxProcessor` converts these into `MouseReport` (x/y + wheel/pan) and button states.

## Examples

- `examples/raw_driver.rs` (no_std polling example)
- `examples/rmk_integration.rs` (RMK integration sketch, `--features rmk`)
- `examples/embassy_rp_pico2w.rs` (embassy-rp async I2C example)
