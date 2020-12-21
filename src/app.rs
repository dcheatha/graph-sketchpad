use crate::app::events::MouseState;
use crate::graph::Graph;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::{EventPump, Sdl};

mod events;

pub struct App {
  pub graph: Graph,
  pub canvas: WindowCanvas,
  pub event_pump: EventPump,
  mouse_state: MouseState,
}

impl App {
  pub fn new(sdl: &Sdl) -> App {
    App {
      graph: Graph {
        nodes: vec![],
        edges: vec![],
      },
      canvas: crate::init::build_canvas(&sdl),
      event_pump: sdl.event_pump().unwrap(),
      mouse_state: MouseState {
        cords: (0, 0),
        history_left: vec![],
        history_right: vec![],
        history_mid: vec![],
        history_hover: vec![],
        drag_left: None,
        drag_right: None,
        drag_mid: None,
      },
    }
  }

  /// Does a single render pass
  pub fn render(&mut self) -> bool {
    self.canvas.clear();
    self.graph.draw(&self.canvas);
    self.canvas.set_draw_color(Color::BLACK);
    self.canvas.present();
    self.event_process()
  }
}
