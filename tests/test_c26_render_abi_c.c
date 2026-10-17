#include <stdio.h>
#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>
#include <assert.h>
#include "EterLib/RenderAbi.h"

/*
 * C-ABI Verification Test: test_c26_render_abi_c.c
 * Czysty plik testowy C weryfikujacy offsetof i sizeof dla wszystkich
 * pakietow renderera i wierzcholkow silnika Metin2.
 * Asercje sa bezwarunkowe dla docelowej architektury (brak #if defined(_WIN64)).
 */

#if (defined(__STDC_VERSION__) && (__STDC_VERSION__ >= 201112L))
    #define C_STATIC_ASSERT(cond, msg) _Static_assert(cond, msg)
#else
    #define C_STATIC_ASSERT_CONCAT_INNER(a, b) a##b
    #define C_STATIC_ASSERT_CONCAT(a, b) C_STATIC_ASSERT_CONCAT_INNER(a, b)
    #define C_STATIC_ASSERT(cond, msg) typedef char C_STATIC_ASSERT_CONCAT(__c_sa_, __LINE__)[(cond) ? 1 : -1]
#endif

/* === Rozmiary i wyrownania struktur (sizeof / alignof) === */
C_STATIC_ASSERT(sizeof(TRenderMatrix4x4) == 64, "Size: TRenderMatrix4x4");
C_STATIC_ASSERT(sizeof(TRenderVec2) == 8, "Size: TRenderVec2");
C_STATIC_ASSERT(sizeof(TRenderVec3) == 12, "Size: TRenderVec3");
C_STATIC_ASSERT(sizeof(TRenderVec4) == 16, "Size: TRenderVec4");
C_STATIC_ASSERT(sizeof(TRect32) == 16, "Size: TRect32");

C_STATIC_ASSERT(sizeof(TUIVertex) == 24, "Size: TUIVertex");
C_STATIC_ASSERT(sizeof(TPNT2Vertex) == 40, "Size: TPNT2Vertex");
C_STATIC_ASSERT(sizeof(TRenderFrameUniforms) == 288, "Size: TRenderFrameUniforms");
C_STATIC_ASSERT(sizeof(TTerrainSectorPacket) == 176, "Size: TTerrainSectorPacket");
C_STATIC_ASSERT(sizeof(TActorRenderPacket) == 5008, "Size: TActorRenderPacket");
C_STATIC_ASSERT(sizeof(TWaterPacket) == 176, "Size: TWaterPacket");
C_STATIC_ASSERT(sizeof(TSpeedTreeInstanceData) == 112, "Size: TSpeedTreeInstanceData");
C_STATIC_ASSERT(sizeof(TSpeedTreePacket) == 48, "Size: TSpeedTreePacket");
C_STATIC_ASSERT(sizeof(TParticleInstanceData) == 64, "Size: TParticleInstanceData");
C_STATIC_ASSERT(sizeof(TEffectInstancePacket) == 48, "Size: TEffectInstancePacket");
C_STATIC_ASSERT(sizeof(TUIBatchPacket) == 64, "Size: TUIBatchPacket");
C_STATIC_ASSERT(sizeof(TSkyBoxPacket) == 112, "Size: TSkyBoxPacket");
C_STATIC_ASSERT(sizeof(TDungeonBlockPacket) == 160, "Size: TDungeonBlockPacket");
C_STATIC_ASSERT(sizeof(TRenderFramePacket) == 416, "Size: TRenderFramePacket");
C_STATIC_ASSERT(sizeof(TPipelineStateKey) == 8, "Size: TPipelineStateKey");
C_STATIC_ASSERT(sizeof(TTerrainPassPacket) == 96, "Size: TTerrainPassPacket");
C_STATIC_ASSERT(sizeof(TEffectMeshPacket) == 88, "Size: TEffectMeshPacket");
C_STATIC_ASSERT(sizeof(TModelDrawItem) == 96, "Size: TModelDrawItem");
C_STATIC_ASSERT(sizeof(TWeaponTracePacket) == 24, "Size: TWeaponTracePacket");
C_STATIC_ASSERT(sizeof(TDebugLinePacket) == 32, "Size: TDebugLinePacket");
C_STATIC_ASSERT(sizeof(TRenderPassDesc) == 144, "Size: TRenderPassDesc");

/* === 1. TUIVertex === */
C_STATIC_ASSERT(offsetof(TUIVertex, position) == 0, "TUIVertex::position");
C_STATIC_ASSERT(offsetof(TUIVertex, color) == 12, "TUIVertex::color");
C_STATIC_ASSERT(offsetof(TUIVertex, texCoord) == 16, "TUIVertex::texCoord");

/* === 2. SPNT2Vertex / TPNT2Vertex === */
C_STATIC_ASSERT(offsetof(TPNT2Vertex, position) == 0, "TPNT2Vertex::position");
C_STATIC_ASSERT(offsetof(TPNT2Vertex, normal) == 12, "TPNT2Vertex::normal");
C_STATIC_ASSERT(offsetof(TPNT2Vertex, texCoord) == 24, "TPNT2Vertex::texCoord");
C_STATIC_ASSERT(offsetof(TPNT2Vertex, texCoord2) == 32, "TPNT2Vertex::texCoord2");
C_STATIC_ASSERT(offsetof(TPNT2Vertex, uv0) == 24, "TPNT2Vertex::uv0");
C_STATIC_ASSERT(offsetof(TPNT2Vertex, uv1) == 32, "TPNT2Vertex::uv1");

/* === 3. TRenderFrameUniforms === */
C_STATIC_ASSERT(offsetof(TRenderFrameUniforms, matView) == 0, "TRenderFrameUniforms::matView");
C_STATIC_ASSERT(offsetof(TRenderFrameUniforms, matProj) == 64, "TRenderFrameUniforms::matProj");
C_STATIC_ASSERT(offsetof(TRenderFrameUniforms, matViewProj) == 128, "TRenderFrameUniforms::matViewProj");
C_STATIC_ASSERT(offsetof(TRenderFrameUniforms, vCameraPos) == 192, "TRenderFrameUniforms::vCameraPos");
C_STATIC_ASSERT(offsetof(TRenderFrameUniforms, fTime) == 204, "TRenderFrameUniforms::fTime");
C_STATIC_ASSERT(offsetof(TRenderFrameUniforms, vSunLightDir) == 208, "TRenderFrameUniforms::vSunLightDir");
C_STATIC_ASSERT(offsetof(TRenderFrameUniforms, dwFrameIndex) == 220, "TRenderFrameUniforms::dwFrameIndex");
C_STATIC_ASSERT(offsetof(TRenderFrameUniforms, vSunLightColor) == 224, "TRenderFrameUniforms::vSunLightColor");
C_STATIC_ASSERT(offsetof(TRenderFrameUniforms, v4SunColor) == 224, "TRenderFrameUniforms::v4SunColor");
C_STATIC_ASSERT(offsetof(TRenderFrameUniforms, vAmbientColor) == 240, "TRenderFrameUniforms::vAmbientColor");
C_STATIC_ASSERT(offsetof(TRenderFrameUniforms, v4AmbientColor) == 240, "TRenderFrameUniforms::v4AmbientColor");
C_STATIC_ASSERT(offsetof(TRenderFrameUniforms, vFogColor) == 256, "TRenderFrameUniforms::vFogColor");
C_STATIC_ASSERT(offsetof(TRenderFrameUniforms, fFogNear) == 272, "TRenderFrameUniforms::fFogNear");
C_STATIC_ASSERT(offsetof(TRenderFrameUniforms, fFogFar) == 276, "TRenderFrameUniforms::fFogFar");
C_STATIC_ASSERT(offsetof(TRenderFrameUniforms, fFogInvRange) == 280, "TRenderFrameUniforms::fFogInvRange");
C_STATIC_ASSERT(offsetof(TRenderFrameUniforms, fFogEnable) == 284, "TRenderFrameUniforms::fFogEnable");
C_STATIC_ASSERT(offsetof(TRenderFrameUniforms, v4FogParams) == 272, "TRenderFrameUniforms::v4FogParams");

/* === 4. TTerrainSectorPacket === */
C_STATIC_ASSERT(offsetof(TTerrainSectorPacket, nCoordX) == 0, "TTerrainSectorPacket::nCoordX");
C_STATIC_ASSERT(offsetof(TTerrainSectorPacket, nCoordY) == 2, "TTerrainSectorPacket::nCoordY");
C_STATIC_ASSERT(offsetof(TTerrainSectorPacket, dwSectorX) == 4, "TTerrainSectorPacket::dwSectorX");
C_STATIC_ASSERT(offsetof(TTerrainSectorPacket, dwSectorY) == 8, "TTerrainSectorPacket::dwSectorY");
C_STATIC_ASSERT(offsetof(TTerrainSectorPacket, sectorX) == 12, "TTerrainSectorPacket::sectorX");
C_STATIC_ASSERT(offsetof(TTerrainSectorPacket, sectorY) == 14, "TTerrainSectorPacket::sectorY");
C_STATIC_ASSERT(offsetof(TTerrainSectorPacket, fWorldPosX) == 16, "TTerrainSectorPacket::fWorldPosX");
C_STATIC_ASSERT(offsetof(TTerrainSectorPacket, fWorldPosY) == 20, "TTerrainSectorPacket::fWorldPosY");
C_STATIC_ASSERT(offsetof(TTerrainSectorPacket, fMinHeight) == 24, "TTerrainSectorPacket::fMinHeight");
C_STATIC_ASSERT(offsetof(TTerrainSectorPacket, fMaxHeight) == 28, "TTerrainSectorPacket::fMaxHeight");
C_STATIC_ASSERT(offsetof(TTerrainSectorPacket, dwTextureSetID) == 32, "TTerrainSectorPacket::dwTextureSetID");
C_STATIC_ASSERT(offsetof(TTerrainSectorPacket, dwPatchCount) == 36, "TTerrainSectorPacket::dwPatchCount");
C_STATIC_ASSERT(offsetof(TTerrainSectorPacket, patchNum) == 40, "TTerrainSectorPacket::patchNum");
C_STATIC_ASSERT(offsetof(TTerrainSectorPacket, splatTextureId) == 44, "TTerrainSectorPacket::splatTextureId");
C_STATIC_ASSERT(offsetof(TTerrainSectorPacket, pSplatTexture) == 48, "TTerrainSectorPacket::pSplatTexture");
C_STATIC_ASSERT(offsetof(TTerrainSectorPacket, layerTextureIds) == 56, "TTerrainSectorPacket::layerTextureIds");
C_STATIC_ASSERT(offsetof(TTerrainSectorPacket, pLayerTextures) == 72, "TTerrainSectorPacket::pLayerTextures");
C_STATIC_ASSERT(offsetof(TTerrainSectorPacket, pVertexData) == 104, "TTerrainSectorPacket::pVertexData");
C_STATIC_ASSERT(offsetof(TTerrainSectorPacket, dwVertexCount) == 112, "TTerrainSectorPacket::dwVertexCount");
C_STATIC_ASSERT(offsetof(TTerrainSectorPacket, pSplatWeights) == 120, "TTerrainSectorPacket::pSplatWeights");
C_STATIC_ASSERT(offsetof(TTerrainSectorPacket, dwSplatWeightCount) == 128, "TTerrainSectorPacket::dwSplatWeightCount");
C_STATIC_ASSERT(offsetof(TTerrainSectorPacket, pHeightMap) == 136, "TTerrainSectorPacket::pHeightMap");
C_STATIC_ASSERT(offsetof(TTerrainSectorPacket, pTileMap) == 144, "TTerrainSectorPacket::pTileMap");
C_STATIC_ASSERT(offsetof(TTerrainSectorPacket, pNormalMap) == 152, "TTerrainSectorPacket::pNormalMap");
C_STATIC_ASSERT(offsetof(TTerrainSectorPacket, bHasWater) == 160, "TTerrainSectorPacket::bHasWater");
C_STATIC_ASSERT(offsetof(TTerrainSectorPacket, fWaterHeight) == 164, "TTerrainSectorPacket::fWaterHeight");
C_STATIC_ASSERT(offsetof(TTerrainSectorPacket, fReserved) == 168, "TTerrainSectorPacket::fReserved");

/* === 5. TActorRenderPacket === */
C_STATIC_ASSERT(offsetof(TActorRenderPacket, dwActorID) == 0, "TActorRenderPacket::dwActorID");
C_STATIC_ASSERT(offsetof(TActorRenderPacket, dwMeshID) == 4, "TActorRenderPacket::dwMeshID");
C_STATIC_ASSERT(offsetof(TActorRenderPacket, dwTextureID) == 8, "TActorRenderPacket::dwTextureID");
C_STATIC_ASSERT(offsetof(TActorRenderPacket, dwBoneCount) == 12, "TActorRenderPacket::dwBoneCount");
C_STATIC_ASSERT(offsetof(TActorRenderPacket, matWorld) == 16, "TActorRenderPacket::matWorld");
C_STATIC_ASSERT(offsetof(TActorRenderPacket, pBoneMatrices) == 80, "TActorRenderPacket::pBoneMatrices");
C_STATIC_ASSERT(offsetof(TActorRenderPacket, boneMatrices) == 88, "TActorRenderPacket::boneMatrices");
C_STATIC_ASSERT(offsetof(TActorRenderPacket, dwWeaponMeshID) == 4184, "TActorRenderPacket::dwWeaponMeshID");
C_STATIC_ASSERT(offsetof(TActorRenderPacket, dwWeaponTextureID) == 4188, "TActorRenderPacket::dwWeaponTextureID");
C_STATIC_ASSERT(offsetof(TActorRenderPacket, matWeaponSocket) == 4192, "TActorRenderPacket::matWeaponSocket");
C_STATIC_ASSERT(offsetof(TActorRenderPacket, matWeaponLeftSocket) == 4256, "TActorRenderPacket::matWeaponLeftSocket");
C_STATIC_ASSERT(offsetof(TActorRenderPacket, matHelmetSocket) == 4320, "TActorRenderPacket::matHelmetSocket");
C_STATIC_ASSERT(offsetof(TActorRenderPacket, matHairSocket) == 4384, "TActorRenderPacket::matHairSocket");
C_STATIC_ASSERT(offsetof(TActorRenderPacket, dwAuraEffectCount) == 4448, "TActorRenderPacket::dwAuraEffectCount");
C_STATIC_ASSERT(offsetof(TActorRenderPacket, matAuraEffectSockets) == 4452, "TActorRenderPacket::matAuraEffectSockets");
C_STATIC_ASSERT(offsetof(TActorRenderPacket, dwAuraEffectIDs) == 4964, "TActorRenderPacket::dwAuraEffectIDs");
C_STATIC_ASSERT(offsetof(TActorRenderPacket, bRenderShadow) == 4996, "TActorRenderPacket::bRenderShadow");
C_STATIC_ASSERT(offsetof(TActorRenderPacket, bIsDead) == 4997, "TActorRenderPacket::bIsDead");
C_STATIC_ASSERT(offsetof(TActorRenderPacket, bSelected) == 4998, "TActorRenderPacket::bSelected");
C_STATIC_ASSERT(offsetof(TActorRenderPacket, bAlphaBlend) == 4999, "TActorRenderPacket::bAlphaBlend");
C_STATIC_ASSERT(offsetof(TActorRenderPacket, fAlpha) == 5000, "TActorRenderPacket::fAlpha");
C_STATIC_ASSERT(offsetof(TActorRenderPacket, fShadowSize) == 5004, "TActorRenderPacket::fShadowSize");

/* === 6. TWaterPacket === */
C_STATIC_ASSERT(offsetof(TWaterPacket, fWorldPosX) == 0, "TWaterPacket::fWorldPosX");
C_STATIC_ASSERT(offsetof(TWaterPacket, fWorldPosY) == 4, "TWaterPacket::fWorldPosY");
C_STATIC_ASSERT(offsetof(TWaterPacket, dwSectorX) == 8, "TWaterPacket::dwSectorX");
C_STATIC_ASSERT(offsetof(TWaterPacket, dwSectorY) == 12, "TWaterPacket::dwSectorY");
C_STATIC_ASSERT(offsetof(TWaterPacket, sectorX) == 16, "TWaterPacket::sectorX");
C_STATIC_ASSERT(offsetof(TWaterPacket, sectorY) == 18, "TWaterPacket::sectorY");
C_STATIC_ASSERT(offsetof(TWaterPacket, fWaterHeight) == 20, "TWaterPacket::fWaterHeight");
C_STATIC_ASSERT(offsetof(TWaterPacket, fWaveTime) == 24, "TWaterPacket::fWaveTime");
C_STATIC_ASSERT(offsetof(TWaterPacket, fWaveFrequency) == 28, "TWaterPacket::fWaveFrequency");
C_STATIC_ASSERT(offsetof(TWaterPacket, fWaveAmplitude) == 32, "TWaterPacket::fWaveAmplitude");
C_STATIC_ASSERT(offsetof(TWaterPacket, vWaterColor) == 36, "TWaterPacket::vWaterColor");
C_STATIC_ASSERT(offsetof(TWaterPacket, fAlpha) == 52, "TWaterPacket::fAlpha");
C_STATIC_ASSERT(offsetof(TWaterPacket, dwWaterTextureID) == 56, "TWaterPacket::dwWaterTextureID");
C_STATIC_ASSERT(offsetof(TWaterPacket, pWaterTexture) == 64, "TWaterPacket::pWaterTexture");
C_STATIC_ASSERT(offsetof(TWaterPacket, dwCellCount) == 72, "TWaterPacket::dwCellCount");
C_STATIC_ASSERT(offsetof(TWaterPacket, dwVertexCount) == 76, "TWaterPacket::dwVertexCount");
C_STATIC_ASSERT(offsetof(TWaterPacket, pVertexBuffer) == 80, "TWaterPacket::pVertexBuffer");
C_STATIC_ASSERT(offsetof(TWaterPacket, dwFaceCount) == 88, "TWaterPacket::dwFaceCount");
C_STATIC_ASSERT(offsetof(TWaterPacket, patchNum) == 92, "TWaterPacket::patchNum");
C_STATIC_ASSERT(offsetof(TWaterPacket, bFarPass) == 96, "TWaterPacket::bFarPass");
C_STATIC_ASSERT(offsetof(TWaterPacket, bAlphaBlend) == 97, "TWaterPacket::bAlphaBlend");
C_STATIC_ASSERT(offsetof(TWaterPacket, fFogDistance) == 100, "TWaterPacket::fFogDistance");
C_STATIC_ASSERT(offsetof(TWaterPacket, fPad) == 104, "TWaterPacket::fPad");
C_STATIC_ASSERT(offsetof(TWaterPacket, matTexTransformWater) == 112, "TWaterPacket::matTexTransformWater");

/* === 7. TSkyBoxPacket === */
C_STATIC_ASSERT(offsetof(TSkyBoxPacket, matWorld) == 0, "TSkyBoxPacket::matWorld");
C_STATIC_ASSERT(offsetof(TSkyBoxPacket, dwFaceCount) == 64, "TSkyBoxPacket::dwFaceCount");
C_STATIC_ASSERT(offsetof(TSkyBoxPacket, dwCloudTextureID) == 68, "TSkyBoxPacket::dwCloudTextureID");
C_STATIC_ASSERT(offsetof(TSkyBoxPacket, dwSkyTextureID) == 72, "TSkyBoxPacket::dwSkyTextureID");
C_STATIC_ASSERT(offsetof(TSkyBoxPacket, fCloudScale) == 76, "TSkyBoxPacket::fCloudScale");
C_STATIC_ASSERT(offsetof(TSkyBoxPacket, fCloudHeight) == 80, "TSkyBoxPacket::fCloudHeight");
C_STATIC_ASSERT(offsetof(TSkyBoxPacket, fCloudScrollSpeed) == 84, "TSkyBoxPacket::fCloudScrollSpeed");
C_STATIC_ASSERT(offsetof(TSkyBoxPacket, fPad) == 88, "TSkyBoxPacket::fPad");
C_STATIC_ASSERT(offsetof(TSkyBoxPacket, vSkyColor) == 92, "TSkyBoxPacket::vSkyColor");

/* === 8. TSpeedTreeInstanceData === */
C_STATIC_ASSERT(offsetof(TSpeedTreeInstanceData, pTreeInstance) == 0, "TSpeedTreeInstanceData::pTreeInstance");
C_STATIC_ASSERT(offsetof(TSpeedTreeInstanceData, matWorld) == 8, "TSpeedTreeInstanceData::matWorld");
C_STATIC_ASSERT(offsetof(TSpeedTreeInstanceData, v3Position) == 72, "TSpeedTreeInstanceData::v3Position");
C_STATIC_ASSERT(offsetof(TSpeedTreeInstanceData, dwTreeCRC) == 84, "TSpeedTreeInstanceData::dwTreeCRC");
C_STATIC_ASSERT(offsetof(TSpeedTreeInstanceData, fWindStrength) == 88, "TSpeedTreeInstanceData::fWindStrength");
C_STATIC_ASSERT(offsetof(TSpeedTreeInstanceData, fPad) == 92, "TSpeedTreeInstanceData::fPad");

/* === 9. TSpeedTreePacket === */
C_STATIC_ASSERT(offsetof(TSpeedTreePacket, pMainTree) == 0, "TSpeedTreePacket::pMainTree");
C_STATIC_ASSERT(offsetof(TSpeedTreePacket, dwTreeCRC) == 8, "TSpeedTreePacket::dwTreeCRC");
C_STATIC_ASSERT(offsetof(TSpeedTreePacket, bRenderBranches) == 12, "TSpeedTreePacket::bRenderBranches");
C_STATIC_ASSERT(offsetof(TSpeedTreePacket, bRenderFronds) == 13, "TSpeedTreePacket::bRenderFronds");
C_STATIC_ASSERT(offsetof(TSpeedTreePacket, bRenderLeaves) == 14, "TSpeedTreePacket::bRenderLeaves");
C_STATIC_ASSERT(offsetof(TSpeedTreePacket, bRenderBillboards) == 15, "TSpeedTreePacket::bRenderBillboards");
C_STATIC_ASSERT(offsetof(TSpeedTreePacket, dwTreeTypeID) == 16, "TSpeedTreePacket::dwTreeTypeID");
C_STATIC_ASSERT(offsetof(TSpeedTreePacket, dwTrunkTextureID) == 20, "TSpeedTreePacket::dwTrunkTextureID");
C_STATIC_ASSERT(offsetof(TSpeedTreePacket, dwLeafTextureID) == 24, "TSpeedTreePacket::dwLeafTextureID");
C_STATIC_ASSERT(offsetof(TSpeedTreePacket, dwInstanceCount) == 28, "TSpeedTreePacket::dwInstanceCount");
C_STATIC_ASSERT(offsetof(TSpeedTreePacket, pInstances) == 32, "TSpeedTreePacket::pInstances");
C_STATIC_ASSERT(offsetof(TSpeedTreePacket, fPad) == 40, "TSpeedTreePacket::fPad");

/* === 10. TParticleInstanceData === */
C_STATIC_ASSERT(offsetof(TParticleInstanceData, vWorldPos) == 0, "TParticleInstanceData::vWorldPos");
C_STATIC_ASSERT(offsetof(TParticleInstanceData, vColor) == 16, "TParticleInstanceData::vColor");
C_STATIC_ASSERT(offsetof(TParticleInstanceData, vTexCoordRect) == 32, "TParticleInstanceData::vTexCoordRect");
C_STATIC_ASSERT(offsetof(TParticleInstanceData, fRotation) == 48, "TParticleInstanceData::fRotation");
C_STATIC_ASSERT(offsetof(TParticleInstanceData, fLifeRatio) == 52, "TParticleInstanceData::fLifeRatio");
C_STATIC_ASSERT(offsetof(TParticleInstanceData, fHalfWidth) == 56, "TParticleInstanceData::fHalfWidth");
C_STATIC_ASSERT(offsetof(TParticleInstanceData, fHalfHeight) == 60, "TParticleInstanceData::fHalfHeight");

/* === 11. TEffectInstancePacket === */
C_STATIC_ASSERT(offsetof(TEffectInstancePacket, dwEffectID) == 0, "TEffectInstancePacket::dwEffectID");
C_STATIC_ASSERT(offsetof(TEffectInstancePacket, dwTextureID) == 8, "TEffectInstancePacket::dwTextureID");
C_STATIC_ASSERT(offsetof(TEffectInstancePacket, dwBlendMode) == 16, "TEffectInstancePacket::dwBlendMode");
C_STATIC_ASSERT(offsetof(TEffectInstancePacket, dwParticleCount) == 20, "TEffectInstancePacket::dwParticleCount");
C_STATIC_ASSERT(offsetof(TEffectInstancePacket, pParticles) == 24, "TEffectInstancePacket::pParticles");
C_STATIC_ASSERT(offsetof(TEffectInstancePacket, fPad) == 32, "TEffectInstancePacket::fPad");

/* === 12. TUIBatchPacket === */
C_STATIC_ASSERT(offsetof(TUIBatchPacket, dwTextureID) == 0, "TUIBatchPacket::dwTextureID");
C_STATIC_ASSERT(offsetof(TUIBatchPacket, dwBlendMode) == 8, "TUIBatchPacket::dwBlendMode");
C_STATIC_ASSERT(offsetof(TUIBatchPacket, dwVertexCount) == 12, "TUIBatchPacket::dwVertexCount");
C_STATIC_ASSERT(offsetof(TUIBatchPacket, bUseScissor) == 16, "TUIBatchPacket::bUseScissor");
C_STATIC_ASSERT(offsetof(TUIBatchPacket, scissorRect) == 20, "TUIBatchPacket::scissorRect");
C_STATIC_ASSERT(offsetof(TUIBatchPacket, pVertices) == 40, "TUIBatchPacket::pVertices");
C_STATIC_ASSERT(offsetof(TUIBatchPacket, fPad) == 48, "TUIBatchPacket::fPad");

/* === 13. TDungeonBlockPacket === */
C_STATIC_ASSERT(offsetof(TDungeonBlockPacket, matWorld) == 0, "TDungeonBlockPacket::matWorld");
C_STATIC_ASSERT(offsetof(TDungeonBlockPacket, dwDiffuseTextureID) == 64, "TDungeonBlockPacket::dwDiffuseTextureID");
C_STATIC_ASSERT(offsetof(TDungeonBlockPacket, dwLightmapTextureID) == 72, "TDungeonBlockPacket::dwLightmapTextureID");
C_STATIC_ASSERT(offsetof(TDungeonBlockPacket, pDiffuseTexture) == 80, "TDungeonBlockPacket::pDiffuseTexture");
C_STATIC_ASSERT(offsetof(TDungeonBlockPacket, pLightmapTexture) == 88, "TDungeonBlockPacket::pLightmapTexture");
C_STATIC_ASSERT(offsetof(TDungeonBlockPacket, pVertexBuffer) == 96, "TDungeonBlockPacket::pVertexBuffer");
C_STATIC_ASSERT(offsetof(TDungeonBlockPacket, pIndexBuffer) == 104, "TDungeonBlockPacket::pIndexBuffer");
C_STATIC_ASSERT(offsetof(TDungeonBlockPacket, pVertices) == 112, "TDungeonBlockPacket::pVertices");
C_STATIC_ASSERT(offsetof(TDungeonBlockPacket, pIndices) == 120, "TDungeonBlockPacket::pIndices");
C_STATIC_ASSERT(offsetof(TDungeonBlockPacket, dwVertexCount) == 128, "TDungeonBlockPacket::dwVertexCount");
C_STATIC_ASSERT(offsetof(TDungeonBlockPacket, dwIndexCount) == 132, "TDungeonBlockPacket::dwIndexCount");
C_STATIC_ASSERT(offsetof(TDungeonBlockPacket, dwPrimitiveCount) == 136, "TDungeonBlockPacket::dwPrimitiveCount");
C_STATIC_ASSERT(offsetof(TDungeonBlockPacket, dwStartIndex) == 140, "TDungeonBlockPacket::dwStartIndex");
C_STATIC_ASSERT(offsetof(TDungeonBlockPacket, nBaseVertexIndex) == 144, "TDungeonBlockPacket::nBaseVertexIndex");
C_STATIC_ASSERT(offsetof(TDungeonBlockPacket, fAlpha) == 148, "TDungeonBlockPacket::fAlpha");
C_STATIC_ASSERT(offsetof(TDungeonBlockPacket, bAlphaBlend) == 152, "TDungeonBlockPacket::bAlphaBlend");
C_STATIC_ASSERT(offsetof(TDungeonBlockPacket, bPadDungeon) == 153, "TDungeonBlockPacket::bPadDungeon");
C_STATIC_ASSERT(offsetof(TDungeonBlockPacket, fPad) == 156, "TDungeonBlockPacket::fPad");

/* === 14. TRenderFramePacket === */
C_STATIC_ASSERT(offsetof(TRenderFramePacket, uniforms) == 0, "TRenderFramePacket::uniforms");
C_STATIC_ASSERT(offsetof(TRenderFramePacket, pTerrainPackets) == 288, "TRenderFramePacket::pTerrainPackets");
C_STATIC_ASSERT(offsetof(TRenderFramePacket, dwTerrainCount) == 296, "TRenderFramePacket::dwTerrainCount");
C_STATIC_ASSERT(offsetof(TRenderFramePacket, dwPad0) == 300, "TRenderFramePacket::dwPad0");
C_STATIC_ASSERT(offsetof(TRenderFramePacket, pWaterPackets) == 304, "TRenderFramePacket::pWaterPackets");
C_STATIC_ASSERT(offsetof(TRenderFramePacket, dwWaterCount) == 312, "TRenderFramePacket::dwWaterCount");
C_STATIC_ASSERT(offsetof(TRenderFramePacket, dwPad1) == 316, "TRenderFramePacket::dwPad1");
C_STATIC_ASSERT(offsetof(TRenderFramePacket, pSpeedTreePackets) == 320, "TRenderFramePacket::pSpeedTreePackets");
C_STATIC_ASSERT(offsetof(TRenderFramePacket, dwSpeedTreeCount) == 328, "TRenderFramePacket::dwSpeedTreeCount");
C_STATIC_ASSERT(offsetof(TRenderFramePacket, dwPad2) == 332, "TRenderFramePacket::dwPad2");
C_STATIC_ASSERT(offsetof(TRenderFramePacket, pActorPackets) == 336, "TRenderFramePacket::pActorPackets");
C_STATIC_ASSERT(offsetof(TRenderFramePacket, dwActorCount) == 344, "TRenderFramePacket::dwActorCount");
C_STATIC_ASSERT(offsetof(TRenderFramePacket, dwPad3) == 348, "TRenderFramePacket::dwPad3");
C_STATIC_ASSERT(offsetof(TRenderFramePacket, pEffectPackets) == 352, "TRenderFramePacket::pEffectPackets");
C_STATIC_ASSERT(offsetof(TRenderFramePacket, dwEffectCount) == 360, "TRenderFramePacket::dwEffectCount");
C_STATIC_ASSERT(offsetof(TRenderFramePacket, dwPad4) == 364, "TRenderFramePacket::dwPad4");
C_STATIC_ASSERT(offsetof(TRenderFramePacket, pUIPackets) == 368, "TRenderFramePacket::pUIPackets");
C_STATIC_ASSERT(offsetof(TRenderFramePacket, dwUICount) == 376, "TRenderFramePacket::dwUICount");
C_STATIC_ASSERT(offsetof(TRenderFramePacket, dwPad5) == 380, "TRenderFramePacket::dwPad5");
C_STATIC_ASSERT(offsetof(TRenderFramePacket, pSkyBoxPackets) == 384, "TRenderFramePacket::pSkyBoxPackets");
C_STATIC_ASSERT(offsetof(TRenderFramePacket, dwSkyBoxCount) == 392, "TRenderFramePacket::dwSkyBoxCount");
C_STATIC_ASSERT(offsetof(TRenderFramePacket, dwPad6) == 396, "TRenderFramePacket::dwPad6");

int main(void)
{
    /* Weryfikacja runtime rozmiarow struktur */
    assert(sizeof(TRenderMatrix4x4) == 64);
    assert(sizeof(TRenderVec2) == 8);
    assert(sizeof(TRenderVec3) == 12);
    assert(sizeof(TRenderVec4) == 16);
    assert(sizeof(TRect32) == 16);

    assert(sizeof(TUIVertex) == 24);
    assert(sizeof(TPNT2Vertex) == 40);
    assert(sizeof(TRenderFrameUniforms) == 288);
    assert(sizeof(TTerrainSectorPacket) == 176);
    assert(sizeof(TActorRenderPacket) == 5008);
    assert(sizeof(TWaterPacket) == 112);
    assert(sizeof(TSpeedTreeInstanceData) == 112);
    assert(sizeof(TSpeedTreePacket) == 48);
    assert(sizeof(TParticleInstanceData) == 64);
    assert(sizeof(TEffectInstancePacket) == 48);
    assert(sizeof(TUIBatchPacket) == 64);
    assert(sizeof(TSkyBoxPacket) == 112);
    assert(sizeof(TDungeonBlockPacket) == 160);
    assert(sizeof(TRenderFramePacket) == 400);

    /* 1. TUIVertex */
    assert(offsetof(TUIVertex, position) == 0);
    assert(offsetof(TUIVertex, color) == 12);
    assert(offsetof(TUIVertex, texCoord) == 16);

    /* 2. TPNT2Vertex */
    assert(offsetof(TPNT2Vertex, position) == 0);
    assert(offsetof(TPNT2Vertex, normal) == 12);
    assert(offsetof(TPNT2Vertex, texCoord) == 24);
    assert(offsetof(TPNT2Vertex, texCoord2) == 32);
    assert(offsetof(TPNT2Vertex, uv0) == 24);
    assert(offsetof(TPNT2Vertex, uv1) == 32);

    /* 3. TRenderFrameUniforms */
    assert(offsetof(TRenderFrameUniforms, matView) == 0);
    assert(offsetof(TRenderFrameUniforms, matProj) == 64);
    assert(offsetof(TRenderFrameUniforms, matViewProj) == 128);
    assert(offsetof(TRenderFrameUniforms, vCameraPos) == 192);
    assert(offsetof(TRenderFrameUniforms, fTime) == 204);
    assert(offsetof(TRenderFrameUniforms, vSunLightDir) == 208);
    assert(offsetof(TRenderFrameUniforms, dwFrameIndex) == 220);
    assert(offsetof(TRenderFrameUniforms, vSunLightColor) == 224);
    assert(offsetof(TRenderFrameUniforms, v4SunColor) == 224);
    assert(offsetof(TRenderFrameUniforms, vAmbientColor) == 240);
    assert(offsetof(TRenderFrameUniforms, v4AmbientColor) == 240);
    assert(offsetof(TRenderFrameUniforms, vFogColor) == 256);
    assert(offsetof(TRenderFrameUniforms, fFogNear) == 272);
    assert(offsetof(TRenderFrameUniforms, fFogFar) == 276);
    assert(offsetof(TRenderFrameUniforms, fFogInvRange) == 280);
    assert(offsetof(TRenderFrameUniforms, fFogEnable) == 284);
    assert(offsetof(TRenderFrameUniforms, v4FogParams) == 272);

    /* 4. TTerrainSectorPacket */
    assert(offsetof(TTerrainSectorPacket, nCoordX) == 0);
    assert(offsetof(TTerrainSectorPacket, nCoordY) == 2);
    assert(offsetof(TTerrainSectorPacket, dwSectorX) == 4);
    assert(offsetof(TTerrainSectorPacket, dwSectorY) == 8);
    assert(offsetof(TTerrainSectorPacket, sectorX) == 12);
    assert(offsetof(TTerrainSectorPacket, sectorY) == 14);
    assert(offsetof(TTerrainSectorPacket, fWorldPosX) == 16);
    assert(offsetof(TTerrainSectorPacket, fWorldPosY) == 20);
    assert(offsetof(TTerrainSectorPacket, fMinHeight) == 24);
    assert(offsetof(TTerrainSectorPacket, fMaxHeight) == 28);
    assert(offsetof(TTerrainSectorPacket, dwTextureSetID) == 32);
    assert(offsetof(TTerrainSectorPacket, dwPatchCount) == 36);
    assert(offsetof(TTerrainSectorPacket, patchNum) == 40);
    assert(offsetof(TTerrainSectorPacket, splatTextureId) == 44);
    assert(offsetof(TTerrainSectorPacket, pSplatTexture) == 48);
    assert(offsetof(TTerrainSectorPacket, layerTextureIds) == 56);
    assert(offsetof(TTerrainSectorPacket, pLayerTextures) == 72);
    assert(offsetof(TTerrainSectorPacket, pVertexData) == 104);
    assert(offsetof(TTerrainSectorPacket, dwVertexCount) == 112);
    assert(offsetof(TTerrainSectorPacket, pSplatWeights) == 120);
    assert(offsetof(TTerrainSectorPacket, dwSplatWeightCount) == 128);
    assert(offsetof(TTerrainSectorPacket, pHeightMap) == 136);
    assert(offsetof(TTerrainSectorPacket, pTileMap) == 144);
    assert(offsetof(TTerrainSectorPacket, pNormalMap) == 152);
    assert(offsetof(TTerrainSectorPacket, bHasWater) == 160);
    assert(offsetof(TTerrainSectorPacket, fWaterHeight) == 164);
    assert(offsetof(TTerrainSectorPacket, fReserved) == 168);

    /* 5. TActorRenderPacket */
    assert(offsetof(TActorRenderPacket, dwActorID) == 0);
    assert(offsetof(TActorRenderPacket, dwMeshID) == 4);
    assert(offsetof(TActorRenderPacket, dwTextureID) == 8);
    assert(offsetof(TActorRenderPacket, dwBoneCount) == 12);
    assert(offsetof(TActorRenderPacket, matWorld) == 16);
    assert(offsetof(TActorRenderPacket, pBoneMatrices) == 80);
    assert(offsetof(TActorRenderPacket, boneMatrices) == 88);
    assert(offsetof(TActorRenderPacket, dwWeaponMeshID) == 4184);
    assert(offsetof(TActorRenderPacket, dwWeaponTextureID) == 4188);
    assert(offsetof(TActorRenderPacket, matWeaponSocket) == 4192);
    assert(offsetof(TActorRenderPacket, matWeaponLeftSocket) == 4256);
    assert(offsetof(TActorRenderPacket, matHelmetSocket) == 4320);
    assert(offsetof(TActorRenderPacket, matHairSocket) == 4384);
    assert(offsetof(TActorRenderPacket, dwAuraEffectCount) == 4448);
    assert(offsetof(TActorRenderPacket, matAuraEffectSockets) == 4452);
    assert(offsetof(TActorRenderPacket, dwAuraEffectIDs) == 4964);
    assert(offsetof(TActorRenderPacket, bRenderShadow) == 4996);
    assert(offsetof(TActorRenderPacket, bIsDead) == 4997);
    assert(offsetof(TActorRenderPacket, bSelected) == 4998);
    assert(offsetof(TActorRenderPacket, bAlphaBlend) == 4999);
    assert(offsetof(TActorRenderPacket, fAlpha) == 5000);
    assert(offsetof(TActorRenderPacket, fShadowSize) == 5004);

    /* 6. TWaterPacket */
    assert(offsetof(TWaterPacket, fWorldPosX) == 0);
    assert(offsetof(TWaterPacket, fWorldPosY) == 4);
    assert(offsetof(TWaterPacket, dwSectorX) == 8);
    assert(offsetof(TWaterPacket, dwSectorY) == 12);
    assert(offsetof(TWaterPacket, sectorX) == 16);
    assert(offsetof(TWaterPacket, sectorY) == 18);
    assert(offsetof(TWaterPacket, fWaterHeight) == 20);
    assert(offsetof(TWaterPacket, fWaveTime) == 24);
    assert(offsetof(TWaterPacket, fWaveFrequency) == 28);
    assert(offsetof(TWaterPacket, fWaveAmplitude) == 32);
    assert(offsetof(TWaterPacket, vWaterColor) == 36);
    assert(offsetof(TWaterPacket, fAlpha) == 52);
    assert(offsetof(TWaterPacket, dwWaterTextureID) == 56);
    assert(offsetof(TWaterPacket, pWaterTexture) == 64);
    assert(offsetof(TWaterPacket, dwCellCount) == 72);
    assert(offsetof(TWaterPacket, dwVertexCount) == 76);
    assert(offsetof(TWaterPacket, pVertexBuffer) == 80);
    assert(offsetof(TWaterPacket, dwFaceCount) == 88);
    assert(offsetof(TWaterPacket, patchNum) == 92);
    assert(offsetof(TWaterPacket, bFarPass) == 96);
    assert(offsetof(TWaterPacket, bAlphaBlend) == 97);
    assert(offsetof(TWaterPacket, fFogDistance) == 100);
    assert(offsetof(TWaterPacket, fPad) == 104);
    assert(offsetof(TWaterPacket, matTexTransformWater) == 112);

    /* 7. TSkyBoxPacket */
    assert(offsetof(TSkyBoxPacket, matWorld) == 0);
    assert(offsetof(TSkyBoxPacket, dwFaceCount) == 64);
    assert(offsetof(TSkyBoxPacket, dwCloudTextureID) == 68);
    assert(offsetof(TSkyBoxPacket, dwSkyTextureID) == 72);
    assert(offsetof(TSkyBoxPacket, fCloudScale) == 76);
    assert(offsetof(TSkyBoxPacket, fCloudHeight) == 80);
    assert(offsetof(TSkyBoxPacket, fCloudScrollSpeed) == 84);
    assert(offsetof(TSkyBoxPacket, fPad) == 88);
    assert(offsetof(TSkyBoxPacket, vSkyColor) == 92);

    /* 8. TSpeedTreeInstanceData */
    assert(offsetof(TSpeedTreeInstanceData, pTreeInstance) == 0);
    assert(offsetof(TSpeedTreeInstanceData, matWorld) == 8);
    assert(offsetof(TSpeedTreeInstanceData, v3Position) == 72);
    assert(offsetof(TSpeedTreeInstanceData, dwTreeCRC) == 84);
    assert(offsetof(TSpeedTreeInstanceData, fWindStrength) == 88);
    assert(offsetof(TSpeedTreeInstanceData, fPad) == 92);

    /* 9. TSpeedTreePacket */
    assert(offsetof(TSpeedTreePacket, pMainTree) == 0);
    assert(offsetof(TSpeedTreePacket, dwTreeCRC) == 8);
    assert(offsetof(TSpeedTreePacket, bRenderBranches) == 12);
    assert(offsetof(TSpeedTreePacket, bRenderFronds) == 13);
    assert(offsetof(TSpeedTreePacket, bRenderLeaves) == 14);
    assert(offsetof(TSpeedTreePacket, bRenderBillboards) == 15);
    assert(offsetof(TSpeedTreePacket, dwTreeTypeID) == 16);
    assert(offsetof(TSpeedTreePacket, dwTrunkTextureID) == 20);
    assert(offsetof(TSpeedTreePacket, dwLeafTextureID) == 24);
    assert(offsetof(TSpeedTreePacket, dwInstanceCount) == 28);
    assert(offsetof(TSpeedTreePacket, pInstances) == 32);
    assert(offsetof(TSpeedTreePacket, fPad) == 40);

    /* 10. TParticleInstanceData */
    assert(offsetof(TParticleInstanceData, vWorldPos) == 0);
    assert(offsetof(TParticleInstanceData, vColor) == 16);
    assert(offsetof(TParticleInstanceData, vTexCoordRect) == 32);
    assert(offsetof(TParticleInstanceData, fRotation) == 48);
    assert(offsetof(TParticleInstanceData, fLifeRatio) == 52);
    assert(offsetof(TParticleInstanceData, fHalfWidth) == 56);
    assert(offsetof(TParticleInstanceData, fHalfHeight) == 60);

    /* 11. TEffectInstancePacket */
    assert(offsetof(TEffectInstancePacket, dwEffectID) == 0);
    assert(offsetof(TEffectInstancePacket, dwTextureID) == 8);
    assert(offsetof(TEffectInstancePacket, dwBlendMode) == 16);
    assert(offsetof(TEffectInstancePacket, dwParticleCount) == 20);
    assert(offsetof(TEffectInstancePacket, pParticles) == 24);
    assert(offsetof(TEffectInstancePacket, fPad) == 32);

    /* 12. TUIBatchPacket */
    assert(offsetof(TUIBatchPacket, dwTextureID) == 0);
    assert(offsetof(TUIBatchPacket, dwBlendMode) == 8);
    assert(offsetof(TUIBatchPacket, dwVertexCount) == 12);
    assert(offsetof(TUIBatchPacket, bUseScissor) == 16);
    assert(offsetof(TUIBatchPacket, scissorRect) == 20);
    assert(offsetof(TUIBatchPacket, pVertices) == 40);
    assert(offsetof(TUIBatchPacket, fPad) == 48);

    /* 13. TDungeonBlockPacket */
    assert(offsetof(TDungeonBlockPacket, matWorld) == 0);
    assert(offsetof(TDungeonBlockPacket, dwDiffuseTextureID) == 64);
    assert(offsetof(TDungeonBlockPacket, dwLightmapTextureID) == 72);
    assert(offsetof(TDungeonBlockPacket, pDiffuseTexture) == 80);
    assert(offsetof(TDungeonBlockPacket, pLightmapTexture) == 88);
    assert(offsetof(TDungeonBlockPacket, pVertexBuffer) == 96);
    assert(offsetof(TDungeonBlockPacket, pIndexBuffer) == 104);
    assert(offsetof(TDungeonBlockPacket, pVertices) == 112);
    assert(offsetof(TDungeonBlockPacket, pIndices) == 120);
    assert(offsetof(TDungeonBlockPacket, dwVertexCount) == 128);
    assert(offsetof(TDungeonBlockPacket, dwIndexCount) == 132);
    assert(offsetof(TDungeonBlockPacket, dwPrimitiveCount) == 136);
    assert(offsetof(TDungeonBlockPacket, dwStartIndex) == 140);
    assert(offsetof(TDungeonBlockPacket, nBaseVertexIndex) == 144);
    assert(offsetof(TDungeonBlockPacket, fAlpha) == 148);
    assert(offsetof(TDungeonBlockPacket, bAlphaBlend) == 152);
    assert(offsetof(TDungeonBlockPacket, bPadDungeon) == 153);
    assert(offsetof(TDungeonBlockPacket, fPad) == 156);

    /* 14. TRenderFramePacket */
    assert(offsetof(TRenderFramePacket, uniforms) == 0);
    assert(offsetof(TRenderFramePacket, pTerrainPackets) == 288);
    assert(offsetof(TRenderFramePacket, dwTerrainCount) == 296);
    assert(offsetof(TRenderFramePacket, dwPad0) == 300);
    assert(offsetof(TRenderFramePacket, pWaterPackets) == 304);
    assert(offsetof(TRenderFramePacket, dwWaterCount) == 312);
    assert(offsetof(TRenderFramePacket, dwPad1) == 316);
    assert(offsetof(TRenderFramePacket, pSpeedTreePackets) == 320);
    assert(offsetof(TRenderFramePacket, dwSpeedTreeCount) == 328);
    assert(offsetof(TRenderFramePacket, dwPad2) == 332);
    assert(offsetof(TRenderFramePacket, pActorPackets) == 336);
    assert(offsetof(TRenderFramePacket, dwActorCount) == 344);
    assert(offsetof(TRenderFramePacket, dwPad3) == 348);
    assert(offsetof(TRenderFramePacket, pEffectPackets) == 352);
    assert(offsetof(TRenderFramePacket, dwEffectCount) == 360);
    assert(offsetof(TRenderFramePacket, dwPad4) == 364);
    assert(offsetof(TRenderFramePacket, pUIPackets) == 368);
    assert(offsetof(TRenderFramePacket, dwUICount) == 376);
    assert(offsetof(TRenderFramePacket, dwPad5) == 380);
    assert(offsetof(TRenderFramePacket, pSkyBoxPackets) == 384);
    assert(offsetof(TRenderFramePacket, dwSkyBoxCount) == 392);
    assert(offsetof(TRenderFramePacket, dwPad6) == 396);
    assert(offsetof(TRenderFramePacket, pDungeonPackets) == 400);
    assert(offsetof(TRenderFramePacket, dwDungeonCount) == 408);
    assert(offsetof(TRenderFramePacket, dwPad7) == 412);

    printf("[PASS] All RenderAbi C offsets and sizes verified successfully.\n");
    return 0;
}
