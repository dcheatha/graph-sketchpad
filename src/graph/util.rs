use crate::graph::Graph;
use crate::node::Node;
use rand::seq::IteratorRandom;
use random_color::RandomColor;
use sdl2::pixels::Color;
use std::collections::HashSet;

impl Graph {
  /// Gets the degree of a graph
  pub fn get_degree(&self) -> i16 {
    self
      .nodes
      .iter()
      .map(|node| node.get_degree(self))
      .max()
      .unwrap_or_default()
  }

  pub fn get_partite(&self) -> i16 {
    let mut uniq_colors: HashSet<Color> = HashSet::new();

    for node in &self.nodes {
      if !node.deleted {
        uniq_colors.insert(node.color);
      }
    }

    uniq_colors.iter().count() as i16
  }

  /// Determines if a graph is k-colored
  /// NOTE: This does not calc the k-value. Ignore for now.
  pub fn is_k_colored(&self) -> bool {
    if self.nodes.len() == 1 {
      return true;
    }

    if self.get_partite() > self.get_degree() + 1 {
      return false;
    }

    for node in &self.nodes {
      let neighbors = node.get_neighbors(self);
      let neighbors = neighbors.iter().map(|idx| &self.nodes[*idx]);

      for neighbor in neighbors {
        if node.color == neighbor.color || node.color == Color::WHITE {
          return false;
        }
      }
    }

    true
  }

  fn generate_k_colors(&self, max_colors: i16) -> Vec<Color> {
    let mut colors = vec![[255, 0, 0], [0, 0, 255], [0, 255, 0], [255, 255, 0]];

    let mut uniq_colors: HashSet<Color> = HashSet::new();
    for node in &self.nodes {
      if !node.deleted {
        uniq_colors.insert(node.color);
      }
    }

    for color in uniq_colors {
      let (r, g, b) = color.rgb();
      let color = [r as u32, g as u32, b as u32];

      if !colors.contains(&color) {
        colors.push(color);
      }
    }


    while colors.len() < max_colors as usize {
      colors.push(RandomColor::new().to_rgb_array());
    }

    while colors.len() > max_colors as usize {
      colors.pop();
    }

    colors
      .iter()
      .map(|[r, g, b]| Color {
        r: *r as u8,
        g: *g as u8,
        b: *b as u8,
        a: 255,
      })
      .into_iter()
      .collect()
  }

  pub fn k_color_painter_rec(
    &mut self,
    colors: &Vec<Color>,
    visited: &mut Vec<usize>,
    node_id: usize,
  ) {

    let node = &self.nodes[node_id];

    if node.deleted || visited.contains(&node_id) {
      return;
    }

    visited.push(node.index);

    for neighbor_id in node.get_neighbors(self) {
      self.k_color_painter_rec(colors, visited, neighbor_id);
    }

    let node = &self.nodes[node_id];

    // get and insert all unique colors
    let mut neighbor_colors: HashSet<Color> = HashSet::new();
    let neighbors = node.get_neighbors(self);
    let neighbors = neighbors
      .iter()
      .map(|id| &self.nodes[*id])
      .filter(|node| node.deleted == false);

    for node in neighbors {
      neighbor_colors.insert(node.color);
    }

    // now we need a color none of the neighbors have...
    let leftover_colors = colors
      .iter()
      .filter(|color| !neighbor_colors.contains(*color));

    let node = &mut self.nodes[node_id];
    match leftover_colors.choose(&mut rand::thread_rng()) {
      None => {
        node.color = Color::WHITE;
      }
      Some(color) => {
        node.color = *color;
      }
    }
  }

  pub fn k_color_painter(&mut self) {
    let max_colors = self.get_degree() + 1;

    let mut loop_count = 0;
    while !self.is_k_colored() && loop_count < 100 {
      loop_count += 1;

      for node in &mut self.nodes {
        node.color = Color::RED;
      }

      for colors_len in 1..=max_colors {
        let colors = self.generate_k_colors(colors_len);
        println!("Generated {} colors to draw with", colors.len());

        let mut random = rand::thread_rng();
        for _rounds in 1..=1000 {
          let node = self.nodes.iter().choose(&mut random).unwrap();
          let mut visited_nodes = vec![];
          self.k_color_painter_rec(&colors, &mut visited_nodes, node.index.clone());

          if self.is_k_colored() {
            return;
          }
        }
      }
    }
  }
}
