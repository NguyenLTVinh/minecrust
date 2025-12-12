use gl::types::*;
use std::ffi::CString;
use std::ptr;

pub struct TextRenderer {
    shader_program: GLuint,
    vao: GLuint,
    string_buffer_id: GLuint,
    font_texture: GLuint,
    time_uniform: GLint,
    resolution_uniform: GLint,
    message_position_uniform: GLint,
    message_scale_uniform: GLint,
    font_uniform: GLint,
    prompt_shader_program: GLuint,
    prompt_vao: GLuint,
    prompt_vbo: GLuint,
}

impl TextRenderer {
    pub fn new() -> Result<Self, String> {
        let font_texture = load_font_texture("./fonts/charmap-oldschool_white.png")?;

        let shader_program = create_text_shader_program()?;

        let vao = unsafe {
            let mut vao = 0;
            gl::GenVertexArrays(1, &mut vao);
            vao
        };

        let string_buffer_id = unsafe {
            let mut buffer_id = 0;
            let string_buffer_data: [u8; 1024] = [0; 1024];

            gl::GenBuffers(1, &mut buffer_id);
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, buffer_id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of_val(&string_buffer_data)
                    .try_into()
                    .unwrap(),
                string_buffer_data.as_ptr() as *const std::ffi::c_void,
                gl::DYNAMIC_DRAW,
            );

            const CHAR_ATTRIB_INDEX: i32 = 0;
            gl::VertexAttribIPointer(
                CHAR_ATTRIB_INDEX.try_into().unwrap(),
                1,
                gl::BYTE,
                0,
                std::ptr::null(),
            );

            gl::EnableVertexAttribArray(CHAR_ATTRIB_INDEX.try_into().unwrap());
            gl::VertexAttribDivisor(CHAR_ATTRIB_INDEX.try_into().unwrap(), 1);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);

            buffer_id
        };

        let time_uniform = unsafe {
            gl::UseProgram(shader_program);
            gl::GetUniformLocation(shader_program, CString::new("time").unwrap().as_ptr())
        };

        let resolution_uniform = unsafe {
            gl::GetUniformLocation(shader_program, CString::new("resolution").unwrap().as_ptr())
        };

        let message_position_uniform = unsafe {
            gl::GetUniformLocation(
                shader_program,
                CString::new("message_position").unwrap().as_ptr(),
            )
        };

        let message_scale_uniform = unsafe {
            gl::GetUniformLocation(
                shader_program,
                CString::new("message_scale").unwrap().as_ptr(),
            )
        };

        let font_uniform = unsafe {
            gl::GetUniformLocation(shader_program, CString::new("font").unwrap().as_ptr())
        };

        let (prompt_vao, prompt_vbo) = unsafe {
            let mut vao = 0;
            let mut vbo = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            (vao, vbo)
        };

        let prompt_shader_program = create_prompt_shader_program()?;

        Ok(TextRenderer {
            shader_program,
            vao,
            string_buffer_id,
            font_texture,
            time_uniform,
            resolution_uniform,
            message_position_uniform,
            message_scale_uniform,
            font_uniform,
            prompt_shader_program,
            prompt_vao,
            prompt_vbo,
        })
    }

    pub fn render_text(
        &self,
        text: &str,
        x: f32,
        y: f32,
        scale: f32,
        width: u32,
        height: u32,
        time: f32,
    ) {
        unsafe {
            let mut string_buffer_data: [u8; 1024] = [0; 1024];

            for (dst, src) in string_buffer_data.iter_mut().zip(text.bytes()) {
                *dst = src;
            }

            gl::BindBuffer(gl::ARRAY_BUFFER, self.string_buffer_id);
            let size = std::mem::size_of_val(&string_buffer_data[0])
                * std::cmp::min(string_buffer_data.len(), text.len());
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                size.try_into().unwrap(),
                string_buffer_data.as_ptr() as *const std::ffi::c_void,
            );

            gl::UseProgram(self.shader_program);

            gl::Uniform2f(self.resolution_uniform, width as f32, height as f32);
            gl::Uniform2f(self.message_position_uniform, x, y);
            gl::Uniform1f(self.message_scale_uniform, scale);
            gl::Uniform1f(self.time_uniform, time);
            gl::Uniform1i(self.font_uniform, 0);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.font_texture);

            gl::BindVertexArray(self.vao);

            gl::DrawArraysInstanced(gl::TRIANGLE_STRIP, 0, 4, text.len().try_into().unwrap());

            gl::BindVertexArray(0);
        }
    }
}

impl Drop for TextRenderer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.string_buffer_id);
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteTextures(1, &self.font_texture);
            gl::DeleteProgram(self.shader_program);
            gl::DeleteBuffers(1, &self.prompt_vbo);
            gl::DeleteVertexArrays(1, &self.prompt_vao);
            gl::DeleteProgram(self.prompt_shader_program);
        }
    }
}

fn load_font_texture(file_path: &str) -> Result<GLuint, String> {
    let img = image::open(file_path).map_err(|e| format!("Failed to load font texture: {}", e))?;
    let img = img.to_rgba8();

    let texture = unsafe {
        let mut texture = 0;
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);

        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MAG_FILTER,
            gl::NEAREST.try_into().unwrap(),
        );
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::NEAREST_MIPMAP_LINEAR.try_into().unwrap(),
        );
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_S,
            gl::CLAMP_TO_EDGE.try_into().unwrap(),
        );
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_T,
            gl::CLAMP_TO_EDGE.try_into().unwrap(),
        );

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA.try_into().unwrap(),
            img.width() as i32,
            img.height() as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            img.as_raw().as_ptr() as *const std::ffi::c_void,
        );

        gl::GenerateMipmap(gl::TEXTURE_2D);

        texture
    };

    Ok(texture)
}

fn create_prompt_shader_program() -> Result<GLuint, String> {
    let vertex_src = CString::new(include_str!("../shaders/text_prompt_vertex.glsl"))
        .map_err(|_| "Failed to create prompt vertex shader source".to_string())?;

    let fragment_src = CString::new(include_str!("../shaders/text_prompt_fragment.glsl"))
        .map_err(|_| "Failed to create prompt fragment shader source".to_string())?;

    unsafe {
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(vertex_shader, 1, &vertex_src.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);

        let mut success = gl::FALSE as GLint;
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            let mut len = 0;
            gl::GetShaderiv(vertex_shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buffer = vec![0u8; len as usize];
            gl::GetShaderInfoLog(
                vertex_shader,
                len,
                ptr::null_mut(),
                buffer.as_mut_ptr() as *mut _,
            );
            return Err(format!(
                "Prompt vertex shader compilation failed: {}",
                String::from_utf8_lossy(&buffer)
            ));
        }

        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(fragment_shader, 1, &fragment_src.as_ptr(), ptr::null());
        gl::CompileShader(fragment_shader);

        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            let mut len = 0;
            gl::GetShaderiv(fragment_shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buffer = vec![0u8; len as usize];
            gl::GetShaderInfoLog(
                fragment_shader,
                len,
                ptr::null_mut(),
                buffer.as_mut_ptr() as *mut _,
            );
            return Err(format!(
                "Prompt fragment shader compilation failed: {}",
                String::from_utf8_lossy(&buffer)
            ));
        }

        let program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);

        let mut success = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            return Err("Prompt shader program linking failed".to_string());
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        Ok(program)
    }
}

fn create_text_shader_program() -> Result<GLuint, String> {
    let vertex_src = CString::new(include_str!("../shaders/text_vertex.glsl"))
        .map_err(|_| "Failed to create vertex shader source".to_string())?;

    let fragment_src = CString::new(include_str!("../shaders/text_fragment.glsl"))
        .map_err(|_| "Failed to create fragment shader source".to_string())?;

    unsafe {
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(vertex_shader, 1, &vertex_src.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);

        let mut success = gl::FALSE as GLint;
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            let mut len = 0;
            gl::GetShaderiv(vertex_shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buffer = vec![0u8; len as usize];
            gl::GetShaderInfoLog(
                vertex_shader,
                len,
                ptr::null_mut(),
                buffer.as_mut_ptr() as *mut _,
            );
            return Err(format!(
                "Text vertex shader compilation failed: {}",
                String::from_utf8_lossy(&buffer)
            ));
        }

        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(fragment_shader, 1, &fragment_src.as_ptr(), ptr::null());
        gl::CompileShader(fragment_shader);

        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            let mut len = 0;
            gl::GetShaderiv(fragment_shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buffer = vec![0u8; len as usize];
            gl::GetShaderInfoLog(
                fragment_shader,
                len,
                ptr::null_mut(),
                buffer.as_mut_ptr() as *mut _,
            );
            return Err(format!(
                "Text fragment shader compilation failed: {}",
                String::from_utf8_lossy(&buffer)
            ));
        }

        let program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);

        let mut success = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            return Err("Text shader program linking failed".to_string());
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        Ok(program)
    }
}
