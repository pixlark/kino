#version 330 core

layout (location = 0) in vec2 SourcePos;
layout (location = 1) in uint SourceIndex;

uniform vec2 ScreenSize;
uniform vec2 RectTopLeft;
uniform vec2 RectBottomRight;

void main()
{
    vec2 ScreenRadius = ScreenSize / 2.0f;

    vec2 NormTopLeft = RectTopLeft / ScreenRadius;
    vec2 NormBottomRight = RectBottomRight / ScreenRadius;

    vec2 OffsetTopLeft = NormTopLeft - vec2(1.0f, 1.0f);
    vec2 OffsetBottomRight = NormBottomRight - vec2(1.0f, 1.0f);

    // Reflect over Y axis
    vec2 BottomLeft = OffsetTopLeft * vec2(1.0, -1.0);
    vec2 TopRight = OffsetBottomRight * vec2(1.0, -1.0);

    vec2 TopLeft = vec2(BottomLeft.x, TopRight.y);
    vec2 BottomRight = vec2(TopRight.x, BottomLeft.y);

    vec2 Pos;

    switch (SourceIndex) {
        case uint(0):
            Pos = TopLeft;
            break;
        case uint(1):
            Pos = vec2(BottomRight.x, TopLeft.y);
            break;
        case uint(2):
            Pos = vec2(TopLeft.x, BottomRight.y);
            break;
        case uint(3):
        default:
            Pos = BottomRight;
            break;
    }

    gl_Position = vec4(Pos, 0.0f, 1.0f);
}
