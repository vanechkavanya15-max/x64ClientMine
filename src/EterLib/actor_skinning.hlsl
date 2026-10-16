// ============================================================================
// Metin2 Modernized Hardware Skeletal Skinning Shader
// HLSL Shader Model 3.0 (Vertex & Pixel Shader)
// Zero CPU Deformation -> GPU Vertex Transformation
// ============================================================================
// Constant Register Mapping:
//   c0 - c3  : g_matViewProj (View-Projection Matrix)
//   c4 - c60 : g_matBones (Bone Matrix Palette, 4x3 row-major matrices)
//   c61      : g_vLightDir (Normalized world space light direction)
//   c62      : g_vLightDiffuse (Light diffuse color RGBA)
//   c63      : g_vLightAmbient (Light ambient color RGBA)
//   c64      : g_vMaterialColor (Actor material diffuse tint & alpha)
//   c65      : g_vFogParams (x = Near, y = Far, z = 1.0 / (Far - Near), w = Enable)
//
// Sampler Mapping:
//   s0       : DiffuseTexture (Model diffuse texture)
// ============================================================================

float4x4 g_matViewProj       : register(c0);
float4x3 g_matBones[60]      : register(c4); // Bone matrix palette starting at c4..c60

float4   g_vLightDir         : register(c61);
float4   g_vLightDiffuse     : register(c62);
float4   g_vLightAmbient     : register(c63);
float4   g_vMaterialColor    : register(c64);
float4   g_vFogParams        : register(c65);

sampler2D DiffuseTexture     : register(s0);

struct VS_INPUT
{
    float3 vPosition     : POSITION;
    float4 vBlendWeights : BLENDWEIGHT;
    float4 vBlendIndices : BLENDINDICES;
    float3 vNormal       : NORMAL;
    float2 vTexCoord     : TEXCOORD0;
};

struct VS_OUTPUT
{
    float4 vPosition     : POSITION;
    float2 vTexCoord     : TEXCOORD0;
    float4 vColor        : COLOR0;
    float  fFog          : FOG;
};

VS_OUTPUT vs_main(VS_INPUT input)
{
    VS_OUTPUT output;

    // Compute normalized blend weights
    float4 weights = input.vBlendWeights;
    float remainderWeight = 1.0f - (weights.x + weights.y + weights.z);
    weights.w = max(0.0f, remainderWeight);

    float3 skinnedPos = float3(0.0f, 0.0f, 0.0f);
    float3 skinnedNormal = float3(0.0f, 0.0f, 0.0f);

    int4 indices = D3DCOLORtoUBYTE4(input.vBlendIndices);

    // 4-bone vertex skinning deformation on GPU
    for (int i = 0; i < 4; ++i)
    {
        int boneIdx = indices[i];
        float w = weights[i];

        if (w > 0.0001f)
        {
            float3 localPos = mul(float4(input.vPosition, 1.0f), g_matBones[boneIdx]);
            skinnedPos += localPos * w;

            float3 localNorm = mul(input.vNormal, (float3x3)g_matBones[boneIdx]);
            skinnedNormal += localNorm * w;
        }
    }

    skinnedNormal = normalize(skinnedNormal);

    // Transform skinned vertex to clip space
    output.vPosition = mul(float4(skinnedPos, 1.0f), g_matViewProj);
    output.vTexCoord = input.vTexCoord;

    // Diffuse lighting calculation (Lambertian + Ambient)
    float nDotL = max(0.0f, dot(skinnedNormal, -g_vLightDir.xyz));
    float3 diffuseLight = g_vLightAmbient.rgb + (g_vLightDiffuse.rgb * nDotL);
    output.vColor = float4(diffuseLight * g_vMaterialColor.rgb, g_vMaterialColor.a);

    // Depth fog computation
    if (g_vFogParams.w > 0.5f)
    {
        float depth = output.vPosition.z;
        output.fFog = saturate((g_vFogParams.y - depth) * g_vFogParams.z);
    }
    else
    {
        output.fFog = 1.0f;
    }

    return output;
}

struct PS_INPUT
{
    float2 vTexCoord     : TEXCOORD0;
    float4 vColor        : COLOR0;
};

float4 ps_main(PS_INPUT input) : COLOR0
{
    float4 texColor = tex2D(DiffuseTexture, input.vTexCoord);
    return texColor * input.vColor;
}
