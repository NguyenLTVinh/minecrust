#version 330 core

layout(location=0) in int letter;

uniform vec2 resolution;
uniform vec2 message_position;
uniform float message_scale;

out vec2 uv;

#define FONT_SHEET_WIDTH 128
#define FONT_SHEET_HEIGHT 64
#define FONT_SHEET_COLS 18
#define FONT_SHEET_ROWS 7
#define FONT_CHAR_WIDTH (FONT_SHEET_WIDTH / FONT_SHEET_COLS)
#define FONT_CHAR_HEIGHT (FONT_SHEET_HEIGHT / FONT_SHEET_ROWS)

void main() {
    vec2 mesh_position = vec2(
        float(gl_VertexID & 1),
        float((gl_VertexID >> 1) & 1));

    vec2 screen_position =
        mesh_position * vec2(float(FONT_CHAR_WIDTH), float(FONT_CHAR_HEIGHT)) * message_scale +
        message_position +
        vec2(float(FONT_CHAR_WIDTH) * message_scale * float(gl_InstanceID), 0.0);

    vec2 ndc = 2.0 * screen_position / resolution - 1.0;
    ndc.y = -ndc.y;
    
    gl_Position = vec4(ndc, 0.0, 1.0);

    int char_index = letter - 32;
    float char_u = (float(char_index % FONT_SHEET_COLS) + mesh_position.x) * float(FONT_CHAR_WIDTH) / float(FONT_SHEET_WIDTH);
    float char_v = (float(char_index / FONT_SHEET_COLS) + mesh_position.y) * float(FONT_CHAR_HEIGHT) / float(FONT_SHEET_HEIGHT);
    uv = vec2(char_u, char_v);
}
