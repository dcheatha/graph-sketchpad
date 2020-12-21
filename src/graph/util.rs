use crate::graph::Graph;
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
      uniq_colors.insert(node.color);
    }

    uniq_colors.iter().count() as i16
  }

  /// Determines if a graph is k-colored
  /// NOTE: This does not calc the k-value. Ignore for now.
  pub fn is_k_colored(&self) -> bool {
    if self.nodes.len() == 1 {
      return true;
    }

    for node in &self.nodes {
      let neighbors = node.get_neighbors(self);
      let neighbors = neighbors.iter().map(|idx| &self.nodes[*idx]);

      for neighbor in neighbors {
        if node.color == neighbor.color {
          return false;
        }
      }
    }

    true
  }

  pub fn crazy_stupid_k_coloring_look_away_professor(&mut self) {
    // start with red, blue, green yellow...
    let mut colors = vec![[255, 0, 0], [0, 0, 255], [0, 255, 0], [255, 255, 0]];

    // X(H) <= Delta(H) + 1, so:
    let k_number = self.get_degree() + 1;

    println!("k={}", k_number);

    // Add more colors until we hit the amount we need:
    while colors.len() < k_number as usize {
      colors.push(RandomColor::new().to_rgb_array());
    }

    while colors.len() > k_number as usize {
      colors.pop();
    }

    // Now that we have the RGB colors, map them to the right structs:
    let colors: Vec<Color> = colors
      .iter()
      .map(|[r, g, b]| Color {
        r: *r as u8,
        g: *g as u8,
        b: *b as u8,
        a: 255,
      })
      .collect();

    // Now set every node to the same color:
    for node in self.nodes.iter_mut() {
      node.color = *colors.first().unwrap();
    }

    let mut loop_cnt = 0;
    // This looks like O(n^m) runtime. Whoops.
    while !self.is_k_colored() && loop_cnt < 100 {
      loop_cnt += 1;
      for pos in 1..(k_number + 1) {
        let mut colors = colors.clone();

        while colors.len() + 1 > pos as usize {
          colors.pop();
        }

        for _pos in 0..loop_cnt {
          let [r, g, b] = RandomColor::new().to_rgb_array();
          colors.push(Color {
            r: r as u8,
            g: g as u8,
            b: b as u8,
            a: 255,
          });
        }

        let mut new_colors = vec![];

        for node in self.nodes.iter() {
          let neighbors = node.get_neighbors(self);
          let mut neighbor_colors = vec![];

          let mut new_color = node.color;

          for neighbor in neighbors {
            let neighbor = &self.nodes[neighbor];

            if neighbor.color == node.color {
              neighbor_colors.push((neighbor.index, neighbor.color));
            } else {
              continue;
            }

            let neighbor_colors: Vec<Color> = neighbor_colors
              .iter()
              .map(|(index, color)| {
                if *index < new_colors.len() {
                  new_colors[*index]
                } else {
                  *color
                }
              })
              .collect();

            new_color = *colors
              .iter()
              .filter(|color| !neighbor_colors.contains(color))
              .next()
              .unwrap_or(&Color::BLUE);
          }

          new_colors.push(new_color);
        }

        for (index, node) in self.nodes.iter_mut().enumerate() {
          node.color = new_colors[index];
        }
      }
    }
  }
}
