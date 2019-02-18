#version 450 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Color;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

out VS_OUTPUT {
    vec4 Color;
} OUT;

void main()
{
    gl_Position = projection * view * model * vec4(Position, 1.0);
    OUT.Color = vec4(Color, 1.0);
}