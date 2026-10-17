// ============================================================================
// Metin2 Modernized Hardware Skeletal Skinning Shader
// HLSL Shader Model 3.0 (Vertex & Pixel Shader)
// Zero CPU Deformation -> GPU Vertex Transformation (WGPU Transition Ready)
// ============================================================================
// Constant Register Mapping (Collision-Free Layout):
//   c0  - c3   : g_matViewProj    (View-Projection Matrix, column-major)
//   c4  - c7   : g_matWorld       (World Matrix of the actor/part)
//   c8         : g_vLightDir      (xyz = Normalized world light dir, w = unused)
//   c9         : g_vLightDiffuse  (Light diffuse color RGBA)
//   c10        : g_vLightAmbient  (Light ambient color RGBA)
//   c11        : g_vMaterialColor (Actor material diffuse tint & alpha)
//   c12        : g_vFogParams     (x = Near, y = Far, z = 1.0/(Far-Near), w = Enable)
//   c13        : g_vSkinningInfo  (x = Active bone count, y = UseGPU, z = 0, w = 0)
//   c16 - c195 : g_matBones[180]  (60 bones * 3 float4 registers = 180 registers)
//
// Sampler Mapping:
//   s0         : DiffuseTexture   (Model diffuse texture)
// ============================================================================

float4x4 g_matViewProj       : register(c0);
float4x4 g_matWorld          : register(c4);

float4   g_vLightDir         : register(c8);
float4   g_vLightDiffuse     : register(c9);
float4   g_vLightAmbient     : register(c10);
float4   g_vMaterialColor    : register(c11);
float4   g_vFogParams        : register(c12);
float4   g_vSkinningInfo     : register(c13);

// Each bone is stored as 3x float4 rows (row0, row1, row2 of affine 4x4 matrix)
// Total registers: 60 * 3 = 180 (c16 .. c195)
float4   g_matBones[180]     : register(c16);

sampler2D DiffuseTexture     : register(s0);

struct VS_INPUT
{
    float3 vPosition     : POSITION0;
    float3 vNormal       : NORMAL0;
    float2 vTexCoord0    : TEXCOORD0;
    float2 vTexCoord1    : TEXCOORD1;
    float4 vBlendIndices : BLENDINDICES0;
    float4 vBlendWeights : BLENDWEIGHT0;
};

struct VS_OUTPUT
{
    float4 vPosition     : POSITION0;
    float2 vTexCoord     : TEXCOORD0;
    float4 vColor        : COLOR0;
    float  fFog          : FOG;
};

float3 TransformBonePosition(float3 pos, int boneIdx)
{
    int baseReg = boneIdx * 3;
    float4 r0 = g_matBones[baseReg + 0];
    float4 r1 = g_matBones[baseReg + 1];
    float4 r2 = g_matBones[baseReg + 2];

    return float3(
        dot(float4(pos, 1.0f), r0),
        dot(float4(pos, 1.0f), r1),
        dot(float4(pos, 1.0f), r2)
    );
}

float3 TransformBoneNormal(float3 norm, int boneIdx)
{
    int baseReg = boneIdx * 3;
    float3 r0 = g_matBones[baseReg + 0].xyz;
    float3 r1 = g_matBones[baseReg + 1].xyz;
    float3 r2 = g_matBones[baseReg + 2].xyz;

    return float3(
        dot(norm, r0),
        dot(norm, r1),
        dot(norm, r2)
    );
}

VS_OUTPUT vs_main(VS_INPUT input)
{
    VS_OUTPUT output;

    float3 localPos = input.vPosition;
    float3 localNormal = input.vNormal;

    // Normalize blend weights if active
    float4 weights = max(input.vBlendWeights, 0.0f);
    float weightSum = weights.x + weights.y + weights.z + weights.w;

    float3 skinnedPos = float3(0.0f, 0.0f, 0.0f);
    float3 skinnedNormal = float3(0.0f, 0.0f, 0.0f);

    if (g_vSkinningInfo.x > 0.5f && weightSum > 0.0001f)
    {
        weights /= weightSum;

        int4 indices = int4(round(input.vBlendIndices));

        [unroll]
        for (int i = 0; i < 4; ++i)
        {
            float w = weights[i];
            if (w > 0.0001f)
            {
                int boneIdx = clamp(indices[i], 0, 59);
                skinnedPos    += TransformBonePosition(localPos, boneIdx) * w;
                skinnedNormal += TransformBoneNormal(localNormal, boneIdx) * w;
            }
        }
    }
    else
    {
        skinnedPos = localPos;
        skinnedNormal = localNormal;
    }

    // Transform from Model Space to World Space
    float4 worldPos = mul(float4(skinnedPos, 1.0f), g_matWorld);
    float3 worldNormal = normalize(mul(skinnedNormal, (float3x3)g_matWorld));

    // Transform from World Space to Clip Space
    output.vPosition = mul(worldPos, g_matViewProj);
    output.vTexCoord = input.vTexCoord0;

    // Lambertian + Ambient Lighting
    float nDotL = max(0.0f, dot(worldNormal, -normalize(g_vLightDir.xyz)));
    float3 diffuseLight = saturate(g_vLightAmbient.rgb + (g_vLightDiffuse.rgb * nDotL));
    output.vColor = float4(diffuseLight * g_vMaterialColor.rgb, g_vMaterialColor.a);

    // Linear Depth Fog
    if (g_vFogParams.w > 0.5f)
    {
        float depth = output.vPosition.w;
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
