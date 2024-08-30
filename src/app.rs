use crossbeam::channel;
use glutin_window::GlutinWindow as Window;
use glutin_window::OpenGL;
use opengl_graphics::GlGraphics;
use piston::Button;
use piston::EventSettings;
use piston::Events;
use piston::PressEvent;
use piston::ReleaseEvent;
use piston::RenderEvent;
use piston::UpdateEvent;
use piston::{RenderArgs, UpdateArgs, WindowSettings};

use crate::utils;

pub struct App {
    gl: GlGraphics,
    window: Window,
    events: Events,
    key_tx: channel::Sender<Vec<f32>>,
}

impl App {
    pub fn new(title: &str, opengl: OpenGL, key_tx: channel::Sender<Vec<f32>>) -> Self {
        let window: Window = WindowSettings::new(title, [200, 200])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();
        return Self {
            gl: GlGraphics::new(opengl),
            window: window,
            events: Events::new(EventSettings::new()),
            key_tx: key_tx,
        };
    }

    pub fn run(&mut self) {
        let mut modified;
        let mut note_played: Vec<f32> = Vec::new();
        while let Some(e) = self.events.next(&mut self.window) {
            modified = false;
            if let Some(args) = e.render_args() {
                self.render(&args);
            }

            if let Some(args) = e.update_args() {
                self.update(&args);
            }
            if let Some(Button::Keyboard(key)) = e.press_args() {
                modified = true;
                let note_result = utils::key_to_note(key);
                if let Some(note) = note_result {
                    note_played.push(note);
                }
            }

            if let Some(Button::Keyboard(key)) = e.release_args() {
                modified = true;
                let note_result = utils::key_to_note(key);
                if let Some(note) = note_result {
                    note_played.retain(|&x| x != note);
                }
            }
            if modified {
                self.key_tx.send(note_played.clone()).unwrap();
            }
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        self.gl.draw(args.viewport(), |_, gl| {
            // Clear the screen.
            clear(WHITE, gl);
        });
    }

    pub fn update(&mut self, _: &UpdateArgs) {}
}
