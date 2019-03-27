#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Color;
layout (location = 2) in vec2 TexCoord;
layout (location = 3) in vec2 Offset;

uniform mat4 projection;
uniform float z_mod;

out VS_OUTPUT {
    vec3 Color;
    vec2 TexCoord;
} OUT;

void main()
{
    gl_Position = projection * vec4(Position, 1.0);
    gl_Position.x += Offset.x;
    gl_Position.y += Offset.y;
    gl_Position.z = -1.0;
    OUT.Color = Color;
    OUT.TexCoord = TexCoord;
}