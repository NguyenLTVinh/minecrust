use cgmath::{InnerSpace, Matrix, Matrix4, Point3, Vector3};
use gl::types::*;
use std::ffi::CString;
use std::mem;
use std::ptr;

const SUN_SIZE: f32 = 0.35;
const MOON_SIZE: f32 = 0.2;

pub struct DayNightCycle {
    pub time: f32,
    pub tick_speed: f32,
    pub fast_forward: bool,
    pub cycle_enabled: bool,
}

impl DayNightCycle {
    pub fn new() -> Self {
        DayNightCycle {
            time: 0.25,
            tick_speed: 0.001,
            fast_forward: false,
            cycle_enabled: true,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if !self.cycle_enabled {
            return;
        }

        let speed = if self.fast_forward {
            self.tick_speed * 100.0
        } else {
            self.tick_speed
        };

        self.time += speed * delta_time;
        if self.time > 1.0 {
            self.time -= 1.0;
        }
    }
}

pub struct Sky {
    shader_program: GLuint,
    body_orbit_tilt: f32,
    quad_vao: GLuint,
    quad_vbo: GLuint,
    pub day_night_cycle: DayNightCycle,
}

impl Sky {
    pub fn new(shader_program: GLuint) -> Result<Self, String> {
        let (quad_vao, quad_vbo) = Self::create_quad_mesh();

        Ok(Sky {
            shader_program,
            body_orbit_tilt: 0.0,
            quad_vao,
            quad_vbo,
            day_night_cycle: DayNightCycle::new(),
        })
    }

    fn create_quad_mesh() -> (GLuint, GLuint) {
        let mut vao = 0;
        let mut vbo = 0;

        #[rustfmt::skip]
        let vertices: [f32; 18] = [
            // Two triangles forming a quad at origin
            -1.0, -1.0, 0.0,
             1.0, -1.0, 0.0,
             1.0,  1.0, 0.0,
            -1.0, -1.0, 0.0,
             1.0,  1.0, 0.0,
            -1.0,  1.0, 0.0,
        ];

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<f32>()) as GLsizeiptr,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // Position attribute
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                3 * mem::size_of::<f32>() as GLsizei,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            gl::BindVertexArray(0);
        }

        (vao, vbo)
    }

    pub fn render(
        &self,
        camera_position: Point3<f32>,
        view: &Matrix4<f32>,
        projection: &Matrix4<f32>,
        time_of_day: f32,
        suncolor: [f32; 4],
        suncolor2: [f32; 4],
        mooncolor: [f32; 4],
        mooncolor2: [f32; 4],
    ) {
        unsafe {
            gl::UseProgram(self.shader_program);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthMask(gl::FALSE);

            let view_loc =
                gl::GetUniformLocation(self.shader_program, CString::new("view").unwrap().as_ptr());
            let proj_loc = gl::GetUniformLocation(
                self.shader_program,
                CString::new("projection").unwrap().as_ptr(),
            );
            let model_loc = gl::GetUniformLocation(
                self.shader_program,
                CString::new("model").unwrap().as_ptr(),
            );
            let color_loc = gl::GetUniformLocation(
                self.shader_program,
                CString::new("bodyColor").unwrap().as_ptr(),
            );

            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, view.as_ptr());
            gl::UniformMatrix4fv(proj_loc, 1, gl::FALSE, projection.as_ptr());

            let wicked_time = get_wicked_time_of_day(time_of_day);

            if self.is_sun_visible(time_of_day) {
                self.draw_sun(
                    camera_position,
                    model_loc,
                    color_loc,
                    wicked_time,
                    suncolor,
                    suncolor2,
                );
            }

            if self.is_moon_visible(time_of_day) {
                self.draw_moon(
                    camera_position,
                    model_loc,
                    color_loc,
                    wicked_time,
                    mooncolor,
                    mooncolor2,
                );
            }

            gl::DepthMask(gl::TRUE);
            gl::Disable(gl::BLEND);
        }
    }

    // Minetest's draw_sun implementation
    fn draw_sun(
        &self,
        camera_position: Point3<f32>,
        model_loc: GLint,
        color_loc: GLint,
        wicked_time: f32,
        suncolor: [f32; 4],
        suncolor2: [f32; 4],
    ) {
        // A magic number that contributes to the ratio 1.57 sun/moon size difference
        let sunsize = SUN_SIZE * 100.0;

        let sunsizes = [sunsize * 1.7, sunsize * 1.2, sunsize, sunsize * 0.7];

        let mut c1 = suncolor;
        let mut c2 = suncolor;
        c1[3] = 0.05;
        c2[3] = 0.15;

        let colors = [c1, c2, suncolor, suncolor2];

        unsafe {
            for i in 0..4 {
                let model = self.create_sky_body_matrix(
                    camera_position,
                    90.0,
                    wicked_time * 360.0 - 90.0,
                    sunsizes[i],
                );

                gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, model.as_ptr());
                gl::Uniform4f(
                    color_loc,
                    colors[i][0],
                    colors[i][1],
                    colors[i][2],
                    colors[i][3],
                );

                self.draw_quad();
            }
        }
    }

    // Minetest's draw_moon implementation
    fn draw_moon(
        &self,
        camera_position: Point3<f32>,
        model_loc: GLint,
        color_loc: GLint,
        wicked_time: f32,
        mooncolor: [f32; 4],
        mooncolor2: [f32; 4],
    ) {
        // A magic number that contributes to the ratio 1.57 sun/moon size difference
        let moonsize = MOON_SIZE * 100.0;

        let moonsizes_1 = [-moonsize * 1.9, -moonsize * 1.3, -moonsize, -moonsize];

        let moonsizes_2 = [moonsize * 1.9, moonsize * 1.3, moonsize, moonsize * 0.6];

        let mut c1 = mooncolor;
        let mut c2 = mooncolor;
        c1[3] = 0.05;
        c2[3] = 0.15;

        let colors = [c1, c2, mooncolor, mooncolor2];

        unsafe {
            for i in 0..4 {
                let model = self.create_sky_body_matrix_moon(
                    camera_position,
                    -90.0,
                    wicked_time * 360.0 - 90.0,
                    moonsizes_1[i],
                    moonsizes_2[i],
                );

                gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, model.as_ptr());
                gl::Uniform4f(
                    color_loc,
                    colors[i][0],
                    colors[i][1],
                    colors[i][2],
                    colors[i][3],
                );

                self.draw_quad();
            }
        }
    }

    fn create_sky_body_matrix(
        &self,
        camera_pos: Point3<f32>,
        horizon_position: f32,
        day_position: f32,
        size: f32,
    ) -> Matrix4<f32> {
        let mut pos = Vector3::new(0.0, 0.0, -1.0);
        pos = Self::rotate_xz(pos, horizon_position);
        pos = Self::rotate_xy(pos, day_position);
        pos = Self::rotate_yz(pos, self.body_orbit_tilt);

        let distance = 500.0;
        let world_pos = camera_pos + pos * distance;

        let to_camera = (camera_pos - world_pos).normalize();
        let right = Vector3::new(0.0, 1.0, 0.0).cross(to_camera).normalize();
        let up = to_camera.cross(right);

        let right = right * size;
        let up = up * size;

        Matrix4::new(
            right.x,
            right.y,
            right.z,
            0.0,
            up.x,
            up.y,
            up.z,
            0.0,
            to_camera.x,
            to_camera.y,
            to_camera.z,
            0.0,
            world_pos.x,
            world_pos.y,
            world_pos.z,
            1.0,
        )
    }

    fn create_sky_body_matrix_moon(
        &self,
        camera_pos: Point3<f32>,
        horizon_position: f32,
        day_position: f32,
        size_x: f32,
        size_y: f32,
    ) -> Matrix4<f32> {
        let mut pos = Vector3::new(0.0, 0.0, -1.0);
        pos = Self::rotate_xz(pos, horizon_position);
        pos = Self::rotate_xy(pos, day_position);
        pos = Self::rotate_yz(pos, self.body_orbit_tilt);

        let distance = 500.0;
        let world_pos = camera_pos + pos * distance;

        let to_camera = (camera_pos - world_pos).normalize();
        let right = Vector3::new(0.0, 1.0, 0.0).cross(to_camera).normalize();
        let up = to_camera.cross(right);

        // Moon can have different x and y sizes
        let right = right * size_x;
        let up = up * size_y;

        Matrix4::new(
            right.x,
            right.y,
            right.z,
            0.0,
            up.x,
            up.y,
            up.z,
            0.0,
            to_camera.x,
            to_camera.y,
            to_camera.z,
            0.0,
            world_pos.x,
            world_pos.y,
            world_pos.z,
            1.0,
        )
    }

    fn rotate_xz(v: Vector3<f32>, angle_deg: f32) -> Vector3<f32> {
        let angle = angle_deg.to_radians();
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        Vector3::new(v.x * cos_a - v.z * sin_a, v.y, v.x * sin_a + v.z * cos_a)
    }

    fn rotate_xy(v: Vector3<f32>, angle_deg: f32) -> Vector3<f32> {
        let angle = angle_deg.to_radians();
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        Vector3::new(v.x * cos_a - v.y * sin_a, v.x * sin_a + v.y * cos_a, v.z)
    }

    fn rotate_yz(v: Vector3<f32>, angle_deg: f32) -> Vector3<f32> {
        let angle = angle_deg.to_radians();
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        Vector3::new(v.x, v.y * cos_a - v.z * sin_a, v.y * sin_a + v.z * cos_a)
    }

    fn is_sun_visible(&self, time_of_day: f32) -> bool {
        time_of_day >= 0.05 && time_of_day <= 0.95
    }

    fn is_moon_visible(&self, time_of_day: f32) -> bool {
        time_of_day <= 0.45 || time_of_day >= 0.55
    }

    pub fn get_sun_direction(&self, time_of_day: f32) -> Vector3<f32> {
        let wicked_time = get_wicked_time_of_day(time_of_day);
        self.get_sky_body_direction(90.0, wicked_time * 360.0 - 90.0)
    }

    pub fn get_moon_direction(&self, time_of_day: f32) -> Vector3<f32> {
        let wicked_time = get_wicked_time_of_day(time_of_day);
        self.get_sky_body_direction(-90.0, wicked_time * 360.0 - 90.0)
    }

    fn get_sky_body_direction(&self, horizon_position: f32, day_position: f32) -> Vector3<f32> {
        let mut pos = Vector3::new(0.0, 0.0, -1.0);
        pos = Self::rotate_xz(pos, horizon_position);
        pos = Self::rotate_xy(pos, day_position);
        pos = Self::rotate_yz(pos, self.body_orbit_tilt);
        pos.normalize()
    }

    fn draw_quad(&self) {
        unsafe {
            gl::BindVertexArray(self.quad_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::BindVertexArray(0);
        }
    }

    pub fn get_sky_color(time_of_day: f32) -> [f32; 4] {
        let wicked_time = get_wicked_time_of_day(time_of_day);

        if wicked_time < 0.1 || wicked_time > 0.9 {
            [0.05, 0.05, 0.15, 1.0]
        } else if wicked_time < 0.25 {
            let t = (wicked_time - 0.1) / 0.15;
            let night = [0.05, 0.05, 0.15];
            let sunrise = [0.9, 0.5, 0.2];
            [
                night[0] + (sunrise[0] - night[0]) * t,
                night[1] + (sunrise[1] - night[1]) * t,
                night[2] + (sunrise[2] - night[2]) * t,
                1.0,
            ]
        } else if wicked_time < 0.35 {
            let t = (wicked_time - 0.25) / 0.1;
            let sunrise = [0.9, 0.5, 0.2];
            let day = [0.53, 0.81, 0.92];
            [
                sunrise[0] + (day[0] - sunrise[0]) * t,
                sunrise[1] + (day[1] - sunrise[1]) * t,
                sunrise[2] + (day[2] - sunrise[2]) * t,
                1.0,
            ]
        } else if wicked_time < 0.65 {
            [0.53, 0.81, 0.92, 1.0]
        } else if wicked_time < 0.75 {
            let t = (wicked_time - 0.65) / 0.1;
            let day = [0.53, 0.81, 0.92];
            let sunset = [0.9, 0.4, 0.15];
            [
                day[0] + (sunset[0] - day[0]) * t,
                day[1] + (sunset[1] - day[1]) * t,
                day[2] + (sunset[2] - day[2]) * t,
                1.0,
            ]
        } else if wicked_time < 0.9 {
            let t = (wicked_time - 0.75) / 0.15;
            let sunset = [0.9, 0.4, 0.15];
            let night = [0.05, 0.05, 0.15];
            [
                sunset[0] + (night[0] - sunset[0]) * t,
                sunset[1] + (night[1] - sunset[1]) * t,
                sunset[2] + (night[2] - sunset[2]) * t,
                1.0,
            ]
        } else {
            [0.05, 0.05, 0.15, 1.0]
        }
    }

    pub fn get_ambient_light(time_of_day: f32) -> f32 {
        let wicked_time = get_wicked_time_of_day(time_of_day);
        if wicked_time < 0.1 || wicked_time > 0.9 {
            0.1
        } else if wicked_time < 0.25 {
            let t = (wicked_time - 0.1) / 0.15;
            0.1 + 0.3 * t
        } else if wicked_time < 0.75 {
            0.4
        } else if wicked_time < 0.9 {
            let t = (wicked_time - 0.75) / 0.15;
            0.4 - 0.3 * t
        } else {
            0.1
        }
    }

    pub fn get_sun_intensity(time_of_day: f32) -> f32 {
        let wicked_time = get_wicked_time_of_day(time_of_day);
        if wicked_time < 0.1 || wicked_time > 0.9 {
            0.0
        } else if wicked_time < 0.25 {
            let t = (wicked_time - 0.1) / 0.15;
            t * t
        } else if wicked_time < 0.75 {
            1.0
        } else if wicked_time < 0.9 {
            let t = (wicked_time - 0.75) / 0.15;
            let fade = 1.0 - t;
            fade * fade
        } else {
            0.0
        }
    }
}

impl Drop for Sky {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.quad_vao);
            gl::DeleteBuffers(1, &self.quad_vbo);
        }
    }
}

// Minetest's "wicked time of day" calculation for more natural day/night cycles
pub fn get_wicked_time_of_day(time_of_day: f32) -> f32 {
    let nightlength = 0.415;
    let wn = nightlength / 2.0;

    if time_of_day > wn && time_of_day < 1.0 - wn {
        (time_of_day - wn) / (1.0 - wn * 2.0) * 0.5 + 0.25
    } else if time_of_day < 0.5 {
        time_of_day / wn * 0.25
    } else {
        1.0 - ((1.0 - time_of_day) / wn * 0.25)
    }
}

pub const SKY_VERTEX_SHADER: &str = r#"
#version 330 core
layout (location = 0) in vec3 aPos;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    gl_Position = projection * view * model * vec4(aPos, 1.0);
}
"#;

pub const SKY_FRAGMENT_SHADER: &str = r#"
#version 330 core

out vec4 FragColor;

uniform vec4 bodyColor;

void main() {
    FragColor = bodyColor;
}
"#;
