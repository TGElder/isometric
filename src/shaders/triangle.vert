#version 330 core

layout (location = 0) in vec3 Position;
uniform mat3 MVP;

void main()
{
    gl_Position = vec4(MVP * Position, 1.0);
}