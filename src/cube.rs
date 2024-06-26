use ratatui::style::Color;

pub struct Cube {
    pub points: Vec<(f64, f64, f64)>,
    pub color: Color,
}

impl Cube {
    pub fn new(size: f64, color: Color) -> Self {
        let points = vec![
            (-size, -size, size),
            (size, -size, size),
            (size, size, size),
            (-size, size, size),
            (-size, -size, -size),
            (size, -size, -size),
            (size, size, -size),
            (-size, size, -size),
        ];

        Self { points, color }
    }
}

