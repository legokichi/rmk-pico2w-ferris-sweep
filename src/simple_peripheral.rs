#![no_std]
#![no_main]

use cyw43_pio::PioSpi;
use cyw43::bluetooth::BtDriver;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::{self, Pio};
use embassy_time::{Timer, Duration};
use static_cell::StaticCell;
use trouble_host::prelude::*;
use rand_core::SeedableRng;
use {defmt_rtt as _, embassy_time as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => pio::InterruptHandler<PIO0>;
});

#[embassy_executor::task]
async fn cyw43_task(runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn ble_runner_task(mut runner: Runner<'static, bt_hci::controller::ExternalController<BtDriver<'static>, 10>, DefaultPacketPool>) -> ! {
    defmt::info!("=== BLE RUNNER STARTED ===");
    if let Err(e) = runner.run().await {
        defmt::error!("BLE runner error: {:?}", defmt::Debug2Format(&e));
    }
    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}

#[gatt_service(uuid = "1234")]
struct MyService {
    #[characteristic(uuid = "5678", read, notify)]
    value: u8,
}

#[gatt_server]
struct Server {
    service: MyService,
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    defmt::info!("=== PERIPHERAL START ===");

    let fw = include_bytes!("../cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("../cyw43-firmware/43439A0_clm.bin");
    let btfw = include_bytes!("../cyw43-firmware/43439A0_btfw.bin");

    let pwr = Output::new(p.PIN_23, Level::Low);
    let cs = Output::new(p.PIN_25, Level::High);
    let mut pio = Pio::new(p.PIO0, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        cyw43_pio::DEFAULT_CLOCK_DIVIDER,
        pio.irq0,
        cs,
        p.PIN_24,
        p.PIN_29,
        p.DMA_CH0,
    );

    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());
    let (_net_device, bt_device, mut control, runner) = cyw43::new_with_bluetooth(state, pwr, spi, fw, btfw).await;
    spawner.spawn(cyw43_task(runner)).unwrap();
    control.init(clm).await;
    defmt::info!("=== CYW43 INIT DONE ===");

    let controller: bt_hci::controller::ExternalController<_, 10> = bt_hci::controller::ExternalController::new(bt_device);
    
    static RESOURCES: StaticCell<HostResources<DefaultPacketPool, 1, 8>> = StaticCell::new();
    let resources = RESOURCES.init(HostResources::new());
    
    static STACK: StaticCell<Stack<bt_hci::controller::ExternalController<BtDriver<'static>, 10>, DefaultPacketPool>> = StaticCell::new();
    
    let mut rosc_rng = embassy_rp::clocks::RoscRng;
    let mut rng = rand_chacha::ChaCha12Rng::from_rng(&mut rosc_rng).unwrap();

    let ble_addr = [0xC0, 0xDE, 0x33, 0x44, 0x55, 0x66];
    let stack = STACK.init(trouble_host::new(controller, resources)
        .set_random_address(Address::random(ble_addr))
        .set_random_generator_seed(&mut rng));
        
    let Host { mut peripheral, runner, .. } = stack.build();
    spawner.spawn(ble_runner_task(runner)).unwrap();
    
    let server = Server::new_with_config(GapConfig::Peripheral(PeripheralConfig {
        name: "SimplePeri",
        appearance: &trouble_host::prelude::appearance::human_interface_device::KEYBOARD,
    })).unwrap();

    defmt::info!("=== STARTING ADVERTISING ===");

    let mut advertiser_data = [0; 31];
    AdStructure::encode_slice(
        &[
            AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
            AdStructure::CompleteLocalName(b"SimplePeri"),
            AdStructure::ServiceUuids16(&[[0x34, 0x12]]),
        ],
        &mut advertiser_data[..],
    ).unwrap();

    let mut led_on = false;

    loop {
        led_on = !led_on;
        control.gpio_set(0, led_on).await; 

        let advertiser = peripheral
            .advertise(
                &Default::default(),
                Advertisement::ConnectableScannableUndirected {
                    adv_data: &advertiser_data[..],
                    scan_data: &[],
                },
            )
            .await
            .unwrap();
            
        defmt::info!("=== ADVERTISING... (LED: {}) ===", led_on);
        
        match embassy_time::with_timeout(Duration::from_millis(1000), advertiser.accept()).await {
            Ok(Ok(conn)) => {
                defmt::info!("=== CONNECTED! ===");
                control.gpio_set(0, true).await; 
                let mut conn = conn.with_attribute_server(&server).unwrap();
                
                let gatt_loop = async {
                    loop {
                        match conn.next().await {
                            GattConnectionEvent::Gatt { event } => {
                                defmt::debug!("GATT request");
                                if let Ok(reply) = event.accept() {
                                    let _ = reply.send().await;
                                }
                            }
                            GattConnectionEvent::Disconnected { .. } => break,
                            _ => {}
                        }
                    }
                };

                let notify_loop = async {
                    let mut counter: u8 = 0;
                    loop {
                        Timer::after(Duration::from_secs(1)).await;
                        counter = counter.wrapping_add(1);
                        defmt::info!("SENDING: {}", counter);
                        if let Err(_) = server.service.value.notify(&conn, &counter).await {
                             defmt::error!("Notify failed");
                             break;
                        }
                    }
                };

                // 並行実行
                embassy_futures::select::select(gatt_loop, notify_loop).await;
                defmt::info!("=== DISCONNECTED ===");
            }
            Ok(Err(e)) => {
                defmt::error!("Advertiser accept error: {:?}", defmt::Debug2Format(&e));
            }
            Err(_) => {
                continue;
            }
        }
    }
}
