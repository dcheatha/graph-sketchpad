use sdl2::render::WindowCanvas;

fn find_sdl_gl_driver() -> Option<u32> {
  for (index, item) in sdl2::render::drivers().enumerate() {
    if item.name == "opengl" {
      return Some(index as u32);
    }
  }
  None
}

pub fn build_canvas(sdl: &sdl2::Sdl) -> WindowCanvas {
  let video_subsystem = sdl.video().unwrap();

  sdl.mouse().capture(true);

  let window = video_subsystem
    .window("Graph Sketchpad", 1600, 900)
    .allow_highdpi()
    .opengl()
    .build()
    .unwrap();

  // Get the drawable size cords here, since ownership of Window will be given to Canvas
  let (size_x, size_y) = window.drawable_size();

  let mut canvas = window
    .into_canvas()
    .index(find_sdl_gl_driver().unwrap())
    .build()
    .unwrap();

  canvas.set_logical_size(size_x, size_y).unwrap();

  canvas
}
