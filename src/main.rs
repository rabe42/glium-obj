#[macro_use]
extern crate glium;

mod teapot;
mod model;
mod view;

use model::Model;
use view::View;

use glium::glutin::event::{Event, KeyboardInput};
use glium::glutin::event_loop::ControlFlow;
use glium::{glutin, Display};

fn main() {

    env_logger::init();

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new()
        .with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let mut model = Model::new();
    let view = View::new(&display, &model);

    event_loop.run(move |event, _, control_flow| {
        run(&display, &mut model, &view, &event, control_flow);
    });
}

fn run<T>(display: &Display,
          model: &mut Model,
          view: &View,
          event: &Event<T>,
          control_flow: &mut ControlFlow)
{
    let next_frame_time = std::time::Instant::now() +
        std::time::Duration::from_nanos(100_000_000 / 30);
    *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

    handle_event(event, model, control_flow);

    // The drawing part
    view.draw(display, model);
}

fn handle_event<T>(event: &Event<T>, model: &mut Model, control_flow: &mut ControlFlow)
{
    match event {
        glutin::event::Event::WindowEvent { event, .. } => match event {
            glutin::event::WindowEvent::CloseRequested => {
                *control_flow = glutin::event_loop::ControlFlow::Exit;
                return;
            },
            glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                log::debug!("KeyboardInput: {:?}", input);
                handle_keyboard_event(control_flow, model, input);
                return;
            },
            _ => {
                log::debug!("Some WindowEvent was detected: {:?}", event);
                return;
            },
        },
        glutin::event::Event::NewEvents(cause) => match cause {
            // TODO: Eigentlich sollten wir nur in diesem Falle die Kalkulation von Bewegung
            // (Kamera oder Objekt) durchführen.
            glutin::event::StartCause::ResumeTimeReached { .. } => (),
            glutin::event::StartCause::Init => (),
            _ => return,
        },
        _ => return,
    }
}

fn handle_keyboard_event(control_flow: &mut ControlFlow, model: &mut Model, input: &KeyboardInput)
{
    use glutin::event::VirtualKeyCode;
    use glutin::event::ElementState;

    let key_code = if let Some(key_code) = input.virtual_keycode {
        key_code
    } else {
        log::warn!("[main::handle_keyboard_event()] Key without key_code pressed or released!");
        return;
    };

    match input.state {
        glutin::event::ElementState::Pressed =>
            match key_code {
                VirtualKeyCode::Key2 => model.view_position_down(),
                VirtualKeyCode::Key5 => model.reset_view(),
                VirtualKeyCode::Key8 => model.view_position_up(),
                VirtualKeyCode::Key4 => model.view_position_left(),
                VirtualKeyCode::Key6 => model.view_position_right(),
                VirtualKeyCode::Key9 => model.view_position_forward(),
                VirtualKeyCode::Key3 => model.view_position_backward(),
                VirtualKeyCode::Left => model.rotate_left(),
                VirtualKeyCode::Right => model.rotate_right(),
                _ => {}
            }

        ElementState::Released =>
            match key_code {
                VirtualKeyCode::Escape => *control_flow = glutin::event_loop::ControlFlow::Exit,
                _ => {},
            }
    }
}

