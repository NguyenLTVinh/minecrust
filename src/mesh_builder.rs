use crate::block::{BlockType, FaceDirection, RenderType};
use crate::chunk::Chunk;
use crate::texture::{TextureAtlas, TextureCoords};

pub struct MeshBuilder;

impl MeshBuilder {
    pub fn build_chunk_mesh(chunk: &Chunk, atlas: &TextureAtlas) -> Vec<f32> {
        let mut vertices = Vec::new();

        for x in 0..crate::chunk::CHUNK_SIZE {
            for y in 0..crate::chunk::CHUNK_HEIGHT {
                for z in 0..crate::chunk::CHUNK_SIZE {
                    let block = chunk.get_block(x, y, z);
                    if block == BlockType::Air {
                        continue;
                    }

                    let wx = (chunk.pos.x * crate::chunk::CHUNK_SIZE + x) as f32;
                    let wy = y as f32;
                    let wz = (chunk.pos.z * crate::chunk::CHUNK_SIZE + z) as f32;

                    let props = block.get_properties();

                    match props.render_type {
                        RenderType::CrossPlant => {
                            Self::add_cross_plant(&mut vertices, wx, wy, wz, block, atlas);
                        }
                        RenderType::ScaledCube => {
                            Self::add_scaled_cube(&mut vertices, wx, wy, wz, block, atlas);
                        }
                        RenderType::FullCube => {
                            Self::add_full_cube(&mut vertices, chunk, x, y, z, block, atlas);
                        }
                    }
                }
            }
        }

        vertices
    }

    pub fn add_full_cube(
        vertices: &mut Vec<f32>,
        chunk: &Chunk,
        x: i32,
        y: i32,
        z: i32,
        block: BlockType,
        atlas: &TextureAtlas,
    ) {
        let faces = [
            (FaceDirection::Top, 0, 1, 0),
            (FaceDirection::Bottom, 0, -1, 0),
            (FaceDirection::Front, 0, 0, 1),
            (FaceDirection::Back, 0, 0, -1),
            (FaceDirection::Right, 1, 0, 0),
            (FaceDirection::Left, -1, 0, 0),
        ];

        for (face_dir, dx, dy, dz) in faces {
            let adjacent = chunk.get_block(x + dx, y + dy, z + dz);

            let should_render = if block == BlockType::Water {
                adjacent == BlockType::Air
            } else {
                adjacent.is_transparent()
            };

            if should_render {
                let wx = (chunk.pos.x * crate::chunk::CHUNK_SIZE + x) as f32;
                let wy = y as f32;
                let wz = (chunk.pos.z * crate::chunk::CHUNK_SIZE + z) as f32;

                let tex_coords = atlas.get_tex_coords(block, face_dir);
                let tint = match block {
                    BlockType::GrassBlock | BlockType::SnowyGrassBlock => {
                        if face_dir == FaceDirection::Top {
                            atlas.get_tint(block)
                        } else {
                            [1.0, 1.0, 1.0]
                        }
                    }
                    _ => atlas.get_tint(block),
                };
                Self::add_face(vertices, wx, wy, wz, dx, dy, dz, tex_coords, tint);
            }
        }
    }

    pub fn add_scaled_cube(
        vertices: &mut Vec<f32>,
        x: f32,
        y: f32,
        z: f32,
        block: BlockType,
        atlas: &TextureAtlas,
    ) {
        let dims = block.get_properties().dimensions;
        let w = dims.width_pixels as f32 / 16.0;
        let h = dims.height_pixels as f32 / 16.0;
        let l = dims.length_pixels as f32 / 16.0;

        let inset_x = (16.0 - dims.width_pixels as f32) / 2.0 / 16.0;
        let inset_z = (16.0 - dims.length_pixels as f32) / 2.0 / 16.0;

        let tint = atlas.get_tint(block);

        let faces = [
            (FaceDirection::Top, 0, 1, 0, FaceDirection::Top),
            (FaceDirection::Bottom, 0, -1, 0, FaceDirection::Bottom),
            (FaceDirection::Front, 0, 0, 1, FaceDirection::Front),
            (FaceDirection::Back, 0, 0, -1, FaceDirection::Back),
            (FaceDirection::Right, 1, 0, 0, FaceDirection::Front),
            (FaceDirection::Left, -1, 0, 0, FaceDirection::Front),
        ];

        for (_, dx, dy, dz, tex_face_dir) in faces {
            let tex_coords = atlas.get_tex_coords(block, tex_face_dir);
            Self::add_scaled_face(
                vertices, x, y, z, dx, dy, dz, w, h, l, inset_x, inset_z, tex_coords, tint,
            );
        }
    }

    fn add_scaled_face(
        vertices: &mut Vec<f32>,
        x: f32,
        y: f32,
        z: f32,
        dx: i32,
        dy: i32,
        dz: i32,
        w: f32,
        h: f32,
        l: f32,
        inset_x: f32,
        inset_z: f32,
        tex: TextureCoords,
        tint: [f32; 3],
    ) {
        let normal = [dx as f32, dy as f32, dz as f32];
        let (u_min, u_max, v_min, v_max) = if dy != 0 {
            let u_range = tex.u_max - tex.u_min;
            let v_range = tex.v_max - tex.v_min;
            (
                tex.u_min + u_range * inset_x,
                tex.u_max - u_range * inset_x,
                tex.v_min + v_range * inset_z,
                tex.v_max - v_range * inset_z,
            )
        } else {
            let u_range = tex.u_max - tex.u_min;
            (
                tex.u_min + u_range * inset_x,
                tex.u_max - u_range * inset_x,
                tex.v_min,
                tex.v_max,
            )
        };

        let inset_tex = TextureCoords {
            u_min,
            u_max,
            v_min,
            v_max,
        };

        let uvs = inset_tex.get_uvs_for_face(dx, dy, dz);

        #[rustfmt::skip]
        let verts = match (dx, dy, dz) {

            (0, 1, 0) => vec![
                x + inset_x,     y + h, z + inset_z,
                x + inset_x,     y + h, z + inset_z + l,
                x + inset_x + w, y + h, z + inset_z + l,

                x + inset_x,     y + h, z + inset_z,
                x + inset_x + w, y + h, z + inset_z + l,
                x + inset_x + w, y + h, z + inset_z,
            ],

            (0, -1, 0) => vec![
                x + inset_x,     y, z + inset_z,
                x + inset_x + w, y, z + inset_z,
                x + inset_x + w, y, z + inset_z + l,

                x + inset_x,     y, z + inset_z,
                x + inset_x + w, y, z + inset_z + l,
                x + inset_x,     y, z + inset_z + l,
             ],

             (0, 0, 1) => vec![
                x + inset_x,     y,     z + inset_z + l,
                x + inset_x,     y + h, z + inset_z + l,
                x + inset_x + w, y + h, z + inset_z + l,

                x + inset_x,     y,     z + inset_z + l,
                x + inset_x + w, y + h, z + inset_z + l,
                x + inset_x + w, y,     z + inset_z + l,
            ],

            (0, 0, -1) => vec![
                x + inset_x,     y,     z + inset_z,
                x + inset_x + w, y,     z + inset_z,
                x + inset_x + w, y + h, z + inset_z,

                x + inset_x,     y,     z + inset_z,
                x + inset_x + w, y + h, z + inset_z,
                x + inset_x,     y + h, z + inset_z,
            ],

            (1, 0, 0) => vec![
                x + inset_x + w, y,     z + inset_z,
                x + inset_x + w, y + h, z + inset_z,
                x + inset_x + w, y + h, z + inset_z + l,

                x + inset_x + w, y,     z + inset_z,
                x + inset_x + w, y + h, z + inset_z + l,
                x + inset_x + w, y,     z + inset_z + l,
            ],

            (-1, 0, 0) => vec![
                x + inset_x, y,     z + inset_z,
                x + inset_x, y,     z + inset_z + l,
                x + inset_x, y + h, z + inset_z + l,

                x + inset_x, y,     z + inset_z,
                x + inset_x, y + h, z + inset_z + l,
                x + inset_x, y + h, z + inset_z,
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

        let uvs = tex.get_uvs_for_face(dx, dy, dz);

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
        block: BlockType,
        atlas: &TextureAtlas,
    ) {
        let tex = atlas.get_tex_coords(block, FaceDirection::Top);
        let tint = atlas.get_tint(block);
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
}
