#version 100

attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;

varying lowp vec2 uv;
varying lowp vec2 uv_screen;
varying lowp vec3 normal;
varying lowp vec3 pos_local;
varying lowp vec3 pos_global;

uniform mat4 Model;
uniform mat4 Projection;
uniform vec3 CameraPos;

void main() {
	normal = normalize(color0.xyz - 0.5);

    vec4 res = Projection * Model * vec4(position, 1);

    uv_screen = res.xy / 2.0 + vec2(0.5, 0.5);
    uv = texcoord;
	pos_local = position;
	pos_global = pos_local + (Model * vec4(0.0, 0.0, 0.0, 1.0)).xyz;

    gl_Position = res;
}