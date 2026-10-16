#pragma once

#include <d3d9.h>
#include <d3dx9.h>
#include <cstdint>
#include <vector>

class CSpeedTreeWrapper;

enum EEffectBlendMode : uint32_t
{
	EFFECT_BLEND_ALPHABLEND = 0,
	EFFECT_BLEND_ADDITIVE = 1
};

struct TRenderFrameUniforms
{
	D3DXMATRIX matView;
	D3DXMATRIX matProj;
	D3DXMATRIX matViewProj;
	D3DXVECTOR3 vCameraPos;
	D3DXVECTOR3 vSunLightDir;
	D3DXCOLOR vSunLightColor;
	D3DXCOLOR vAmbientColor;
	D3DXCOLOR vFogColor;
	float fFogNear;
	float fFogFar;
	float fTime;
	uint32_t dwFrameIndex;

	D3DXVECTOR4 v4SunDirection;
	D3DXVECTOR4 v4SunColor;
	D3DXVECTOR4 v4AmbientColor;
	D3DXVECTOR4 v4FogParams;
};

struct TTerrainSectorPacket
{
	int16_t nCoordX = 0;
	int16_t nCoordY = 0;
	uint32_t dwSectorX = 0;
	uint32_t dwSectorY = 0;
	uint16_t sectorX = 0;
	uint16_t sectorY = 0;
	float fWorldPosX = 0.0f;
	float fWorldPosY = 0.0f;
	float fMinHeight = 0.0f;
	float fMaxHeight = 0.0f;
	uint32_t dwTextureSetID = 0;
	uint32_t dwPatchCount = 0;
	uint32_t patchNum = 0;
	uint32_t splatTextureId = 0;
	uint32_t layerTextureIds[4]{ 0, 0, 0, 0 };
	void* pVertexData = nullptr;
	uint32_t dwVertexCount = 0;
	const uint32_t* pSplatWeights = nullptr;
	uint32_t dwSplatWeightCount = 0;
	const void* pHeightMap = nullptr;
	const void* pTileMap = nullptr;
	const void* pNormalMap = nullptr;
	bool bHasWater = false;
	uint8_t bPad = 0;
	float fWaterHeight = 0.0f;
	float fReserved = 0.0f;
};

struct TActorRenderPacket
{
	uint32_t dwActorID;
	uint32_t dwMeshID;
	uint32_t dwTextureID;
	uint32_t dwBoneCount;
	D3DXMATRIX matWorld;
	const D3DXMATRIX* pBoneMatrices;
	D3DXMATRIX boneMatrices[64];
	uint32_t dwWeaponMeshID;
	uint32_t dwWeaponTextureID;
	D3DXMATRIX matWeaponSocket;
	D3DXMATRIX matWeaponLeftSocket;
	D3DXMATRIX matHelmetSocket;
	D3DXMATRIX matHairSocket;
	uint32_t dwAuraEffectCount;
	D3DXMATRIX matAuraEffectSockets[8];
	uint32_t dwAuraEffectIDs[8];
	bool bRenderShadow;
	bool bIsDead;
	bool bSelected;
	bool bAlphaBlend;
	float fAlpha;
	float fPad;
};

struct TWaterPacket
{
	float fWorldPosX = 0.0f;
	float fWorldPosY = 0.0f;
	uint32_t dwSectorX = 0;
	uint32_t dwSectorY = 0;
	uint16_t sectorX = 0;
	uint16_t sectorY = 0;
	float fWaterHeight = 0.0f;
	float fWaveTime = 0.0f;
	float fWaveFrequency = 1.5f;
	float fWaveAmplitude = 5.0f;
	D3DXCOLOR vWaterColor{ 0.2f, 0.5f, 0.7f, 0.6f };
	float fAlpha = 0.6f;
	uint32_t dwWaterTextureID = 0;
	uint32_t dwCellCount = 0;
	uint32_t dwVertexCount = 0;
	void* pVertexBuffer = nullptr;
	uint32_t dwFaceCount = 0;
	uint32_t patchNum = 0;
	float fPad = 0.0f;
};

struct TSpeedTreeInstanceData
{
	const CSpeedTreeWrapper* pTreeInstance{nullptr};
	D3DXMATRIX matWorld{};
	D3DXVECTOR3 v3Position{};
	uint32_t dwTreeCRC{0};
	float fWindStrength{0.0f};
	float fPad[3]{};
};

struct TSpeedTreePacket
{
	CSpeedTreeWrapper* pMainTree{nullptr};
	std::vector<TSpeedTreeInstanceData> instances{};
	uint32_t dwTreeCRC{0};
	bool bRenderBranches{false};
	bool bRenderFronds{false};
	bool bRenderLeaves{false};
	bool bRenderBillboards{false};
	uint32_t dwTreeTypeID{0};
	uint32_t dwTrunkTextureID{0};
	uint32_t dwLeafTextureID{0};
	uint32_t dwInstanceCount{0};
	const TSpeedTreeInstanceData* pInstances{nullptr};
	float fPad[2]{};
};

struct TParticleInstanceData
{
	D3DXVECTOR4 vWorldPos;       // x, y, z, scale
	D3DXCOLOR   vColor;          // ARGB color tint
	D3DXVECTOR4 vTexCoordRect;   // minU, minV, maxU, maxV
	float       fRotation;
	float       fLifeRatio;
	float       fPad[2];
};

struct TEffectInstancePacket
{
	uint32_t dwEffectID;
	uint32_t dwTextureID;
	uint32_t dwBlendMode;
	uint32_t dwParticleCount;
	const TParticleInstanceData* pParticles;
	float fPad[2];
};

struct TUIVertex
{
	D3DXVECTOR3 position;
	D3DXCOLOR color;
	D3DXVECTOR2 texCoord;
};

struct TUIBatchPacket
{
	std::vector<TUIVertex> vertices;
	std::vector<uint16_t> indices;
	uint32_t dwTextureId{0};
	uint32_t dwTextureID{0};
	bool bScissorEnable{false};
	RECT scissorRect{};
	uint32_t dwBlendMode{0};
	uint32_t dwVertexCount{0};
	uint32_t bUseScissor{0};
	const TUIVertex* pVertices{nullptr};
	float fPad[2]{};
};

namespace EterLib::Render
{
	struct TActorBonePalette
	{
		const D3DXMATRIX* pBoneMatrices{ nullptr };
		uint32_t boneCount{ 0 };
	};
}

