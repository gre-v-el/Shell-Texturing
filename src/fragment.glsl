#version 100
precision lowp float;

varying vec2 uv;
varying vec2 uv_screen;
varying vec3 pos_local;
varying vec3 pos_global;
varying vec3 normal;

uniform vec3 CameraPos;

void main() {
	float light = 0.5 + 0.5 * dot(normal, normalize(vec3(10.0, 20.0, 30.0)));
	float fresnel = pow(1.0 - dot(normal, normalize(CameraPos - pos_global)), 8.0);
	gl_FragColor = vec4(vec3(pow(light + fresnel, 2.2)), 0.0);
}