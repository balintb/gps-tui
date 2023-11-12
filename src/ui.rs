use tui::{
    backend::Backend,
    layout::Alignment,
    style::Color,
    widgets::{canvas::*, Block, BorderType, Borders},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    frame.render_widget(
        Canvas::default()
            .block(Block::default().borders(Borders::ALL).title(" GPS "))
            .paint(|ctx| {
                ctx.draw(&Map {
                    color: Color::Green,
                    resolution: MapResolution::High,
                });
                // this is where we are according to gpsd
                ctx.print(app.lon, app.lat, "X");
            })
            .x_bounds([-180.0, 180.0])
            .y_bounds([-90.0, 90.0])
            .block(
                Block::default()
                    .title("Map")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            ),
        frame.size(),
    )
}
