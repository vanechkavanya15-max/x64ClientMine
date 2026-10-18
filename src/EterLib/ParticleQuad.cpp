#include "StdAfx.h"
#include "ParticleQuad.h"
#include "Camera.h"

void BuildParticleQuad(
	uint8_t byBillboardType,
	bool bStretch,
	const D3DXVECTOR3& center,
	float halfWidth,
	float halfHeight,
	float fRotationDeg,
	const D3DXVECTOR3& camUp,
	const D3DXVECTOR3& camCross,
	const D3DXVECTOR3& camView,
	const D3DXVECTOR3& velocity,
	D3DXVECTOR3 outPositions[4],
	const D3DXMATRIX* pMatLocal,
	bool bAttach,
	float fZRotationRad
)
{
	SCameraParameters camera;
	camera.v3Up = camUp;
	camera.v3Cross = camCross;
	camera.v3View = camView;
	CCamera* pCamera = CCameraManager::Instance().GetCurrentCamera();
	if (pCamera)
		camera.v3Eye = pCamera->GetEye();

	const SParticleQuad quad = ComputeParticleBillboardQuad(
		byBillboardType, bStretch, center, halfWidth, halfHeight, fRotationDeg,
		camera, velocity, pMatLocal, bAttach, fZRotationRad
	);

	outPositions[0] = quad.positions[0];
	outPositions[1] = quad.positions[1];
	outPositions[2] = quad.positions[2];
	outPositions[3] = quad.positions[3];
}
