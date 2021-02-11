use sdl2::gfx::primitives::DrawRenderer;
use sdl2::render::WindowCanvas;

use crate::graph::Graph;
use sdl2::pixels::Color;

pub struct Edge {
  pub color: sdl2::pixels::Color,
  pub nodes_idx: (usize, usize),
  pub count: u16,
  pub directed: bool,
  pub index: usize,
}

impl Edge {
  pub fn draw(&self, graph: &Graph, canvas: &WindowCanvas) {
    let node_a = &graph.nodes[self.nodes_idx.0];
    let node_b = &graph.nodes[self.nodes_idx.1];

    let (x_a, y_a) = node_a.cords;
    let (x_b, y_b) = node_b.cords;

    let (mid_x, mid_y) = ((x_a + x_b) / 2, (y_a + y_b) / 2);

    if self.count == 0 {
      return;
    }

    canvas.aa_line(x_a, y_a, x_b, y_b, self.color).unwrap();

    if self.count > 1 {
      canvas
        .string(
          mid_x - 3,
          mid_y + 15,
          &*format!("{}", self.count),
          self.color,
        )
        .unwrap();
    }
  }

  pub fn new(
    index: usize,
    nodes_idx: (usize, usize),
    count: u16,
    color: Color,
    directed: bool,
  ) -> Edge {
    Edge {
      index,
      color,
      nodes_idx,
      count,
      directed,
    }
  }
}
