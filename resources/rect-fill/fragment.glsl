#version 330 core

out vec4 frag_Color;

uniform vec4 RectColor;

void main()
{
    frag_Color = RectColor;
}
