use crate::model::{Model, Vertex};
use glium::{Display, IndexBuffer, Program, Surface, VertexBuffer};
use nalgebra::Matrix4;

pub struct View {
    positions: VertexBuffer<Vertex>,
    indices: IndexBuffer<u16>,
    program: Program,
}
impl View {
    pub fn new(display: &Display, model: &Model) -> Self
    {
        let positions = model.object.vertex_buffer(display).unwrap();
        let indices = model.object.index_buffer(display).unwrap();

        let vertex_shader_src = include_str!("teapot.vertex.glsl");
        let fragment_shader_src = include_str!("teapot.fragment.glsl");

        let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src,
                                                  None).unwrap();
        Self { positions, indices, program }
    }

    pub fn draw(&self, display: &Display, model: &Model) {
        if model.has_changed() {
            let mut target = display.draw();
            target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

            let model_matrix = model_matrix(&model.rot, model.scaling_factor);
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

            target.draw(&self.positions,
                        &self.indices,
                        &self.program,
                        &uniform! { model: model_matrix, view: view, perspective: perspective, u_light: light },
                        &params).unwrap();
            target.finish().unwrap();
        }
    }

}

/// Transformation of the model size and rotation to the OpenGL 1x1x1 box.
fn model_matrix(rot: &[f32], sf: f32) -> [[f32; 4]; 4]
{
    let mut final_matrix = Matrix4::from_euler_angles(rot[0], rot[1], rot[2]).append_scaling(sf);
    final_matrix[(2, 3)] = 2.0;
    log::trace!("The final matrix is: {final_matrix}");
    final_matrix.into()
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
