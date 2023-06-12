use crate::model::{Model, Vertex};
use glium::{Display, IndexBuffer, Program, Surface, VertexBuffer};
use glium_glyph::{glyph_brush::{ab_glyph::FontRef, Section, Text, Layout, HorizontalAlign}, GlyphBrushBuilder, GlyphBrush};
use nalgebra::Matrix4;

pub struct View {
    positions: VertexBuffer<Vertex>,
    indices: IndexBuffer<u16>,
    program: Program,
    glyph_brush: GlyphBrush<'static, FontRef<'static>>,
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

        let dejavu: &[u8] = include_bytes!("../fonts/NotoMonoNerdFontMono-Regular.ttf");
        let dejavu_font = FontRef::try_from_slice(dejavu).unwrap();

        let glyph_brush = GlyphBrushBuilder::using_font(dejavu_font).build(display);

        Self { positions, indices, program, glyph_brush }
    }

    pub fn draw(&mut self, display: &Display, model: &Model) {
        if model.has_changed() {
            self.draw_object(display, model);
            self.draw_head_up_display(display, model);
        }
    }

    fn draw_object(&self, display: &Display, model: &Model) {
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let model_matrix = model_matrix(&model);
        let view = view_matrix(&model.view_position,
                               &model.view_direction,
                               &model.up);
        let (width, height) = target.get_dimensions();
        let perspective = perspective_matrix(width, height);
        let offset: [f32; 3] = model.object_position.into();

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
                    &uniform! { model: model_matrix, offset: offset, view: view, perspective: perspective, u_light: light },
                    &params).unwrap();
        target.finish().unwrap();
    }

    fn draw_head_up_display(&mut self, display: &Display, model: &Model) {
        let screen_dims = display.get_framebuffer_dimensions();
        log::debug!("[View::draw_head_up_display()] screen_dims=({}, {})", screen_dims.0, screen_dims.1);

        // Top Left Corner (Coordinates of the object)
        let coordinates = format!("(x={}, y={}, z={})\n(rx={}, ry={}, rz={})",
                                model.object_position[0], model.object_position[1], model.object_position[2],
                                model.rot[0], model.rot[1], model.rot[2]);
        self.glyph_brush.queue(
            Section::default()
                .add_text(Text::new(&coordinates).with_scale(18.0))
                .with_bounds((screen_dims.0 as f32, screen_dims.1 as f32 / 2.0)),
        );

        // Top Right Corner
        let view_coordinates = format!("(x={}, y={}, z={})",
                                model.view_position[0], model.view_position[1], model.view_position[2]);
        self.glyph_brush.queue(
            Section::default()
                .add_text(Text::new(&view_coordinates).with_scale(18.0))
                .with_screen_position((screen_dims.0 as f32, 0.0))
                .with_bounds((screen_dims.0 as f32, screen_dims.1 as f32))
                .with_layout(Layout::default().h_align(HorizontalAlign::Right))
            );

        // Middle Chenter
        // self.glyph_brush.queue(
        //     Section::default()
        //         .add_text(Text::new("This is in the middle of the screen").with_scale(48.0))
        //         .with_screen_position((screen_dims.0 as f32 / 2.0, screen_dims.1 as f32 / 2.0))
        //         .with_bounds((screen_dims.0 as f32, screen_dims.1 as f32))
        //         .with_layout(
        //             Layout::default()
        //                 .h_align(HorizontalAlign::Center)
        //                 .v_align(VerticalAlign::Center),
        //         )
        // );

        // Bottom Left Corner (One line only!)
        let scaling_factor = format!("Scaling factor: {}", model.scaling_factor);
        self.glyph_brush.queue(
            Section::default()
                .add_text(Text::new(&scaling_factor).with_scale(18.0))
                .with_screen_position((0.0, screen_dims.1 as f32 - 19.0))
            );

        // Bottom Right Corner
        let direction = format!("Direction ({}, {}, {})",
                                model.view_direction[0], model.view_direction[1], model.view_direction[2]);
        self.glyph_brush.queue(
            Section::default()
                .add_text(Text::new(&direction).with_scale(18.0))
                .with_screen_position((screen_dims.0 as f32, screen_dims.1 as f32 - 19.0))
                .with_layout(Layout::default().h_align(HorizontalAlign::Right))
            );

        let mut target = display.draw();
        self.glyph_brush.draw_queued(display, &mut target);

        target.finish().unwrap();
    }
}

/// Transformation of the model size and rotation to the OpenGL 1x1x1 box.
fn model_matrix(model: &Model) -> [[f32; 4]; 4]
{
    let rot = model.rot;
    let sf = model.scaling_factor;
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
