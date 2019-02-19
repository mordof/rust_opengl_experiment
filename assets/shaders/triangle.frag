#version 450 core

in vec4 VertColor;
out vec4 Color;

void main()
{
    Color = vec4(VertColor);
}