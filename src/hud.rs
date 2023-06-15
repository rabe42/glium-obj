use glium::{Display, Frame};
use glium_glyph::{GlyphBrush, glyph_brush::{ab_glyph::FontRef, Text, Section, Layout, HorizontalAlign}, GlyphBrushBuilder};

use crate::model::Model;

const FONT_SIZE: f32 = 18.0;

/// The Vertexes of the HUD will be stored in this structure, grabbed from the triangle example
/// :-).
#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
}
implement_vertex!(Vertex, position);

/// The Head-Up-Display of the application. The HUD presents numeric informations regarding the
/// position of the object and the viewers position and direction. It has a murky white background
/// to improve the contrast between text and background/object.
pub struct HudView {
    glyph_brush: GlyphBrush<'static, FontRef<'static>>,
}
impl HudView {
    pub fn new(display: &Display) -> Self {
        let dejavu: &[u8] = include_bytes!("../fonts/NotoMonoNerdFontMono-Regular.ttf");
        let dejavu_font = FontRef::try_from_slice(dejavu).unwrap();

        let glyph_brush = GlyphBrushBuilder::using_font(dejavu_font).build(display);

        // TODO: Create the shaders for the background of the HUD
        // TODO: Create the geometry for the background of the HUD
        // let top_left = Vertex { position: [-1.0, 1.0, 1.0] };
        // let top_right = Vertex { position: [1.0, 1.0, 1.0] };
        // let bottom_left = Vertex { position: [-1.0, 0.9, 1.0] };
        // let bottom_right = Vertex { position: [1.0, 0.9, 1.0] };

        HudView { glyph_brush }
    }

    /// Draws all information of the Hud on the provided target. The provided display is also
    /// needed for the draw_queued call of the framework.
    pub fn draw(
        &mut self,
        target: &mut Frame,
        display: &Display,
        model: &Model
    ) {
        let screen_dims = display.get_framebuffer_dimensions();

        // TODO: Recalculate the dimesions of the top HUD
        // TODO: Recalculate the dimesions of the bottom HUD

        log::debug!("[View::draw_head_up_display()] screen_dims=({}, {})", screen_dims.0, screen_dims.1);

        self.draw_hud_background(target);

        // Top Left Corner (Coordinates of the object)
        let coordinates = format!("(x={}, y={}, z={})\n(rx={}, ry={}, rz={})",
                                model.object_position[0], model.object_position[1], model.object_position[2],
                                model.rot[0], model.rot[1], model.rot[2]);
        self.glyph_brush.queue(
            Section::default()
                .add_text(Text::new(&coordinates).with_scale(FONT_SIZE))
                .with_bounds((screen_dims.0 as f32, screen_dims.1 as f32 / 2.0)),
        );

        // Top Right Corner
        let view_coordinates = format!("(x={}, y={}, z={})",
                                model.view_position[0], model.view_position[1], model.view_position[2]);
        self.glyph_brush.queue(
            Section::default()
                .add_text(Text::new(&view_coordinates).with_scale(FONT_SIZE))
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
                .add_text(Text::new(&scaling_factor).with_scale(FONT_SIZE))
                .with_screen_position((0.0, screen_dims.1 as f32 - (FONT_SIZE + 1.0)))
            );

        // Bottom Right Corner
        let direction = format!("Direction ({}, {}, {})",
                                model.view_direction[0], model.view_direction[1], model.view_direction[2]);
        self.glyph_brush.queue(
            Section::default()
                .add_text(Text::new(&direction).with_scale(FONT_SIZE))
                .with_screen_position((screen_dims.0 as f32, screen_dims.1 as f32 - (FONT_SIZE + 1.0)))
                .with_layout(Layout::default().h_align(HorizontalAlign::Right))
            );

        // let mut target: Frame = display.draw();
        self.glyph_brush.draw_queued(display, target);

        // target.finish().unwrap();
    }

    fn draw_hud_background(&self, _target: &Frame) {
        // TODO: Render the background of the HUD
    }
}
