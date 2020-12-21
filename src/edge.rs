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
  /*
  fn rotate_about_pivot(&self, (pivot_x, pivot_y): (i16, i16), (cords_x, cords_y): (i16, i16), angle: f32) -> (i16, i16) {
    let (angle_x, angle_y) = (angle.cos(), angle.sin());

    let (pivot_x, pivot_y) = (pivot_x as f32, pivot_y as f32);
    let (cords_x, cords_y) = (cords_x as f32, cords_y as f32);

    let mut cords_x = cords_x - pivot_x as f32;
    let mut cords_y = cords_y - pivot_y as f32;

    cords_x = cords_x * angle_x - cords_y * angle_y + pivot_x;
    cords_y = cords_x * angle_y + cords_y * angle_x + pivot_y;

    (cords_x as i16, cords_y as i16)
  }

  fn draw_triangle(&self, (pivot_x, pivot_y): (i16, i16), angle: f32, canvas: &WindowCanvas) {
    let size = 8;

    let mut top = (pivot_x, pivot_y + size);
    let mut bottom_left = (pivot_x + size, pivot_y);
    let mut bottom_right = (pivot_x - size, pivot_y);

    top = self.rotate_about_pivot((pivot_x, pivot_y), top, angle);
    bottom_left = self.rotate_about_pivot((pivot_x, pivot_y), bottom_left, angle);
    bottom_right = self.rotate_about_pivot((pivot_x, pivot_y), bottom_right, angle);

    println!("Drawing triangle at {:?}, {:?}, {:?} at angle {:?}", top, bottom_left, bottom_right, angle);


    canvas.filled_trigon(top.0, top.1, bottom_left.0, bottom_left.1, bottom_right.0, bottom_right.1, self.color).unwrap();
    canvas.aa_trigon(top.0, top.1, bottom_left.0, bottom_left.1, bottom_right.0, bottom_right.1, self.color).unwrap();
  }
  */

  pub fn draw(&self, graph: &Graph, canvas: &WindowCanvas) {
    let (x_a, y_a) = graph.nodes[self.nodes_idx.0].cords;
    let (x_b, y_b) = graph.nodes[self.nodes_idx.1].cords;

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
