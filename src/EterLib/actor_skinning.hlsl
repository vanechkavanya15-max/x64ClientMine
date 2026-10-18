// ============================================================================
// Metin2 Modernized Hardware Skeletal Skinning Shader
// HLSL Shader Model 3.0 (Vertex & Pixel Shader)
// Zero CPU Deformation -> GPU Vertex Transformation
// ============================================================================
// Constant Register Mapping (Collision-Free):
//   c0   - c3   : g_matViewProj    (View-Projection Matrix)
//   c4   - c243 : g_matBones[80]   (Bone Matrix Palette, 80 * 4x3 matrices = 240 regs)
//   c244        : g_vLightDir      (Normalized world space light direction)
//   c245        : g_vLightDiffuse  (Light diffuse color RGBA)
//   c246        : g_vLightAmbient  (Light ambient color RGBA)
//   c247        : g_vMaterialColor (Actor material diffuse tint & alpha)
//   c248        : g_vFogParams     (x = Near, y = Far, z = 1.0 / (Far - Near), w = Enable)
//   c249 - c252 : g_matWorld       (World Transformation Matrix)
//
// Sampler Mapping:
//   s0          : DiffuseTexture   (Model diffuse texture)
// ============================================================================

float4x4 g_matViewProj       : register(c0);
float4x3 g_matBones[80]      : register(c4);   // Bone matrix palette occupying c4..c243

float4   g_vLightDir         : register(c244);
float4   g_vLightDiffuse     : register(c245);
float4   g_vLightAmbient     : register(c246);
float4   g_vMaterialColor    : register(c247);
float4   g_vFogParams        : register(c248);
float4x4 g_matWorld          : register(c249); // World matrix occupying c249..c252

sampler2D DiffuseTexture     : register(s0);

struct VS_INPUT
{
    float3 vPosition     : POSITION0;
    float3 vNormal       : NORMAL0;
    float2 vTexCoord     : TEXCOORD0;
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

VS_OUTPUT vs_main(VS_INPUT input)
{
    VS_OUTPUT output;

    // Normalize blend weights safely
    float4 weights = max(input.vBlendWeights, 0.0f);
    float weightSum = weights.x + weights.y + weights.z + weights.w;

    float3 skinnedPos = float3(0.0f, 0.0f, 0.0f);
    float3 skinnedNormal = float3(0.0f, 0.0f, 0.0f);

    if (weightSum > 0.0001f)
    {
        weights /= weightSum;
        int4 indices = int4(round(input.vBlendIndices));

        // 4-bone vertex skinning deformation on GPU
        [unroll]
        for (int i = 0; i < 4; ++i)
        {
            int boneIdx = clamp(indices[i], 0, 79);
            float w = weights[i];

            if (w > 0.0001f)
            {
                float3 localPos = mul(float4(input.vPosition, 1.0f), g_matBones[boneIdx]);
                skinnedPos += localPos * w;

                float3 localNorm = mul(input.vNormal, (float3x3)g_matBones[boneIdx]);
                skinnedNormal += localNorm * w;
            }
        }
    }
    else
    {
        skinnedPos = input.vPosition;
        skinnedNormal = input.vNormal;
    }

    // Transform skinned vertex to world space and then clip space
    float4 worldPos = mul(float4(skinnedPos, 1.0f), g_matWorld);
    float3 worldNormal = mul(skinnedNormal, (float3x3)g_matWorld);
    float normalLenSq = dot(worldNormal, worldNormal);
    skinnedNormal = (normalLenSq > 1e-8f) ? (worldNormal * rsqrt(normalLenSq)) : float3(0.0f, 0.0f, 1.0f);

    output.vPosition = mul(worldPos, g_matViewProj);
    output.vTexCoord = input.vTexCoord;

    // Diffuse lighting calculation (Lambertian + Ambient)
    float3 lightDir = (dot(g_vLightDir.xyz, g_vLightDir.xyz) > 1e-8f) ? normalize(g_vLightDir.xyz) : float3(0.0f, 0.0f, -1.0f);
    float nDotL = max(0.0f, dot(skinnedNormal, -lightDir));
    float3 diffuseLight = saturate(g_vLightAmbient.rgb + (g_vLightDiffuse.rgb * nDotL));
    output.vColor = float4(diffuseLight * g_vMaterialColor.rgb, g_vMaterialColor.a);

    // Depth fog computation
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
