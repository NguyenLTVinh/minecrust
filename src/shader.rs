use gl::types::*;
use std::ffi::CString;
use std::ptr;

pub fn compile_shader(src: &str, shader_type: GLenum) -> GLuint {
    unsafe {
        let shader = gl::CreateShader(shader_type);
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        let mut success = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buffer = vec![0u8; len as usize];
            gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buffer.as_mut_ptr() as *mut _);
            panic!(
                "Shader compilation failed: {}",
                String::from_utf8_lossy(&buffer)
            );
        }

        shader
    }
}

pub fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);

        let mut success = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            panic!("Program linking failed");
        }

        gl::DeleteShader(vs);
        gl::DeleteShader(fs);

        program
    }
}

pub const VERTEX_SHADER: &str = r#"
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;
layout (location = 2) in vec3 aTint;
layout (location = 3) in vec3 aNormal;

out vec2 TexCoord;
out vec3 Tint;
out vec3 Normal;

uniform mat4 view;
uniform mat4 projection;

void main() {
    gl_Position = projection * view * vec4(aPos, 1.0);
    TexCoord = aTexCoord;
    Tint = aTint;
    Normal = aNormal;
}
"#;

pub const FRAGMENT_SHADER: &str = r#"
#version 330 core
in vec2 TexCoord;
in vec3 Tint;
in vec3 Normal;

out vec4 color;

uniform sampler2D blockTexture;
uniform vec3 sunDirection;
uniform float ambientLight;
uniform float sunIntensity;

void main() {
    vec4 texColor = texture(blockTexture, TexCoord);
    vec3 tintedColor = texColor.rgb * Tint;
    
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(sunDirection);
    
    float diff = max(dot(norm, lightDir), 0.0) * sunIntensity;
    float totalLight = ambientLight + diff * (1.0 - ambientLight);
    
    vec3 result = tintedColor * totalLight;
    color = vec4(result, texColor.a);
}
"#;
