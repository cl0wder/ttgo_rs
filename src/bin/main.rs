
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output};
use esp_hal::main;
use esp_hal::spi::master::{Config, Spi};
use esp_hal::spi::Mode;
use esp_hal::time::RateExtU32;

use display_interface_spi::SPIInterface;
use embedded_graphics::{
    mono_font::{ascii::FONT_9X18, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    text::Text,
};
use mipidsi::{models::ST7789, Builder};

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut delay = Delay::new();
    let sclk = peripherals.GPIO18;
    let mosi = peripherals.GPIO19;
    let cs = peripherals.GPIO5;
    let dc = Output::new(peripherals.GPIO16, Level::Low);
    let mut rst = Output::new(peripherals.GPIO23, Level::Low);
    let mut backlight = Output::new(peripherals.GPIO4, Level::Low); // Start with backlight off

    // Initialize reset pin high first
    rst.set_high();
    delay.delay_millis(10);

    // FIX: Spi::new now takes (Peripheral, Config)
    let spi_config = Config::default()
        .with_frequency(26_u32.MHz()) // Try lower frequency for stability
        .with_mode(Mode::_0);

    let spi_bus = Spi::new(peripherals.SPI2, spi_config)
        .expect("SPI init failed")
        .with_sck(sclk)
        .with_mosi(mosi);

    // FIX: We must unwrap ExclusiveDevice so 'di' gets a Device, not a Result
    let spi_device = ExclusiveDevice::new(spi_bus, Output::new(cs, Level::High), delay)
        .expect("Failed to create SPI device");

    let di = SPIInterface::new(spi_device, dc);

    // Perform hardware reset
    rst.set_low();
    delay.delay_millis(10);
    rst.set_high();
    delay.delay_millis(120); // Wait for display to power up

    let mut display = Builder::new(ST7789, di)
        .display_size(170, 135)
        .orientation(mipidsi::options::Orientation::new().rotate(mipidsi::options::Rotation::Deg90))
        .invert_colors(mipidsi::options::ColorInversion::Inverted)
        .reset_pin(rst)
        .init(&mut delay)
        .expect("Display init failed");

    delay.delay_millis(100); // Let initialization settle
    backlight.set_high();
    delay.delay_millis(100); // Wait for backlight to turn on
    display.clear(Rgb565::BLACK).unwrap();

    let text_style = MonoTextStyle::new(&FONT_9X18, Rgb565::WHITE);
    Text::new("1234567890123456789012345", Point::new(50, 10), text_style) 
        .draw(&mut display)
        .unwrap();

    loop {
        delay.delay_millis(1000);
    }
}
