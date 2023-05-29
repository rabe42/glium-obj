use crate::teapot;

/// The vertical increment
const VERTICAL_INCR: f32 = 0.1;

// The increment, we allow to rotate the object in RAD.
const ROTATION_INCR: f32 = std::f32::consts::PI / 20.0;

pub type Vertex = teapot::Vertex;
pub type Normal = teapot::Normal;

/// This model manages the different system states, which will be manipulated by the controller.
/// It starts with the camara but must also contain information regarding the orientation of the
/// graphic model, under investigation..
pub struct Model {
    pub scaling_factor: f32,
    pub theta: f32,
    pub view_position: [f32; 3],
    pub view_direction: [f32; 3],
    pub up: [f32; 3],

}
impl Model {
    /// Creates a new model with a reset on the coordinates.
    pub fn new() -> Self {
        let scaling_factor = 0.01;
        let theta = 0.0;
        let view_position = [3.0, 1.0, 1.0];
        let view_direction = [-3.0, -1.0, 1.0];
        let up = [0.0, 1.0, 0.0];
        Self { scaling_factor, theta, view_position, view_direction, up }
    }

    pub fn get_vertices(&self) -> &[Vertex] {
        &teapot::VERTICES
    }

    pub fn get_normals(&self) -> &[Normal] {
        &teapot::NORMALS
    }

    pub fn get_indices(&self) -> &[u16] {
        &teapot::INDICES
    }

    pub fn reset_view(&mut self) {
        self.view_position = [3.0, 1.0, 1.0];
        self.view_direction = [-3.0, -1.0, 1.0];
        self.up = [0.0, 1.0, 0.0];
    }

    pub fn view_position_up(&mut self) {
        self.view_position[1] += VERTICAL_INCR;
    }

    pub fn view_position_down(&mut self) {
        self.view_position[1] -= VERTICAL_INCR;
    }

    pub fn view_position_forward(&mut self) {
        self.view_position[0] -= 0.2;
    }

    pub fn view_position_backward(&mut self) {
        self.view_position[0] += 0.2;
    }

    pub fn view_position_left(&mut self) {
        self.view_position[2] += 0.2;
    }

    pub fn view_position_right(&mut self) {
        self.view_position[2] -= 0.2;
    }

    pub fn rotate_left(&mut self) {
        self.theta -= ROTATION_INCR;
    }

    pub fn rotate_right(&mut self) {
        self.theta += ROTATION_INCR;
    }
}
