/// This model manages the different system states, which will be manipulated by the controller.
/// It starts with the camara but must also contain information regarding the orientation of the
/// graphic model, under investigation..
pub struct Model {
    pub view_position: [f32; 3],
    pub view_direction: [f32; 3],
    pub up: [f32; 3],
}
impl Model {
    /// Creates a new model with a reset on the coordinates.
    pub fn new() -> Self {
        let view_position = [3.0, 1.0, 1.0];
        let view_direction = [-3.0, -1.0, 1.0];
        let up = [0.0, 1.0, 0.0];
        Self { view_position, view_direction, up }
    }

    pub fn reset_view(&mut self) {
        self.view_position = [3.0, 1.0, 1.0];
        self.view_direction = [-3.0, -1.0, 1.0];
        self.up = [0.0, 1.0, 0.0];
    }

    pub fn view_position_up(&mut self) {
        self.view_position[1] += 0.5;
    }
}
