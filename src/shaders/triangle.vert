#version 330 core

layout (location = 0) in vec3 Position;
uniform mat4 MVP;
uniform mat4 scale;

void main()
{
    gl_Position = MVP * scale * vec4(Position, 1.0);
}