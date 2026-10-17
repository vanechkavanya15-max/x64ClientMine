#pragma once

#include <d3d9.h>
#include <d3dx9.h>
#include <cstdint>
#include <cstring>

class CSpeedTreeWrapper;

#define RENDER_MATRIX_TYPE D3DXMATRIX
#define RENDER_VEC2_TYPE D3DXVECTOR2
#define RENDER_VEC3_TYPE D3DXVECTOR3
#define RENDER_VEC4_TYPE D3DXVECTOR4
#define RENDER_COLOR4_TYPE D3DXCOLOR
#define RENDER_RECT_TYPE RECT
#define RENDER_TREE_WRAPPER_TYPE CSpeedTreeWrapper

#include "RenderAbi.h"

// Pelna zgodnosc i konwersje z typami D3DX
inline D3DXMATRIX& ToD3DX(TRenderMatrix4x4& m) noexcept { return *reinterpret_cast<D3DXMATRIX*>(&m); }
inline const D3DXMATRIX& ToD3DX(const TRenderMatrix4x4& m) noexcept { return *reinterpret_cast<const D3DXMATRIX*>(&m); }
inline TRenderMatrix4x4& ToAbi(D3DXMATRIX& m) noexcept { return *reinterpret_cast<TRenderMatrix4x4*>(&m); }
inline const TRenderMatrix4x4& ToAbi(const D3DXMATRIX& m) noexcept { return *reinterpret_cast<const TRenderMatrix4x4*>(&m); }

inline D3DXVECTOR2& ToD3DX(TRenderVec2& v) noexcept { return *reinterpret_cast<D3DXVECTOR2*>(&v); }
inline const D3DXVECTOR2& ToD3DX(const TRenderVec2& v) noexcept { return *reinterpret_cast<const D3DXVECTOR2*>(&v); }
inline TRenderVec2& ToAbi(D3DXVECTOR2& v) noexcept { return *reinterpret_cast<TRenderVec2*>(&v); }
inline const TRenderVec2& ToAbi(const D3DXVECTOR2& v) noexcept { return *reinterpret_cast<const TRenderVec2*>(&v); }

inline D3DXVECTOR3& ToD3DX(TRenderVec3& v) noexcept { return *reinterpret_cast<D3DXVECTOR3*>(&v); }
inline const D3DXVECTOR3& ToD3DX(const TRenderVec3& v) noexcept { return *reinterpret_cast<const D3DXVECTOR3*>(&v); }
inline TRenderVec3& ToAbi(D3DXVECTOR3& v) noexcept { return *reinterpret_cast<TRenderVec3*>(&v); }
inline const TRenderVec3& ToAbi(const D3DXVECTOR3& v) noexcept { return *reinterpret_cast<const TRenderVec3*>(&v); }

inline D3DXVECTOR4& ToD3DX(TRenderVec4& v) noexcept { return *reinterpret_cast<D3DXVECTOR4*>(&v); }
inline const D3DXVECTOR4& ToD3DX(const TRenderVec4& v) noexcept { return *reinterpret_cast<const D3DXVECTOR4*>(&v); }
inline TRenderVec4& ToAbi(D3DXVECTOR4& v) noexcept { return *reinterpret_cast<TRenderVec4*>(&v); }
inline const TRenderVec4& ToAbi(const D3DXVECTOR4& v) noexcept { return *reinterpret_cast<const TRenderVec4*>(&v); }

inline D3DXCOLOR& ToD3DColor(TRenderVec4& v) noexcept { return *reinterpret_cast<D3DXCOLOR*>(&v); }
inline const D3DXCOLOR& ToD3DColor(const TRenderVec4& v) noexcept { return *reinterpret_cast<const D3DXCOLOR*>(&v); }

inline RECT& ToRect(TRect32& r) noexcept { return *reinterpret_cast<RECT*>(&r); }
inline const RECT& ToRect(const TRect32& r) noexcept { return *reinterpret_cast<const RECT*>(&r); }
inline TRect32& ToAbi(RECT& r) noexcept { return *reinterpret_cast<TRect32*>(&r); }
inline const TRect32& ToAbi(const RECT& r) noexcept { return *reinterpret_cast<const TRect32*>(&r); }

namespace EterLib::Render
{
	struct TActorBonePalette
	{
		const D3DXMATRIX* pBoneMatrices{ nullptr };
		uint32_t boneCount{ 0 };
	};
}

// Twarde asercje ABI x64 - musza byc identyczne z const assert! w E:\metin2ToRust\crates\m2-client-render\src\packets.rs
#if defined(_WIN64)
#include <cstddef>
static_assert(sizeof(TTerrainSectorPacket) == 176, "ABI: TTerrainSectorPacket != packets.rs");
static_assert(sizeof(TDebugLinePacket) == 32, "ABI: TDebugLinePacket == 32");
static_assert(sizeof(TActorRenderPacket) == 5008, "ABI: TActorRenderPacket != packets.rs");
static_assert(sizeof(TSpeedTreeInstanceData) == 112, "ABI: TSpeedTreeInstanceData != packets.rs");
static_assert(sizeof(TWaterPacket) == 176, "ABI: TWaterPacket != packets.rs");
static_assert(alignof(TWaterPacket) == 16, "ABI: TWaterPacket align != 16");
static_assert(offsetof(TWaterPacket, dwWaterTextureID) == 56, "ABI: water dwWaterTextureID");
static_assert(offsetof(TWaterPacket, pWaterTexture) == 64, "ABI: water pWaterTexture");
static_assert(offsetof(TWaterPacket, bFarPass) == 96, "ABI: water bFarPass");
static_assert(offsetof(TWaterPacket, bAlphaBlend) == 97, "ABI: water bAlphaBlend");
static_assert(offsetof(TWaterPacket, fFogDistance) == 100, "ABI: water fFogDistance");
static_assert(offsetof(TWaterPacket, matTexTransformWater) == 112, "ABI: water matTexTransformWater");
static_assert(sizeof(TSpeedTreePacket) == 48, "ABI: TSpeedTreePacket != packets.rs");
static_assert(sizeof(TParticleInstanceData) == 64, "ABI: TParticleInstanceData != packets.rs");
static_assert(sizeof(TEffectInstancePacket) == 48, "ABI: TEffectInstancePacket != packets.rs");
static_assert(sizeof(TUIVertex) == 24, "ABI: TUIVertex != packets.rs");
static_assert(sizeof(TUIBatchPacket) == 64, "ABI: TUIBatchPacket != packets.rs");
static_assert(sizeof(TRenderFramePacket) == 416, "ABI: TRenderFramePacket == 416");
static_assert(offsetof(TRenderFramePacket, pSkyBoxPackets) == 384, "ABI: frame skybox ptr");
static_assert(offsetof(TRenderFramePacket, pDungeonPackets) == 400, "ABI: frame dungeon ptr");
static_assert(sizeof(TSkyBoxPacket) == 112, "ABI: TSkyBoxPacket != packets.rs");
static_assert(alignof(TSkyBoxPacket) == 16, "ABI: TSkyBoxPacket align != 16");
static_assert(offsetof(TSkyBoxPacket, matWorld) == 0, "ABI: skybox matWorld");
static_assert(offsetof(TSkyBoxPacket, dwFaceCount) == 64, "ABI: skybox dwFaceCount");
static_assert(offsetof(TSkyBoxPacket, dwCloudTextureID) == 68, "ABI: skybox dwCloudTextureID");
static_assert(offsetof(TSkyBoxPacket, dwSkyTextureID) == 72, "ABI: skybox dwSkyTextureID");
static_assert(offsetof(TSkyBoxPacket, fCloudScale) == 76, "ABI: skybox fCloudScale");
static_assert(offsetof(TSkyBoxPacket, fCloudHeight) == 80, "ABI: skybox fCloudHeight");
static_assert(offsetof(TSkyBoxPacket, fCloudScrollSpeed) == 84, "ABI: skybox fCloudScrollSpeed");
static_assert(offsetof(TSkyBoxPacket, vSkyColor) == 92, "ABI: skybox vSkyColor");
static_assert(offsetof(TRenderFrameUniforms, vCameraPos) == 192, "ABI: uniforms slot0");
static_assert(offsetof(TRenderFrameUniforms, vSunLightDir) == 208, "ABI: uniforms slot1 sun dir");
static_assert(offsetof(TRenderFrameUniforms, dwFrameIndex) == 220, "ABI: uniforms slot1 frame index");
static_assert(offsetof(TRenderFrameUniforms, vFogColor) == 256, "ABI: uniforms slot4");
static_assert(offsetof(TActorRenderPacket, matWorld) == 16, "ABI: actor matWorld");
static_assert(offsetof(TActorRenderPacket, boneMatrices) == 88, "ABI: actor boneMatrices");
static_assert(offsetof(TActorRenderPacket, fShadowSize) == 5004, "ABI: actor fShadowSize");
static_assert(offsetof(TParticleInstanceData, fHalfWidth) == 56, "ABI: particle fHalfWidth");
static_assert(offsetof(TParticleInstanceData, fHalfHeight) == 60, "ABI: particle fHalfHeight");
static_assert(offsetof(TRenderFramePacket, pTerrainPackets) == 288, "ABI: frame terrain ptr");
static_assert(sizeof(TDungeonBlockPacket) == 160, "ABI: TDungeonBlockPacket != packets.rs");
static_assert(alignof(TDungeonBlockPacket) == 16, "ABI: TDungeonBlockPacket align != 16");
static_assert(offsetof(TDungeonBlockPacket, matWorld) == 0, "ABI: dungeon matWorld");
static_assert(offsetof(TDungeonBlockPacket, dwDiffuseTextureID) == 64, "ABI: dungeon dwDiffuseTextureID");
static_assert(offsetof(TDungeonBlockPacket, dwLightmapTextureID) == 72, "ABI: dungeon dwLightmapTextureID");
static_assert(offsetof(TDungeonBlockPacket, pDiffuseTexture) == 80, "ABI: dungeon pDiffuseTexture");
static_assert(offsetof(TDungeonBlockPacket, pLightmapTexture) == 88, "ABI: dungeon pLightmapTexture");
static_assert(offsetof(TDungeonBlockPacket, pVertexBuffer) == 96, "ABI: dungeon pVertexBuffer");
static_assert(offsetof(TDungeonBlockPacket, pIndexBuffer) == 104, "ABI: dungeon pIndexBuffer");
static_assert(offsetof(TDungeonBlockPacket, pVertices) == 112, "ABI: dungeon pVertices");
static_assert(offsetof(TDungeonBlockPacket, pIndices) == 120, "ABI: dungeon pIndices");
static_assert(offsetof(TDungeonBlockPacket, dwVertexCount) == 128, "ABI: dungeon dwVertexCount");
static_assert(offsetof(TDungeonBlockPacket, dwIndexCount) == 132, "ABI: dungeon dwIndexCount");
static_assert(offsetof(TDungeonBlockPacket, dwPrimitiveCount) == 136, "ABI: dungeon dwPrimitiveCount");
static_assert(offsetof(TDungeonBlockPacket, dwStartIndex) == 140, "ABI: dungeon dwStartIndex");
static_assert(offsetof(TDungeonBlockPacket, nBaseVertexIndex) == 144, "ABI: dungeon nBaseVertexIndex");
static_assert(offsetof(TDungeonBlockPacket, fAlpha) == 148, "ABI: dungeon fAlpha");
static_assert(offsetof(TDungeonBlockPacket, bAlphaBlend) == 152, "ABI: dungeon bAlphaBlend");
static_assert(offsetof(TDungeonBlockPacket, bPadDungeon) == 153, "ABI: dungeon bPadDungeon");
static_assert(offsetof(TDungeonBlockPacket, fPad) == 156, "ABI: dungeon fPad");
static_assert(sizeof(TWeaponTracePacket) == 24, "ABI: TWeaponTracePacket != packets.rs");
static_assert(offsetof(TWeaponTracePacket, textureHandle) == 0, "ABI: trace textureHandle");
static_assert(offsetof(TWeaponTracePacket, stateKey) == 4, "ABI: trace stateKey");
static_assert(offsetof(TWeaponTracePacket, vertexOffset) == 12, "ABI: trace vertexOffset");
static_assert(offsetof(TWeaponTracePacket, vertexCount) == 16, "ABI: trace vertexCount");
static_assert(offsetof(TWeaponTracePacket, dwPad) == 20, "ABI: trace dwPad");
#endif

