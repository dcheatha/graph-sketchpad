mod add_del;
mod util;

use sdl2::render::WindowCanvas;

use crate::edge::Edge;
use crate::node::Node;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;

pub struct Graph {
  pub nodes: Vec<Node>,
  pub edges: Vec<Edge>,
}

impl Graph {
  pub fn draw(&self, canvas: &WindowCanvas) {
    self.edges.iter().for_each(|edge| edge.draw(self, &canvas));
    self.nodes.iter().for_each(|node| node.draw(self, &canvas));

    let num_nodes = self.nodes.iter().filter(|node| !node.deleted).count();
    let num_edges = self.edges.iter().fold(0, |value, edge| value + edge.count);
    let degree = self.get_degree();
    let partite = self.get_partite();

    canvas
      .string(
        20,
        20,
        &*format!(
          "{vertices} vertices, {edges} edges, {degree} degree, {partite}-partite",
          vertices = num_nodes,
          edges = num_edges,
          degree = degree,
          partite = partite,
        ),
        Color::WHITE,
      )
      .unwrap();
  }
}
