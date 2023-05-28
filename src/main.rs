#[macro_use]
extern crate glium;

mod teapot;
mod model;

use glium::glutin::event::{Event, KeyboardInput};
use glium::glutin::event_loop::ControlFlow;
use glium::{glutin, Surface, Display, IndexBuffer, VertexBuffer, Program, Frame};
use teapot::{Normal, Vertex};

use model::Model;

fn main() {

    env_logger::init();

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new()
        .with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let mut model = Model::new();

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
                                          &teapot::INDICES).unwrap();

    let vertex_shader_src = include_str!("teapot.vertex.glsl");
    let fragment_shader_src = include_str!("teapot.fragment.glsl");

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src,
                                              None).unwrap();

    event_loop.run(move |event, _, control_flow| {
        run(&display, &mut model, &program, &positions, &normals, &indices, &event, control_flow);
    });
}

fn run<T>(display: &Display,
          model: &mut Model,
          program: &Program,
          positions: &VertexBuffer<Vertex>,
          normals: &VertexBuffer<Normal>,
          indices: &IndexBuffer<u16>,
          event: &Event<T>,
          control_flow: &mut ControlFlow)
{
    let next_frame_time = std::time::Instant::now() +
        std::time::Duration::from_nanos(100_000_000 / 30);
    *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

    handle_event(event, model, control_flow);

    // The drawing part
    let mut target = display.draw();
    draw(model, &mut target, program, positions, normals, indices);
    target.finish().unwrap();
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

fn draw(model: &Model,
        target: &mut Frame,
        program: &Program,
        positions: &VertexBuffer<Vertex>,
        normals: &VertexBuffer<Normal>,
        indices: &IndexBuffer<u16>)
{
    target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

    let model_matrix = model_matrix(model.theta, model.scaling_factor);
    let view = view_matrix(&model.view_position,
                           &model.view_direction,
                           &model.up);
    let (width, height) = target.get_dimensions();
    let perspective = perspective_matrix(width, height);

    let light = [1.4, 0.4, -0.7f32];

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        //backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockWise,
        .. Default::default()
    };

    target.draw((positions, normals), indices, program,
                &uniform! { model: model_matrix, view: view, perspective: perspective, u_light: light },
                &params).unwrap();
    // target.finish().unwrap();
}

/// Transformation of the model size and rotation to the OpenGL 1x1x1 box.
fn model_matrix(theta: f32, sf: f32) -> [[f32; 4]; 4]
{
    // Rotate around Z-Axis
    // [
    //     [ theta.cos() * 0.01, theta.sin() * 0.01,  0.0, 0.0],
    //     [-theta.sin() * 0.01, theta.cos() * 0.01,  0.0, 0.0],
    //     [                0.0,                0.0, 0.01, 0.0],
    //     [                0.0,                0.0,  2.0, 1.0f32]
    // ]
    [
        [ theta.cos() * sf, 0.0, theta.sin() * sf, 0.0],
        [ 0.0, sf, 0.0, 0.0],
        [-theta.sin() * sf, 0.0, theta.cos() * sf, 0.0],
        [ 0.0, 0.0, 2.0, 1.0f32],
    ]
}

/// Giving all this a nice perspective.
fn perspective_matrix(width: u32, height: u32) -> [[f32; 4]; 4]
{
    let aspect_ratio = height as f32 / width as f32;

    let fov: f32 = std::f32::consts::PI / 3.0;
    let zfar = 1024.0;
    let znear = 0.1;

    let f = 1.0 / (fov / 2.0).tan();

    [
        [f * aspect_ratio,    0.0,              0.0              ,   0.0],
        [       0.0      ,     f ,              0.0              ,   0.0],
        [       0.0      ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
        [       0.0      ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
    ]
}

/// The POV on the model.
fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}
