use obj::Obj;

/// The vertical increment
const VERTICAL_INCR: f32 = 0.1;

// The increment, we allow to rotate the object in RAD.
const ROTATION_INCR: f32 = std::f32::consts::PI / 20.0;

pub type Vertex = obj::Vertex;

/// This model manages the different system states, which will be manipulated by the controller.
/// It starts with the camara but must also contain information regarding the orientation of the
/// graphic model, under investigation..
pub struct Model {
    changed: bool,
    pub object: Obj,
    pub scaling_factor: f32,
    pub rot: [f32; 3],
    pub view_position: [f32; 3],
    pub view_direction: [f32; 3],
    pub up: [f32; 3],

}
impl Model {
    /// Creates a new model with a reset on the coordinates.
    pub fn new(file_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let scaling_factor = 1.0;
        let rot = [0.0, 0.0, 0.0];
        let view_position = [3.0, 1.0, 1.0];
        let view_direction = [-3.0, -1.0, 1.0];
        let up = [0.0, 1.0, 0.0];
        let input = std::fs::read(file_name)?;
        let rh_object = obj::load_obj(input.as_slice())?;
        let object = to_left_handed(rh_object);
        Ok(Self { changed: true, object, scaling_factor, rot, view_position, view_direction, up })
    }

    pub fn reset_changed(&mut self) {
        self.changed = false;
    }

    pub fn has_changed(&self) -> bool {
        self.changed
    }

    /// Resets the viewers position to the original.
    pub fn reset_view(&mut self) {
        self.view_position = [3.0, 1.0, 1.0];
        self.view_direction = [-3.0, -1.0, 1.0];
        self.up = [0.0, 1.0, 0.0];
        self.changed = true;
    }

    /// Move the viewers position up.
    pub fn view_position_up(&mut self) {
        self.view_position[1] += VERTICAL_INCR;
        self.changed = true;
    }

    /// Move the viewers position down.
    pub fn view_position_down(&mut self) {
        self.view_position[1] -= VERTICAL_INCR;
        self.changed = true;
    }

    /// Move the viewers position forward.
    pub fn view_position_forward(&mut self) {
        self.view_position[0] -= 0.2;
        self.changed = true;
    }

    /// Move the viewers position backward.
    pub fn view_position_backward(&mut self) {
        self.view_position[0] += 0.2;
        self.changed = true;
    }

    /// Move the viewers position to the left.
    pub fn view_position_left(&mut self) {
        self.view_position[2] += 0.2;
        self.changed = true;
    }

    /// Move the viewers position to the right.
    pub fn view_position_right(&mut self) {
        self.view_position[2] -= 0.2;
        self.changed = true;
    }

    /// Rols the object up. This is a rotation around the X-Axis (Eula roll).
    pub fn roll_up(&mut self) {
        self.rot[0] += ROTATION_INCR;
        self.changed = true;
    }

    /// Rols the object down. This is a rotation around the X-Axis (Eula roll).
    pub fn roll_down(&mut self) {
        self.rot[0] -= ROTATION_INCR;
        self.changed = true;
    }

    /// Rotate the object to the left. This is a rotation around the Y-Axis (Eula pitch).
    pub fn rotate_left(&mut self) {
        self.rot[1] += ROTATION_INCR;
        self.changed = true;
    }

    /// Rotate the object to the right. This is a rotation around the Y-Axis (Eula pitch).
    pub fn rotate_right(&mut self) {
        self.rot[1] -= ROTATION_INCR;
        self.changed = true;
    }

    /// Rotate the object up. This is a rotation around the Z-Axis (Eula yaw).
    pub fn rotate_up(&mut self) {
        self.rot[2] += ROTATION_INCR;
        self.changed = true;
    }

    /// Rotate the object to the right. This is a rotation around the Z-Axis (Eula yaw).
    pub fn rotate_down(&mut self) {
        self.rot[2] -= ROTATION_INCR;
        self.changed = true;
    }

    pub fn scale_up(&mut self) {
        self.scaling_factor *= 2.0;
        self.changed = true;
    }

    pub fn scale_down(&mut self) {
        self.scaling_factor /= 2.0;
        self.changed = true;
    }
}

/// Creates a new Obj for the left handed GL universe. The obj files seems to be right handed. As
/// OpenGL is left handed, a conversion must take place, to make sure, that we see no mirrored
/// ojbects. To convert from righthanded to left handed for each coordinate, we have to negate the
/// z-axis.
///
/// This is exactly the result of negating the y-axis and a rotation around the x-axis by Pi.
///
/// # Arguments
///
/// * 'obj' - The object to convert.
fn to_left_handed(obj: Obj) -> Obj {
    let name = obj.name.clone();
    let indices = obj.indices.clone();
    let mut vertices = Vec::new();

    for v in obj.vertices {
        vertices.push(Vertex {
            position: [v.position[0], v.position[1], -v.position[2]],
            normal: [v.normal[0], v.normal[1], -v.normal[2]],
        });
    }

    // Rotate by Pi
    Obj { name, vertices, indices }
}
