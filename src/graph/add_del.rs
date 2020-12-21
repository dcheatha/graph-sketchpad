use crate::edge::Edge;
use crate::graph::Graph;
use crate::node::Node;
use random_color::RandomColor;
use sdl2::pixels::Color;

impl Graph {
  pub fn add_edge(&mut self, nodes_idx: (usize, usize), color: Color) {
    let mut found_edges = self
      .edges
      .iter_mut()
      .filter(|edge| edge.nodes_idx == nodes_idx);

    if let Some(edge) = found_edges.next() {
      edge.count += 1;
      return;
    }

    let edge = Edge::new(self.edges.len(), nodes_idx, 1, color, false);

    self.edges.push(edge);

    self.crazy_stupid_k_coloring_look_away_professor();
  }

  pub fn del_edge(&mut self, nodes_idx: (usize, usize)) {
    let mut found_edges = self
      .edges
      .iter_mut()
      .filter(|edge| edge.nodes_idx == nodes_idx || edge.nodes_idx == (nodes_idx.1, nodes_idx.0));

    if let Some(edge) = found_edges.next() {
      if edge.count == 0 {
        return;
      }

      edge.count -= 1;
      return;
    }
  }

  pub fn add_node(&mut self, cords: (i16, i16)) {
    let [r, g, b] = RandomColor::new().to_rgb_array();

    let color = Color {
      r: r as u8,
      g: g as u8,
      b: b as u8,
      a: 255,
    };

    self
      .nodes
      .push(Node::new(cords, String::new(), color, self.nodes.len()));

    self.crazy_stupid_k_coloring_look_away_professor();
  }

  pub fn del_node(&mut self, node_idx: usize) {
    if let Some(node) = self.nodes.get_mut(node_idx) {
      let found_edges = self
        .edges
        .iter_mut()
        .filter(|edge| edge.nodes_idx.0 == node_idx || edge.nodes_idx.1 == node_idx);

      for edge in found_edges {
        edge.count = 0;
      }

      node.deleted = true;
    }
  }
}
