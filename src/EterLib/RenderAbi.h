#ifndef __RENDER_ABI_H__
#define __RENDER_ABI_H__

#ifdef __cplusplus
#include <cstdint>
#include <cstddef>
#include <cstring>
extern "C" {
#else
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <string.h>
#endif

#if defined(__cplusplus)
    #define RENDER_ABI_ALIGN(n) alignas(n)
#elif defined(_MSC_VER)
    #define RENDER_ABI_ALIGN(n) __declspec(align(n))
#elif defined(__GNUC__) || defined(__clang__)
    #define RENDER_ABI_ALIGN(n) __attribute__((aligned(n)))
#else
    #define RENDER_ABI_ALIGN(n)
#endif

#if defined(__cplusplus)
    #define RENDER_ABI_STATIC_ASSERT(cond, msg) static_assert(cond, msg)
#elif (defined(__STDC_VERSION__) && (__STDC_VERSION__ >= 201112L))
    #define RENDER_ABI_STATIC_ASSERT(cond, msg) _Static_assert(cond, msg)
#else
    #define RENDER_ABI_SA_CONCAT_INNER(a, b) a##b
    #define RENDER_ABI_SA_CONCAT(a, b) RENDER_ABI_SA_CONCAT_INNER(a, b)
    #define RENDER_ABI_STATIC_ASSERT(cond, msg) typedef char RENDER_ABI_SA_CONCAT(__render_abi_sa_, __LINE__)[(cond) ? 1 : -1]
#endif

/* Typy bazowe czystego C-ABI */
typedef union TRenderMatrix4x4
{
    float m[16];
    struct
    {
        float _11, _12, _13, _14;
        float _21, _22, _23, _24;
        float _31, _32, _33, _34;
        float _41, _42, _43, _44;
    };
} TRenderMatrix4x4;

typedef struct TRenderVec2
{
    union
    {
        struct { float x, y; };
        struct { float u, v; };
        float v2[2];
    };
} TRenderVec2;

typedef struct TRenderVec3
{
    union
    {
        struct { float x, y, z; };
        float v3[3];
    };
} TRenderVec3;

typedef struct TRenderVec4
{
    union
    {
        struct { float x, y, z, w; };
        struct { float r, g, b, a; };
        float v4[4];
    };
} TRenderVec4;

typedef struct TRect32
{
    int32_t left;
    int32_t top;
    int32_t right;
    int32_t bottom;
} TRect32;

typedef uint32_t TRenderColorARGB;

typedef enum EEffectBlendMode
{
    EFFECT_BLEND_ALPHABLEND = 0,
    EFFECT_BLEND_ADDITIVE = 1
} EEffectBlendMode;

typedef enum ERenderBlendFactor
{
    BLEND_FACTOR_ZERO = 1,
    BLEND_FACTOR_ONE = 2,
    BLEND_FACTOR_SRC_COLOR = 3,
    BLEND_FACTOR_INV_SRC_COLOR = 4,
    BLEND_FACTOR_SRC_ALPHA = 5,
    BLEND_FACTOR_INV_SRC_ALPHA = 6,
    BLEND_FACTOR_DEST_ALPHA = 7,
    BLEND_FACTOR_INV_DEST_ALPHA = 8,
    BLEND_FACTOR_DEST_COLOR = 9,
    BLEND_FACTOR_INV_DEST_COLOR = 10,
    BLEND_FACTOR_SRC_ALPHA_SAT = 11,
    BLEND_FACTOR_BOTH_SRC_ALPHA = 12,
    BLEND_FACTOR_BOTH_INV_SRC_ALPHA = 13,
    BLEND_FACTOR_BLEND_FACTOR = 14,
    BLEND_FACTOR_INV_BLEND_FACTOR = 15,

    RENDER_BLEND_ZERO = 1,
    RENDER_BLEND_ONE = 2,
    RENDER_BLEND_SRC_COLOR = 3,
    RENDER_BLEND_INV_SRC_COLOR = 4,
    RENDER_BLEND_SRC_ALPHA = 5,
    RENDER_BLEND_INV_SRC_ALPHA = 6,
    RENDER_BLEND_DEST_ALPHA = 7,
    RENDER_BLEND_INV_DEST_ALPHA = 8,
    RENDER_BLEND_DEST_COLOR = 9,
    RENDER_BLEND_INV_DEST_COLOR = 10,
    RENDER_BLEND_SRC_ALPHA_SAT = 11
} ERenderBlendFactor;

typedef enum EParticleBillboardType
{
    PARTICLE_BILLBOARD_ALL = 1,
    PARTICLE_BILLBOARD_Y = 2,
    PARTICLE_BILLBOARD_LIE = 3,
    PARTICLE_BILLBOARD_2FACE = 4,
    PARTICLE_BILLBOARD_3FACE = 5,
    PARTICLE_BILLBOARD_STRETCH = 6
} EParticleBillboardType;

typedef enum EUIBlendMode
{
    UI_BLEND_NORMAL = 0,
    UI_BLEND_SCREEN = 1,
    UI_BLEND_ADDITIVE = 2,
    UI_BLEND_MODULATE = 3,
    UI_BLEND_LCD_PASS1 = 4,
    UI_BLEND_LCD_PASS2 = 5
} EUIBlendMode;

typedef struct TPipelineStateKey
{
    uint8_t srcBlend;
    uint8_t destBlend;
    uint8_t cullMode;
    uint8_t zTest;
    uint8_t zWrite;
    uint8_t alphaRef;
    uint8_t fogEnable;
    uint8_t colorWrite;
} TPipelineStateKey;

typedef enum EMaterialType
{
    MATERIAL_DIFFUSE = 0,
    MATERIAL_LIGHTMAP_MODULATE = 1,
    MATERIAL_LIGHTMAP_MODULATE2X = 2,
    MATERIAL_ADDITIVE = 3,
    MATERIAL_SUBTRACTIVE = 4
} EMaterialType;

#if !defined(RENDER_MATRIX_TYPE)
    #define RENDER_MATRIX_TYPE TRenderMatrix4x4
#endif
#if !defined(RENDER_VEC2_TYPE)
    #define RENDER_VEC2_TYPE TRenderVec2
#endif
#if !defined(RENDER_VEC3_TYPE)
    #define RENDER_VEC3_TYPE TRenderVec3
#endif
#if !defined(RENDER_VEC4_TYPE)
    #define RENDER_VEC4_TYPE TRenderVec4
#endif
#if !defined(RENDER_COLOR4_TYPE)
    #define RENDER_COLOR4_TYPE TRenderVec4
#endif
#if !defined(RENDER_RECT_TYPE)
    #define RENDER_RECT_TYPE TRect32
#endif
#if !defined(RENDER_TREE_WRAPPER_TYPE)
    #define RENDER_TREE_WRAPPER_TYPE void
#endif

/* 1. TUIVertex: 24-bajtowy wierzcholek interfejsu (Vec3 + ARGB uint32 + Vec2) */
typedef struct TUIVertex
{
    RENDER_VEC3_TYPE position;
    uint32_t color;
    RENDER_VEC2_TYPE texCoord;

#ifdef __cplusplus
    TUIVertex()
    {
        std::memset(this, 0, sizeof(*this));
    }
#endif
} TUIVertex;

/* 2. SPNT2Vertex / TPNT2Vertex: 40-bajtowy wierzcholek dwuteksturowy */
#ifndef _SPNT2VERTEX_DEFINED_
#define _SPNT2VERTEX_DEFINED_
typedef struct SPNT2Vertex
{
    RENDER_VEC3_TYPE position;
    RENDER_VEC3_TYPE normal;
    union
    {
        struct
        {
            RENDER_VEC2_TYPE texCoord;
            RENDER_VEC2_TYPE texCoord2;
        };
        struct
        {
            RENDER_VEC2_TYPE uv0;
            RENDER_VEC2_TYPE uv1;
        };
    };
} TPNT2Vertex;
#endif

/* 3. TRenderFrameUniforms: 288 bajtow (18x vec4, wyrownanie 16B) */
typedef struct RENDER_ABI_ALIGN(16) TRenderFrameUniforms
{
    RENDER_MATRIX_TYPE matView;
    RENDER_MATRIX_TYPE matProj;
    RENDER_MATRIX_TYPE matViewProj;

    /* Slot 0 (offset 192, 16B): Camera Position + Global Time */
    RENDER_VEC3_TYPE vCameraPos;
    float fTime;

    /* Slot 1 (offset 208, 16B): Sun Light Direction + Frame Index */
    RENDER_VEC3_TYPE vSunLightDir;
    uint32_t dwFrameIndex;

    /* Slot 2 (offset 224, 16B): Sun Light Diffuse Color */
    union
    {
        RENDER_COLOR4_TYPE vSunLightColor;
        RENDER_VEC4_TYPE v4SunColor;
    };

    /* Slot 3 (offset 240, 16B): Ambient Light Color */
    union
    {
        RENDER_COLOR4_TYPE vAmbientColor;
        RENDER_VEC4_TYPE v4AmbientColor;
    };

    /* Slot 4 (offset 256, 16B): Fog Color */
    RENDER_COLOR4_TYPE vFogColor;

    /* Slot 5 (offset 272, 16B): Fog Range & Linear Scale Params */
    union
    {
        struct
        {
            float fFogNear;
            float fFogFar;
            float fFogInvRange;
            float fFogEnable;
        };
        RENDER_VEC4_TYPE v4FogParams;
    };

#ifdef __cplusplus
    TRenderFrameUniforms()
    {
        std::memset(this, 0, sizeof(*this));
    }
#endif
} TRenderFrameUniforms;

/* Forward declaration dla TTerrainPassPacket */
typedef struct TTerrainPassPacket TTerrainPassPacket;

/* 4. TTerrainSectorPacket: 176 bajtow, wyrownanie 16B */
typedef struct RENDER_ABI_ALIGN(16) TTerrainSectorPacket
{
    int16_t nCoordX;
    int16_t nCoordY;
    uint32_t dwSectorX;
    uint32_t dwSectorY;
    uint16_t sectorX;
    uint16_t sectorY;
    float fWorldPosX;
    float fWorldPosY;
    float fMinHeight;
    float fMaxHeight;
    union
    {
        uint32_t dwTextureSetID;
        uint32_t dwPrimitiveType;
    };
    union
    {
        uint32_t dwPatchCount;
        uint32_t dwPrimitiveCount;
    };
    uint32_t patchNum;
    uint32_t splatTextureId;
    void* pSplatTexture;
    uint32_t layerTextureIds[4];
    void* pLayerTextures[4];
    void* pVertexData;
    uint32_t dwVertexCount;
    union
    {
        const uint32_t* pSplatWeights;
        const TTerrainPassPacket* pPassPackets;
        const void* pPassData;
    };
    union
    {
        uint32_t dwSplatWeightCount;
        uint32_t dwPassCount;
    };
    union
    {
        const void* pHeightMap;
        void* pIndexBuffer;
    };
    union
    {
        const void* pTileMap;
        void* pShadowTexture;
    };
    union
    {
        const void* pNormalMap;
        void* pChrShadowTexture;
    };
    uint8_t bHasWater;
    uint8_t byBand;
    uint8_t byLODLevel;
    uint8_t bDrawShadow;
    float fWaterHeight;
    union
    {
        float fReserved;
        uint32_t dwShadowFlags;
    };

#ifdef __cplusplus
    TTerrainSectorPacket()
    {
        std::memset(this, 0, sizeof(*this));
    }

    [[nodiscard]] bool HasShadow() const noexcept { return bDrawShadow != 0; }
    [[nodiscard]] bool HasCharacterShadow() const noexcept { return (dwShadowFlags & 0xFF) != 0; }
    void SetCharacterShadow(bool bEnable) noexcept { dwShadowFlags = (dwShadowFlags & ~0xFFu) | (bEnable ? 1u : 0u); }
#endif
} TTerrainSectorPacket;

/* 5. TActorRenderPacket: 5008 bajtow, wyrownanie 16B */
typedef struct RENDER_ABI_ALIGN(16) TActorRenderPacket
{
    uint32_t dwActorID;
    uint32_t dwMeshID;
    uint32_t dwTextureID;
    uint32_t dwBoneCount;
    RENDER_MATRIX_TYPE matWorld;
    const RENDER_MATRIX_TYPE* pBoneMatrices;
    RENDER_MATRIX_TYPE boneMatrices[64];
    uint32_t dwWeaponMeshID;
    uint32_t dwWeaponTextureID;
    RENDER_MATRIX_TYPE matWeaponSocket;
    RENDER_MATRIX_TYPE matWeaponLeftSocket;
    RENDER_MATRIX_TYPE matHelmetSocket;
    RENDER_MATRIX_TYPE matHairSocket;
    uint32_t dwAuraEffectCount;
    RENDER_MATRIX_TYPE matAuraEffectSockets[8];
    uint32_t dwAuraEffectIDs[8];
    uint8_t bRenderShadow;
    uint8_t bIsDead;
    uint8_t bSelected;
    uint8_t bAlphaBlend;
    float fAlpha;
    float fShadowSize;

#ifdef __cplusplus
    TActorRenderPacket()
    {
        std::memset(this, 0, sizeof(*this));
    }

    [[nodiscard]] const RENDER_MATRIX_TYPE* GetBoneMatrices() const noexcept
    {
        return pBoneMatrices ? pBoneMatrices : boneMatrices;
    }
#endif
} TActorRenderPacket;

/* 6. TWaterPacket: 176 bajtow, wyrownanie 16B (ABI v2 z macierza UV i pasmem near/far) */
typedef struct RENDER_ABI_ALIGN(16) TWaterPacket
{
    float fWorldPosX;
    float fWorldPosY;
    uint32_t dwSectorX;
    uint32_t dwSectorY;
    uint16_t sectorX;
    uint16_t sectorY;
    float fWaterHeight;
    float fWaveTime;
    float fWaveFrequency;
    float fWaveAmplitude;
    RENDER_COLOR4_TYPE vWaterColor;
    float fAlpha;
    uint32_t dwWaterTextureID;
    uint32_t dwPadTexture;
    void* pWaterTexture;
    uint32_t dwCellCount;
    uint32_t dwVertexCount;
    void* pVertexBuffer;
    uint32_t dwFaceCount;
    uint32_t patchNum;
    uint8_t bFarPass;
    uint8_t bAlphaBlend;
    uint8_t bPadWater[2];
    float fFogDistance;
    float fPad;
    uint32_t dwPadAlign16;
    union
    {
        RENDER_MATRIX_TYPE matTexTransformWater;
        RENDER_MATRIX_TYPE matUV;
    };

#ifdef __cplusplus
    TWaterPacket()
    {
        std::memset(this, 0, sizeof(*this));
        fWaveFrequency = 1.5f;
        fWaveAmplitude = 5.0f;
#if defined(RENDER_COLOR4_TYPE) && !defined(RENDER_ABI_C_ONLY)
        vWaterColor = RENDER_COLOR4_TYPE(0.2f, 0.5f, 0.7f, 0.6f);
#else
        vWaterColor.r = 0.2f; vWaterColor.g = 0.5f; vWaterColor.b = 0.7f; vWaterColor.a = 0.6f;
#endif
        fAlpha = 0.6f;
        bAlphaBlend = 1;
        bFarPass = 0;
    }
#endif
} TWaterPacket;

/* 7. TSkyBoxPacket: 112 bajtow, wyrownanie 16B */
typedef struct RENDER_ABI_ALIGN(16) TSkyBoxPacket
{
    RENDER_MATRIX_TYPE matWorld;
    uint32_t dwFaceCount;
    uint32_t dwCloudTextureID;
    uint32_t dwSkyTextureID;
    float fCloudScale;
    float fCloudHeight;
    float fCloudScrollSpeed;
    float fPad;
    RENDER_COLOR4_TYPE vSkyColor;

#ifdef __cplusplus
    TSkyBoxPacket()
    {
        std::memset(this, 0, sizeof(*this));
        fCloudScale = 1.0f;
        vSkyColor.r = 0.0f; vSkyColor.g = 0.0f; vSkyColor.b = 0.0f; vSkyColor.a = 1.0f;
    }
#endif
} TSkyBoxPacket;

/* 8. TSpeedTreeInstanceData: 112 bajtow, wyrownanie 16B */
typedef struct RENDER_ABI_ALIGN(16) TSpeedTreeInstanceData
{
    const RENDER_TREE_WRAPPER_TYPE* pTreeInstance;
    RENDER_MATRIX_TYPE matWorld;
    RENDER_VEC3_TYPE v3Position;
    uint32_t dwTreeCRC;
    float fWindStrength;
    union
    {
        float fPad[3];
        struct
        {
            float fAlphaRefBranch;
            float fAlphaRefFrond;
            float fAlphaRefLeaf;
        };
        struct
        {
            uint8_t ucAlphaRefBranch;
            uint8_t ucAlphaRefFrond;
            uint8_t ucAlphaRefLeaf0;
            uint8_t ucAlphaRefLeaf1;
            int8_t nBranchLod;
            int8_t nFrondLod;
            int8_t nLeafLod0;
            int8_t nLeafLod1;
            uint8_t ucAlphaRefBillboard0;
            uint8_t ucAlphaRefBillboard1;
            uint16_t wInstanceFlags;
        };
    };

#ifdef __cplusplus
    TSpeedTreeInstanceData()
    {
        std::memset(this, 0, sizeof(*this));
    }
#endif
} TSpeedTreeInstanceData;

/* 9. TSpeedTreePacket: 48 bajtow, wyrownanie 16B */
typedef struct RENDER_ABI_ALIGN(16) TSpeedTreePacket
{
    RENDER_TREE_WRAPPER_TYPE* pMainTree;
    uint32_t dwTreeCRC;
    uint8_t bRenderBranches;
    uint8_t bRenderFronds;
    uint8_t bRenderLeaves;
    uint8_t bRenderBillboards;
    uint32_t dwTreeTypeID;
    uint32_t dwTrunkTextureID;
    uint32_t dwLeafTextureID;
    uint32_t dwInstanceCount;
    const TSpeedTreeInstanceData* pInstances;
    union
    {
        float fPad[2];
        struct
        {
            float fAlphaRef;
            uint32_t dwFlags;
        };
        struct
        {
            uint8_t ucAlphaRefBranch;
            uint8_t ucAlphaRefFrond;
            uint8_t ucAlphaRefLeaf;
            uint8_t ucAlphaRefBillboard;
            int16_t nDiscreteLod;
            uint16_t wFlags;
        };
    };

#ifdef __cplusplus
    TSpeedTreePacket()
    {
        std::memset(this, 0, sizeof(*this));
    }
#endif
} TSpeedTreePacket;

/* 9b. TSpeedTreeBillboardVertex: 20 bajtow (Vec3 + Vec2, D3DFVF_XYZ | D3DFVF_TEX1) */
typedef struct TSpeedTreeBillboardVertex
{
    RENDER_VEC3_TYPE position;
    RENDER_VEC2_TYPE texCoord;

#ifdef __cplusplus
    TSpeedTreeBillboardVertex()
    {
        std::memset(this, 0, sizeof(*this));
    }
    TSpeedTreeBillboardVertex(float x, float y, float z, float u, float v)
    {
#if defined(RENDER_VEC3_TYPE) && !defined(RENDER_ABI_C_ONLY)
        position = RENDER_VEC3_TYPE(x, y, z);
        texCoord = RENDER_VEC2_TYPE(u, v);
#else
        position.x = x; position.y = y; position.z = z;
        texCoord.x = u; texCoord.y = v;
#endif
    }
#endif
} TSpeedTreeBillboardVertex;

/* 10. TParticleInstanceData: 64 bajty, wyrownanie 16B */
typedef struct RENDER_ABI_ALIGN(16) TParticleInstanceData
{
    RENDER_VEC4_TYPE vWorldPos;
    RENDER_COLOR4_TYPE vColor;
    RENDER_VEC4_TYPE vTexCoordRect;
    float fRotation;
    float fLifeRatio;
    float fHalfWidth;
    float fHalfHeight;

#ifdef __cplusplus
    TParticleInstanceData()
    {
        std::memset(this, 0, sizeof(*this));
    }
#endif
} TParticleInstanceData;

/* 11. TEffectInstancePacket: 48 bajtow, wyrownanie 16B */
typedef struct RENDER_ABI_ALIGN(16) TEffectInstancePacket
{
    uint32_t dwEffectID;
    uintptr_t dwTextureID;
    union
    {
        uint32_t dwBlendMode;
        struct
        {
            uint8_t srcBlend;
            uint8_t destBlend;
            uint8_t billboardMode;
            uint8_t bStretch;
        };
    };
    uint32_t dwParticleCount;
    const TParticleInstanceData* pParticles;
    float fPad[2];

#ifdef __cplusplus
    TEffectInstancePacket()
    {
        std::memset(this, 0, sizeof(*this));
    }
#endif
} TEffectInstancePacket;

/* 12. TUIBatchPacket: 64 bajty, wyrownanie 16B */
typedef struct RENDER_ABI_ALIGN(16) TUIBatchPacket
{
    uintptr_t dwTextureID;
    uint32_t dwBlendMode;
    uint32_t dwVertexCount;
    uint32_t bUseScissor;
    RENDER_RECT_TYPE scissorRect;
    const TUIVertex* pVertices;
    float fPad[2];

#ifdef __cplusplus
    TUIBatchPacket()
    {
        std::memset(this, 0, sizeof(*this));
    }
#endif
} TUIBatchPacket;

/* 13. TDungeonBlockPacket: 160 bajtow, wyrownanie 16B */
typedef struct RENDER_ABI_ALIGN(16) TDungeonBlockPacket
{
    RENDER_MATRIX_TYPE matWorld;
    uintptr_t dwDiffuseTextureID;
    uintptr_t dwLightmapTextureID;
    void* pDiffuseTexture;
    void* pLightmapTexture;
    void* pVertexBuffer;
    void* pIndexBuffer;
    const TPNT2Vertex* pVertices;
    const uint16_t* pIndices;
    uint32_t dwVertexCount;
    uint32_t dwIndexCount;
    uint32_t dwPrimitiveCount;
    uint32_t dwStartIndex;
    int32_t nBaseVertexIndex;
    float fAlpha;
    uint8_t bAlphaBlend;
    uint8_t bPadDungeon[3];
    float fPad;

#ifdef __cplusplus
    TDungeonBlockPacket()
    {
        std::memset(this, 0, sizeof(*this));
        matWorld._11 = 1.0f;
        matWorld._22 = 1.0f;
        matWorld._33 = 1.0f;
        matWorld._44 = 1.0f;
        fAlpha = 1.0f;
    }
#endif
} TDungeonBlockPacket;

/* 14. TRenderFramePacket: 416 bajtow, wyrownanie 16B (ABI v2 z sekcja lochow) */
typedef struct RENDER_ABI_ALIGN(16) TRenderFramePacket
{
    TRenderFrameUniforms uniforms;
    const TTerrainSectorPacket* pTerrainPackets;
    uint32_t dwTerrainCount;
    uint32_t dwPad0;
    const TWaterPacket* pWaterPackets;
    uint32_t dwWaterCount;
    uint32_t dwPad1;
    const TSpeedTreePacket* pSpeedTreePackets;
    uint32_t dwSpeedTreeCount;
    uint32_t dwPad2;
    const TActorRenderPacket* pActorPackets;
    uint32_t dwActorCount;
    uint32_t dwPad3;
    const TEffectInstancePacket* pEffectPackets;
    uint32_t dwEffectCount;
    uint32_t dwPad4;
    const TUIBatchPacket* pUIPackets;
    uint32_t dwUICount;
    uint32_t dwPad5;
    const TSkyBoxPacket* pSkyBoxPackets;
    uint32_t dwSkyBoxCount;
    uint32_t dwPad6;
    const TDungeonBlockPacket* pDungeonPackets;
    uint32_t dwDungeonCount;
    uint32_t dwPad7;

#ifdef __cplusplus
    TRenderFramePacket()
    {
        std::memset(this, 0, sizeof(*this));
    }
#endif
} TRenderFramePacket;

/* 15. TTerrainPassPacket: 96 bajtow */
typedef struct TTerrainPassPacket
{
    uint32_t layerIndex;
    uint32_t alphaSplatHandle;
    uint32_t lodLevel;
    uint32_t indexBufferHandle;
    TRenderMatrix4x4 matLayer;
    float nearDistance;
    float fogDistance;
    float farDistance;
    uint32_t shadowTextureHandle;
} TTerrainPassPacket;

/* 16. TEffectMeshPacket: 88 bajtow */
typedef struct TEffectMeshPacket
{
    union
    {
        TRenderMatrix4x4 matWorld;
        TRenderMatrix4x4 m_matWorld;
    };
    union
    {
        uint32_t textureHandle;
        uint32_t hTexture;
    };
    union
    {
        uint32_t textureFactorColor;
        uint32_t dwTFactor;
    };
    union
    {
        uint32_t colorOp;
        uint32_t dwColorOp;
    };
    uint8_t srcBlend;
    uint8_t destBlend;
    uint8_t bPad[2];
    uint32_t vertexOffset;
    uint32_t vertexCount;

#ifdef __cplusplus
    TEffectMeshPacket()
    {
        std::memset(this, 0, sizeof(*this));
    }
#endif
} TEffectMeshPacket;

#ifndef _EMODELPASS_DEFINED_
#define _EMODELPASS_DEFINED_
#ifdef __cplusplus
enum class EModelPass : uint32_t
{
    Opaque = 0,
    Blend = 1,
    ShadowDepth = 2
};
#else
typedef enum EModelPass
{
    MODEL_PASS_OPAQUE = 0,
    MODEL_PASS_BLEND = 1,
    MODEL_PASS_SHADOW_DEPTH = 2
} EModelPass;
#endif
#endif

/* 17. TModelDrawItem: 96 bajtow */
typedef struct TModelDrawItem
{
    union
    {
        uint32_t modelId;
        uint32_t model;
    };
    union
    {
        uint32_t subMeshIndex;
        uint32_t submeshIndex;
    };
    uint32_t materialHandle;
    union
    {
        uint32_t passType;
#ifdef __cplusplus
        EModelPass pass;
#endif
    };
    TRenderMatrix4x4 matWorld;
    float fAlpha;
    uint32_t addColor;
    uint32_t materialColor;
    union
    {
        uint32_t dwPad;
        struct
        {
            uint8_t bTwoSided;
            uint8_t bPad[3];
        };
    };

#ifdef __cplusplus
    TModelDrawItem()
    {
        std::memset(this, 0, sizeof(*this));
        matWorld._11 = 1.0f;
        matWorld._22 = 1.0f;
        matWorld._33 = 1.0f;
        matWorld._44 = 1.0f;
        fAlpha = 1.0f;
        materialColor = 0xffffffff;
        pass = EModelPass::Opaque;
    }
#endif
} TModelDrawItem;

/* 18. TWeaponTracePacket: 24 bajty */
typedef struct TWeaponTracePacket
{
    uint32_t textureHandle;
    TPipelineStateKey stateKey;
    uint32_t vertexOffset;
    uint32_t vertexCount;
    uint32_t dwPad;
} TWeaponTracePacket;

/* 19. TDebugLinePacket: 32 bajty */
typedef struct TDebugLinePacket
{
    TRenderVec3 v3Start;
    TRenderVec3 v3End;
    uint32_t dwColor;
    uint8_t bXRay;
    uint8_t bPad[3];
} TDebugLinePacket;

typedef enum ERenderPassType
{
    RENDER_PASS_MAIN = 0,
    RENDER_PASS_CHARACTER_SHADOW = 1,
    RENDER_PASS_MINIMAP = 2,
    RENDER_PASS_UI = 3,
    RENDER_PASS_CUSTOM = 4
} ERenderPassType;

/* 20. TRenderPassDesc: 144 bajty, wyrownanie 16B dla optymalizacji SIMD */
typedef struct RENDER_ABI_ALIGN(16) TRenderPassDesc
{
    uint32_t passType;
    uint32_t targetHandle;
    uint32_t clearColor;
    uint8_t bClearTarget;
    uint8_t bPad[3];
    TRenderMatrix4x4 matView;
    TRenderMatrix4x4 matProj;

#ifdef __cplusplus
    TRenderPassDesc()
    {
        std::memset(this, 0, sizeof(*this));
    }
#endif
} TRenderPassDesc;

/* 21. TParticlePacket: 96 bajtow, wyrownanie 16B (v2 pakiet czasteczki z pelna wiernoscia parametrow) */
typedef struct RENDER_ABI_ALIGN(16) TParticlePacket
{
    RENDER_VEC3_TYPE vPosition;
    float fHalfWidth;
    RENDER_VEC3_TYPE vVelocity;
    float fHalfHeight;
    RENDER_COLOR4_TYPE vColor;
    RENDER_VEC4_TYPE vTexCoordRect;
    float fRotation;
    uint8_t srcBlend;
    uint8_t destBlend;
    uint8_t billboardMode;
    uint8_t bStretch;
    uint32_t dwEffectID;
    float fPad0;
    uintptr_t dwTextureID;
    float fPad1[2];

#ifdef __cplusplus
    TParticlePacket()
    {
        std::memset(this, 0, sizeof(*this));
        vTexCoordRect.x = 0.0f;
        vTexCoordRect.y = 0.0f;
        vTexCoordRect.z = 1.0f;
        vTexCoordRect.w = 1.0f;
    }
#endif
} TParticlePacket;

/* 22. TShadowPassData: 272 bajty, wyrownanie 16B */
typedef struct RENDER_ABI_ALIGN(16) TShadowPassData
{
    TRenderPassDesc passDesc;           /* offset 0, 144B */
    TRenderMatrix4x4 matDynamicShadow;  /* offset 144, 64B */
    TRenderVec3 v3LightDir;             /* offset 208, 12B */
    float fRange;                       /* offset 220, 4B */
    TRenderVec3 v3TargetPos;            /* offset 224, 12B */
    uint16_t wShadowMapSize;            /* offset 236, 2B */
    uint16_t wPad;                      /* offset 238, 2B */
    uint32_t hRenderTarget;             /* offset 240, 4B */
    TRect32 viewport;                   /* offset 244, 16B */
    float fPad[3];                      /* offset 260, 12B */

#ifdef __cplusplus
    TShadowPassData()
    {
        std::memset(this, 0, sizeof(*this));
    }
#endif
} TShadowPassData;

/* Statyczne asercje rozmiarow i wyrownan struktur pakietowych C-ABI */
RENDER_ABI_STATIC_ASSERT(sizeof(TRenderMatrix4x4) == 64, "ABI: TRenderMatrix4x4 == 64B");
RENDER_ABI_STATIC_ASSERT(sizeof(TRenderVec2) == 8, "ABI: TRenderVec2 == 8B");
RENDER_ABI_STATIC_ASSERT(sizeof(TRenderVec3) == 12, "ABI: TRenderVec3 == 12B");
RENDER_ABI_STATIC_ASSERT(sizeof(TRenderVec4) == 16, "ABI: TRenderVec4 == 16B");
RENDER_ABI_STATIC_ASSERT(sizeof(TRect32) == 16, "ABI: TRect32 == 16B");
RENDER_ABI_STATIC_ASSERT(sizeof(TPipelineStateKey) == 8, "ABI: TPipelineStateKey == 8B");

RENDER_ABI_STATIC_ASSERT(sizeof(TUIVertex) == 24, "ABI: TUIVertex == 24B");
RENDER_ABI_STATIC_ASSERT(sizeof(TPNT2Vertex) == 40, "ABI: TPNT2Vertex == 40B");
RENDER_ABI_STATIC_ASSERT(sizeof(TRenderFrameUniforms) == 288, "ABI: TRenderFrameUniforms == 288B");
RENDER_ABI_STATIC_ASSERT(sizeof(TTerrainSectorPacket) == 176, "ABI: TTerrainSectorPacket == 176B");
RENDER_ABI_STATIC_ASSERT(sizeof(TActorRenderPacket) == 5008, "ABI: TActorRenderPacket == 5008B");
RENDER_ABI_STATIC_ASSERT(sizeof(TWaterPacket) == 176, "ABI: TWaterPacket == 176B");
RENDER_ABI_STATIC_ASSERT(sizeof(TSpeedTreeInstanceData) == 112, "ABI: TSpeedTreeInstanceData == 112B");
RENDER_ABI_STATIC_ASSERT(sizeof(TSpeedTreePacket) == 48, "ABI: TSpeedTreePacket == 48B");
RENDER_ABI_STATIC_ASSERT(sizeof(TParticleInstanceData) == 64, "ABI: TParticleInstanceData == 64B");
RENDER_ABI_STATIC_ASSERT(sizeof(TEffectInstancePacket) == 48, "ABI: TEffectInstancePacket == 48B");
RENDER_ABI_STATIC_ASSERT(sizeof(TUIBatchPacket) == 64, "ABI: TUIBatchPacket == 64B");
RENDER_ABI_STATIC_ASSERT(sizeof(TSkyBoxPacket) == 112, "ABI: TSkyBoxPacket == 112B");
RENDER_ABI_STATIC_ASSERT(sizeof(TDungeonBlockPacket) == 160, "ABI: TDungeonBlockPacket == 160B");
RENDER_ABI_STATIC_ASSERT(sizeof(TRenderFramePacket) == 416, "ABI: TRenderFramePacket == 416B");
RENDER_ABI_STATIC_ASSERT(sizeof(TTerrainPassPacket) == 96, "ABI: TTerrainPassPacket == 96B");
RENDER_ABI_STATIC_ASSERT(sizeof(TEffectMeshPacket) == 88, "ABI: TEffectMeshPacket == 88B");
RENDER_ABI_STATIC_ASSERT(sizeof(TModelDrawItem) == 96, "ABI: TModelDrawItem == 96B");
RENDER_ABI_STATIC_ASSERT(sizeof(TWeaponTracePacket) == 24, "ABI: TWeaponTracePacket == 24B");
RENDER_ABI_STATIC_ASSERT(sizeof(TDebugLinePacket) == 32, "ABI: TDebugLinePacket == 32B");
RENDER_ABI_STATIC_ASSERT(sizeof(TRenderPassDesc) == 144, "ABI: TRenderPassDesc == 144B");
RENDER_ABI_STATIC_ASSERT(sizeof(TParticlePacket) == 96, "ABI: TParticlePacket == 96B");
RENDER_ABI_STATIC_ASSERT(sizeof(TSpeedTreeBillboardVertex) == 20, "ABI: TSpeedTreeBillboardVertex == 20B");
RENDER_ABI_STATIC_ASSERT(sizeof(TShadowPassData) == 272, "ABI: TShadowPassData == 272B");

#ifdef __cplusplus
}
#endif

#endif /* __RENDER_ABI_H__ */
