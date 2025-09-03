#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use defmt::info;
use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_hal::time::{Duration, Instant};
use esp_println as _;

use esp_hal::time::Rate;
use rtc_hal::datetime::DateTime;

use crate::app::DemoApp;

mod app;

#[panic_handler]
fn panic(err: &core::panic::PanicInfo) -> ! {
    loop {
        defmt::error!("Error:{:?}", err);
    }
}
extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // generator version: 0.4.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 72 * 1024);

    let i2c_bus = esp_hal::i2c::master::I2c::new(
        peripherals.I2C0,
        esp_hal::i2c::master::Config::default().with_frequency(Rate::from_khz(400)),
    )
    .unwrap()
    .with_scl(peripherals.GPIO22)
    .with_sda(peripherals.GPIO21);

    let dt = DateTime::new(2025, 9, 2, 23, 41, 30).unwrap();

    #[cfg(feature = "ds3231")]
    let rtc = ds3231_rtc::Ds3231::new(i2c_bus);

    #[cfg(not(feature = "ds3231"))]
    let rtc = ds1307_rtc::Ds1307::new(i2c_bus);

    let mut app = DemoApp::new(rtc);
    info!("setting datetime");
    app.set_datetime(&dt).unwrap();

    info!("starting square wave");
    app.start_square_wave().unwrap();

    let delay_start = Instant::now();
    while delay_start.elapsed() < Duration::from_secs(60) {}
    info!("stopping square wave");
    app.stop_square_wave().unwrap();

    loop {
        info!("----------");
        info!("getting datetime");
        info!("----------");
        if let Err(e) = app.print_current_time() {
            info!("RTC Error: {}", e);
        }
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_minutes(1) {}
    }
}
