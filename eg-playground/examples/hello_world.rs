use embedded_graphics::{
    drawable::Drawable,
    fonts::*,
    pixelcolor::{Rgb565, RgbColor},
    prelude::*,
    style::*,
};
use embedded_graphics_simulator::*;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(320, 240));
    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("hello world", &output_settings);

    Text::new("hello world!", Point::new(0, 0))
        .into_styled(TextStyle::new(Font12x16, Rgb565::GREEN))
        .draw(&mut display)?;
    Text::new("hello world!", Point::new(0, 16))
        .into_styled(TextStyle::new(Font6x12, Rgb565::MAGENTA))
        .draw(&mut display)?;

    window.show_static(&display);

    Ok(())
}
