use cgmath::{InnerSpace, Matrix4, Point3, Vector3};

pub struct Camera {
    pub position: Point3<f32>,
    pub front: Vector3<f32>,
    pub up: Vector3<f32>,
    pub yaw: f32,
    pub pitch: f32,
    pub velocity: Vector3<f32>,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            position: Point3::new(0.0, 50.0, 0.0),
            front: Vector3::new(0.0, 0.0, -1.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            yaw: -90.0,
            pitch: 0.0,
            velocity: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn update_vectors(&mut self) {
        let front = Vector3::new(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        );
        self.front = front.normalize();
    }

    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(self.position, self.position + self.front, self.up)
    }
}
