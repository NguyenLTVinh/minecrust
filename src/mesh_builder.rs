use crate::texture::TextureCoords;

pub struct MeshBuilder;

impl MeshBuilder {
    pub fn add_face(
        vertices: &mut Vec<f32>,
        x: f32,
        y: f32,
        z: f32,
        dx: i32,
        dy: i32,
        dz: i32,
        tex: TextureCoords,
        tint: [f32; 3],
    ) {
        let normal = [dx as f32, dy as f32, dz as f32];

        let uvs = match (dx, dy, dz) {
            (-1, 0, 0) => [
                [tex.u_max, tex.v_max],
                [tex.u_min, tex.v_max],
                [tex.u_min, tex.v_min],
                [tex.u_max, tex.v_max],
                [tex.u_min, tex.v_min],
                [tex.u_max, tex.v_min],
            ],
            (1, 0, 0) => [
                [tex.u_min, tex.v_max],
                [tex.u_min, tex.v_min],
                [tex.u_max, tex.v_min],
                [tex.u_min, tex.v_max],
                [tex.u_max, tex.v_min],
                [tex.u_max, tex.v_max],
            ],
            (0, 0, -1) => [
                [tex.u_min, tex.v_max],
                [tex.u_max, tex.v_max],
                [tex.u_max, tex.v_min],
                [tex.u_min, tex.v_max],
                [tex.u_max, tex.v_min],
                [tex.u_min, tex.v_min],
            ],
            (0, 0, 1) => [
                [tex.u_min, tex.v_max],
                [tex.u_min, tex.v_min],
                [tex.u_max, tex.v_min],
                [tex.u_min, tex.v_max],
                [tex.u_max, tex.v_min],
                [tex.u_max, tex.v_max],
            ],
            _ => [
                [tex.u_min, tex.v_min],
                [tex.u_min, tex.v_max],
                [tex.u_max, tex.v_max],
                [tex.u_min, tex.v_min],
                [tex.u_max, tex.v_max],
                [tex.u_max, tex.v_min],
            ],
        };

        #[rustfmt::skip]
        let verts = match (dx, dy, dz) {
            (0, 1, 0) => vec![
                x,         y + 1.0, z,
                x,         y + 1.0, z + 1.0,
                x + 1.0,   y + 1.0, z + 1.0,

                x,         y + 1.0, z,
                x + 1.0,   y + 1.0, z + 1.0,
                x + 1.0,   y + 1.0, z,
            ],
            (0, -1, 0) => vec![
                x,         y, z,
                x + 1.0,   y, z,
                x + 1.0,   y, z + 1.0,

                x,         y, z,
                x + 1.0,   y, z + 1.0,
                x,         y, z + 1.0,
            ],
            (0, 0, 1) => vec![
                x,         y,       z + 1.0,
                x,         y + 1.0, z + 1.0,
                x + 1.0,   y + 1.0, z + 1.0,

                x,         y,       z + 1.0,
                x + 1.0,   y + 1.0, z + 1.0,
                x + 1.0,   y,       z + 1.0,
            ],
            (0, 0, -1) => vec![
                x,         y,       z,
                x + 1.0,   y,       z,
                x + 1.0,   y + 1.0, z,

                x,         y,       z,
                x + 1.0,   y + 1.0, z,
                x,         y + 1.0, z,
            ],
            (1, 0, 0) => vec![
                x + 1.0,   y,       z,
                x + 1.0,   y + 1.0, z,
                x + 1.0,   y + 1.0, z + 1.0,

                x + 1.0,   y,       z,
                x + 1.0,   y + 1.0, z + 1.0,
                x + 1.0,   y,       z + 1.0,
            ],
            (-1, 0, 0) => vec![
                x, y,       z,
                x, y,       z + 1.0,
                x, y + 1.0, z + 1.0,

                x, y,       z,
                x, y + 1.0, z + 1.0,
                x, y + 1.0, z,
            ],
            _ => vec![],
        };

        for (i, pos_idx) in (0..verts.len()).step_by(3).enumerate() {
            vertices.extend_from_slice(&verts[pos_idx..pos_idx + 3]);
            vertices.extend_from_slice(&uvs[i]);
            vertices.extend_from_slice(&tint);
            vertices.extend_from_slice(&normal);
        }
    }

    pub fn add_cross_plant(
        vertices: &mut Vec<f32>,
        x: f32,
        y: f32,
        z: f32,
        tex: TextureCoords,
        tint: [f32; 3],
    ) {
        let cx = x + 0.5;
        let cz = z + 0.5;

        let normal1 = [0.7071, 0.0, 0.7071];
        #[rustfmt::skip]
        let square1 = vec![
            cx - 0.5, y,       cz - 0.5,
            cx - 0.5, y + 1.0, cz - 0.5,
            cx + 0.5, y + 1.0, cz + 0.5,

            cx - 0.5, y,       cz - 0.5,
            cx + 0.5, y + 1.0, cz + 0.5,
            cx + 0.5, y,       cz + 0.5,
        ];
        let uvs1 = [
            [tex.u_min, tex.v_max],
            [tex.u_min, tex.v_min],
            [tex.u_max, tex.v_min],
            [tex.u_min, tex.v_max],
            [tex.u_max, tex.v_min],
            [tex.u_max, tex.v_max],
        ];

        for (i, pos_idx) in (0..square1.len()).step_by(3).enumerate() {
            vertices.extend_from_slice(&square1[pos_idx..pos_idx + 3]);
            vertices.extend_from_slice(&uvs1[i]);
            vertices.extend_from_slice(&tint);
            vertices.extend_from_slice(&normal1);
        }

        let normal2 = [-0.7071, 0.0, 0.7071];
        #[rustfmt::skip]
        let square2 = vec![
            cx + 0.5, y,       cz - 0.5,
            cx + 0.5, y + 1.0, cz - 0.5,
            cx - 0.5, y + 1.0, cz + 0.5,

            cx + 0.5, y,       cz - 0.5,
            cx - 0.5, y + 1.0, cz + 0.5,
            cx - 0.5, y,       cz + 0.5,
        ];
        let uvs2 = [
            [tex.u_min, tex.v_max],
            [tex.u_min, tex.v_min],
            [tex.u_max, tex.v_min],
            [tex.u_min, tex.v_max],
            [tex.u_max, tex.v_min],
            [tex.u_max, tex.v_max],
        ];

        for (i, pos_idx) in (0..square2.len()).step_by(3).enumerate() {
            vertices.extend_from_slice(&square2[pos_idx..pos_idx + 3]);
            vertices.extend_from_slice(&uvs2[i]);
            vertices.extend_from_slice(&tint);
            vertices.extend_from_slice(&normal2);
        }
    }

    pub fn add_snow_layer(
        vertices: &mut Vec<f32>,
        x: f32,
        y: f32,
        z: f32,
        height: f32,
        tex: TextureCoords,
        tint: [f32; 3],
    ) {
        let top_normal = [0.0, 1.0, 0.0];
        #[rustfmt::skip]
        let top_verts = vec![
            x,         y + height, z,
            x,         y + height, z + 1.0,
            x + 1.0,   y + height, z + 1.0,

            x,         y + height, z,
            x + 1.0,   y + height, z + 1.0,
            x + 1.0,   y + height, z,
        ];
        let top_uvs = [
            [tex.u_min, tex.v_min],
            [tex.u_min, tex.v_max],
            [tex.u_max, tex.v_max],
            [tex.u_min, tex.v_min],
            [tex.u_max, tex.v_max],
            [tex.u_max, tex.v_min],
        ];
        for (i, pos_idx) in (0..top_verts.len()).step_by(3).enumerate() {
            vertices.extend_from_slice(&top_verts[pos_idx..pos_idx + 3]);
            vertices.extend_from_slice(&top_uvs[i]);
            vertices.extend_from_slice(&tint);
            vertices.extend_from_slice(&top_normal);
        }

        let bottom_normal = [0.0, -1.0, 0.0];

        #[rustfmt::skip]
        let bottom_verts = vec![
            x,         y, z,
            x + 1.0,   y, z,
            x + 1.0,   y, z + 1.0,

            x,         y, z,
            x + 1.0,   y, z + 1.0,
            x,         y, z + 1.0,
        ];
        let bottom_uvs = [
            [tex.u_min, tex.v_min],
            [tex.u_min, tex.v_max],
            [tex.u_max, tex.v_max],
            [tex.u_min, tex.v_min],
            [tex.u_max, tex.v_max],
            [tex.u_max, tex.v_min],
        ];
        for (i, pos_idx) in (0..bottom_verts.len()).step_by(3).enumerate() {
            vertices.extend_from_slice(&bottom_verts[pos_idx..pos_idx + 3]);
            vertices.extend_from_slice(&bottom_uvs[i]);
            vertices.extend_from_slice(&tint);
            vertices.extend_from_slice(&bottom_normal);
        }

        let side_v_max = tex.v_min + (tex.v_max - tex.v_min) * height;

        let front_normal = [0.0, 0.0, 1.0];

        #[rustfmt::skip]
        let front_verts = vec![
            x,         y,          z + 1.0,
            x,         y + height, z + 1.0,
            x + 1.0,   y + height, z + 1.0,

            x,         y,          z + 1.0,
            x + 1.0,   y + height, z + 1.0,
            x + 1.0,   y,          z + 1.0,
        ];
        let front_uvs = [
            [tex.u_min, side_v_max],
            [tex.u_min, tex.v_min],
            [tex.u_max, tex.v_min],
            [tex.u_min, side_v_max],
            [tex.u_max, tex.v_min],
            [tex.u_max, side_v_max],
        ];
        for (i, pos_idx) in (0..front_verts.len()).step_by(3).enumerate() {
            vertices.extend_from_slice(&front_verts[pos_idx..pos_idx + 3]);
            vertices.extend_from_slice(&front_uvs[i]);
            vertices.extend_from_slice(&tint);
            vertices.extend_from_slice(&front_normal);
        }

        let back_normal = [0.0, 0.0, -1.0];
        #[rustfmt::skip]
        let back_verts = vec![
            x,         y,          z,
            x + 1.0,   y,          z,
            x + 1.0,   y + height, z,

            x,         y,          z,
            x + 1.0,   y + height, z,
            x,         y + height, z,
        ];
        let back_uvs = [
            [tex.u_min, side_v_max],
            [tex.u_max, side_v_max],
            [tex.u_max, tex.v_min],
            [tex.u_min, side_v_max],
            [tex.u_max, tex.v_min],
            [tex.u_min, tex.v_min],
        ];
        for (i, pos_idx) in (0..back_verts.len()).step_by(3).enumerate() {
            vertices.extend_from_slice(&back_verts[pos_idx..pos_idx + 3]);
            vertices.extend_from_slice(&back_uvs[i]);
            vertices.extend_from_slice(&tint);
            vertices.extend_from_slice(&back_normal);
        }

        let right_normal = [1.0, 0.0, 0.0];
        #[rustfmt::skip]
        let right_verts = vec![
            x + 1.0,   y,          z,
            x + 1.0,   y + height, z,
            x + 1.0,   y + height, z + 1.0,

            x + 1.0,   y,          z,
            x + 1.0,   y + height, z + 1.0,
            x + 1.0,   y,          z + 1.0,
        ];
        let right_uvs = [
            [tex.u_min, side_v_max],
            [tex.u_min, tex.v_min],
            [tex.u_max, tex.v_min],
            [tex.u_min, side_v_max],
            [tex.u_max, tex.v_min],
            [tex.u_max, side_v_max],
        ];
        for (i, pos_idx) in (0..right_verts.len()).step_by(3).enumerate() {
            vertices.extend_from_slice(&right_verts[pos_idx..pos_idx + 3]);
            vertices.extend_from_slice(&right_uvs[i]);
            vertices.extend_from_slice(&tint);
            vertices.extend_from_slice(&right_normal);
        }

        let left_normal = [-1.0, 0.0, 0.0];
        #[rustfmt::skip]
        let left_verts = vec![
            x, y,          z,
            x, y,          z + 1.0,
            x, y + height, z + 1.0,

            x, y,          z,
            x, y + height, z + 1.0,
            x, y + height, z,
        ];
        let left_uvs = [
            [tex.u_max, side_v_max],
            [tex.u_min, side_v_max],
            [tex.u_min, tex.v_min],
            [tex.u_max, side_v_max],
            [tex.u_min, tex.v_min],
            [tex.u_max, tex.v_min],
        ];
        for (i, pos_idx) in (0..left_verts.len()).step_by(3).enumerate() {
            vertices.extend_from_slice(&left_verts[pos_idx..pos_idx + 3]);
            vertices.extend_from_slice(&left_uvs[i]);
            vertices.extend_from_slice(&tint);
            vertices.extend_from_slice(&left_normal);
        }
    }
}
