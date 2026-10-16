#pragma once

#include "IRenderBackend.h"
#include <d3d9.h>
#include <d3dx9.h>

class CD3D9RenderBackend : public IRenderBackend
{
public:
	static CD3D9RenderBackend& Instance()
	{
		static CD3D9RenderBackend s_instance;
		return s_instance;
	}

	CD3D9RenderBackend();
	virtual ~CD3D9RenderBackend() override;

	virtual bool Initialize(HWND hWnd, unsigned int width, unsigned int height) override;
	virtual void Shutdown() override;
	virtual void Resize(unsigned int width, unsigned int height) override;
	virtual void BeginFrame(const TRenderFrameUniforms& uniforms) override;

	virtual void RenderTerrain(const TTerrainSectorPacket* pPackets, unsigned int count) override;
	virtual void RenderActors(const TActorRenderPacket* pPackets, unsigned int count) override;
	virtual void RenderWater(const TWaterPacket* pPackets, unsigned int count) override;
	virtual void RenderSpeedTree(const TSpeedTreePacket* pPackets, unsigned int count) override;
	virtual void RenderEffects(const TEffectInstancePacket* pPackets, unsigned int count) override;
	virtual void RenderUI(const TUIBatchPacket* pPackets, unsigned int count) override;

	virtual void EndFrame() override;
	virtual const char* GetBackendName() const override;
	virtual bool IsDeviceLost() const override;

private:
	void InitVertexDeclarations();
	void ReleaseVertexDeclarations();

	HWND m_hWnd{ nullptr };
	unsigned int m_width{ 0 };
	unsigned int m_height{ 0 };
	bool m_bInitialized{ false };
	LPDIRECT3DVERTEXDECLARATION9 m_pUIVertexDecl{ nullptr };
	LPDIRECT3DVERTEXDECLARATION9 m_pActorVertexDecl{ nullptr };
};
