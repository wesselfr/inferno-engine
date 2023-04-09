#version 410
layout (location = 0) in vec2 _pos;

uniform mat4 _mvp;
out vec2 uvCoords;

void main() {
    gl_Position = _mvp * vec4(_pos.x, _pos.y, 0.0, 1.0);
    uvCoords = _pos;
}