// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0

use simple_logger::SimpleLogger;
use tao::{
  event::{DeviceEvent, ElementState, Event, KeyboardInput, ModifiersState, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::WindowBuilder,
};

#[allow(clippy::single_match)]
fn main() {
  SimpleLogger::new().init().unwrap();
  let event_loop = EventLoop::new();

  let window = WindowBuilder::new()
    .with_title("Super Cursor Grab'n'Hide Simulator 9000")
    .build(&event_loop)
    .unwrap();

  let mut modifiers = ModifiersState::default();

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::WindowEvent { event, .. } => match event {
        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
        WindowEvent::KeyboardInput {
          input:
            KeyboardInput {
              state: ElementState::Released,
              virtual_keycode: Some(key),
              ..
            },
          ..
        } => {
          use tao::event::VirtualKeyCode::*;
          match key {
            Escape => *control_flow = ControlFlow::Exit,
            G => window.set_cursor_grab(!modifiers.shift()).unwrap(),
            H => window.set_cursor_visible(modifiers.shift()),
            _ => (),
          }
        }
        WindowEvent::ModifiersChanged(m) => modifiers = m,
        _ => (),
      },
      Event::DeviceEvent { event, .. } => match event {
        DeviceEvent::MouseMotion { delta } => println!("mouse moved: {:?}", delta),
        DeviceEvent::Button { button, state } => match state {
          ElementState::Pressed => println!("mouse button {} pressed", button),
          ElementState::Released => println!("mouse button {} released", button),
        },
        _ => (),
      },
      _ => (),
    }
  });
}
