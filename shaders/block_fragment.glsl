#version 330 core
in vec2 TexCoord;
in vec3 Tint;
in vec3 Normal;

out vec4 color;

uniform sampler2D blockTexture;
uniform vec3 sunDirection;
uniform float ambientLight;
uniform float sunIntensity;
uniform float wickedTime;

void main() {
    vec4 texColor = texture(blockTexture, TexCoord);
    
    if (texColor.a < 0.5) {
        discard;
    }
    
    vec3 tintedColor = texColor.rgb * Tint;
    
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(sunDirection);
    
    float diff = max(dot(norm, lightDir), 0.0) * sunIntensity;
    float totalLight = ambientLight + diff * (1.0 - ambientLight);
    
    vec3 result = tintedColor * totalLight;
    color = vec4(result, 1.0);
}
