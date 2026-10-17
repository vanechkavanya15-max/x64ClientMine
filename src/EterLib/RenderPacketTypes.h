#pragma once

#include <d3d9.h>
#include <d3dx9.h>
#include <cstdint>
#include <cstring>

class CSpeedTreeWrapper;

enum EEffectBlendMode : uint32_t
{
	EFFECT_BLEND_ALPHABLEND = 0,
	EFFECT_BLEND_ADDITIVE = 1
};

enum EUIBlendMode : uint32_t
{
	UI_BLEND_NORMAL = 0,
	UI_BLEND_SCREEN = 1,
	UI_BLEND_ADDITIVE = 2
};

struct alignas(16) TRenderFrameUniforms
{
	D3DXMATRIX matView{};
	D3DXMATRIX matProj{};
	D3DXMATRIX matViewProj{};

	// Slot 0 (offset 192, 16B): Camera Position + Global Time
	D3DXVECTOR3 vCameraPos{};
	float fTime{ 0.0f };

	// Slot 1 (offset 208, 16B): Sun Light Direction + Frame Index (F-NEW-018: unia usunieta)
	D3DXVECTOR3 vSunLightDir{};
	uint32_t dwFrameIndex{ 0 };

	// Slot 2 (offset 224, 16B): Sun Light Diffuse Color
	union
	{
		D3DXCOLOR vSunLightColor{};
		D3DXVECTOR4 v4SunColor;
	};

	// Slot 3 (offset 240, 16B): Ambient Light Color
	union
	{
		D3DXCOLOR vAmbientColor{};
		D3DXVECTOR4 v4AmbientColor;
	};

	// Slot 4 (offset 256, 16B): Fog Color
	D3DXCOLOR vFogColor{};

	// Slot 5 (offset 272, 16B): Fog Range & Linear Scale Params (Near, Far, InvRange, Enable)
	union
	{
		struct
		{
			float fFogNear;
			float fFogFar;
			float fFogInvRange;
			float fFogEnable;
		};
		D3DXVECTOR4 v4FogParams;
	};

	TRenderFrameUniforms()
	{
		std::memset(this, 0, sizeof(*this));
	}
};
static_assert(sizeof(TRenderFrameUniforms) == 288, "TRenderFrameUniforms must be 288 bytes (18x vec4, 16B aligned)");
static_assert(alignof(TRenderFrameUniforms) == 16, "TRenderFrameUniforms must be 16-byte aligned");

struct alignas(16) TTerrainSectorPacket
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
	void* pSplatTexture = nullptr; // Bezpieczny wskaznik x64 na teksture splattingu D3D9/WGPU
	uint32_t layerTextureIds[4]{ 0, 0, 0, 0 };
	void* pLayerTextures[4]{ nullptr, nullptr, nullptr, nullptr }; // Bezpieczne wskazniki x64 na tekstury warstw
	void* pVertexData = nullptr;
	uint32_t dwVertexCount = 0;
	const uint32_t* pSplatWeights = nullptr;
	uint32_t dwSplatWeightCount = 0;
	const void* pHeightMap = nullptr;
	const void* pTileMap = nullptr;
	const void* pNormalMap = nullptr;
	bool bHasWater = false;
	float fWaterHeight = 0.0f;
	float fReserved = 0.0f;

	TTerrainSectorPacket()
	{
		std::memset(this, 0, sizeof(*this));
	}
};

struct alignas(16) TActorRenderPacket
{
	uint32_t dwActorID{ 0 };
	uint32_t dwMeshID{ 0 };
	uint32_t dwTextureID{ 0 };
	uint32_t dwBoneCount{ 0 };
	D3DXMATRIX matWorld{};
	const D3DXMATRIX* pBoneMatrices{ nullptr };
	D3DXMATRIX boneMatrices[64]{};
	uint32_t dwWeaponMeshID{ 0 };
	uint32_t dwWeaponTextureID{ 0 };
	D3DXMATRIX matWeaponSocket{};
	D3DXMATRIX matWeaponLeftSocket{};
	D3DXMATRIX matHelmetSocket{};
	D3DXMATRIX matHairSocket{};
	uint32_t dwAuraEffectCount{ 0 };
	D3DXMATRIX matAuraEffectSockets[8]{};
	uint32_t dwAuraEffectIDs[8]{};
	bool bRenderShadow{ false };
	bool bIsDead{ false };
	bool bSelected{ false };
	bool bAlphaBlend{ false };
	float fAlpha{ 0.0f };
	float fShadowSize{ 0.0f };

	TActorRenderPacket()
	{
		std::memset(this, 0, sizeof(*this));
	}

	// Bezpieczny dostep do macierzy kosci eliminujacy problem wiszacego wskaznika na stos
	[[nodiscard]] const D3DXMATRIX* GetBoneMatrices() const noexcept
	{
		return pBoneMatrices ? pBoneMatrices : boneMatrices;
	}
};

struct alignas(16) TWaterPacket
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
	void* pWaterTexture = nullptr; // Bezpieczny wskaznik x64 na teksture wody D3D9
	uint32_t dwCellCount = 0;
	uint32_t dwVertexCount = 0;
	void* pVertexBuffer = nullptr;
	uint32_t dwFaceCount = 0;
	uint32_t patchNum = 0;
	float fPad = 0.0f;

	TWaterPacket()
	{
		std::memset(this, 0, sizeof(*this));
		fWaveFrequency = 1.5f;
		fWaveAmplitude = 5.0f;
		vWaterColor = D3DXCOLOR(0.2f, 0.5f, 0.7f, 0.6f);
		fAlpha = 0.6f;
	}
};

struct alignas(16) TSkyBoxPacket
{
	D3DXMATRIX matWorld{};
	uint32_t dwFaceCount{ 0 };
	uint32_t dwCloudTextureID{ 0 };
	uint32_t dwSkyTextureID{ 0 };
	float fCloudScale{ 1.0f };
	float fCloudHeight{ 0.0f };
	float fCloudScrollSpeed{ 0.0f };
	float fPad{ 0.0f };
	D3DXCOLOR vSkyColor{ 0.0f, 0.0f, 0.0f, 1.0f };

	TSkyBoxPacket()
	{
		std::memset(this, 0, sizeof(*this));
		fCloudScale = 1.0f;
		vSkyColor = D3DXCOLOR(0.0f, 0.0f, 0.0f, 1.0f);
	}
};

struct alignas(16) TSpeedTreeInstanceData
{
	const CSpeedTreeWrapper* pTreeInstance{nullptr};
	D3DXMATRIX matWorld{};
	D3DXVECTOR3 v3Position{};
	uint32_t dwTreeCRC{0};
	float fWindStrength{0.0f};
	float fPad[3]{};

	TSpeedTreeInstanceData()
	{
		std::memset(this, 0, sizeof(*this));
	}
};

// 100% C-ABI POD Packet dla instancji drzew SpeedTree (zero std::vector)
struct alignas(16) TSpeedTreePacket
{
	CSpeedTreeWrapper* pMainTree{nullptr};
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

	TSpeedTreePacket()
	{
		std::memset(this, 0, sizeof(*this));
	}
};

struct alignas(16) TParticleInstanceData
{
	D3DXVECTOR4 vWorldPos{};       // x, y, z, scale
	D3DXCOLOR   vColor{};          // ARGB color tint
	D3DXVECTOR4 vTexCoordRect{};   // minU, minV, maxU, maxV
	float       fRotation{ 0.0f };
	float       fLifeRatio{ 0.0f };
	float       fHalfWidth{ 0.0f };
	float       fHalfHeight{ 0.0f };

	TParticleInstanceData()
	{
		std::memset(this, 0, sizeof(*this));
	}
};

struct alignas(16) TEffectInstancePacket
{
	uint32_t dwEffectID{0};
	uintptr_t dwTextureID{0};    // Pelny 64-bitowy uchwyt tekstury w x64
	uint32_t dwBlendMode{0};
	uint32_t dwParticleCount{0};
	const TParticleInstanceData* pParticles{nullptr};
	float fPad[2]{ 0.0f, 0.0f };

	TEffectInstancePacket()
	{
		std::memset(this, 0, sizeof(*this));
	}
};

// 24-bajtowy wierzcholek UI (zgodny binarnie z TPDTVertex: FLOAT3 + D3DCOLOR + FLOAT2)
struct TUIVertex
{
	D3DXVECTOR3 position{};
	uint32_t color{ 0 };
	D3DXVECTOR2 texCoord{};

	TUIVertex()
	{
		std::memset(this, 0, sizeof(*this));
	}
};
static_assert(sizeof(TUIVertex) == 24, "TUIVertex must be 24 bytes (FLOAT3 + D3DCOLOR + FLOAT2)");

// 100% C-ABI POD Packet dla partii wierzcholkow UI (zero alokacji sterty std::vector)
struct alignas(16) TUIBatchPacket
{
	uintptr_t dwTextureID{0};    // Pelny 64-bitowy wskaznik/uchwyt tekstury D3D9
	uint32_t dwBlendMode{0};
	uint32_t dwVertexCount{0};
	uint32_t bUseScissor{0};
	RECT scissorRect{ 0, 0, 0, 0 };
	const TUIVertex* pVertices{nullptr};
	float fPad[2]{ 0.0f, 0.0f };

	TUIBatchPacket()
	{
		std::memset(this, 0, sizeof(*this));
	}
};

struct SPNT2Vertex;
typedef struct SPNT2Vertex TPNT2Vertex;

// 160-bajtowy pakiet bloku dungeonu z 2-teksturowym probkowaniem bazy i mapy swiatla Lightmap (TPNT2Vertex)
struct alignas(16) TDungeonBlockPacket
{
	D3DXMATRIX matWorld{
		1.0f, 0.0f, 0.0f, 0.0f,
		0.0f, 1.0f, 0.0f, 0.0f,
		0.0f, 0.0f, 1.0f, 0.0f,
		0.0f, 0.0f, 0.0f, 1.0f
	};
	uintptr_t dwDiffuseTextureID{ 0 };
	uintptr_t dwLightmapTextureID{ 0 };
	void* pDiffuseTexture{ nullptr };
	void* pLightmapTexture{ nullptr };
	void* pVertexBuffer{ nullptr };
	void* pIndexBuffer{ nullptr };
	const TPNT2Vertex* pVertices{ nullptr };
	const uint16_t* pIndices{ nullptr };
	uint32_t dwVertexCount{ 0 };
	uint32_t dwIndexCount{ 0 };
	uint32_t dwPrimitiveCount{ 0 };
	uint32_t dwStartIndex{ 0 };
	int32_t nBaseVertexIndex{ 0 };
	float fAlpha{ 1.0f };
	uint8_t bAlphaBlend{ 0 };
	uint8_t bPadDungeon[3]{ 0, 0, 0 };
	float fPad{ 0.0f };

	TDungeonBlockPacket()
	{
		std::memset(this, 0, sizeof(*this));
		matWorld._11 = 1.0f;
		matWorld._22 = 1.0f;
		matWorld._33 = 1.0f;
		matWorld._44 = 1.0f;
		fAlpha = 1.0f;
	}
};

// Zbiorczy pakiet C-ABI calej klatki dla mostu FFI (m2_wgpu_render_frame)
struct alignas(16) TRenderFramePacket
{
	TRenderFrameUniforms uniforms{};
	const TTerrainSectorPacket* pTerrainPackets{nullptr};
	uint32_t dwTerrainCount{0};
	uint32_t dwPad0{0};
	const TWaterPacket* pWaterPackets{nullptr};
	uint32_t dwWaterCount{0};
	uint32_t dwPad1{0};
	const TSpeedTreePacket* pSpeedTreePackets{nullptr};
	uint32_t dwSpeedTreeCount{0};
	uint32_t dwPad2{0};
	const TActorRenderPacket* pActorPackets{nullptr};
	uint32_t dwActorCount{0};
	uint32_t dwPad3{0};
	const TEffectInstancePacket* pEffectPackets{nullptr};
	uint32_t dwEffectCount{0};
	uint32_t dwPad4{0};
	const TUIBatchPacket* pUIPackets{nullptr};
	uint32_t dwUICount{0};
	uint32_t dwPad5{0};
	const TSkyBoxPacket* pSkyBoxPackets{nullptr};
	uint32_t dwSkyBoxCount{0};
	uint32_t dwPad6{0};

	TRenderFramePacket()
	{
		std::memset(this, 0, sizeof(*this));
	}
};

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
static_assert(sizeof(TActorRenderPacket) == 5008, "ABI: TActorRenderPacket != packets.rs");
static_assert(sizeof(TWaterPacket) == 112, "ABI: TWaterPacket != packets.rs");
static_assert(sizeof(TSpeedTreeInstanceData) == 112, "ABI: TSpeedTreeInstanceData != packets.rs");
static_assert(sizeof(TSpeedTreePacket) == 48, "ABI: TSpeedTreePacket != packets.rs");
static_assert(sizeof(TParticleInstanceData) == 64, "ABI: TParticleInstanceData != packets.rs");
static_assert(sizeof(TEffectInstancePacket) == 48, "ABI: TEffectInstancePacket != packets.rs");
static_assert(sizeof(TUIBatchPacket) == 64, "ABI: TUIBatchPacket != packets.rs");
static_assert(sizeof(TRenderFramePacket) == 400, "ABI: TRenderFramePacket != packets.rs");
static_assert(offsetof(TRenderFramePacket, pSkyBoxPackets) == 384, "ABI: frame skybox ptr");
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
#endif


