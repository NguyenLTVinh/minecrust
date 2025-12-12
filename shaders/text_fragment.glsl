#version 330 core

uniform sampler2D font;

in vec2 uv;
out vec4 color;

void main() {
    vec4 tex_color = texture(font, uv);
    if (tex_color.a < 0.5) {
        discard;
    }
    color = tex_color;
}
