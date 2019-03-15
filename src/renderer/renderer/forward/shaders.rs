pub static SHADER_UNIORMS_MODEL:&'static str = r#"uniform mat4 matModel;"#;
pub static SHADER_UNIORMS_VIEW:&'static str = r#"uniform mat4 matView;"#;
pub static SHADER_UNIORMS_PROJECT:&'static str = r#"uniform mat4 matProject;"#;
pub static SHADER_UNIORMS_VIEWPROJECT:&'static str = r#"uniform mat4 matViewProject;"#;
pub static SHADER_UNIORMS_CAMERAPOSITION:&'static str = r#"uniform vec3 CameraPosition;"#;
pub static SHADER_UNIORMS_EXPOSURE:&'static str = r#"uniform float Exposure;"#;

pub static SHADER_VARYING_POSITION:&'static str = r#" 
	#define SHADER_VARYING_POSITION 1
	varying vec3 vposition;
"#;
pub static SHADER_VARYING_NORMAL:&'static str = r#"
	#define SHADER_VARYING_NORMAL 1
	varying vec3 vnormal;
"#;
pub static SHADER_VARYING_COLOR:&'static str = r#"
	#define SHADER_VARYING_COLOR 1
	varying vec3 vcolor;
"#;
pub static SHADER_VARYING_TEXCOORD0:&'static str = r#"
	#define SHADER_VARYING_TEXCOORD0 1
	varying vec3 vcoord;
"#;
pub static SHADER_VARYING_TEXCOORD1:&'static str = r#"
	#define SHADER_VARYING_TEXCOORD1 1
	varying vec3 vcoord1;
"#;
pub static SHADER_VARYING_TEXCOORD2:&'static str = r#"
	#define SHADER_VARYING_TEXCOORD2 1
	varying vec3 vcoord2;
"#;
pub static SHADER_VARYING_TEXCOORD3:&'static str = r#"
	#define SHADER_VARYING_TEXCOORD3 1
	varying vec3 vcoord3;
"#;
pub static SHADER_VARYING_TEXCOORD4:&'static str = r#"
	#define SHADER_VARYING_TEXCOORD4 1
	varying vec3 vcoord4;
"#;
pub static SHADER_VARYING_TEXCOORD5:&'static str = r#"
	#define SHADER_VARYING_TEXCOORD5 1
	varying vec3 vcoord5;
"#;
pub static SHADER_VARYING_TEXCOORD6:&'static str = r#"
	#define SHADER_VARYING_TEXCOORD6 1
	varying vec3 vcoord6;
"#;
pub static SHADER_VARYING_TEXCOORD7:&'static str = r#"
	#define SHADER_VARYING_TEXCOORD7 1
	varying vec3 vcoord7;
"#;

pub static SHADER_ATTRIB_POSITION:&'static str = r#"attribute vec3 position;"#;
pub static SHADER_ATTRIB_NORMAL:&'static str = r#"attribute vec3 normal;"#;
pub static SHADER_ATTRIB_COLOR:&'static str = r#"attribute vec3 color;"#;
pub static SHADER_ATTRIB_TEXCOORD0:&'static str = r#"attribute vec3 coord;"#;
pub static SHADER_ATTRIB_TEXCOORD1:&'static str = r#"attribute vec3 coord1;"#;
pub static SHADER_ATTRIB_TEXCOORD2:&'static str = r#"attribute vec3 coord2;"#;
pub static SHADER_ATTRIB_TEXCOORD3:&'static str = r#"attribute vec3 coord3;"#;
pub static SHADER_ATTRIB_TEXCOORD4:&'static str = r#"attribute vec3 coord4;"#;
pub static SHADER_ATTRIB_TEXCOORD5:&'static str = r#"attribute vec3 coord5;"#;
pub static SHADER_ATTRIB_TEXCOORD6:&'static str = r#"attribute vec3 coord6;"#;
pub static SHADER_ATTRIB_TEXCOORD7:&'static str = r#"attribute vec3 coord7;"#;

pub static SHADER_CODE_BEGIN:&'static str = r#"
	#define float2 vec2
	#define float3 vec3
	#define float4 vec4

	float saturate(float x) { return clamp(x,0.0,1.0); }
	float2 saturate(float2 x) { return clamp(x,float2(0.0),float2(1.0)); }
	float3 saturate(float3 x) { return clamp(x,float3(0.0),float3(1.0)); }
	float4 saturate(float4 x) { return clamp(x,float4(0.0),float4(1.0)); }

	float  pow2(float x)  { return x * x; }
	float2 pow2(float2 x) { return x * x; }
	float3 pow2(float3 x) { return x * x; }
	float4 pow2(float4 x) { return x * x; }

	float  pow5(float x)  { float  xx = x * x; return xx * xx * x; }
	float2 pow5(float2 x) { float2 xx = x * x; return xx * xx * x; }
	float3 pow5(float3 x) { float3 xx = x * x; return xx * xx * x; }
	float4 pow5(float4 x) { float4 xx = x * x; return xx * xx * x; }

	float sum(float2 v) { return dot(v, float2(1.0)); }
	float sum(float3 v) { return dot(v, float3(1.0)); }
	float sum(float4 v) { return dot(v, float4(1.0)); }

	float length2(float2 v) { return dot(v, v); }
	float length2(float3 v) { return dot(v, v); }
	float length2(float4 v) { return dot(v, v); }

	float min2(float2 v) { return min(v.x, v.y); }
	float max3(float3 v) { return max(v.x, max(v.y, v.z)); }

	float  lerp(float a, float b, float t)    { return mix(a, b, t); }
	float2 lerp(float2 a, float2 b, float t) { return mix(a, b, float2(t)); }
	float3 lerp(float3 a, float3 b, float t) { return mix(a, b, float3(t)); }
	float4 lerp(float4 a, float4 b, float t) { return mix(a, b, float4(t)); }
"#;

pub static SHADER_CODE_END:&'static str = r#"
	struct Args
	{
	#ifdef SHADER_VARYING_POSITION
		float3 position;
	#endif
	#ifdef SHADER_VARYING_NORMAL
		float3 normal;
	#endif
	#ifdef SHADER_VARYING_COLOR
		float3 color;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD0
		float3 coord;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD1
		float3 coord1;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD2
		float3 coord2;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD3
		float3 coord3;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD4
		float3 coord4;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD5
		float3 coord5;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD6
		float3 coord6;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD7
		float3 coord7;
	#endif
	};

	struct Result
	{
	#ifdef SHADER_VARYING_POSITION
		float3 position;
	#endif
	#ifdef SHADER_VARYING_NORMAL
		float3 normal;
	#endif
	#ifdef SHADER_VARYING_COLOR
		float3 color;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD0
		float3 coord;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD1
		float3 coord1;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD2
		float3 coord2;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD3
		float3 coord3;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD4
		float3 coord4;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD5
		float3 coord5;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD6
		float3 coord6;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD7
		float3 coord7;
	#endif
	};

	struct Gbuffer
	{
		float3 albedo;
		float3 specular;
		float3 emissive;
		float3 normal;
		float smoothness;
		float metalness;
	};
"#;

pub static VERT_CODE_HEADER:&'static str = r#"
"#;

pub static VERT_CODE_BODY_HELPER:&'static str = r#"
	float3 transform(float3 position)
	{
		float4 p = matViewProject * matModel * vec4(position, 1.0);
		p /= p.w;
		return p.xyz;
	}

	float4 transform(float4 position)
	{
		return matViewProject * matModel * position;
	}
"#;

pub static VERT_CODE_BODY:&'static str = r#"
	void main()
	{
		Args args;
	#ifdef SHADER_VARYING_POSITION
		args.position = position;
	#endif
	#ifdef SHADER_VARYING_NORMAL
		args.normal = normal;
	#endif
	#ifdef SHADER_VARYING_COLOR
		args.color = color;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD0
		args.coord = coord;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD1
		args.coord1 = coord1;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD2
		args.coord2 = coord2;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD3
		args.coord3 = coord3;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD4
		args.coord4 = coord4;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD5
		args.coord5 = coord5;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD6
		args.coord6 = coord6;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD7
		args.coord7 = coord7;
	#endif

		Result result = main_vs(args);

	#ifdef SHADER_VARYING_POSITION
		vposition = result.position;
	#endif
	#ifdef SHADER_VARYING_NORMAL
		vnormal = result.normal;
	#endif
	#ifdef SHADER_VARYING_COLOR
		vcolor = result.color;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD0
		vcoord = result.coord;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD1
		vcoord1 = result.coord1;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD2
		vcoord2 = result.coord2;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD3
		vcoord3 = result.coord3;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD4
		vcoord4 = result.coord4;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD5
		vcoord5 = result.coord5;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD6
		vcoord6 = result.coord6;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD7
		vcoord7 = result.coord7;
	#endif

		gl_Position = float4(result.position, 1.0);
	}
"#;

pub static FRAG_CODE_HEANDER:&'static str = r#"precision mediump float;"#;

pub static FRAG_CODE_HELPER:&'static str = r#"
	float2 ComputeSphereCoord(float3 normal)
	{
		normal = clamp(normal, -1.0, 1.0);
		float2 coord = float2((atan(normal.x, normal.z) / 3.1415926 * 0.5 + 0.5), acos(normal.y) / 3.1415926);
		return coord;
	}

	float ComputeSpecularMicroOcclusion(float f0)
	{
		return saturate(dot(f0, 0.33333) * 50.0);
	}

	float3 ComputeSpecularMicroOcclusion(float3 f0)
	{
		return float3(saturate(dot(f0, float3(0.33333)) * 50.0));
	}

	float fresnelSchlick(float f0, float f9, float vh)
	{
		return lerp(f0, f9, pow5(1.0 - vh));
	}
	
	float3 fresnelSchlick(float3 f0, float3 f9, float vh)
	{
		return lerp(f0, f9, pow5(1.0 - vh));
	}

	float3 noise3(float2 seed)
	{
		return fract(sin(dot(seed.xy, float2(34.483, 89.637))) * float3(29156.4765, 38273.5639, 47843.7546));
	}

	float3 ColorBanding(float2 uv)
	{
		float3 noise = noise3(uv) + noise3(uv + 0.5789) - 0.5;
		return noise / float3(255.0);
	}

	float3 ColorDithering(float3 color, float2 uv)
	{
		color += ColorBanding(uv);
		return color;
	}

	float BurleyBRDF(float nl, float nv, float vh, float roughness)
	{
		float energyBias = 0.5 * roughness;
		float energyFactor = lerp(1.0, 1.0 / 1.51, roughness);

		float Fd90 = energyBias + 2.0 * vh * vh * roughness;
		float FdV = lerp(1.0, Fd90, pow5(1.0 - max(nv, 0.1)));
		float FdL = lerp(1.0, Fd90, pow5(1.0 - nl));

		return FdV * FdL * energyFactor;
	}

	float DiffuseBRDF(float3 N, float3 L, float3 V, float roughness)
	{
		float3 H = normalize(V + L);

		float nl = saturate(dot(N, L));
		float vh = saturate(dot(V, H));
		float nv = abs(dot(N, V));

		return BurleyBRDF(nl, nv, vh, roughness);
	}

	float3 SpecularBRDF_GGX(float nh, float nl, float vh, float nv, float roughness, float3 specular, float NormalizationFactor)
	{
		float m2 = roughness * roughness;
		float spec = (nh * m2 - nh) * nh + 1.0;
		spec = m2 / (spec * spec) * NormalizationFactor;
	
		float Gv = nl * (nv * (1.0 - roughness) + roughness);
		float Gl = nv * (nl * (1.0 - roughness) + roughness);
		spec *= 0.5 / (Gv + Gl);
	
		float3 f0 = specular;
		float3 f90 = ComputeSpecularMicroOcclusion(f0);
		float3 fresnel = fresnelSchlick(f0, f90, vh);
	
		return fresnel * spec;
	}

	vec4 LightModel(Gbuffer buffer, float3 V, float3 LightColor, float3 LightPosition, float3 L)
	{
		float3 H = normalize(V + L);

		float nh  = saturate(dot(buffer.normal, H));
		float nl  = saturate(dot(buffer.normal, L));
		float vh  = saturate(dot(V, H));
		float nv  = abs(dot(buffer.normal, V)) + 1e-5;

		float roughness = max((1.0 - buffer.smoothness) * (1.0 - buffer.smoothness), 1e-3);

		float3 baseColor = buffer.albedo;
		float3 f0 = lerp(pow2(buffer.specular) * 0.16, baseColor, buffer.metalness);
		float3 color = lerp(baseColor, float3(0), buffer.metalness);

		float3 diffuseLight = color * BurleyBRDF(nl, nv, vh, roughness) * float3(nl);
		float3 specularLight = SpecularBRDF_GGX(nh, nl, vh, nv, roughness, f0, 1.0) * float3(nl);

		return vec4(LightColor * (diffuseLight + specularLight), 1.0);
	}

	vec4 ImageBasedLighting(Gbuffer buffer, sampler2D irradiance, float3 LightColor)
	{
		float3 irr = texture2D(irradiance, ComputeSphereCoord(buffer.normal)).xyz;
		float3 lighting = buffer.albedo * pow(irr, float3(2.2));
		return vec4(LightColor * lighting, 1.0);
	}

	float GetSpotLightAttenuation(float3 L, float3 Ld, float cos_angle)
	{
		float falloff = cos_angle / (saturate(dot(L, Ld)) + 1e-6);	
		float attenuation = 1.0 - saturate(falloff);
		return attenuation;
	}

	float linear2srgb(float v)
	{
	    if (v <= 0.0031308) {
	        return 12.92 * v;
	    } else {
	        return 1.055 * pow(v, 1.0 / 2.4) - 0.055;
	    }
	}

	float3 ACESTonemap(const float3 x)
	{
	    const float a = 2.51;
	    const float b = 0.03;
	    const float c = 2.43;
	    const float d = 0.59;
	    const float e = 0.14;
	    return (x * (a * x + b)) / (x * (c * x + d) + e);
	}
"#;

pub static FRAG_CODE_BODY_BEGIN:&'static str = r#"
void main()
{
	Args args;
	#ifdef SHADER_VARYING_POSITION
		args.position = vposition;
	#endif
	#ifdef SHADER_VARYING_NORMAL
		args.normal = vnormal;
	#endif
	#ifdef SHADER_VARYING_COLOR
		args.color = vcolor;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD0
		args.coord = vcoord;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD1
		args.coord1 = vcoord1;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD2
		args.coord2 = vcoord2;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD3
		args.coord3 = vcoord3;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD4
		args.coord4 = vcoord4;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD5
		args.coord5 = vcoord5;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD6
		args.coord6 = vcoord6;
	#endif
	#ifdef SHADER_VARYING_TEXCOORD7
		args.coord7 = vcoord7;
	#endif

	Gbuffer buffer = main_fs(args);

	vec4 finalColor = vec4(buffer.emissive, 1.0);
"#;

pub static FRAG_CODE_BODY_END:&'static str = r#"
	finalColor.xyz = ACESTonemap(finalColor.xyz * Exposure);

	finalColor.x = linear2srgb(finalColor.x);
	finalColor.y = linear2srgb(finalColor.y);
	finalColor.z = linear2srgb(finalColor.z);
	finalColor.xyz = ColorDithering(finalColor.xyz, vcoord.xy);

	gl_FragColor = finalColor;
}
"#;

pub fn sky_light_uniforms(index:usize) -> String
{
	format!(r#"
		uniform sampler2D LightRadiance{};
		uniform sampler2D LightIrradiance{};
		uniform float3 LightColor{};
		"#, index, index, index)
}

pub fn point_light_uniforms(index:usize) -> String
{
	format!(r#"
		uniform float3 LightPosition{};
		uniform float3 LightColor{};
		"#, index, index)
}

pub fn spot_light_uniforms(index:usize) -> String
{
	format!(r#"
		uniform float3 LightPosition{};
		uniform float3 LightColor{};
		uniform float3 LightDirection{};
		uniform float LightAngle{};
		"#, index, index, index, index)
}

pub fn directional_light_uniforms(index:usize) -> String
{
	format!(r#"
		uniform float3 LightPosition{};
		uniform float3 LightColor{};
		uniform float3 LightDirection{};
		"#, index, index, index)
}

pub fn sky_light_shading(index:usize) -> String
{
	format!("finalColor += ImageBasedLighting(buffer, LightIrradiance{}, LightColor{});", index, index)
}

pub fn point_light_shading(index:usize) -> String
{
	format!("
		float3 V{} = normalize(CameraPosition - vposition.xyz);
		float3 L{} = LightPosition{} - vposition.xyz;
		finalColor += LightModel(buffer, V{}, LightColor{}, LightPosition{}, normalize(L{})) / max(1.0, dot(L{}, L{}));",
		index, index, index, index, index, index, index, index, index
	)
}

pub fn spot_light_shading(index:usize) -> String
{
	format!("
		float3 V{} = normalize(CameraPosition - vposition.xyz);
		float3 L{} = LightPosition{} - vposition.xyz;
		float3 Ld{} = normalize(L{});
		float3 lighting{} = LightModel(buffer, V{}, LightColor{}, LightPosition{}, Ld{}).xyz;
		lighting{} /= max(1.0, dot(L{}, L{}));
		lighting{} *= GetSpotLightAttenuation(Ld{}, LightDirection{}, LightAngle{});
		finalColor.xyz += lighting{};",
		index, index, index, index, index, index, index, index, index, index, index, index, index, index, index, index, index, index
	)
}

pub fn directional_light_shading(index:usize) -> String
{
	format!("
		float3 V{} = normalize(CameraPosition - vposition.xyz);
		float3 L{} = -LightDirection{};
		finalColor += LightModel(buffer, V{}, LightColor{}, LightPosition{}, L{});",
		index, index, index, index, index, index, index
	)
}