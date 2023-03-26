in vec2 _pos;
uniform mat4 _mvp;
out vec2 vert;

void main() {
    vert = _pos;
    gl_Position = _mvp * vec4(_pos.x, _pos.y, 1.0, 1.0);
}