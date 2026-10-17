#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "EterLib/ParticleQuad.h"
#include "EterLib/RenderPacketTypes.h"
#include <cmath>

static constexpr float kEpsilon = 1e-4f;

static bool Vec3ApproxEqual(const D3DXVECTOR3& a, const D3DXVECTOR3& b, float eps = kEpsilon)
{
	return fabsf(a.x - b.x) < eps &&
		   fabsf(a.y - b.y) < eps &&
		   fabsf(a.z - b.z) < eps;
}

TEST_CASE("ParticleQuad - ABI verification")
{
	CHECK(sizeof(TParticlePacket) == 96u);
	CHECK(alignof(TParticlePacket) == 16u);
	CHECK(sizeof(TParticleInstanceData) == 64u);
	CHECK(sizeof(TEffectInstancePacket) == 48u);

	// Weryfikacja unii dwBlendMode w TEffectInstancePacket
	TEffectInstancePacket pkt{};
	pkt.srcBlend = static_cast<uint8_t>(BLEND_FACTOR_SRC_ALPHA);
	pkt.destBlend = static_cast<uint8_t>(BLEND_FACTOR_ONE);
	pkt.billboardMode = static_cast<uint8_t>(PARTICLE_BILLBOARD_Y);
	pkt.bStretch = 1;

	const uint32_t expectedKey = static_cast<uint32_t>(BLEND_FACTOR_SRC_ALPHA) |
		(static_cast<uint32_t>(BLEND_FACTOR_ONE) << 8) |
		(static_cast<uint32_t>(PARTICLE_BILLBOARD_Y) << 16) |
		(static_cast<uint32_t>(1) << 24);

	CHECK(pkt.dwBlendMode == expectedKey);

	// Weryfikacja blend factorow
	CHECK(static_cast<int>(RENDER_BLEND_ZERO) == 1);
	CHECK(static_cast<int>(RENDER_BLEND_ONE) == 2);
	CHECK(static_cast<int>(RENDER_BLEND_SRC_ALPHA) == 5);
	CHECK(static_cast<int>(RENDER_BLEND_INV_SRC_ALPHA) == 6);
}

TEST_CASE("ParticleQuad - Billboard ALL (Facing Camera) without rotation")
{
	// Kamera: Up=(0,0,1), Cross=(1,0,0), View=(0,1,0)
	const D3DXVECTOR3 camUp(0.0f, 0.0f, 1.0f);
	const D3DXVECTOR3 camCross(1.0f, 0.0f, 0.0f);
	const D3DXVECTOR3 camView(0.0f, 1.0f, 0.0f);
	const D3DXVECTOR3 center(100.0f, 200.0f, 50.0f);
	const float halfW = 10.0f;
	const float halfH = 20.0f;

	SParticleQuad quad = BuildParticleQuad(
		PARTICLE_BILLBOARD_ALL,
		false,
		center,
		halfW,
		halfH,
		0.0f,
		camUp,
		camCross,
		camView
	);

	// Legacy math:
	// v3Up = -camCross = (-1, 0, 0)
	// v3Cross = camUp = (0, 0, 1)
	// v3Cross_scaled = -halfW * v3Cross = (0, 0, -10)
	// v3Up_scaled = halfH * v3Up = (-20, 0, 0)
	// quad[0] = center - v3Up_scaled + v3Cross_scaled = (100 - (-20) + 0, 200, 50 + 0 - 10) = (120, 200, 40)
	// quad[1] = center - v3Up_scaled - v3Cross_scaled = (100 - (-20) - 0, 200, 50 - 0 - (-10)) = (120, 200, 60)
	// quad[2] = center + v3Up_scaled + v3Cross_scaled = (100 + (-20) + 0, 200, 50 + 0 - 10) = (80, 200, 40)
	// quad[3] = center + v3Up_scaled - v3Cross_scaled = (100 + (-20) - 0, 200, 50 - 0 - (-10)) = (80, 200, 60)
	CHECK(Vec3ApproxEqual(quad.positions[0], D3DXVECTOR3(120.0f, 200.0f, 40.0f)));
	CHECK(Vec3ApproxEqual(quad.positions[1], D3DXVECTOR3(120.0f, 200.0f, 60.0f)));
	CHECK(Vec3ApproxEqual(quad.positions[2], D3DXVECTOR3(80.0f, 200.0f, 40.0f)));
	CHECK(Vec3ApproxEqual(quad.positions[3], D3DXVECTOR3(80.0f, 200.0f, 60.0f)));

	// Weryfikacja symetrii: srodek ciezkosci wierzcholkow to dokladnie pozycja poczatkowa
	D3DXVECTOR3 computedCenter = (quad.positions[0] + quad.positions[1] + quad.positions[2] + quad.positions[3]) * 0.25f;
	CHECK(Vec3ApproxEqual(computedCenter, center));
}

TEST_CASE("ParticleQuad - Billboard ALL with 90 degree rotation")
{
	const D3DXVECTOR3 camUp(0.0f, 0.0f, 1.0f);
	const D3DXVECTOR3 camCross(1.0f, 0.0f, 0.0f);
	const D3DXVECTOR3 camView(0.0f, 1.0f, 0.0f);
	const D3DXVECTOR3 center(0.0f, 0.0f, 0.0f);
	const float halfW = 10.0f;
	const float halfH = 10.0f;

	SParticleQuad quad = BuildParticleQuad(
		PARTICLE_BILLBOARD_ALL,
		false,
		center,
		halfW,
		halfH,
		90.0f, // 90 stopni rotacji wokol camView (Y)
		camUp,
		camCross,
		camView
	);

	// Obracamy negCross=(-1, 0, 0) o 90 stopni wokol (0, 1, 0): staje sie (0, 0, 1)
	// Obracamy camUp=(0, 0, 1) o 90 stopni wokol (0, 1, 0): staje sie (1, 0, 0)
	// v3Cross_scaled = -halfW * (1, 0, 0) = (-10, 0, 0)
	// v3Up_scaled = halfH * (0, 0, 1) = (0, 0, 10)
	// quad[0] = center - v3Up + v3Cross = (-10, 0, -10)
	// quad[1] = center - v3Up - v3Cross = (10, 0, -10)
	// quad[2] = center + v3Up + v3Cross = (-10, 0, 10)
	// quad[3] = center + v3Up - v3Cross = (10, 0, 10)
	CHECK(Vec3ApproxEqual(quad.positions[0], D3DXVECTOR3(-10.0f, 0.0f, -10.0f)));
	CHECK(Vec3ApproxEqual(quad.positions[1], D3DXVECTOR3(10.0f, 0.0f, -10.0f)));
	CHECK(Vec3ApproxEqual(quad.positions[2], D3DXVECTOR3(-10.0f, 0.0f, 10.0f)));
	CHECK(Vec3ApproxEqual(quad.positions[3], D3DXVECTOR3(10.0f, 0.0f, 10.0f)));
}

TEST_CASE("ParticleQuad - Billboard LIE (Ground XY Plane)")
{
	const D3DXVECTOR3 camUp(0.0f, 0.0f, 1.0f);
	const D3DXVECTOR3 camCross(1.0f, 0.0f, 0.0f);
	const D3DXVECTOR3 camView(0.0f, 1.0f, 0.0f);
	const D3DXVECTOR3 center(50.0f, 50.0f, 0.0f);
	const float halfW = 5.0f;
	const float halfH = 5.0f;

	SParticleQuad quad = BuildParticleQuad(
		PARTICLE_BILLBOARD_LIE,
		false,
		center,
		halfW,
		halfH,
		0.0f, // rotacja 0: v3Up=(1, 0, 0), v3Cross=(0, 1, 0)
		camUp,
		camCross,
		camView
	);

	// v3Cross = -5 * (0, 1, 0) = (0, -5, 0)
	// v3Up = 5 * (1, 0, 0) = (5, 0, 0)
	// Wszystkie wierzcholki leza w plaszczyznie Z = 0
	CHECK(quad.positions[0].z == doctest::Approx(0.0f));
	CHECK(quad.positions[1].z == doctest::Approx(0.0f));
	CHECK(quad.positions[2].z == doctest::Approx(0.0f));
	CHECK(quad.positions[3].z == doctest::Approx(0.0f));

	CHECK(Vec3ApproxEqual(quad.positions[0], D3DXVECTOR3(45.0f, 45.0f, 0.0f)));
	CHECK(Vec3ApproxEqual(quad.positions[1], D3DXVECTOR3(45.0f, 55.0f, 0.0f)));
	CHECK(Vec3ApproxEqual(quad.positions[2], D3DXVECTOR3(55.0f, 45.0f, 0.0f)));
	CHECK(Vec3ApproxEqual(quad.positions[3], D3DXVECTOR3(55.0f, 55.0f, 0.0f)));
}

TEST_CASE("ParticleQuad - Billboard Y (Cylindrical Billboard)")
{
	const D3DXVECTOR3 camUp(0.0f, 0.0f, 1.0f);
	const D3DXVECTOR3 camCross(1.0f, 0.0f, 0.0f);
	const D3DXVECTOR3 camView(0.0f, 1.0f, 0.0f);
	const D3DXVECTOR3 center(0.0f, 0.0f, 10.0f);
	const float halfW = 4.0f;
	const float halfH = 8.0f;

	SParticleQuad quad = BuildParticleQuad(
		PARTICLE_BILLBOARD_Y,
		false,
		center,
		halfW,
		halfH,
		0.0f,
		camUp,
		camCross,
		camView
	);

	// v3Up = (0, 0, 1)
	// d3dd = (0, 1, 0)
	// v3Cross = Up x d3dd = (0, 0, 1) x (0, 1, 0) = (-1, 0, 0)
	// v3Cross_scaled = -4 * (-1, 0, 0) = (4, 0, 0)
	// v3Up_scaled = 8 * (0, 0, 1) = (0, 0, 8)
	// quad[0] = (0, 0, 10) - (0, 0, 8) + (4, 0, 0) = (4, 0, 2)
	// quad[1] = (0, 0, 10) - (0, 0, 8) - (4, 0, 0) = (-4, 0, 2)
	// quad[2] = (0, 0, 10) + (0, 0, 8) + (4, 0, 0) = (4, 0, 18)
	// quad[3] = (0, 0, 10) + (0, 0, 8) - (4, 0, 0) = (-4, 0, 18)
	CHECK(Vec3ApproxEqual(quad.positions[0], D3DXVECTOR3(4.0f, 0.0f, 2.0f)));
	CHECK(Vec3ApproxEqual(quad.positions[1], D3DXVECTOR3(-4.0f, 0.0f, 2.0f)));
	CHECK(Vec3ApproxEqual(quad.positions[2], D3DXVECTOR3(4.0f, 0.0f, 18.0f)));
	CHECK(Vec3ApproxEqual(quad.positions[3], D3DXVECTOR3(-4.0f, 0.0f, 18.0f)));
}

TEST_CASE("ParticleQuad - Stretch Mode (Velocity-driven Expansion)")
{
	const D3DXVECTOR3 camUp(0.0f, 0.0f, 1.0f);
	const D3DXVECTOR3 camCross(1.0f, 0.0f, 0.0f);
	const D3DXVECTOR3 camView(0.0f, 1.0f, 0.0f);
	const D3DXVECTOR3 center(0.0f, 0.0f, 0.0f);
	const float halfW = 2.0f;
	const float halfH = 1.0f;

	// Predkosc wzdluz osi Z o dlugosci 10.0f
	const D3DXVECTOR3 velocity(0.0f, 0.0f, 10.0f);

	SParticleQuad quad = BuildParticleQuad(
		PARTICLE_BILLBOARD_ALL,
		true, // bStretch = true
		center,
		halfW,
		halfH,
		0.0f,
		camUp,
		camCross,
		camView,
		velocity
	);

	// W trybie stretch dlugosc skaluje sie o (1 + ln(1 + 10)) / 10
	// Up kieruje sie wzdluz osi Z
	// Sprawdzamy, ze wierzcholki sa rozciagniete wzdluz Z bardziej niz standardowy halfH
	const float spanZ = fabsf(quad.positions[2].z - quad.positions[0].z);
	const float expectedFactor = 1.0f + logf(1.0f + 10.0f);
	const float expectedSpanZ = 2.0f * halfH * expectedFactor;

	CHECK(spanZ == doctest::Approx(expectedSpanZ).epsilon(1e-3f));
}

TEST_CASE("ParticleQuad - Local Matrix Attachment")
{
	const D3DXVECTOR3 camUp(0.0f, 0.0f, 1.0f);
	const D3DXVECTOR3 camCross(1.0f, 0.0f, 0.0f);
	const D3DXVECTOR3 camView(0.0f, 1.0f, 0.0f);
	const D3DXVECTOR3 center(0.0f, 0.0f, 0.0f);

	D3DXMATRIX matTranslation;
	D3DXMatrixTranslation(&matTranslation, 50.0f, 100.0f, -25.0f);

	SParticleQuad quad = BuildParticleQuad(
		PARTICLE_BILLBOARD_ALL,
		false,
		center,
		5.0f,
		5.0f,
		0.0f,
		camUp,
		camCross,
		camView,
		D3DXVECTOR3(0.0f, 0.0f, 0.0f),
		&matTranslation,
		true // bAttach = true
	);

	// Pozycja powinna byc przetransformowana do (50, 100, -25)
	D3DXVECTOR3 computedCenter = (quad.positions[0] + quad.positions[1] + quad.positions[2] + quad.positions[3]) * 0.25f;
	CHECK(Vec3ApproxEqual(computedCenter, D3DXVECTOR3(50.0f, 100.0f, -25.0f)));
}
