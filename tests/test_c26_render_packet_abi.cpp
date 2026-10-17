#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "EterLib/RenderPacketTypes.h"
#include "EterLib/GrpBase.h"
#include <cstddef>
#include <cstdint>
#include <cstring>

// Static assertions dla kompilacji
static_assert(sizeof(TRenderFrameUniforms) == 288, "ABI size: TRenderFrameUniforms");
static_assert(alignof(TRenderFrameUniforms) == 16, "ABI align: TRenderFrameUniforms");

static_assert(sizeof(TTerrainSectorPacket) == 176, "ABI size: TTerrainSectorPacket");
static_assert(alignof(TTerrainSectorPacket) == 16, "ABI align: TTerrainSectorPacket");

static_assert(sizeof(TActorRenderPacket) == 5008, "ABI size: TActorRenderPacket");
static_assert(alignof(TActorRenderPacket) == 16, "ABI align: TActorRenderPacket");

static_assert(sizeof(TWaterPacket) == 176, "ABI size: TWaterPacket");
static_assert(alignof(TWaterPacket) == 16, "ABI align: TWaterPacket");
static_assert(offsetof(TWaterPacket, dwWaterTextureID) == 56, "Offset: water dwWaterTextureID");
static_assert(offsetof(TWaterPacket, pWaterTexture) == 64, "Offset: water pWaterTexture");
static_assert(offsetof(TWaterPacket, bFarPass) == 96, "Offset: water bFarPass");
static_assert(offsetof(TWaterPacket, bAlphaBlend) == 97, "Offset: water bAlphaBlend");
static_assert(offsetof(TWaterPacket, fFogDistance) == 100, "Offset: water fFogDistance");
static_assert(offsetof(TWaterPacket, matTexTransformWater) == 112, "Offset: water matTexTransformWater");

static_assert(sizeof(TSpeedTreeInstanceData) == 112, "ABI size: TSpeedTreeInstanceData");
static_assert(alignof(TSpeedTreeInstanceData) == 16, "ABI align: TSpeedTreeInstanceData");

static_assert(sizeof(TSpeedTreePacket) == 48, "ABI size: TSpeedTreePacket");
static_assert(alignof(TSpeedTreePacket) == 16, "ABI align: TSpeedTreePacket");

static_assert(sizeof(TParticleInstanceData) == 64, "ABI size: TParticleInstanceData");
static_assert(alignof(TParticleInstanceData) == 16, "ABI align: TParticleInstanceData");

static_assert(sizeof(TEffectInstancePacket) == 48, "ABI size: TEffectInstancePacket");
static_assert(alignof(TEffectInstancePacket) == 16, "ABI align: TEffectInstancePacket");

static_assert(sizeof(TUIVertex) == 24, "ABI size: TUIVertex");

static_assert(sizeof(TUIBatchPacket) == 64, "ABI size: TUIBatchPacket");
static_assert(alignof(TUIBatchPacket) == 16, "ABI align: TUIBatchPacket");

static_assert(sizeof(TSkyBoxPacket) == 112, "ABI size: TSkyBoxPacket");
static_assert(alignof(TSkyBoxPacket) == 16, "ABI align: TSkyBoxPacket");

static_assert(sizeof(TPNT2Vertex) == 40, "ABI size: TPNT2Vertex");

static_assert(sizeof(TDungeonBlockPacket) == 160, "ABI size: TDungeonBlockPacket");
static_assert(alignof(TDungeonBlockPacket) == 16, "ABI align: TDungeonBlockPacket");

static_assert(sizeof(TRenderFramePacket) == 416, "ABI size: TRenderFramePacket");
static_assert(alignof(TRenderFramePacket) == 16, "ABI align: TRenderFramePacket");

static_assert(sizeof(TPipelineStateKey) == 8, "ABI size: TPipelineStateKey");
static_assert(sizeof(TTerrainPassPacket) == 96, "ABI size: TTerrainPassPacket");
static_assert(sizeof(TEffectMeshPacket) == 88, "ABI size: TEffectMeshPacket");
static_assert(sizeof(TModelDrawItem) == 96, "ABI size: TModelDrawItem");
static_assert(sizeof(TWeaponTracePacket) == 24, "ABI size: TWeaponTracePacket");
static_assert(sizeof(TDebugLinePacket) == 32, "ABI size: TDebugLinePacket");
static_assert(sizeof(TRenderPassDesc) == 144, "ABI size: TRenderPassDesc");

// Weryfikacja offsetow pol
static_assert(offsetof(TRenderFrameUniforms, matView) == 0, "Offset: matView");
static_assert(offsetof(TRenderFrameUniforms, matProj) == 64, "Offset: matProj");
static_assert(offsetof(TRenderFrameUniforms, matViewProj) == 128, "Offset: matViewProj");
static_assert(offsetof(TRenderFrameUniforms, vCameraPos) == 192, "Offset: vCameraPos");
static_assert(offsetof(TRenderFrameUniforms, fTime) == 204, "Offset: fTime");
static_assert(offsetof(TRenderFrameUniforms, vSunLightDir) == 208, "Offset: vSunLightDir");
static_assert(offsetof(TRenderFrameUniforms, dwFrameIndex) == 220, "Offset: dwFrameIndex");
static_assert(offsetof(TRenderFrameUniforms, vSunLightColor) == 224, "Offset: vSunLightColor");
static_assert(offsetof(TRenderFrameUniforms, vAmbientColor) == 240, "Offset: vAmbientColor");
static_assert(offsetof(TRenderFrameUniforms, vFogColor) == 256, "Offset: vFogColor");
static_assert(offsetof(TRenderFrameUniforms, fFogNear) == 272, "Offset: fFogNear");
static_assert(offsetof(TRenderFrameUniforms, fFogFar) == 276, "Offset: fFogFar");
static_assert(offsetof(TRenderFrameUniforms, fFogInvRange) == 280, "Offset: fFogInvRange");
static_assert(offsetof(TRenderFrameUniforms, fFogEnable) == 284, "Offset: fFogEnable");

static_assert(offsetof(TActorRenderPacket, dwActorID) == 0, "Offset: dwActorID");
static_assert(offsetof(TActorRenderPacket, matWorld) == 16, "Offset: matWorld");
static_assert(offsetof(TActorRenderPacket, pBoneMatrices) == 80, "Offset: pBoneMatrices");
static_assert(offsetof(TActorRenderPacket, boneMatrices) == 88, "Offset: boneMatrices");
static_assert(offsetof(TActorRenderPacket, dwWeaponMeshID) == 4184, "Offset: dwWeaponMeshID");
static_assert(offsetof(TActorRenderPacket, matWeaponSocket) == 4192, "Offset: matWeaponSocket");
static_assert(offsetof(TActorRenderPacket, matWeaponLeftSocket) == 4256, "Offset: matWeaponLeftSocket");
static_assert(offsetof(TActorRenderPacket, matHelmetSocket) == 4320, "Offset: matHelmetSocket");
static_assert(offsetof(TActorRenderPacket, matHairSocket) == 4384, "Offset: matHairSocket");
static_assert(offsetof(TActorRenderPacket, dwAuraEffectCount) == 4448, "Offset: dwAuraEffectCount");
static_assert(offsetof(TActorRenderPacket, matAuraEffectSockets) == 4452, "Offset: matAuraEffectSockets");
static_assert(offsetof(TActorRenderPacket, dwAuraEffectIDs) == 4964, "Offset: dwAuraEffectIDs");
static_assert(offsetof(TActorRenderPacket, bRenderShadow) == 4996, "Offset: bRenderShadow");
static_assert(offsetof(TActorRenderPacket, bIsDead) == 4997, "Offset: bIsDead");
static_assert(offsetof(TActorRenderPacket, bSelected) == 4998, "Offset: bSelected");
static_assert(offsetof(TActorRenderPacket, bAlphaBlend) == 4999, "Offset: bAlphaBlend");
static_assert(offsetof(TActorRenderPacket, fAlpha) == 5000, "Offset: fAlpha");
static_assert(offsetof(TActorRenderPacket, fShadowSize) == 5004, "Offset: fShadowSize");

static_assert(offsetof(TParticleInstanceData, vWorldPos) == 0, "Offset: vWorldPos");
static_assert(offsetof(TParticleInstanceData, vColor) == 16, "Offset: vColor");
static_assert(offsetof(TParticleInstanceData, vTexCoordRect) == 32, "Offset: vTexCoordRect");
static_assert(offsetof(TParticleInstanceData, fRotation) == 48, "Offset: fRotation");
static_assert(offsetof(TParticleInstanceData, fLifeRatio) == 52, "Offset: fLifeRatio");
static_assert(offsetof(TParticleInstanceData, fHalfWidth) == 56, "Offset: fHalfWidth");
static_assert(offsetof(TParticleInstanceData, fHalfHeight) == 60, "Offset: fHalfHeight");

static_assert(offsetof(TDungeonBlockPacket, matWorld) == 0, "Offset: dungeon matWorld");
static_assert(offsetof(TDungeonBlockPacket, dwDiffuseTextureID) == 64, "Offset: dungeon dwDiffuseTextureID");
static_assert(offsetof(TDungeonBlockPacket, dwLightmapTextureID) == 72, "Offset: dungeon dwLightmapTextureID");
static_assert(offsetof(TDungeonBlockPacket, pDiffuseTexture) == 80, "Offset: dungeon pDiffuseTexture");
static_assert(offsetof(TDungeonBlockPacket, pLightmapTexture) == 88, "Offset: dungeon pLightmapTexture");
static_assert(offsetof(TDungeonBlockPacket, pVertexBuffer) == 96, "Offset: dungeon pVertexBuffer");
static_assert(offsetof(TDungeonBlockPacket, pIndexBuffer) == 104, "Offset: dungeon pIndexBuffer");
static_assert(offsetof(TDungeonBlockPacket, pVertices) == 112, "Offset: dungeon pVertices");
static_assert(offsetof(TDungeonBlockPacket, pIndices) == 120, "Offset: dungeon pIndices");
static_assert(offsetof(TDungeonBlockPacket, dwVertexCount) == 128, "Offset: dungeon dwVertexCount");
static_assert(offsetof(TDungeonBlockPacket, dwIndexCount) == 132, "Offset: dungeon dwIndexCount");
static_assert(offsetof(TDungeonBlockPacket, dwPrimitiveCount) == 136, "Offset: dungeon dwPrimitiveCount");
static_assert(offsetof(TDungeonBlockPacket, dwStartIndex) == 140, "Offset: dungeon dwStartIndex");
static_assert(offsetof(TDungeonBlockPacket, nBaseVertexIndex) == 144, "Offset: dungeon nBaseVertexIndex");
static_assert(offsetof(TDungeonBlockPacket, fAlpha) == 148, "Offset: dungeon fAlpha");
static_assert(offsetof(TDungeonBlockPacket, bAlphaBlend) == 152, "Offset: dungeon bAlphaBlend");
static_assert(offsetof(TDungeonBlockPacket, bPadDungeon) == 153, "Offset: dungeon bPadDungeon");
static_assert(offsetof(TDungeonBlockPacket, fPad) == 156, "Offset: dungeon fPad");

static_assert(offsetof(TRenderFramePacket, uniforms) == 0, "Offset: frame uniforms");
static_assert(offsetof(TRenderFramePacket, pTerrainPackets) == 288, "Offset: frame pTerrainPackets");
static_assert(offsetof(TRenderFramePacket, dwTerrainCount) == 296, "Offset: frame dwTerrainCount");
static_assert(offsetof(TRenderFramePacket, pWaterPackets) == 304, "Offset: frame pWaterPackets");
static_assert(offsetof(TRenderFramePacket, dwWaterCount) == 312, "Offset: frame dwWaterCount");
static_assert(offsetof(TRenderFramePacket, pSpeedTreePackets) == 320, "Offset: frame pSpeedTreePackets");
static_assert(offsetof(TRenderFramePacket, dwSpeedTreeCount) == 328, "Offset: frame dwSpeedTreeCount");
static_assert(offsetof(TRenderFramePacket, pActorPackets) == 336, "Offset: frame pActorPackets");
static_assert(offsetof(TRenderFramePacket, dwActorCount) == 344, "Offset: frame dwActorCount");
static_assert(offsetof(TRenderFramePacket, pEffectPackets) == 352, "Offset: frame pEffectPackets");
static_assert(offsetof(TRenderFramePacket, dwEffectCount) == 360, "Offset: frame dwEffectCount");
static_assert(offsetof(TRenderFramePacket, pUIPackets) == 368, "Offset: frame pUIPackets");
static_assert(offsetof(TRenderFramePacket, dwUICount) == 376, "Offset: frame dwUICount");
static_assert(offsetof(TRenderFramePacket, pSkyBoxPackets) == 384, "Offset: frame pSkyBoxPackets");
static_assert(offsetof(TRenderFramePacket, dwSkyBoxCount) == 392, "Offset: frame dwSkyBoxCount");

TEST_CASE("RenderPacketABI - Exact Sizes Check")
{
	CHECK(sizeof(TRenderFrameUniforms) == 288u);
	CHECK(sizeof(TTerrainSectorPacket) == 176u);
	CHECK(sizeof(TActorRenderPacket) == 5008u);
	CHECK(sizeof(TWaterPacket) == 176u);
	CHECK(sizeof(TSpeedTreeInstanceData) == 112u);
	CHECK(sizeof(TSpeedTreePacket) == 48u);
	CHECK(sizeof(TParticleInstanceData) == 64u);
	CHECK(sizeof(TEffectInstancePacket) == 48u);
	CHECK(sizeof(TUIVertex) == 24u);
	CHECK(sizeof(TUIBatchPacket) == 64u);
	CHECK(sizeof(TSkyBoxPacket) == 112u);
	CHECK(sizeof(TPNT2Vertex) == 40u);
	CHECK(sizeof(TDungeonBlockPacket) == 160u);
	CHECK(sizeof(TRenderFramePacket) == 416u);
}

TEST_CASE("RenderPacketABI - TDungeonBlockPacket Independence and Zero-Init")
{
	TDungeonBlockPacket pkt{};
	CHECK(pkt.dwDiffuseTextureID == 0u);
	CHECK(pkt.dwLightmapTextureID == 0u);
	CHECK(pkt.pDiffuseTexture == nullptr);
	CHECK(pkt.pLightmapTexture == nullptr);
	CHECK(pkt.pVertexBuffer == nullptr);
	CHECK(pkt.pIndexBuffer == nullptr);
	CHECK(pkt.pVertices == nullptr);
	CHECK(pkt.pIndices == nullptr);
	CHECK(pkt.dwVertexCount == 0u);
	CHECK(pkt.dwIndexCount == 0u);
	CHECK(pkt.dwPrimitiveCount == 0u);
	CHECK(pkt.dwStartIndex == 0u);
	CHECK(pkt.nBaseVertexIndex == 0);
	CHECK(pkt.bAlphaBlend == 0u);
	CHECK(pkt.bPadDungeon[0] == 0u);
	CHECK(pkt.bPadDungeon[1] == 0u);
	CHECK(pkt.bPadDungeon[2] == 0u);
	CHECK(pkt.fPad == 0.0f);

	// Wymog S4.1 (F-P1-01): fAlpha zachowuje wartosc 0.99f po ustawieniu bAlphaBlend
	pkt.fAlpha = 0.99f;
	pkt.bAlphaBlend = 1u;
	CHECK(pkt.fAlpha == 0.99f);
	CHECK(pkt.bAlphaBlend == 1u);

	// Dodatkowe sprawdzenie z bAlphaBlend = 0
	pkt.bAlphaBlend = 0u;
	CHECK(pkt.fAlpha == 0.99f);
	CHECK(pkt.bAlphaBlend == 0u);
}

TEST_CASE("RenderPacketABI - TRenderFrameUniforms Independence (F-NEW-018)")
{
	TRenderFrameUniforms uniforms{};
	CHECK(uniforms.dwFrameIndex == 0u);
	CHECK(uniforms.vSunLightDir.x == 0.0f);
	CHECK(uniforms.vSunLightDir.y == 0.0f);
	CHECK(uniforms.vSunLightDir.z == 0.0f);

	// Test wymogu F-NEW-018: vSunLightDir i dwFrameIndex nie nachodza na siebie (brak unii ze szkodliwym float w)
	uniforms.dwFrameIndex = 0x12345678u;
	uniforms.vSunLightDir = D3DXVECTOR3(0.5f, -0.8f, 0.3f);
	CHECK(uniforms.dwFrameIndex == 0x12345678u);
	CHECK(uniforms.vSunLightDir.x == 0.5f);
	CHECK(uniforms.vSunLightDir.y == -0.8f);
	CHECK(uniforms.vSunLightDir.z == 0.3f);

	uniforms.dwFrameIndex = 42u;
	CHECK(uniforms.vSunLightDir.x == 0.5f);
	CHECK(uniforms.dwFrameIndex == 42u);
}

TEST_CASE("RenderPacketABI - TParticleInstanceData Named Dimensions")
{
	TParticleInstanceData particle{};
	CHECK(particle.fHalfWidth == 0.0f);
	CHECK(particle.fHalfHeight == 0.0f);
	CHECK(particle.fRotation == 0.0f);
	CHECK(particle.fLifeRatio == 0.0f);
	CHECK(particle.vWorldPos.x == 0.0f);

	// Wymog S4.1: fHalfWidth i fHalfHeight jako nazwane pola zamiast fPad[2]
	particle.fHalfWidth = 16.5f;
	particle.fHalfHeight = 32.25f;
	particle.fRotation = 45.0f;
	particle.fLifeRatio = 0.75f;
	CHECK(particle.fHalfWidth == 16.5f);
	CHECK(particle.fHalfHeight == 32.25f);
	CHECK(particle.fRotation == 45.0f);
	CHECK(particle.fLifeRatio == 0.75f);
}

TEST_CASE("RenderPacketABI - TActorRenderPacket Zero-Init and ShadowSize")
{
	TActorRenderPacket actor{};
	CHECK(actor.dwActorID == 0u);
	CHECK(actor.dwMeshID == 0u);
	CHECK(actor.dwTextureID == 0u);
	CHECK(actor.dwBoneCount == 0u);
	CHECK(actor.pBoneMatrices == nullptr);
	CHECK(actor.dwWeaponMeshID == 0u);
	CHECK(actor.dwWeaponTextureID == 0u);
	CHECK(actor.dwAuraEffectCount == 0u);
	CHECK_FALSE(actor.bRenderShadow);
	CHECK_FALSE(actor.bIsDead);
	CHECK_FALSE(actor.bSelected);
	CHECK_FALSE(actor.bAlphaBlend);
	CHECK(actor.fAlpha == 0.0f);
	CHECK(actor.fShadowSize == 0.0f);

	actor.fShadowSize = 128.0f;
	CHECK(actor.fShadowSize == 128.0f);

	// GetBoneMatrices() zwraca bufor wewnetrzny gdy wskaznik jest nullptr
	CHECK(actor.GetBoneMatrices() == actor.boneMatrices);
}
