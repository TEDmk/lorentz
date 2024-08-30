use app::App;
use crossbeam::channel;
use glutin_window::OpenGL;
use synth::Synth;

mod app;
mod synth;
mod utils;
fn main() {
    println!("Hello, world!");
    let (key_tx, key_rx) = channel::bounded(0);
    let opengl = OpenGL::V3_2;
    let mut app = App::new("My Synth", opengl, key_tx);
    let mut synth = Synth::new();
    synth.run(key_rx);
    app.run();
}
