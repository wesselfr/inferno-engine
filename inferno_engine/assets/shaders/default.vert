#version 410
layout (location = 0) in vec2 _pos;

uniform mat4 _mvp;
out vec2 uvCoords;

float map(float value, float min1, float max1, float min2, float max2) {
  return min2 + (value - min1) * (max2 - min2) / (max1 - min1);
}

void main() {
    gl_Position = _mvp * vec4(_pos.x, _pos.y, 0.0, 1.0);
    uvCoords = vec2(map(_pos.x, -1.0, 1.0, 0.0, 1.0), map(_pos.y, -1.0, 1.0, 0.0, 1.0));
}