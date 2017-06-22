#version 120

attribute vec2 a_Pos;
attribute vec3 a_Color;

varying vec4 v_Color;

uniform mat4 u_Transform;

void main() {
    v_Color = vec4(a_Color, 1.0);
    gl_Position = u_Transform * vec4(a_Pos, 0.0, 1.0);
}
