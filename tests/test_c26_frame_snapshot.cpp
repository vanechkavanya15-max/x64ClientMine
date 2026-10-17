#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include <windows.h>
#include <d3d9.h>
#include <d3dx9.h>
#include <cmath>

#include "EterLib/CFrameSnapshot.h"
#include "EterLib/Camera.h"

static bool IsMatrixIdentity(const D3DXMATRIX& mat, float eps = 1e-4f)
{
	for (int r = 0; r < 4; ++r)
	{
		for (int c = 0; c < 4; ++c)
		{
			float expected = (r == c) ? 1.0f : 0.0f;
			if (std::abs(mat.m[r][c] - expected) > eps)
				return false;
		}
	}
	return true;
}

static bool AreMatricesEqual(const D3DXMATRIX& a, const D3DXMATRIX& b, float eps = 1e-4f)
{
	for (int r = 0; r < 4; ++r)
	{
		for (int c = 0; c < 4; ++c)
		{
			if (std::abs(a.m[r][c] - b.m[r][c]) > eps)
				return false;
		}
	}
	return true;
}

TEST_SUITE("CFrameSnapshot")
{
	TEST_CASE("TFrameSnapshot - Default Initialization")
	{
		TFrameSnapshot snapshot;

		CHECK(IsMatrixIdentity(snapshot.matView));
		CHECK(IsMatrixIdentity(snapshot.matProj));
		CHECK(IsMatrixIdentity(snapshot.matViewProj));
		CHECK(IsMatrixIdentity(snapshot.matInvView));

		CHECK(snapshot.v3CameraPos.x == 0.0f);
		CHECK(snapshot.v3CameraPos.y == 0.0f);
		CHECK(snapshot.v3CameraPos.z == 0.0f);

		CHECK(snapshot.v4FogParams.x == 0.0f);
		CHECK(snapshot.v4FogParams.y == 0.0f);
		CHECK(snapshot.v4FogParams.z == 0.0f);
		CHECK(snapshot.v4FogParams.w == 0.0f);

		CHECK(snapshot.fTime == 0.0f);
		CHECK(snapshot.dwFrameIndex == 0);

		for (int i = 0; i < 3; ++i)
		{
			CHECK(snapshot.lights[i].Type == 0);
			CHECK(snapshot.lights[i].Diffuse.r == 0.0f);
			CHECK(snapshot.lights[i].Diffuse.g == 0.0f);
			CHECK(snapshot.lights[i].Diffuse.b == 0.0f);
		}
	}

	TEST_CASE("CFrameSnapshot::Capture - Camera view, inverse, and perspective projection")
	{
		CCamera camera;
		const D3DXVECTOR3 eye(150.0f, -400.0f, 250.0f);
		const D3DXVECTOR3 target(150.0f, 0.0f, 50.0f);
		const D3DXVECTOR3 up(0.0f, 0.0f, 1.0f);
		camera.SetViewParams(eye, target, up);

		const float fFov = 30.0f;
		const float fAspect = 16.0f / 9.0f;
		const float fNear = 100.0f;
		const float fFar = 5000.0f;
		const float fTime = 42.125f;
		const uint32_t dwFrameIndex = 1337;

		CFrameSnapshot::Capture(camera, fFov, fAspect, fNear, fFar, fTime, dwFrameIndex);

		const TFrameSnapshot& snap = CFrameSnapshot::Current();

		CHECK(snap.fTime == doctest::Approx(fTime));
		CHECK(snap.dwFrameIndex == dwFrameIndex);

		// Kamera pozycja oka
		CHECK(snap.v3CameraPos.x == doctest::Approx(eye.x));
		CHECK(snap.v3CameraPos.y == doctest::Approx(eye.y));
		CHECK(snap.v3CameraPos.z == doctest::Approx(eye.z));

		// Macierz widoku zgodna z camera.GetViewMatrix()
		CHECK(AreMatricesEqual(snap.matView, camera.GetViewMatrix()));

		// matView * matInvView == Identity
		D3DXMATRIX matProd;
		D3DXMatrixMultiply(&matProd, &snap.matView, &snap.matInvView);
		CHECK(IsMatrixIdentity(matProd, 1e-3f));

		// Macierz projekcji perspektywicznej
		D3DXMATRIX matExpectedProj;
		D3DXMatrixPerspectiveFovRH(&matExpectedProj, D3DXToRadian(fFov), fAspect, fNear, fFar);
		CHECK(AreMatricesEqual(snap.matProj, matExpectedProj));

		// Zlozona macierz ViewProj == matView * matProj
		D3DXMATRIX matExpectedViewProj;
		D3DXMatrixMultiply(&matExpectedViewProj, &snap.matView, &matExpectedProj);
		CHECK(AreMatricesEqual(snap.matViewProj, matExpectedViewProj));

		// Domyslne parametry mgly wyznaczone z fNear i fFar
		CHECK(snap.v4FogParams.x == doctest::Approx(fNear));
		CHECK(snap.v4FogParams.y == doctest::Approx(fFar));
		CHECK(snap.v4FogParams.z == doctest::Approx(1.0f / (fFar - fNear)));
	}

	TEST_CASE("CFrameSnapshot::Capture - Custom Fog and Lights Injection")
	{
		CCamera camera;
		camera.SetViewParams(D3DXVECTOR3(0.0f, -200.0f, 100.0f), D3DXVECTOR3(0.0f, 0.0f, 0.0f), D3DXVECTOR3(0.0f, 0.0f, 1.0f));

		D3DXVECTOR4 customFog(200.0f, 4000.0f, 1.0f / 3800.0f, 1.0f);

		D3DLIGHT9 customLights[3];
		std::memset(customLights, 0, sizeof(customLights));

		// Light 0: Slonce
		customLights[0].Type = D3DLIGHT_DIRECTIONAL;
		customLights[0].Diffuse = { 1.0f, 0.95f, 0.8f, 1.0f };
		customLights[0].Direction = { 0.577f, -0.577f, -0.577f };

		// Light 1: Wypelnienie
		customLights[1].Type = D3DLIGHT_DIRECTIONAL;
		customLights[1].Diffuse = { 0.3f, 0.35f, 0.4f, 1.0f };
		customLights[1].Direction = { -0.577f, 0.577f, -0.577f };

		// Light 2: Punktowe
		customLights[2].Type = D3DLIGHT_POINT;
		customLights[2].Diffuse = { 1.0f, 0.2f, 0.1f, 1.0f };
		customLights[2].Position = { 100.0f, 200.0f, 50.0f };
		customLights[2].Range = 1000.0f;

		CFrameSnapshot::Capture(camera, 45.0f, 1.333f, 50.0f, 4000.0f, 99.0f, 777, &customFog, customLights);

		const TFrameSnapshot& snap = CFrameSnapshot::Current();

		CHECK(snap.v4FogParams.x == doctest::Approx(200.0f));
		CHECK(snap.v4FogParams.y == doctest::Approx(4000.0f));
		CHECK(snap.v4FogParams.z == doctest::Approx(1.0f / 3800.0f));
		CHECK(snap.v4FogParams.w == doctest::Approx(1.0f));

		CHECK(snap.lights[0].Type == D3DLIGHT_DIRECTIONAL);
		CHECK(snap.lights[0].Diffuse.r == doctest::Approx(1.0f));
		CHECK(snap.lights[0].Diffuse.g == doctest::Approx(0.95f));

		CHECK(snap.lights[1].Type == D3DLIGHT_DIRECTIONAL);
		CHECK(snap.lights[1].Diffuse.b == doctest::Approx(0.4f));

		CHECK(snap.lights[2].Type == D3DLIGHT_POINT);
		CHECK(snap.lights[2].Range == doctest::Approx(1000.0f));
	}

	TEST_CASE("CFrameSnapshot::Capture - Robustness with Null Camera pointer")
	{
		CFrameSnapshot::Capture(static_cast<const CCamera*>(nullptr), 30.0f, 1.777f, 100.0f, 3000.0f, 1.0f, 5);

		const TFrameSnapshot& snap = CFrameSnapshot::Current();

		CHECK(snap.dwFrameIndex == 5);
		CHECK(snap.fTime == doctest::Approx(1.0f));

		// Projekcja jest poprawnie obliczona nawet dla nullptr kamery
		D3DXMATRIX matExpectedProj;
		D3DXMatrixPerspectiveFovRH(&matExpectedProj, D3DXToRadian(30.0f), 1.777f, 100.0f, 3000.0f);
		CHECK(AreMatricesEqual(snap.matProj, matExpectedProj));
	}

	TEST_CASE("CFrameSnapshot - SetCurrent and Reset")
	{
		CFrameSnapshot::Reset();
		CHECK(CFrameSnapshot::Current().dwFrameIndex == 0);
		CHECK(CFrameSnapshot::Current().fTime == 0.0f);
		CHECK(IsMatrixIdentity(CFrameSnapshot::Current().matView));

		TFrameSnapshot custom;
		custom.dwFrameIndex = 99999;
		custom.fTime = 123.456f;
		custom.v3CameraPos = D3DXVECTOR3(1.0f, 2.0f, 3.0f);

		CFrameSnapshot::SetCurrent(custom);
		CHECK(CFrameSnapshot::Current().dwFrameIndex == 99999);
		CHECK(CFrameSnapshot::Current().fTime == doctest::Approx(123.456f));
		CHECK(CFrameSnapshot::Current().v3CameraPos.x == 1.0f);

		CFrameSnapshot::Reset();
		CHECK(CFrameSnapshot::Current().dwFrameIndex == 0);
	}
}
