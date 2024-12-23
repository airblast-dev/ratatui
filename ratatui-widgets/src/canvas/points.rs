use ratatui_core::style::Color;

use crate::canvas::{Painter, Shape};

/// A group of points with a given color
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Points<'a> {
    /// List of points to draw
    pub coords: &'a [(f64, f64)],
    /// Color of the points
    pub color: Color,
}

impl Shape for Points<'_> {
    fn draw(&self, painter: &mut Painter) {
        for (x, y) in self.coords {
            if let Some((x, y)) = painter.get_point(*x, *y) {
                painter.paint(x, y, self.color);
            }
        }
    }
}
