use crate::graph::Graph;
use crate::node::Node;
use sdl2::pixels::Color;

impl Node {
  /// Determines if the given x y cords would collide with the node
  pub fn is_collision(&self, x: i16, y: i16) -> bool {
    let (node_x, node_y) = self.cords;
    let radii = 10;

    node_x - radii < x && x < node_x + radii && node_y - radii < y && y < node_y + radii
  }

  /*
  pub fn get_edges(&self, graph: &Graph) -> Vec<usize> {
    graph
      .edges
      .iter()
      .filter(|edge| edge.nodes_idx.0 == self.index || edge.nodes_idx.1 == self.index)
      .map(|edge| edge.index)
      .collect()
  }
   */

  /// Gets the degree of the node
  pub fn get_degree(&self, graph: &Graph) -> i16 {
    graph
      .edges
      .iter()
      .filter(|edge| edge.nodes_idx.0 == self.index || edge.nodes_idx.1 == self.index)
      .fold(0, |value, edge| (edge.count + value as u16) as i16)
  }

  pub fn get_neighbors(&self, graph: &Graph) -> Vec<usize> {
    graph
      .edges
      .iter()
      .filter(|edge| edge.nodes_idx.0 == self.index || edge.nodes_idx.1 == self.index)
      .map(|edge| {
        if edge.nodes_idx.0 == self.index {
          edge.nodes_idx.1
        } else {
          edge.nodes_idx.0
        }
      })
      .collect()
  }

  pub fn new(cords: (i16, i16), label: String, color: Color, node_idx: usize) -> Node {
    let label = if label.is_empty() {
      node_idx.to_string()
    } else {
      label
    };

    Node {
      color,
      label,
      cords,
      index: node_idx,
      selected: false,
      deleted: false,
    }
  }
}
