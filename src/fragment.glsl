#version 100
precision lowp float;

varying vec2 uv;
varying vec2 uv_screen;
varying vec3 pos_local;
varying vec3 pos_global;

uniform vec3 CameraPos;

void main() {
	vec3 norm = normalize(pos_local); // WORKS ONLY FOR SPHERES
	float light = 0.5 + 0.5 * dot(norm, normalize(vec3(10.0, 20.0, 30.0)));
	float fresnel = pow(1.0 - dot(norm, normalize(CameraPos - pos_global)), 2.0);
	gl_FragColor = vec4(vec3(pow(light, 2.2)), 0.0);
}