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

uniform float WindPower;
uniform float WindSpeed;
uniform float WindTurbulence;
uniform float Time;
uniform vec3 CameraPos;
uniform int NumShells;
uniform int CurShell;
uniform float LengthVar;
uniform float Jitter;
uniform float Thickness;
uniform float Profile;
uniform float Density;
uniform vec3 SkinCol;
uniform vec3 BaseCol;
uniform vec3 TopCol;
uniform vec3 Ambient;
uniform float Shading;

//https://iquilezles.org/articles/smoothvoronoi
vec3 hash2f( vec2 p )
{
    vec3 q = vec3( dot(p,vec2(127.1,311.7)), 
				   dot(p,vec2(269.5,183.3)), 
				   dot(p,vec2(419.2,371.9)) );
	return fract(sin(q)*43758.5453);
}

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

// https://www.shadertoy.com/view/4dBcWy
vec3 mod289(vec3 x) {
  return x - floor(x * (1.0 / 289.0)) * 289.0;
}

vec4 mod289(vec4 x) {
  return x - floor(x * (1.0 / 289.0)) * 289.0;
}

vec4 permute(vec4 x) {
     return mod289(((x*34.0)+1.0)*x);
}

vec4 taylorInvSqrt(vec4 r)
{
  return 1.79284291400159 - 0.85373472095314 * r;
}

float snoise(vec3 v)
  { 
  const vec2  C = vec2(0.1666666666666667, 0.3333333333333333) ; // 1.0/6.0, 1.0/3.0
  const vec4  D = vec4(0.0, 0.5, 1.0, 2.0);

// First corner
  vec3 i  = floor(v + dot(v, C.yyy) );
  vec3 x0 =   v - i + dot(i, C.xxx) ;

// Other corners
  vec3 g = step(x0.yzx, x0.xyz);
  vec3 l = 1.0 - g;
  vec3 i1 = min( g.xyz, l.zxy );
  vec3 i2 = max( g.xyz, l.zxy );

  vec3 x1 = x0 - i1 + C.xxx;
  vec3 x2 = x0 - i2 + C.yyy; // 2.0*C.x = 1/3 = C.y
  vec3 x3 = x0 - D.yyy;      // -1.0+3.0*C.x = -0.5 = -D.y

// Permutations
  i = mod289(i); 
  vec4 p = permute( permute( permute( 
             i.z + vec4(0.0, i1.z, i2.z, 1.0 ))
           + i.y + vec4(0.0, i1.y, i2.y, 1.0 )) 
           + i.x + vec4(0.0, i1.x, i2.x, 1.0 ));

// Gradients: 7x7 points over a square, mapped onto an octahedron.
// The ring size 17*17 = 289 is close to a multiple of 49 (49*6 = 294)
  float n_ = 0.142857142857; // 1.0/7.0
  vec3  ns = n_ * D.wyz - D.xzx;

  vec4 j = p - 49.0 * floor(p * ns.z * ns.z);  //  mod(p,7*7)

  vec4 x_ = floor(j * ns.z);
  vec4 y_ = floor(j - 7.0 * x_ );    // mod(j,N)

  vec4 x = x_ *ns.x + ns.yyyy;
  vec4 y = y_ *ns.x + ns.yyyy;
  vec4 h = 1.0 - abs(x) - abs(y);

  vec4 b0 = vec4( x.xy, y.xy );
  vec4 b1 = vec4( x.zw, y.zw );

  //vec4 s0 = vec4(lessThan(b0,0.0))*2.0 - 1.0;
  //vec4 s1 = vec4(lessThan(b1,0.0))*2.0 - 1.0;
  vec4 s0 = floor(b0)*2.0 + 1.0;
  vec4 s1 = floor(b1)*2.0 + 1.0;
  vec4 sh = -step(h, vec4(0.0));

  vec4 a0 = b0.xzyw + s0.xzyw*sh.xxyy ;
  vec4 a1 = b1.xzyw + s1.xzyw*sh.zzww ;

  vec3 p0 = vec3(a0.xy,h.x);
  vec3 p1 = vec3(a0.zw,h.y);
  vec3 p2 = vec3(a1.xy,h.z);
  vec3 p3 = vec3(a1.zw,h.w);

//Normalise gradients
  vec4 norm = taylorInvSqrt(vec4(dot(p0,p0), dot(p1,p1), dot(p2, p2), dot(p3,p3)));
  p0 *= norm.x;
  p1 *= norm.y;
  p2 *= norm.z;
  p3 *= norm.w;

// Mix final noise value
  vec4 m = max(0.6 - vec4(dot(x0,x0), dot(x1,x1), dot(x2,x2), dot(x3,x3)), 0.0);
  m = m * m;
  return 42.0 * dot( m*m, vec4( dot(p0,x0), dot(p1,x1), 
                                dot(p2,x2), dot(p3,x3) ) );
  }

// custom

float profile(float h) {
	float p = (Profile < 0.5) ? (0.5 / Profile) : 2.0*(1.0 - Profile);
	return pow(1.0 - pow(h, 2.0*Profile), p);
}

void main() {
	vec3 ambient = pow(Ambient, vec3(1.0 / 2.2));
	vec3 skin_col = pow(SkinCol, vec3(1.0 / 2.2));
	vec3 top_col =  pow(TopCol,  vec3(1.0 / 2.2));
	vec3 base_col = pow(BaseCol, vec3(1.0 / 2.2));

	// one voronoi cell is a single fur strand
	vec2 vor = voronoi(uv * Density, Jitter);
	float rand = vor.x;
	
	float strand_height = 1.0 - LengthVar*rand;
	float h = float(CurShell) / float(NumShells - 1) / strand_height;
	
	// wind - domain warping
	vec2 uv_new = uv + h*vec2(snoise(pos_local_surface*WindTurbulence + vec3(Time*WindSpeed))) * WindPower;
	vec2 vor_new = voronoi(uv_new * Density, Jitter);
	
	rand = vor_new.x;
	strand_height = 1.0 - LengthVar*rand;
	h = float(CurShell) / float(NumShells - 1) / strand_height;

	float in_strand = step(vor_new.y, Thickness * profile(h));

	if(in_strand < 0.5 && CurShell != 0 || h > 1.0) {
		discard;
	}

	float skin_mask = step(float(CurShell), 0.5);
	vec3 col = mix(mix(base_col, top_col, h), skin_col, skin_mask);

	float light = clamp((1.0-Shading) + Shading * dot(normal, normalize(vec3(10.0, 20.0, 30.0))), 0.0, 1.0) * (1.0 - length(Ambient));
	
	fragColor = vec4(pow(col * light + col * ambient, vec3(2.2)), 0.0);
}