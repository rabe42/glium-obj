#[macro_use]
extern crate glium;

mod model;
mod view;

use model::Model;
use view::View;

use glium::glutin::event::{Event, KeyboardInput};
use glium::glutin::event_loop::ControlFlow;
use glium::{glutin, Display};

/// This application expected one parameter on the command line, which must be the path to a
/// wavefront obj file with triangulated surfaces. It loads this file and allows to manipulte it
/// with the QWEASD-+ keys. The view point might be changed with the Numpad-Keys.
///
/// # Usage
/// glutin-obj <obj-file-name>
///
fn main() {

    env_logger::init();

    let obj_file_name = std::env::args().nth(1).expect("No object file provied!");

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Obj viewer based on obj-rs");
    let cb = glutin::ContextBuilder::new()
        .with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let mut model = Model::new(&obj_file_name).expect("Cannot read object file!");
    let mut view = View::new(&display, &model);

    event_loop.run(move |event, _, control_flow| {
        run(&display, &mut model, &mut view, &event, control_flow);
    });
}

/// This is the central controller of the application. It receives all user input, distributes this
/// to the model and controls the update of the view.
///
/// # Arguments
///
/// * 'display' - The object, where we should render upon.
/// * 'model' - The model of the application.
/// * 'view' - The presentation of the model.
/// * 'event' - The event, wich has to be processed now.
/// * 'control_flow' - A glutin specific object, which is basically used to end the application.
fn run<T>(display: &Display,
          model: &mut Model,
          view: &mut View,
          event: &Event<T>,
          control_flow: &mut ControlFlow)
{
    let next_frame_time = std::time::Instant::now() +
        std::time::Duration::from_nanos(100_000_000 / 30);
    *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

    handle_event(event, model, control_flow);

    // The drawing part
    view.draw(display, model);

    // In cases, that there is more than one view, we should reset the changes here. For more
    // complex systems, we have to establish an observer pattern.
    model.reset_changed();
}

/// Handles the events of the window manager and the devices, including keyboards.
/// It must be understood, that keyboard input is typically provided in the context of the window
/// and therefore a window event.
///
/// # Arguments
///
/// * 'event' - The device or windows event to be handled.
/// * 'model' - The model, which can be modified by the events.
/// * 'control_flow' - A glutin specific object, which is basically used to end the application.
fn handle_event<T>(event: &Event<T>, model: &mut Model, control_flow: &mut ControlFlow)
{
    use glutin::event::WindowEvent;
    match event {
        Event::WindowEvent { event, .. } => match event {
            glutin::event::WindowEvent::CloseRequested => {
                *control_flow = glutin::event_loop::ControlFlow::Exit;
                return;
            },
            WindowEvent::KeyboardInput { input, .. } => {
                log::debug!("KeyboardInput: {:?}", input);
                handle_keyboard_event(control_flow, model, input);
                return;
            },
            _ => {
                log::debug!("Some WindowEvent was detected: {:?}", event);
                return;
            },
        },
        Event::NewEvents(cause) => match cause {
            glutin::event::StartCause::ResumeTimeReached { .. } => (),
            glutin::event::StartCause::Init => (),
            _ => return,
        },
        _ => return,
    }
}

/// Handles only the keyboard events send to the application window.
///
/// # Arguments
///
/// * 'input' - The KeyboardInput structure of glutin. This provides access to the scan code and
/// the key code of the event.
/// * 'model' - The model, which can be modified by the events.
/// * 'control_flow' - A glutin specific object, which is basically used to end the application.
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
        ElementState::Pressed =>
            match key_code {
                VirtualKeyCode::Key2 => model.view_position_down(),
                VirtualKeyCode::Key5 => model.reset_view(),
                VirtualKeyCode::Key8 => model.view_position_up(),
                VirtualKeyCode::Key4 => model.view_position_left(),
                VirtualKeyCode::Key6 => model.view_position_right(),
                VirtualKeyCode::Key9 => model.view_position_forward(),
                VirtualKeyCode::Key3 => model.view_position_backward(),
                VirtualKeyCode::A => model.rotate_left(),
                VirtualKeyCode::D => model.rotate_right(),
                VirtualKeyCode::W => model.rotate_up(),
                VirtualKeyCode::S => model.rotate_down(),
                VirtualKeyCode::Q => model.roll_up(),
                VirtualKeyCode::E => model.roll_down(),
                VirtualKeyCode::Minus => model.scale_down(),
                VirtualKeyCode::Equals => model.scale_up(),
                VirtualKeyCode::Down => model.move_x_pos(),
                VirtualKeyCode::Up => model.move_x_neg(),
                VirtualKeyCode::PageUp => model.move_y_pos(),
                VirtualKeyCode::PageDown => model.move_y_neg(),
                VirtualKeyCode::Left => model.move_z_neg(),
                VirtualKeyCode::Right => model.move_z_pos(),
                _ => {}
            }

        ElementState::Released =>
            match key_code {
                VirtualKeyCode::Escape => *control_flow = glutin::event_loop::ControlFlow::Exit,
                _ => {},
            }
    }
}

