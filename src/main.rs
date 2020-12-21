use crate::app::App;
use std::time::Duration;

mod app;
mod edge;
mod graph;
mod init;
mod node;

/// Renders a window, builds a 2d canvas inside it and returns the Canvas

fn main() {
  let sdl = sdl2::init().unwrap();

  let mut app = App::new(&sdl);

  loop {
    if app.render() {
      break;
    }
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
  }
}
