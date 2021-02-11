mod util;

use crate::graph::Graph;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

pub struct Node {
  pub color: sdl2::pixels::Color,
  pub label: String,
  pub cords: (i16, i16),
  pub selected: bool,
  pub deleted: bool,
  pub index: usize,
}

impl Node {
  pub fn draw(&self, graph: &Graph, canvas: &WindowCanvas) {
    if self.deleted {
      return;
    }

    // draw the selection circle
    if self.selected {
      canvas
        .filled_circle(self.cords.0, self.cords.1, 15, self.color)
        .unwrap();
      canvas
        .aa_circle(self.cords.0, self.cords.1, 15, self.color)
        .unwrap();
      canvas
        .filled_circle(self.cords.0, self.cords.1, 13, Color::BLACK)
        .unwrap();
      canvas
        .aa_circle(self.cords.0, self.cords.1, 13, Color::BLACK)
        .unwrap();
    }

    let degree = self.get_degree(graph);
    if degree > 1 {
      canvas
        .string(
          self.cords.0 - 20,
          self.cords.1 - 20,
          &*format!("{}", degree),
          self.color,
        )
        .unwrap();
    }

    // draw the normal circle
    canvas
      .filled_circle(self.cords.0, self.cords.1, 10, self.color)
      .unwrap();
    canvas
      .aa_circle(self.cords.0, self.cords.1, 10, self.color)
      .unwrap();

    // draw any defined labels
    canvas
      .string(
        self.cords.0 - (self.label.len() * 4) as i16,
        self.cords.1 + 15,
        self.label.as_str(),
        self.color,
      )
      .unwrap();
  }
}
