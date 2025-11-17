use crate::block::BlockType;
use crate::chunk::{CHUNK_SIZE, Chunk, ChunkPos};
use crate::gamemode::GameMode;
use cgmath::{InnerSpace, Matrix, Point3, SquareMatrix, Vector3};
use gl::types::*;
use std::collections::HashMap;
use std::ffi::CString;
use std::ptr;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

pub struct UserInterface {
    vao: GLuint,
    vbo: GLuint,
    vertex_count: GLsizei,
    shader_program: GLuint,
    highlight_vao: GLuint,
    highlight_vbo: GLuint,
    highlight_vertex_count: GLsizei,
    highlight_shader_program: GLuint,
    pub highlighted_block: Option<BlockPos>,
    exposed_faces: [bool; 6],
    pub mode: GameMode,
    last_hit_face: usize,
    crosshair_proj_loc: GLint,
    crosshair_color_loc: GLint,
    highlight_view_loc: GLint,
    highlight_proj_loc: GLint,
    highlight_color_loc: GLint,
}

impl UserInterface {
    pub fn new() -> Self {
        let shader_program = create_crosshair_shader();

        let vertices: Vec<f32> = vec![
            -0.01, 0.0, 0.0, 0.01, 0.0, 0.0, 0.0, -0.016, 0.0, 0.0, 0.016, 0.0,
        ];

        let vertex_count = vertices.len() as GLsizei / 3;

        let (vao, vbo) = unsafe {
            let mut vao = 0;
            let mut vbo = 0;

            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                3 * std::mem::size_of::<f32>() as GLsizei,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);

            (vao, vbo)
        };

        let highlight_shader_program = create_highlight_shader();
        let (highlight_vao, highlight_vbo) = unsafe {
            let mut vao = 0;
            let mut vbo = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            (vao, vbo)
        };

        let (crosshair_proj_loc, crosshair_color_loc) = unsafe {
            gl::UseProgram(shader_program);
            (
                gl::GetUniformLocation(shader_program, CString::new("projection").unwrap().as_ptr()),
                gl::GetUniformLocation(shader_program, CString::new("crosshairColor").unwrap().as_ptr()),
            )
        };

        let (highlight_view_loc, highlight_proj_loc, highlight_color_loc) = unsafe {
            gl::UseProgram(highlight_shader_program);
            (
                gl::GetUniformLocation(highlight_shader_program, CString::new("view").unwrap().as_ptr()),
                gl::GetUniformLocation(highlight_shader_program, CString::new("projection").unwrap().as_ptr()),
                gl::GetUniformLocation(highlight_shader_program, CString::new("highlightColor").unwrap().as_ptr()),
            )
        };

        UserInterface {
            vao,
            vbo,
            vertex_count,
            shader_program,
            highlight_vao,
            highlight_vbo,
            highlight_vertex_count: 0,
            highlight_shader_program,
            highlighted_block: None,
            exposed_faces: [true; 6],
            mode: GameMode::Normal,
            last_hit_face: 0,
            crosshair_proj_loc,
            crosshair_color_loc,
            highlight_view_loc,
            highlight_proj_loc,
            highlight_color_loc,
        }
    }

    pub fn update_highlighted_block(
        &mut self,
        camera_pos: Point3<f32>,
        camera_front: Vector3<f32>,
        chunks: &HashMap<ChunkPos, Chunk>,
    ) {
        match self.mode {
            GameMode::Normal => {
                self.highlighted_block = None;
            }
            GameMode::Delete => {
                self.highlighted_block = Self::raycast_for_block(camera_pos, camera_front, chunks);
                self.exposed_faces = self.calculate_exposed_faces(chunks);
            }
            GameMode::Insert => {
                self.highlighted_block =
                    self.raycast_for_insert_block(camera_pos, camera_front, chunks);
            }
        }
        self.update_highlight_mesh();
    }

    fn calculate_exposed_faces(&self, chunks: &HashMap<ChunkPos, Chunk>) -> [bool; 6] {
        let mut exposed = [false; 6];

        if let Some(block_pos) = self.highlighted_block {
            let adjacent_checks = [
                (
                    BlockPos {
                        x: block_pos.x,
                        y: block_pos.y + 1,
                        z: block_pos.z,
                    },
                    0,
                ),
                (
                    BlockPos {
                        x: block_pos.x,
                        y: block_pos.y - 1,
                        z: block_pos.z,
                    },
                    1,
                ),
                (
                    BlockPos {
                        x: block_pos.x,
                        y: block_pos.y,
                        z: block_pos.z + 1,
                    },
                    2,
                ),
                (
                    BlockPos {
                        x: block_pos.x,
                        y: block_pos.y,
                        z: block_pos.z - 1,
                    },
                    3,
                ),
                (
                    BlockPos {
                        x: block_pos.x + 1,
                        y: block_pos.y,
                        z: block_pos.z,
                    },
                    4,
                ),
                (
                    BlockPos {
                        x: block_pos.x - 1,
                        y: block_pos.y,
                        z: block_pos.z,
                    },
                    5,
                ),
            ];

            for (adj_pos, face_idx) in adjacent_checks.iter() {
                let is_exposed = self.is_block_exposed(adj_pos, chunks);
                exposed[*face_idx] = is_exposed;
            }
        }

        exposed
    }

    fn is_block_exposed(&self, pos: &BlockPos, chunks: &HashMap<ChunkPos, Chunk>) -> bool {
        let chunk_x = (pos.x as f32 / CHUNK_SIZE as f32).floor() as i32;
        let chunk_z = (pos.z as f32 / CHUNK_SIZE as f32).floor() as i32;
        let chunk_pos = ChunkPos {
            x: chunk_x,
            z: chunk_z,
        };

        let local_x = pos.x - chunk_x * CHUNK_SIZE;
        let local_z = pos.z - chunk_z * CHUNK_SIZE;

        if let Some(chunk) = chunks.get(&chunk_pos) {
            if local_x >= 0
                && local_x < CHUNK_SIZE
                && local_z >= 0
                && local_z < CHUNK_SIZE
                && pos.y >= 0
            {
                let block = chunk.get_block(local_x, pos.y, local_z);
                return block == BlockType::Air;
            }
        }
        true
    }

    fn raycast_for_block(
        camera_pos: Point3<f32>,
        camera_front: Vector3<f32>,
        chunks: &HashMap<ChunkPos, Chunk>,
    ) -> Option<BlockPos> {
        const MAX_DISTANCE: f32 = 20.0;
        const STEP_SIZE: f32 = 0.1;

        let mut distance_traveled = 0.0;

        while distance_traveled < MAX_DISTANCE {
            let current_pos = camera_pos + camera_front * distance_traveled;

            let block_x = current_pos.x.floor() as i32;
            let block_y = current_pos.y.floor() as i32;
            let block_z = current_pos.z.floor() as i32;

            let chunk_x = (block_x as f32 / CHUNK_SIZE as f32).floor() as i32;
            let chunk_z = (block_z as f32 / CHUNK_SIZE as f32).floor() as i32;
            let chunk_pos = ChunkPos {
                x: chunk_x,
                z: chunk_z,
            };

            let local_x = block_x - chunk_x * CHUNK_SIZE;
            let local_z = block_z - chunk_z * CHUNK_SIZE;

            if let Some(chunk) = chunks.get(&chunk_pos) {
                if local_x >= 0 && local_x < CHUNK_SIZE && local_z >= 0 && local_z < CHUNK_SIZE {
                    let block = chunk.get_block(local_x, block_y, local_z);
                    if block != BlockType::Air {
                        return Some(BlockPos {
                            x: block_x,
                            y: block_y,
                            z: block_z,
                        });
                    }
                }
            }

            distance_traveled += STEP_SIZE;
        }

        None
    }

    fn raycast_for_insert_block(
        &mut self,
        camera_pos: Point3<f32>,
        camera_front: Vector3<f32>,
        chunks: &HashMap<ChunkPos, Chunk>,
    ) -> Option<BlockPos> {
        const MAX_DISTANCE: f32 = 20.0;
        const STEP_SIZE: f32 = 0.05;

        let mut distance_traveled = 0.0;
        let mut prev_pos = BlockPos { x: 0, y: 0, z: 0 };

        while distance_traveled < MAX_DISTANCE {
            let current_pos = camera_pos + camera_front * distance_traveled;

            let block_x = current_pos.x.floor() as i32;
            let block_y = current_pos.y.floor() as i32;
            let block_z = current_pos.z.floor() as i32;

            let chunk_x = (block_x as f32 / CHUNK_SIZE as f32).floor() as i32;
            let chunk_z = (block_z as f32 / CHUNK_SIZE as f32).floor() as i32;
            let chunk_pos = ChunkPos {
                x: chunk_x,
                z: chunk_z,
            };

            let local_x = block_x - chunk_x * CHUNK_SIZE;
            let local_z = block_z - chunk_z * CHUNK_SIZE;

            if let Some(chunk) = chunks.get(&chunk_pos) {
                if local_x >= 0 && local_x < CHUNK_SIZE && local_z >= 0 && local_z < CHUNK_SIZE {
                    let block = chunk.get_block(local_x, block_y, local_z);
                    if block != BlockType::Air {
                        self.last_hit_face = self.determine_face(
                            prev_pos,
                            BlockPos {
                                x: block_x,
                                y: block_y,
                                z: block_z,
                            },
                        );
                        return Some(prev_pos);
                    }
                }
            }

            prev_pos = BlockPos {
                x: block_x,
                y: block_y,
                z: block_z,
            };
            distance_traveled += STEP_SIZE;
        }

        None
    }

    fn determine_face(&self, air_block: BlockPos, solid_block: BlockPos) -> usize {
        let dx = air_block.x - solid_block.x;
        let dy = air_block.y - solid_block.y;
        let dz = air_block.z - solid_block.z;

        if dy > 0 {
            0
        } else if dy < 0 {
            1
        } else if dz > 0 {
            2
        } else if dz < 0 {
            3
        } else if dx > 0 {
            4
        } else {
            5
        }
    }

    fn build_highlight_mesh(block_pos: BlockPos) -> Vec<f32> {
        let mut vertices = Vec::new();
        let x = block_pos.x as f32;
        let y = block_pos.y as f32;
        let z = block_pos.z as f32;
        let offset = 0.0;

        let edges = vec![
            [
                x - offset,
                y - offset,
                z - offset,
                x + 1.0 + offset,
                y - offset,
                z - offset,
            ],
            [
                x + 1.0 + offset,
                y - offset,
                z - offset,
                x + 1.0 + offset,
                y - offset,
                z + 1.0 + offset,
            ],
            [
                x + 1.0 + offset,
                y - offset,
                z + 1.0 + offset,
                x - offset,
                y - offset,
                z + 1.0 + offset,
            ],
            [
                x - offset,
                y - offset,
                z + 1.0 + offset,
                x - offset,
                y - offset,
                z - offset,
            ],
            [
                x - offset,
                y + 1.0 + offset,
                z - offset,
                x + 1.0 + offset,
                y + 1.0 + offset,
                z - offset,
            ],
            [
                x + 1.0 + offset,
                y + 1.0 + offset,
                z - offset,
                x + 1.0 + offset,
                y + 1.0 + offset,
                z + 1.0 + offset,
            ],
            [
                x + 1.0 + offset,
                y + 1.0 + offset,
                z + 1.0 + offset,
                x - offset,
                y + 1.0 + offset,
                z + 1.0 + offset,
            ],
            [
                x - offset,
                y + 1.0 + offset,
                z + 1.0 + offset,
                x - offset,
                y + 1.0 + offset,
                z - offset,
            ],
            [
                x - offset,
                y - offset,
                z - offset,
                x - offset,
                y + 1.0 + offset,
                z - offset,
            ],
            [
                x + 1.0 + offset,
                y - offset,
                z - offset,
                x + 1.0 + offset,
                y + 1.0 + offset,
                z - offset,
            ],
            [
                x + 1.0 + offset,
                y - offset,
                z + 1.0 + offset,
                x + 1.0 + offset,
                y + 1.0 + offset,
                z + 1.0 + offset,
            ],
            [
                x - offset,
                y - offset,
                z + 1.0 + offset,
                x - offset,
                y + 1.0 + offset,
                z + 1.0 + offset,
            ],
        ];

        for edge in edges {
            vertices.push(edge[0]);
            vertices.push(edge[1]);
            vertices.push(edge[2]);
            vertices.push(edge[3]);
            vertices.push(edge[4]);
            vertices.push(edge[5]);
        }

        vertices
    }

    fn update_highlight_mesh(&mut self) {
        unsafe {
            if let Some(block_pos) = self.highlighted_block {
                let vertices = Self::build_highlight_mesh(block_pos);
                self.highlight_vertex_count = (vertices.len() as GLsizei) / 3;

                gl::BindVertexArray(self.highlight_vao);
                gl::BindBuffer(gl::ARRAY_BUFFER, self.highlight_vbo);
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
                    vertices.as_ptr() as *const _,
                    gl::DYNAMIC_DRAW,
                );

                gl::VertexAttribPointer(
                    0,
                    3,
                    gl::FLOAT,
                    gl::FALSE,
                    3 * std::mem::size_of::<f32>() as GLsizei,
                    ptr::null(),
                );
                gl::EnableVertexAttribArray(0);

                gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                gl::BindVertexArray(0);
            } else {
                self.highlight_vertex_count = 0;
            }
        }
    }

    pub fn render_highlight(
        &self,
        view_matrix: &cgmath::Matrix4<f32>,
        projection_matrix: &cgmath::Matrix4<f32>,
        camera_pos: Point3<f32>,
    ) {
        if let Some(block_pos) = self.highlighted_block {
            unsafe {
                gl::UseProgram(self.highlight_shader_program);

                gl::UniformMatrix4fv(self.highlight_view_loc, 1, gl::FALSE, view_matrix.as_ptr());
                gl::UniformMatrix4fv(self.highlight_proj_loc, 1, gl::FALSE, projection_matrix.as_ptr());

                let (r, g, b) = match self.mode {
                    GameMode::Delete => (220.0 / 255.0, 20.0 / 255.0, 60.0 / 255.0),
                    GameMode::Insert => (1.0, 1.0, 1.0),
                    GameMode::Normal => (1.0, 1.0, 1.0),
                };
                gl::Uniform4f(self.highlight_color_loc, r, g, b, 1.0);

                if self.highlight_vertex_count > 0 {
                    let depth_test_enabled = gl::IsEnabled(gl::DEPTH_TEST) == gl::TRUE;
                    gl::Disable(gl::DEPTH_TEST);

                    let block_center = cgmath::Point3::new(
                        block_pos.x as f32 + 0.5,
                        block_pos.y as f32 + 0.5,
                        block_pos.z as f32 + 0.5,
                    );
                    let to_camera = camera_pos - block_center;

                    self.render_highlight_edges(&block_pos, &to_camera);

                    if depth_test_enabled {
                        gl::Enable(gl::DEPTH_TEST);
                    }

                    gl::LineWidth(1.0);
                }
            }
        }
    }

    fn render_highlight_edges(&self, block_pos: &BlockPos, camera_dir: &Vector3<f32>) {
        unsafe {
            gl::BindVertexArray(self.highlight_vao);
            gl::LineWidth(2.0);

            let x = block_pos.x as f32;
            let y = block_pos.y as f32;
            let z = block_pos.z as f32;
            let offset = 0.002;

            if self.mode == GameMode::Insert {
                let all_edges = vec![
                    [
                        x - offset,
                        y - offset,
                        z - offset,
                        x + 1.0 + offset,
                        y - offset,
                        z - offset,
                    ],
                    [
                        x + 1.0 + offset,
                        y - offset,
                        z - offset,
                        x + 1.0 + offset,
                        y - offset,
                        z + 1.0 + offset,
                    ],
                    [
                        x + 1.0 + offset,
                        y - offset,
                        z + 1.0 + offset,
                        x - offset,
                        y - offset,
                        z + 1.0 + offset,
                    ],
                    [
                        x - offset,
                        y - offset,
                        z + 1.0 + offset,
                        x - offset,
                        y - offset,
                        z - offset,
                    ],
                    [
                        x - offset,
                        y + 1.0 + offset,
                        z - offset,
                        x + 1.0 + offset,
                        y + 1.0 + offset,
                        z - offset,
                    ],
                    [
                        x + 1.0 + offset,
                        y + 1.0 + offset,
                        z - offset,
                        x + 1.0 + offset,
                        y + 1.0 + offset,
                        z + 1.0 + offset,
                    ],
                    [
                        x + 1.0 + offset,
                        y + 1.0 + offset,
                        z + 1.0 + offset,
                        x - offset,
                        y + 1.0 + offset,
                        z + 1.0 + offset,
                    ],
                    [
                        x - offset,
                        y + 1.0 + offset,
                        z + 1.0 + offset,
                        x - offset,
                        y + 1.0 + offset,
                        z - offset,
                    ],
                    [
                        x - offset,
                        y - offset,
                        z - offset,
                        x - offset,
                        y + 1.0 + offset,
                        z - offset,
                    ],
                    [
                        x + 1.0 + offset,
                        y - offset,
                        z - offset,
                        x + 1.0 + offset,
                        y + 1.0 + offset,
                        z - offset,
                    ],
                    [
                        x + 1.0 + offset,
                        y - offset,
                        z + 1.0 + offset,
                        x + 1.0 + offset,
                        y + 1.0 + offset,
                        z + 1.0 + offset,
                    ],
                    [
                        x - offset,
                        y - offset,
                        z + 1.0 + offset,
                        x - offset,
                        y + 1.0 + offset,
                        z + 1.0 + offset,
                    ],
                ];

                for edge in all_edges {
                    let start = [edge[0], edge[1], edge[2]];
                    let end = [edge[3], edge[4], edge[5]];
                    let dx = end[0] - start[0];
                    let dy = end[1] - start[1];
                    let dz = end[2] - start[2];
                    let length = (dx * dx + dy * dy + dz * dz).sqrt();

                    let dash_length = 0.15;
                    let gap_length = 0.1;
                    let segment_length = dash_length + gap_length;
                    let num_segments = (length / segment_length).ceil() as i32;

                    for i in 0..num_segments {
                        let start_t = (i as f32 * segment_length) / length;
                        let dash_end_t =
                            ((i as f32 * segment_length + dash_length) / length).min(1.0);

                        if start_t < 1.0 {
                            let dash_start = [
                                start[0] + dx * start_t,
                                start[1] + dy * start_t,
                                start[2] + dz * start_t,
                            ];
                            let dash_end = [
                                start[0] + dx * dash_end_t,
                                start[1] + dy * dash_end_t,
                                start[2] + dz * dash_end_t,
                            ];

                            let mut vertices = Vec::new();
                            vertices.push(dash_start[0]);
                            vertices.push(dash_start[1]);
                            vertices.push(dash_start[2]);
                            vertices.push(dash_end[0]);
                            vertices.push(dash_end[1]);
                            vertices.push(dash_end[2]);

                            gl::BindBuffer(gl::ARRAY_BUFFER, self.highlight_vbo);
                            gl::BufferData(
                                gl::ARRAY_BUFFER,
                                (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
                                vertices.as_ptr() as *const _,
                                gl::DYNAMIC_DRAW,
                            );

                            gl::VertexAttribPointer(
                                0,
                                3,
                                gl::FLOAT,
                                gl::FALSE,
                                3 * std::mem::size_of::<f32>() as GLsizei,
                                ptr::null(),
                            );
                            gl::EnableVertexAttribArray(0);

                            gl::DrawArrays(gl::LINES, 0, 2);
                        }
                    }
                }
            } else {
                let edges = vec![
                    (
                        Vector3::new(0.0, -1.0, 0.0),
                        1,
                        vec![
                            [
                                x - offset,
                                y - offset,
                                z - offset,
                                x + 1.0 + offset,
                                y - offset,
                                z - offset,
                            ],
                            [
                                x + 1.0 + offset,
                                y - offset,
                                z - offset,
                                x + 1.0 + offset,
                                y - offset,
                                z + 1.0 + offset,
                            ],
                            [
                                x + 1.0 + offset,
                                y - offset,
                                z + 1.0 + offset,
                                x - offset,
                                y - offset,
                                z + 1.0 + offset,
                            ],
                            [
                                x - offset,
                                y - offset,
                                z + 1.0 + offset,
                                x - offset,
                                y - offset,
                                z - offset,
                            ],
                        ],
                    ),
                    (
                        Vector3::new(0.0, 1.0, 0.0),
                        0,
                        vec![
                            [
                                x - offset,
                                y + 1.0 + offset,
                                z - offset,
                                x + 1.0 + offset,
                                y + 1.0 + offset,
                                z - offset,
                            ],
                            [
                                x + 1.0 + offset,
                                y + 1.0 + offset,
                                z - offset,
                                x + 1.0 + offset,
                                y + 1.0 + offset,
                                z + 1.0 + offset,
                            ],
                            [
                                x + 1.0 + offset,
                                y + 1.0 + offset,
                                z + 1.0 + offset,
                                x - offset,
                                y + 1.0 + offset,
                                z + 1.0 + offset,
                            ],
                            [
                                x - offset,
                                y + 1.0 + offset,
                                z + 1.0 + offset,
                                x - offset,
                                y + 1.0 + offset,
                                z - offset,
                            ],
                        ],
                    ),
                    (
                        Vector3::new(0.0, 0.0, 1.0),
                        2,
                        vec![
                            [
                                x - offset,
                                y - offset,
                                z + 1.0 + offset,
                                x + 1.0 + offset,
                                y - offset,
                                z + 1.0 + offset,
                            ],
                            [
                                x + 1.0 + offset,
                                y - offset,
                                z + 1.0 + offset,
                                x + 1.0 + offset,
                                y + 1.0 + offset,
                                z + 1.0 + offset,
                            ],
                            [
                                x + 1.0 + offset,
                                y + 1.0 + offset,
                                z + 1.0 + offset,
                                x - offset,
                                y + 1.0 + offset,
                                z + 1.0 + offset,
                            ],
                            [
                                x - offset,
                                y + 1.0 + offset,
                                z + 1.0 + offset,
                                x - offset,
                                y - offset,
                                z + 1.0 + offset,
                            ],
                        ],
                    ),
                    (
                        Vector3::new(0.0, 0.0, -1.0),
                        3,
                        vec![
                            [
                                x - offset,
                                y - offset,
                                z - offset,
                                x + 1.0 + offset,
                                y - offset,
                                z - offset,
                            ],
                            [
                                x + 1.0 + offset,
                                y - offset,
                                z - offset,
                                x + 1.0 + offset,
                                y + 1.0 + offset,
                                z - offset,
                            ],
                            [
                                x + 1.0 + offset,
                                y + 1.0 + offset,
                                z - offset,
                                x - offset,
                                y + 1.0 + offset,
                                z - offset,
                            ],
                            [
                                x - offset,
                                y + 1.0 + offset,
                                z - offset,
                                x - offset,
                                y - offset,
                                z - offset,
                            ],
                        ],
                    ),
                    (
                        Vector3::new(1.0, 0.0, 0.0),
                        4,
                        vec![
                            [
                                x + 1.0 + offset,
                                y - offset,
                                z - offset,
                                x + 1.0 + offset,
                                y - offset,
                                z + 1.0 + offset,
                            ],
                            [
                                x + 1.0 + offset,
                                y - offset,
                                z + 1.0 + offset,
                                x + 1.0 + offset,
                                y + 1.0 + offset,
                                z + 1.0 + offset,
                            ],
                            [
                                x + 1.0 + offset,
                                y + 1.0 + offset,
                                z + 1.0 + offset,
                                x + 1.0 + offset,
                                y + 1.0 + offset,
                                z - offset,
                            ],
                            [
                                x + 1.0 + offset,
                                y + 1.0 + offset,
                                z - offset,
                                x + 1.0 + offset,
                                y - offset,
                                z - offset,
                            ],
                        ],
                    ),
                    (
                        Vector3::new(-1.0, 0.0, 0.0),
                        5,
                        vec![
                            [
                                x - offset,
                                y - offset,
                                z - offset,
                                x - offset,
                                y - offset,
                                z + 1.0 + offset,
                            ],
                            [
                                x - offset,
                                y - offset,
                                z + 1.0 + offset,
                                x - offset,
                                y + 1.0 + offset,
                                z + 1.0 + offset,
                            ],
                            [
                                x - offset,
                                y + 1.0 + offset,
                                z + 1.0 + offset,
                                x - offset,
                                y + 1.0 + offset,
                                z - offset,
                            ],
                            [
                                x - offset,
                                y + 1.0 + offset,
                                z - offset,
                                x - offset,
                                y - offset,
                                z - offset,
                            ],
                        ],
                    ),
                ];

                for (normal, face_idx, edges_for_face) in edges {
                    if normal.dot(*camera_dir) > 0.0 && self.exposed_faces[face_idx] {
                        for edge in edges_for_face {
                            let mut vertices = Vec::new();
                            vertices.push(edge[0]);
                            vertices.push(edge[1]);
                            vertices.push(edge[2]);
                            vertices.push(edge[3]);
                            vertices.push(edge[4]);
                            vertices.push(edge[5]);

                            gl::BindBuffer(gl::ARRAY_BUFFER, self.highlight_vbo);
                            gl::BufferData(
                                gl::ARRAY_BUFFER,
                                (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
                                vertices.as_ptr() as *const _,
                                gl::DYNAMIC_DRAW,
                            );

                            gl::VertexAttribPointer(
                                0,
                                3,
                                gl::FLOAT,
                                gl::FALSE,
                                3 * std::mem::size_of::<f32>() as GLsizei,
                                ptr::null(),
                            );
                            gl::EnableVertexAttribArray(0);

                            gl::DrawArrays(gl::LINES, 0, 2);
                        }
                    }
                }
            }

            gl::BindVertexArray(0);
        }
    }

    pub fn render(&self, _width: u32, _height: u32) {
        if self.mode == GameMode::Normal {
            return;
        }

        unsafe {
            gl::UseProgram(self.shader_program);
            gl::Disable(gl::DEPTH_TEST);
            gl::LineWidth(2.0);

            let identity = cgmath::Matrix4::<f32>::identity();

            gl::UniformMatrix4fv(self.crosshair_proj_loc, 1, gl::FALSE, identity.as_ptr());
            gl::Uniform4f(self.crosshair_color_loc, 1.0, 1.0, 1.0, 1.0);

            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::LINES, 0, self.vertex_count);
            gl::BindVertexArray(0);

            gl::LineWidth(1.0);
            gl::Enable(gl::DEPTH_TEST);
        }
    }
}

impl Drop for UserInterface {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.highlight_vbo);
            gl::DeleteVertexArrays(1, &self.highlight_vao);
            gl::DeleteProgram(self.shader_program);
            gl::DeleteProgram(self.highlight_shader_program);
        }
    }
}

fn create_crosshair_shader() -> GLuint {
    let vertex_src = r#"
#version 330 core
layout (location = 0) in vec3 aPos;

uniform mat4 projection;

void main() {
    gl_Position = projection * vec4(aPos, 1.0);
}
"#;

    let fragment_src = r#"
#version 330 core
uniform vec4 crosshairColor;

out vec4 color;

void main() {
    color = crosshairColor;
}
"#;

    unsafe {
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let c_str = CString::new(vertex_src.as_bytes()).unwrap();
        gl::ShaderSource(vertex_shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);

        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let c_str = CString::new(fragment_src.as_bytes()).unwrap();
        gl::ShaderSource(fragment_shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(fragment_shader);

        let program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        program
    }
}

fn create_highlight_shader() -> GLuint {
    let vertex_src = r#"
#version 330 core
layout (location = 0) in vec3 aPos;

uniform mat4 view;
uniform mat4 projection;

void main() {
    gl_Position = projection * view * vec4(aPos, 1.0);
}
"#;

    let fragment_src = r#"
#version 330 core
uniform vec4 highlightColor;

out vec4 color;

void main() {
    color = highlightColor;
}
"#;

    unsafe {
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let c_str = CString::new(vertex_src.as_bytes()).unwrap();
        gl::ShaderSource(vertex_shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);

        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let c_str = CString::new(fragment_src.as_bytes()).unwrap();
        gl::ShaderSource(fragment_shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(fragment_shader);

        let program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        program
    }
}
