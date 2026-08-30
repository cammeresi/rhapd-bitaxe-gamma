//! SSD1306 OLED display.

use defmt::warn;
use embedded_graphics::{
    mono_font::{ascii::FONT_8X13, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use esp_hal::{i2c::master::I2c, Blocking};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

/// Shows two lines of text, then hands the I2C bus back.
pub fn show(i2c: I2c<'static, Blocking>, line1: &str, line2: &str) -> I2c<'static, Blocking> {
    let mut display = Ssd1306::new(
        I2CDisplayInterface::new(i2c),
        DisplaySize128x32,
        DisplayRotation::Rotate180,
    )
    .into_buffered_graphics_mode();
    let font = MonoTextStyle::new(&FONT_8X13, BinaryColor::On);
    let centered = TextStyleBuilder::new()
        .alignment(Alignment::Center)
        .baseline(Baseline::Top)
        .build();
    // Two 13-pixel lines in 32 pixels, with 3 above, between, and below.
    let drawn = display.init().and_then(|()| {
        Text::with_text_style(line1, Point::new(64, 3), font, centered)
            .draw(&mut display)
            .ok();
        Text::with_text_style(line2, Point::new(64, 16), font, centered)
            .draw(&mut display)
            .ok();
        display.flush()
    });
    if drawn.is_err() {
        warn!("display: not updated");
    }
    display.release().release()
}
