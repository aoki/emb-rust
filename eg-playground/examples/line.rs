use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::*;
use embedded_graphics::style::*;
use embedded_graphics_simulator::*;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(320, 240));

    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("draw a line", &output_settings);

    let style = PrimitiveStyle::with_stroke(Rgb565::GREEN, 1);
    Line::new(Point::new(50, 20), Point::new(270, 220))
        .into_styled(style)
        .draw(&mut display)?;
    Line::new(Point::new(50, 220), Point::new(270, 20))
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 2))
        .draw(&mut display)?;
    Line::new(Point::new(160, 20), Point::new(160, 220))
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::BLUE, 3))
        .draw(&mut display)?;

    window.show_static(&display);

    Ok(())
}
