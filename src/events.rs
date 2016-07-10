use nalgebra::*;
use glium::Display;
use std::default::Default;

#[derive(Clone, Copy, Debug)]
pub struct Events {
    pub quit: bool,
    pub mouse: Point2<f32>,
    pub mouse_delta: Vector2<f32>,
    pub left_click: bool,
}

impl Events {
    pub fn pump(&mut self, display: &Display) {
        // Handle updates.
        for event in display.poll_events() {
            use glutin::Event::*;
            use glutin::ElementState::*;
            use glutin::VirtualKeyCode;
            use glutin::MouseButton;

            match event {
                Closed | KeyboardInput(Pressed, _, Some(VirtualKeyCode::Escape)) => {
                    self.quit = true;
                },
                MouseInput(Pressed, MouseButton::Left) => {
                    self.left_click = true;
                },
                MouseInput(Released, MouseButton::Left) => {
                    self.left_click = false;
                },
                MouseMoved(x, y) => {
                    let new_mouse = Point2 { x: x as f32, y: y as f32 };
                    self.mouse_delta = new_mouse - self.mouse;
                    self.mouse = new_mouse;
                },
                _ => {
                    println!("event = {:?}", event);
                },
            }
        }
    }
}

impl Default for Events {
    fn default() -> Events {
        Events {
            quit: false,
            mouse: Point2 { x: 0.0, y: 0.0 },
            mouse_delta: Vector2 { x: 0.0, y: 0.0 },
            left_click: false,
        }
    }
}
