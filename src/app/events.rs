use crate::app::App;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

pub struct MouseState {
  pub cords: (i16, i16),

  pub history_left: Vec<usize>,
  pub history_right: Vec<usize>,
  pub history_mid: Vec<usize>,
  pub history_hover: Vec<usize>,

  pub drag_left: Option<usize>,
  pub drag_right: Option<usize>,
  pub drag_mid: Option<usize>,

  pub label_buffer: String,
  pub renaming: bool,
}

impl App {
  /// Updates the stored mouse state & history
  fn event_mouse_state_helper(
    button_pressed: bool,
    dragging: &mut Option<usize>,
    index: usize,
    history: &mut Vec<usize>,
  ) -> bool {
    match (button_pressed, dragging.is_some()) {
      (false, _) => {
        *dragging = None;
        false
      }
      (true, false) => {
        *dragging = Some(index);
        if history.len() == 0 {
          history.push(index);
        } else if *history.get(history.len() - 1).unwrap() != index {
          history.push(index);
        }
        false
      }
      (true, true) => true,
    }
  }

  /// Processes events from the mouse
  fn event_mouse(&mut self) {
    let mouse_state = self.event_pump.mouse_state();
    let mouse_cords = (mouse_state.x() as i16, mouse_state.y() as i16);
    self.mouse_state.cords = mouse_cords;

    let hover_nodes = self
      .graph
      .nodes
      .iter()
      .enumerate()
      .filter(|(_pos, node)| node.is_collision(mouse_cords.0, mouse_cords.1));

    for (index, _) in hover_nodes {
      self.mouse_state.history_hover.push(index);

      App::event_mouse_state_helper(
        mouse_state.left(),
        &mut self.mouse_state.drag_left,
        index,
        &mut self.mouse_state.history_left,
      );

      App::event_mouse_state_helper(
        mouse_state.middle(),
        &mut self.mouse_state.drag_mid,
        index,
        &mut self.mouse_state.history_mid,
      );

      App::event_mouse_state_helper(
        mouse_state.right(),
        &mut self.mouse_state.drag_right,
        index,
        &mut self.mouse_state.history_right,
      );
    }
  }

  // Moves a node that is being clicked on
  fn event_move_node(&mut self) {
    let node_idx = match self.mouse_state.drag_left {
      None => return,
      Some(node_idx) => node_idx,
    };

    let node = match self.graph.nodes.get_mut(node_idx) {
      None => return,
      Some(node) => node,
    };

    node.cords = self.mouse_state.cords;
  }

  // Selects the last two nodes that have been clicked on
  fn event_select_node(&mut self) {
    let history = &self.mouse_state.history_left;

    for node_idx in history {
      let node = match self.graph.nodes.get_mut(*node_idx) {
        None => return,
        Some(node) => node,
      };

      node.selected = false;
    }

    for pos in 0..2 {
      if history.len() <= pos {
        return;
      }

      let node_idx = match history.get(history.len() - pos - 1) {
        None => return,
        Some(node_idx) => node_idx,
      };

      let node = match self.graph.nodes.get_mut(*node_idx) {
        None => return,
        Some(node) => node,
      };

      node.selected = true;
    }
  }

  fn event_create_edge(&mut self) {
    let history = &self.mouse_state.history_left;

    if history.len() < 2 {
      return;
    }

    let nodes = (history[history.len() - 1], history[history.len() - 2]);

    self.graph.add_edge(nodes, Color::WHITE);
  }

  fn event_del_edge(&mut self) {
    let history = &self.mouse_state.history_left;

    if history.len() < 2 {
      return;
    }

    let nodes = (history[history.len() - 1], history[history.len() - 2]);

    self.graph.del_edge(nodes);
  }

  fn event_add_node(&mut self) {
    self.graph.add_node(self.mouse_state.cords);
  }

  fn event_del_node(&mut self) {
    let history = &mut self.mouse_state.history_left;

    if history.len() < 1 {
      return;
    }

    let idx = history[history.len() - 1];

    self.graph.del_node(idx);

    history.retain(|node_idx| *node_idx != idx)
  }

  fn event_rename_node(&mut self) {
    if !self.mouse_state.renaming {
      self.mouse_state.renaming = true;
      self.mouse_state.label_buffer = String::new();
    }

    self.mouse_state.renaming = false;

    let history = &mut self.mouse_state.history_left;

    if history.len() < 1 {
      return;
    }

    let idx = history[history.len() - 1];
    let node = &mut self.graph.nodes[idx];

    node.label = self.mouse_state.label_buffer.clone();
  }

  fn event_rename_process(&mut self, events: Vec<Event>) {
    for event in events {
      match event {
        Event::KeyDown {
          keycode: Some(Keycode::KpEnter),
          ..
        } => {
          self.event_rename_node();
        }
        Event::KeyDown {
          keycode: Some(key),
          ..
        } => {
          self.mouse_state.label_buffer.push(key.to_string().parse().unwrap());
        }
        _ => {}
      }
    }
  }

  //fn event_

  /// Processes events from the OS
  pub(crate) fn event_process(&mut self) -> bool {
    let events: Vec<Event> = self.event_pump.poll_iter().collect();

    if self.mouse_state.renaming {
      self.event_rename_process(events);
      return false;
    }

    for event in events {
      match event {
        Event::MouseButtonDown { .. } | Event::MouseMotion { .. } => {
          self.event_mouse();
          self.event_move_node();
          self.event_select_node();
        }
        Event::KeyDown {
          keycode: Some(Keycode::E),
          ..
        } => self.event_create_edge(),
        Event::KeyDown {
          keycode: Some(Keycode::R),
          ..
        } => self.event_del_edge(),
        Event::KeyDown {
          keycode: Some(Keycode::L),
          ..
        } => self.event_rename_node(),
        Event::KeyDown {
          keycode: Some(Keycode::N),
          ..
        } => self.event_add_node(),
        Event::KeyDown {
          keycode: Some(Keycode::M),
          ..
        } => self.event_del_node(),
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => return true,
        _ => {}
      }
    }

    false
  }
}
