#pragma once

#include <d3d9.h>
#include <d3dx9.h>
#include <cstdint>
#include <cstddef>
#include <type_traits>
#include <atomic>
#include <cstring>

class CCamera;

// Wyrownana do 16 bajtow reprezentacja pojedynczego swiatla zgodna z WGSL (std140) i bytemuck::Pod (112B)
struct alignas(16) TFrameLightPod
{
	union {
		float diffuse[4];        // offset 0,  16B (r, g, b, a)
		D3DCOLORVALUE Diffuse;
	};
	union {
		float specular[4];       // offset 16, 16B (r, g, b, a)
		D3DCOLORVALUE Specular;
	};
	union {
		float ambient[4];        // offset 32, 16B (r, g, b, a)
		D3DCOLORVALUE Ambient;
	};
	union {
		float position[3];       // offset 48, 12B (x, y, z)
		D3DVECTOR Position;
	};
	union {
		float range;             // offset 60, 4B  (dopelnia wektor do 16B!)
		float Range;
	};
	union {
		float direction[3];      // offset 64, 12B (x, y, z)
		D3DVECTOR Direction;
	};
	union {
		float falloff;           // offset 76, 4B  (dopelnia wektor do 16B!)
		float Falloff;
	};
	union {
		float attenuation[3];    // offset 80, 12B (att0, att1, att2)
		struct { float a0, a1, a2; } Attenuation;
	};
	union {
		float theta;             // offset 92, 4B  (dopelnia wektor do 16B!)
		float Theta;
	};
	union {
		float phi;               // offset 96, 4B
		float Phi;
	};
	union {
		uint32_t lightType;      // offset 100, 4B (jawny uint32_t zamiast enum D3DLIGHTTYPE)
		uint32_t Type;           // offset 100, 4B (kompatybilnosc wsteczna z D3DLIGHT9)
	};
	uint32_t isEnabled;          // offset 104, 4B (0 lub 1)
	uint32_t _pad0;              // offset 108, 4B (jawny padding do 112B = 7 * 16B)

	TFrameLightPod& operator=(const D3DLIGHT9& src) noexcept
	{
		diffuse[0] = src.Diffuse.r; diffuse[1] = src.Diffuse.g; diffuse[2] = src.Diffuse.b; diffuse[3] = src.Diffuse.a;
		specular[0] = src.Specular.r; specular[1] = src.Specular.g; specular[2] = src.Specular.b; specular[3] = src.Specular.a;
		ambient[0] = src.Ambient.r; ambient[1] = src.Ambient.g; ambient[2] = src.Ambient.b; ambient[3] = src.Ambient.a;
		position[0] = src.Position.x; position[1] = src.Position.y; position[2] = src.Position.z;
		range = src.Range;
		direction[0] = src.Direction.x; direction[1] = src.Direction.y; direction[2] = src.Direction.z;
		falloff = src.Falloff;
		attenuation[0] = src.Attenuation0; attenuation[1] = src.Attenuation1; attenuation[2] = src.Attenuation2;
		theta = src.Theta; phi = src.Phi;
		lightType = static_cast<uint32_t>(src.Type);
		isEnabled = 1u;
		_pad0 = 0u;
		return *this;
	}

	operator D3DLIGHT9() const noexcept
	{
		D3DLIGHT9 l{};
		l.Type = static_cast<D3DLIGHTTYPE>(lightType);
		l.Diffuse = D3DCOLORVALUE{ Diffuse.r, Diffuse.g, Diffuse.b, Diffuse.a };
		l.Specular = D3DCOLORVALUE{ Specular.r, Specular.g, Specular.b, Specular.a };
		l.Ambient = D3DCOLORVALUE{ Ambient.r, Ambient.g, Ambient.b, Ambient.a };
		l.Position = D3DVECTOR{ Position.x, Position.y, Position.z };
		l.Direction = D3DVECTOR{ Direction.x, Direction.y, Direction.z };
		l.Range = range;
		l.Falloff = falloff;
		l.Attenuation0 = attenuation[0];
		l.Attenuation1 = attenuation[1];
		l.Attenuation2 = attenuation[2];
		l.Theta = theta;
		l.Phi = phi;
		return l;
	}
};

static_assert(sizeof(TFrameLightPod) == 112, "TFrameLightPod must be exactly 112 bytes (multiple of 16)");
static_assert(alignof(TFrameLightPod) == 16, "TFrameLightPod must be 16-byte aligned");
static_assert(std::is_trivially_copyable_v<TFrameLightPod> && std::is_standard_layout_v<TFrameLightPod>, "TFrameLightPod must be POD");

// Niezmienna migawka klatki przekazywana przez granice FFI do Rust WGPU (640B)
struct alignas(16) TImmutableFrameSnapshot
{
	union {
		float matViewFloats[16];     // offset 0,   size 64
		D3DXMATRIX matView;
		D3DMATRIX d3dMatView;
	};
	union {
		float matProjFloats[16];     // offset 64,  size 64
		D3DXMATRIX matProj;
		D3DMATRIX d3dMatProj;
	};
	union {
		float matViewProjFloats[16]; // offset 128, size 64
		D3DXMATRIX matViewProj;
		D3DMATRIX d3dMatViewProj;
	};
	union {
		float matInvViewFloats[16];  // offset 192, size 64
		D3DXMATRIX matInvView;
		D3DMATRIX d3dMatInvView;
	};

	union {
		float v3CameraPosFloats[3];  // offset 256, size 12
		D3DXVECTOR3 v3CameraPos;
		struct { float x, y, z; } CameraPos;
	};
	float fTime;                     // offset 268, size 4 (dopelnia wektor do 16B: 256..272)

	union {
		float v4FogParamsFloats[4];  // offset 272, size 16 (wyrownane do 16B)
		D3DXVECTOR4 v4FogParams;
		struct { float x, y, z, w; } FogParams;
	};

	TFrameLightPod lights[3];        // offset 288, size 336 (3 * 112B, koniec na 624)

	uint32_t dwFrameIndex;           // offset 624, size 4
	uint32_t dwOwnershipMask;        // offset 628, size 4 (zatrzasnieta maska bitowa WGPU)
	uint64_t uSequenceNumber;        // offset 632, size 8 (monotoniczny licznik klatek)

	TImmutableFrameSnapshot() noexcept
	{
		std::memset(this, 0, sizeof(*this));
		matView._11 = matView._22 = matView._33 = matView._44 = 1.0f;
		matProj._11 = matProj._22 = matProj._33 = matProj._44 = 1.0f;
		matViewProj._11 = matViewProj._22 = matViewProj._33 = matViewProj._44 = 1.0f;
		matInvView._11 = matInvView._22 = matInvView._33 = matInvView._44 = 1.0f;
	}

	const D3DXMATRIX& GetViewMatrix() const noexcept { return matView; }
	const D3DXMATRIX& GetProjMatrix() const noexcept { return matProj; }
	const D3DXMATRIX& GetViewProjMatrix() const noexcept { return matViewProj; }
	const D3DXMATRIX& GetInvViewMatrix() const noexcept { return matInvView; }
};

static_assert(sizeof(TImmutableFrameSnapshot) == 640, "TImmutableFrameSnapshot size must be 640 bytes");
static_assert(alignof(TImmutableFrameSnapshot) == 16, "TImmutableFrameSnapshot alignment must be 16 bytes");
static_assert(offsetof(TImmutableFrameSnapshot, v3CameraPos) == 256, "Invalid offset: v3CameraPos");
static_assert(offsetof(TImmutableFrameSnapshot, fTime) == 268, "Invalid offset: fTime");
static_assert(offsetof(TImmutableFrameSnapshot, v4FogParams) == 272, "Invalid offset: v4FogParams");
static_assert(offsetof(TImmutableFrameSnapshot, lights) == 288, "Invalid offset: lights");
static_assert(offsetof(TImmutableFrameSnapshot, dwFrameIndex) == 624, "Invalid offset: dwFrameIndex");
static_assert(offsetof(TImmutableFrameSnapshot, dwOwnershipMask) == 628, "Invalid offset: dwOwnershipMask");
static_assert(offsetof(TImmutableFrameSnapshot, uSequenceNumber) == 632, "Invalid offset: uSequenceNumber");
static_assert(std::is_trivially_copyable_v<TImmutableFrameSnapshot> && std::is_standard_layout_v<TImmutableFrameSnapshot>, "TImmutableFrameSnapshot must be POD");

using TFrameSnapshot = TImmutableFrameSnapshot;

class CFrameSnapshot
{
public:
	static constexpr size_t BUFFER_COUNT = 2;

	// Watek symulacji (Producer): Buduje migawke w BackBufferze i wykonuje Atomic Swap
	static void Capture(
		const CCamera& camera,
		float fFov,
		float fAspect,
		float fNear,
		float fFar,
		float fTime = 0.0f,
		uint32_t dwFrameIndex = 0,
		const D3DXVECTOR4* pFogParams = nullptr,
		const D3DLIGHT9* pLights = nullptr
	) noexcept;

	static void Capture(
		const CCamera* pCamera,
		float fFov,
		float fAspect,
		float fNear,
		float fFar,
		float fTime = 0.0f,
		uint32_t dwFrameIndex = 0,
		const D3DXVECTOR4* pFogParams = nullptr,
		const D3DLIGHT9* pLights = nullptr
	) noexcept;

	// Watek renderera (Consumer - Rust WGPU / C++): Zwraca niezmienny wskaznik do FrontBuffera (Acquire)
	static const TImmutableFrameSnapshot* GetPublishedPtr() noexcept;
	static const TImmutableFrameSnapshot& Current() noexcept;

	static void SetCurrent(const TImmutableFrameSnapshot& snapshot) noexcept;
	static void Reset() noexcept;

private:
	static void InitBufferDefaults(TImmutableFrameSnapshot& outSnap) noexcept;
	static void PackD3DLight(const D3DLIGHT9& src, bool bEnabled, TFrameLightPod& dst) noexcept;

	alignas(64) static TImmutableFrameSnapshot ms_buffers[BUFFER_COUNT];
	alignas(64) static std::atomic<const TImmutableFrameSnapshot*> ms_pFrontBuffer;
	static uint32_t ms_uWriteIndex;
	static uint64_t ms_uSequenceCounter;
};

// Eksport C-ABI dla skrzynki m2-client-render (bezposredni lock-free odczyt po stronie Rust)
extern "C" const TImmutableFrameSnapshot* m2_frame_snapshot_get_published() noexcept;
