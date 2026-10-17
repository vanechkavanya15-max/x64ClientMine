#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include <d3d9.h>
#include <d3dx9.h>
#include <cstdint>
#include <cstring>
#include <cmath>

#include "EterLib/RenderPacketTypes.h"
#include "EterLib/RenderResourceRegistry.h"
#include "EterLib/RenderOwnership.h"
#include "GameLib/MapOutdoorCharacterShadow.h"
#include "UserInterface/MiniMap/MiniMapCoordinateProjector.h"

TEST_CASE("ShadowMinimap - ABI i uklad pamieci struktur passow")
{
	CHECK(sizeof(TRenderPassDesc) == 144u);
	CHECK(sizeof(TShadowPassData) == 272u);
	CHECK(alignof(TShadowPassData) == 16u);

	CHECK(static_cast<uint32_t>(RENDER_PASS_MAIN) == 0u);
	CHECK(static_cast<uint32_t>(RENDER_PASS_CHARACTER_SHADOW) == 1u);
	CHECK(static_cast<uint32_t>(RENDER_PASS_MINIMAP) == 2u);

	TRenderPassDesc passDesc{};
	CHECK(passDesc.passType == 0u);
	CHECK(passDesc.targetHandle == 0u);
	CHECK(passDesc.clearColor == 0u);
	CHECK(passDesc.bClearTarget == 0u);

	TShadowPassData shadowData{};
	CHECK(shadowData.passDesc.passType == 0u);
	CHECK(shadowData.fRange == 0.0f);
	CHECK(shadowData.wShadowMapSize == 0u);
	CHECK(shadowData.hRenderTarget == 0u);
	CHECK(shadowData.viewport.left == 0);
	CHECK(shadowData.viewport.right == 0);
}

TEST_CASE("Shadow - Poprawnosc i determinizm CalculateLightViewMatrix")
{
	const D3DXVECTOR3 v3Target(1500.0f, 2500.0f, 120.0f);

	// Dawne wyliczenie z MapOutdoorRender.cpp:
	D3DXVECTOR3 v3OldLightEye(v3Target.x - 1.732f * 1250.0f,
	                          v3Target.y - 1250.0f,
	                          v3Target.z + 2.0f * 1.732f * 1250.0f);
	const auto v3Up = D3DXVECTOR3(0.0f, 0.0f, 1.0f);
	D3DXMATRIX matOldLightView;
	D3DXMatrixLookAtRH(&matOldLightView, &v3OldLightEye, &v3Target, &v3Up);

	// Nowe wspolne wyliczenie przez CalculateLightViewMatrix:
	D3DXMATRIX matNewLightView;
	CalculateLightViewMatrix(&matNewLightView, kDefaultCharacterShadowLightDir, v3Target, 5000.0f);

	// Weryfikacja matematycznej tozsamosci macierzy
	for (int i = 0; i < 4; ++i)
	{
		for (int j = 0; j < 4; ++j)
		{
			CHECK(doctest::Approx(matNewLightView.m[i][j]).epsilon(0.001f) == matOldLightView.m[i][j]);
		}
	}

	// Alias BuildCharacterShadowMatrix musi zwracac dokladnie to samo
	D3DXMATRIX matAliasView;
	BuildCharacterShadowMatrix(&matAliasView, kDefaultCharacterShadowLightDir, v3Target, 5000.0f);
	for (int i = 0; i < 4; ++i)
	{
		for (int j = 0; j < 4; ++j)
		{
			CHECK(matAliasView.m[i][j] == matNewLightView.m[i][j]);
		}
	}
}

TEST_CASE("Shadow - Odpornosc CalculateLightViewMatrix na wektory brzegowe")
{
	const D3DXVECTOR3 v3Target(0.0f, 0.0f, 0.0f);

	// Zerowy kierunek swiatla -> uzycie domyslnego kierunku swiatla
	D3DXMATRIX matZeroDir;
	CalculateLightViewMatrix(&matZeroDir, D3DXVECTOR3(0.0f, 0.0f, 0.0f), v3Target, 5000.0f);

	D3DXMATRIX matDefaultDir;
	CalculateLightViewMatrix(&matDefaultDir, kDefaultCharacterShadowLightDir, v3Target, 5000.0f);

	for (int i = 0; i < 4; ++i)
	{
		for (int j = 0; j < 4; ++j)
		{
			CHECK(doctest::Approx(matZeroDir.m[i][j]).epsilon(0.0001f) == matDefaultDir.m[i][j]);
		}
	}

	// Kierunek swiatla w osi Z (rownolegly do wektora up) -> przelaczenie wektora up na os Y
	D3DXMATRIX matParallelZ;
	CalculateLightViewMatrix(&matParallelZ, D3DXVECTOR3(0.0f, 0.0f, 1.0f), v3Target, 5000.0f);
	// Macierz nie moze zawierac wartosci NaN
	for (int i = 0; i < 4; ++i)
	{
		for (int j = 0; j < 4; ++j)
		{
			CHECK_FALSE(std::isnan(matParallelZ.m[i][j]));
		}
	}
}

TEST_CASE("Shadow - Rzut ortogonalny i macierz dynamicznego cienia")
{
	D3DXMATRIX matProj;
	CalculateLightProjMatrix(&matProj, 2550.0f, 2550.0f, 1.0f, 15000.0f);

	// Sprawdzenie elementow macierzy ortogonalnej RH
	CHECK(doctest::Approx(matProj._11).epsilon(0.001f) == 2.0f / 2550.0f);
	CHECK(doctest::Approx(matProj._22).epsilon(0.001f) == 2.0f / 2550.0f);
	CHECK(matProj._33 < 0.0f); // Projekcja RH ma ujemne skalowanie Z
	CHECK(matProj._44 == 1.0f);

	D3DXMATRIX matInvView;
	D3DXMatrixIdentity(&matInvView);

	D3DXMATRIX matLightView;
	CalculateLightViewMatrix(&matLightView, kDefaultCharacterShadowLightDir, D3DXVECTOR3(100.0f, 200.0f, 0.0f), 5000.0f);

	D3DXMATRIX matShadowScale;
	D3DXMatrixScaling(&matShadowScale, 0.5f, -0.5f, 1.0f);

	D3DXMATRIX matDynamicShadow;
	CalculateDynamicShadowMatrix(&matDynamicShadow, matInvView, matLightView, matShadowScale);

	// Oczekiwana macierz = matInvView * matLightView * matShadowScale
	D3DXMATRIX matExpected = matInvView * matLightView * matShadowScale;
	for (int i = 0; i < 4; ++i)
	{
		for (int j = 0; j < 4; ++j)
		{
			CHECK(doctest::Approx(matDynamicShadow.m[i][j]).epsilon(0.0001f) == matExpected.m[i][j]);
		}
	}
}

TEST_CASE("Shadow - TRenderPassDesc i integracja z CRenderResourceRegistry")
{
	CRenderResourceRegistry registry;

	// Symulacja uchwytu render targetu cienia w rejestrze zasobow
	IDirect3DTexture9* pDummyTex = reinterpret_cast<IDirect3DTexture9*>(static_cast<uintptr_t>(0xAABBCCDD));
	TRenderTextureHandle hRT = registry.RegisterTextureWithInfo(pDummyTex, 256, 256, D3DFMT_R5G6B5, "CharacterShadowRT");

	CHECK(hRT != 0u);
	CHECK(registry.IsValidHandle(hRT));
	CHECK(registry.GetWidth(hRT) == 256u);
	CHECK(registry.GetHeight(hRT) == 256u);
	CHECK(registry.GetFormat(hRT) == D3DFMT_R5G6B5);

	// Konfiguracja TRenderPassDesc z uzyciem uchwytu
	TRenderPassDesc passDesc{};
	passDesc.passType = RENDER_PASS_CHARACTER_SHADOW;
	passDesc.targetHandle = hRT;
	passDesc.clearColor = 0xFFFFFFFF;
	passDesc.bClearTarget = 1;

	D3DXMATRIX matLightView, matLightProj;
	CalculateLightViewMatrix(&matLightView, kDefaultCharacterShadowLightDir, D3DXVECTOR3(0.0f, 0.0f, 0.0f), 5000.0f);
	CalculateLightProjMatrix(&matLightProj, 2550.0f, 2550.0f, 1.0f, 15000.0f);
	passDesc.matView = ToAbi(matLightView);
	passDesc.matProj = ToAbi(matLightProj);

	CHECK(passDesc.passType == RENDER_PASS_CHARACTER_SHADOW);
	CHECK(passDesc.targetHandle == hRT);
	CHECK(passDesc.clearColor == 0xFFFFFFFF);
	CHECK(passDesc.bClearTarget == 1);

	// Wypelnienie TShadowPassData
	TShadowPassData shadowPass{};
	shadowPass.passDesc = passDesc;
	shadowPass.v3LightDir = ToAbi(kDefaultCharacterShadowLightDir);
	shadowPass.fRange = 5000.0f;
	shadowPass.wShadowMapSize = 256;
	shadowPass.hRenderTarget = hRT;
	shadowPass.viewport.left = 1;
	shadowPass.viewport.top = 1;
	shadowPass.viewport.right = 255;
	shadowPass.viewport.bottom = 255;

	CHECK(shadowPass.hRenderTarget == hRT);
	CHECK(shadowPass.wShadowMapSize == 256u);
	CHECK(shadowPass.viewport.right - shadowPass.viewport.left == 254);

	registry.UnregisterTexture(hRT);
	CHECK_FALSE(registry.IsValidHandle(hRT));
}

TEST_CASE("MiniMap - Determinizm macierzy cover i parametrow materialu")
{
	const float fScreenX = 120.0f;
	const float fScreenY = 80.0f;
	const float fFilterWidth = 256.0f;
	const float fFilterHeight = 256.0f;

	D3DXMATRIX matCover1 = SMiniMapTerrainPass::ComputeCoverMatrix(fScreenX, fScreenY, fFilterWidth, fFilterHeight);
	D3DXMATRIX matCover2 = SMiniMapTerrainPass::ComputeCoverMatrix(fScreenX, fScreenY, fFilterWidth, fFilterHeight);

	// Scisly determinizm:
	for (int i = 0; i < 4; ++i)
	{
		for (int j = 0; j < 4; ++j)
		{
			CHECK(matCover1.m[i][j] == matCover2.m[i][j]);
		}
	}

	CHECK(doctest::Approx(matCover1._11).epsilon(0.0001f) == 1.0f / 256.0f);
	CHECK(doctest::Approx(matCover1._22).epsilon(0.0001f) == 1.0f / 256.0f);
	CHECK(matCover1._33 == 0.0f);
	CHECK(doctest::Approx(matCover1._41).epsilon(0.0001f) == -120.0f / 256.0f);
	CHECK(doctest::Approx(matCover1._42).epsilon(0.0001f) == -80.0f / 256.0f);

	// Domyslne parametry materialu minimapy
	SMiniMapTerrainPass terrainPass{};
	CHECK(terrainPass.filterSamplerMin == D3DTEXF_POINT);
	CHECK(terrainPass.filterSamplerMag == D3DTEXF_POINT);
	CHECK(terrainPass.addressModeU == D3DTADDRESS_CLAMP);
	CHECK(terrainPass.addressModeV == D3DTADDRESS_CLAMP);
	CHECK(terrainPass.dwTextureFactor == 0xFF000000);
	CHECK(terrainPass.colorOpStage0 == D3DTOP_SELECTARG1);
	CHECK(terrainPass.colorOpStage1 == D3DTOP_MODULATE);
}

TEST_CASE("MiniMap - Integracja z TerrainMeshTransform i rzutowaniem wspolrzednych")
{
	const float fWidth = 128.0f;
	const float fHeight = 128.0f;
	const float fScale = 2.0f;
	const float fCenterCellX = 32.0f;
	const float fCenterCellY = 32.0f;
	const float fScreenX = 10.0f;
	const float fScreenY = 20.0f;

	MiniMap::TerrainMeshTransform t1 = MiniMap::MiniMapCoordinateProjector::ComputeTerrainMeshTransform(
		fWidth, fHeight, fScale, fCenterCellX, fCenterCellY, fScreenX, fScreenY);
	MiniMap::TerrainMeshTransform t2 = MiniMap::MiniMapCoordinateProjector::ComputeTerrainMeshTransform(
		fWidth, fHeight, fScale, fCenterCellX, fCenterCellY, fScreenX, fScreenY);

	CHECK(t1.scaleX == t2.scaleX);
	CHECK(t1.scaleY == t2.scaleY);
	CHECK(t1.translationX == t2.translationX);
	CHECK(t1.translationY == t2.translationY);

	// Przypisanie do macierzy swiata
	D3DXMATRIX matWorld;
	D3DXMatrixIdentity(&matWorld);
	matWorld._11 = t1.scaleX;
	matWorld._22 = t1.scaleY;
	matWorld._41 = t1.translationX;
	matWorld._42 = t1.translationY;

	CHECK(matWorld._11 == t1.scaleX);
	CHECK(matWorld._22 == t1.scaleY);
	CHECK(matWorld._41 == t1.translationX);
	CHECK(matWorld._42 == t1.translationY);
}

TEST_CASE("RenderOwnership - Bramkowanie Shadow i MiniMap")
{
	RenderOwnership::ResetDefaults();

	// Domyslnie wlasnosc nalezy do Legacy
	CHECK(RenderOwnership::Owner(ERenderCategory::Shadow) == ERenderOwner::Legacy);
	CHECK(RenderOwnership::LegacyDraws(ERenderCategory::Shadow) == true);

	CHECK(RenderOwnership::Owner(ERenderCategory::MiniMap) == ERenderOwner::Legacy);
	CHECK(RenderOwnership::LegacyDraws(ERenderCategory::MiniMap) == true);

	// Przelaczenie Shadow na D3D9Backend
	RenderOwnership::SetOwner(ERenderCategory::Shadow, ERenderOwner::D3D9Backend);
	CHECK(RenderOwnership::Owner(ERenderCategory::Shadow) == ERenderOwner::D3D9Backend);
	CHECK_FALSE(RenderOwnership::LegacyDraws(ERenderCategory::Shadow));

	// Przelaczenie MiniMap na Wgpu
	RenderOwnership::SetOwner(ERenderCategory::MiniMap, ERenderOwner::Wgpu);
	CHECK(RenderOwnership::Owner(ERenderCategory::MiniMap) == ERenderOwner::Wgpu);
	CHECK_FALSE(RenderOwnership::LegacyDraws(ERenderCategory::MiniMap));

	// Reset do domyslnych
	RenderOwnership::ResetDefaults();
	CHECK(RenderOwnership::LegacyDraws(ERenderCategory::Shadow) == true);
	CHECK(RenderOwnership::LegacyDraws(ERenderCategory::MiniMap) == true);
}
