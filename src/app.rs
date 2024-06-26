use std::f64::consts::PI;
use ratatui::style::Color;

use crate::cube::Cube;

pub struct App {
    pub cube: Cube,
    pub rotation: (f64, f64, f64),
    pub last_actions: Vec<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            cube: Cube::new(50.0, Color::Red),
            rotation: (10.0, 10.0, 10.0),
            last_actions: Vec::new(),
        }
    }

    pub fn rotate(&mut self, axis: &str, angle: f64) {
        let (x, y, z) = &mut self.rotation;
        let target = match axis {
            "x" => x,
            "y" => y,
            "z" => z,
            _ => return,
        };

        *target += angle;
        *target %= 2.0 * PI;
        if *target < 0.0 {
            *target += 2.0 * PI;
        }

        self.last_actions.push(format!("Rotated {} radians in {} axis", angle, axis));
        if self.last_actions.len() > 5 {
            self.last_actions.remove(0);
        }
    }
}
