use ratatui::Frame;
use ratatui::layout::{Layout, Constraint, Rect};
use ratatui::widgets::{Block, Borders, List, ListItem};
use ratatui::style::{Style, Color};
use ratatui::widgets::canvas::Canvas;

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(frame.size());

    let right_chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    draw_cube(frame, app, chunks[0]);
    draw_actions(frame, app, right_chunks[0]);
}

fn draw_cube(frame: &mut Frame, app: &App, area: Rect) {
    frame.render_widget(
        Canvas::default()
            .block(Block::default().borders(Borders::ALL).title("Cube"))
            .paint(|ctx| {
                let coords = app.cube.points.iter().map(|&(x, y, z)| {
                    let (rotated_x, rotated_y, rotated_z) = rotate_point(x, y, z, app.rotation);
                    isometric_projection(rotated_x, rotated_y, rotated_z)
                }).collect::<Vec<_>>();

                ctx.draw(&ratatui::widgets::canvas::Points {
                    coords: &coords,
                    color: app.cube.color,
                });

                draw_cube_edges(ctx, &coords, app.cube.color);
            })
            .x_bounds([-180.0, 180.0])
            .y_bounds([-90.0, 90.0]),
        area,
    );
}

fn draw_actions(frame: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app.last_actions.iter()
        .map(|action| ListItem::new(action.as_str()))
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Last Actions"))
        .style(Style::default().fg(Color::White));

    frame.render_widget(list, area);
}


fn rotate_point(x: f64, y: f64, z: f64, (rx, ry, rz): (f64, f64, f64)) -> (f64, f64, f64) {
    // X-axis
    let (y, z) = (
        y * rx.cos() - z * rx.sin(),
        y * rx.sin() + z * rx.cos()
    );

    // Y-axis
    let (x, z) = (
        z * ry.sin() + x * ry.cos(),
        z * ry.cos() - x * ry.sin()
    );

    // Z-axis
    let (x, y) = (
        x * rz.cos() - y * rz.sin(),
        x * rz.sin() + y * rz.cos()
    );

    (x, y, z)
}

fn isometric_projection(x: f64, y: f64, z: f64) -> (f64, f64) {
    let angle = std::f64::consts::FRAC_PI_6;
    let iso_x = (x - z) * angle.cos();
    let iso_y = y + (x + z) * angle.sin() / 2.0;
    (iso_x, iso_y)
}

fn draw_cube_edges(ctx: &mut ratatui::widgets::canvas::Context, coords: &[(f64, f64)], color: Color) {
    let edges = [
        (0, 1), (1, 2), (2, 3), (3, 0),
        (4, 5), (5, 6), (6, 7), (7, 4),
        (0, 4), (1, 5), (2, 6), (3, 7),
    ];

    for (start, end) in edges.iter() {
        let (x1, y1) = coords[*start];
        let (x2, y2) = coords[*end];
        ctx.draw(&ratatui::widgets::canvas::Line {
            x1, y1, x2, y2, color
        });
    }
}

