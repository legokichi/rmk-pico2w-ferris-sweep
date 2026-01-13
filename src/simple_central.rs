#![no_std]
#![no_main]

use cyw43_pio::PioSpi;
use cyw43::bluetooth::BtDriver;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::{self, Pio};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
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
    if let Err(e) = runner.run_with_handler(&MyScanHandler).await {
        defmt::error!("BLE runner error: {:?}", defmt::Debug2Format(&e));
    }
    loop {
        embassy_time::Timer::after_secs(1).await;
    }
}

static FOUND_SIGNAL: Signal<CriticalSectionRawMutex, Address> = Signal::new();

struct MyScanHandler;

impl EventHandler for MyScanHandler {
    fn on_adv_reports(&self, mut it: LeAdvReportsIter<'_>) {
         while let Some(Ok(report)) = it.next() {
             let mut data = report.data;
             let mut found = false;
             
             while data.len() > 1 {
                 let len = data[0] as usize;
                 if len == 0 { break; }
                 if len + 1 > data.len() { break; }
                 
                 let type_ = data[1];
                 let value = &data[2..1+len];
                 
                 if type_ == 0x09 { // Complete Local Name
                     if value == b"SimplePeri" {
                         found = true;
                     }
                 }
                 
                 data = &data[1+len..];
             }
             
             if found {
                 let addr = Address { kind: report.addr_kind, addr: report.addr };
                 defmt::info!("!!! Found SimplePeri !!!");
                 FOUND_SIGNAL.signal(addr);
             }
         }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    defmt::info!("=== MAIN START ===");

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

    embassy_time::Timer::after_millis(500).await;

    let controller: bt_hci::controller::ExternalController<_, 10> = bt_hci::controller::ExternalController::new(bt_device);
    
    static RESOURCES: StaticCell<HostResources<DefaultPacketPool, 1, 8>> = StaticCell::new();
    let resources = RESOURCES.init(HostResources::new());
    
    static STACK: StaticCell<Stack<bt_hci::controller::ExternalController<BtDriver<'static>, 10>, DefaultPacketPool>> = StaticCell::new();
    
    let mut rosc_rng = embassy_rp::clocks::RoscRng;
    let mut rng = rand_chacha::ChaCha12Rng::from_rng(&mut rosc_rng).unwrap();

    let ble_addr = [0xC1, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];
    let stack = STACK.init(trouble_host::new(controller, resources)
        .set_random_address(Address::random(ble_addr))
        .set_random_generator_seed(&mut rng));
    defmt::info!("=== STACK INITED ===");
        
    let Host { central, runner, .. } = stack.build();
    defmt::info!("=== HOST BUILT ===");

    spawner.spawn(ble_runner_task(runner)).unwrap();
    
    let mut scanner = Scanner::new(central);
    let scan_config = ScanConfig {
        active: true,
        filter_accept_list: &[],
        ..Default::default()
    };
    
    defmt::info!("=== STARTING SCAN ===");
    let _session = scanner.scan(&scan_config).await.unwrap();

    let addr = FOUND_SIGNAL.wait().await;
    defmt::info!("=== FOUND TARGET, CONNECTING ===");
    
    drop(_session);
    drop(scanner); 

    let Host { mut central, .. } = stack.build();
    
    let config = ConnectConfig {
        connect_params: ConnectParams::default(),
        scan_config: ScanConfig {
            filter_accept_list: &[(addr.kind, &addr.addr)],
            ..Default::default()
        },
    };

    match central.connect(&config).await {
         Ok(conn) => {
             defmt::info!("=== CONNECTED! ===");
             let client = GattClient::<_, _, 10>::new(stack, &conn).await.unwrap();
             
             let discovery_and_logic = async {
                 let uuid1234 = Uuid::Uuid16([0x34, 0x12]);
                 let services = client.services_by_uuid(&uuid1234).await.unwrap();
                 
                 if let Some(service) = services.first() {
                     defmt::info!("Found Service 0x1234");
                     
                     let uuid5678 = Uuid::Uuid16([0x78, 0x56]);
                     match client.characteristic_by_uuid::<u8>(service, &uuid5678).await {
                         Ok(char) => {
                             defmt::info!("Found Characteristic 0x5678, Subscribing...");
                             match client.subscribe(&char, false).await {
                                  Ok(mut listener) => {
                                      defmt::info!("Subscribed! Waiting for notifications...");
                                      loop {
                                          let data = listener.next().await;
                                          defmt::info!("DATA: {:?}", data.as_ref());
                                      }
                                  }
                                  Err(_) => defmt::error!("Subscribe failed"),
                             }
                         }
                         Err(_) => defmt::error!("Char not found"),
                     }
                 } else {
                     defmt::error!("Service not found");
                 }
             };

             // client.task() を並行実行
             embassy_futures::select::select(client.task(), discovery_and_logic).await;
         }
         Err(_) => {
             defmt::error!("Connect failed");
         }
    }
}
