#version 130
precision lowp float;

in vec2 uv;
in vec2 uv_screen;
in vec3 pos_local;
in vec3 pos_local_surface;
in vec3 pos_global;
in vec3 pos_global_surface;
in vec3 normal;

out vec4 fragColor;

uniform vec3 CameraPos;
uniform int NumShells;
uniform int CurShell;
uniform float LengthVar;
uniform float Jitter;
uniform float Thickness;
uniform float Profile;
uniform float Density;

vec3 hash2f( vec2 p )
{
    vec3 q = vec3( dot(p,vec2(127.1,311.7)), 
				   dot(p,vec2(269.5,183.3)), 
				   dot(p,vec2(419.2,371.9)) );
	return fract(sin(q)*43758.5453);
}


//https://iquilezles.org/articles/smoothvoronoi
vec2 voronoi( in vec2 x, float interp )
{
    x += vec2(0.5 * interp);

    ivec2 base = ivec2( x );
    vec2  relativePosition = fract( x );

    float dist = 8.0;
    float col = 1.0;
    
    for( int j=-1; j<=1; j++ ){
        for( int i=-1; i<=1; i++ )
        {
            ivec2 currentCell = ivec2( i, j );

            vec3 hash = hash2f( vec2(base + currentCell) );

            vec2  r = vec2( currentCell ) - relativePosition + interp*hash.xy;
            float d = length(r);

            if(dist > d){
                dist = d;
                col = hash.z;
            }
        }
    }
    return vec2(col, dist);
}

// https://www.shadertoy.com/view/WttXWX
uint triple32(uint x)
{
    x ^= x >> 17;
    x *= 0xed5ad4bbU;
    x ^= x >> 11;
    x *= 0xac4c1b51U;
    x ^= x >> 15;
    x *= 0x31848babU;
    x ^= x >> 14;
    return x;
}

float hash(uint x) {
    return ( float( triple32(x) ) / float( 0xffffffffU ) );
}

float hash2(uvec2 x) {
    return hash(x.x + triple32(x.y));
}

// custom

float profile(float h) {
	float p = (Profile < 0.5) ? (0.5 / Profile) : 2.0*(1.0 - Profile);
	return pow(1.0 - pow(h, 2.0*Profile), p);
}

void main() {
	vec2 vor = voronoi(uv * Density, Jitter);
	float rand = vor.x;
	float dist_to_center = vor.y;

	float strand_height = 1.0 - LengthVar*rand;

	float h = float(CurShell) / float(NumShells - 1) / strand_height;
	if(h > 1.0) {
		discard;
	}

	float in_strand = step(dist_to_center, Thickness * profile(h));

	if(in_strand == 0.0 && CurShell != 0) {
		discard;
	}

	float light = (0.5 + 0.5 * dot(normal, normalize(vec3(10.0, 20.0, 30.0)))) * (0.5 + 0.5*h);
	float fresnel = pow(1.0 - dot(normal, normalize(CameraPos - pos_global)), 8.0);
	
	// fragColor = vec4(vec3(strand_height), 0.0);
	fragColor = vec4(vec3(pow(light, 2.2)), 0.0);
}