#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "GameLib/SnowParticle.h"
#include <cmath>

TEST_CASE("CSnowParticle - Basic Initialization and Setters")
{
	CSnowParticle particle;
	const D3DXVECTOR3 pos(100.0f, 200.0f, 300.0f);
	const D3DXVECTOR3 vel(0.0f, 0.0f, -50.0f);

	particle.SetPosition(pos);
	particle.SetVelocity(vel);
	particle.SetPeriod(2.5f);
	particle.SetCurRadian(0.5f);
	particle.SetHalfWidth(5.0f);
	particle.SetHalfHeight(5.0f);
	particle.SetActivate(true);

	CHECK(particle.GetPosition().x == doctest::Approx(100.0f));
	CHECK(particle.GetPosition().y == doctest::Approx(200.0f));
	CHECK(particle.GetPosition().z == doctest::Approx(300.0f));
	CHECK(particle.GetVelocity().z == doctest::Approx(-50.0f));
	CHECK(particle.GetPeriod() == doctest::Approx(2.5f));
	CHECK(particle.GetCurRadian() == doctest::Approx(0.5f));
	CHECK(particle.GetHalfWidth() == doctest::Approx(5.0f));
	CHECK(particle.GetHalfHeight() == doctest::Approx(5.0f));
	CHECK(particle.IsActivate() == true);
}

TEST_CASE("CSnowParticle - Frame-rate Independent Sway (30 FPS vs 60 FPS vs 144 FPS)")
{
	// NB.4 / W4-05: przemieszczenie w ciagu 1.0 s przy roznych krokach dt (30, 60, 144 FPS)
	// z formula * 6.0f * fElapsedTime daje zbieznosc calkowania numerycznego (roznica < 2.5% miedzy 30 a 144 FPS, < 1% miedzy 60 a 144 FPS).
	// W starym kodzie roznica wynosila 480% (144 / 30 = 4.8x).
	auto simulate_sway = [](int fps, float duration) -> D3DXVECTOR3 {
		CSnowParticle p;
		p.SetPosition(D3DXVECTOR3(0.0f, 0.0f, 2000.0f));
		p.SetVelocity(D3DXVECTOR3(0.0f, 0.0f, -100.0f));
		p.SetHalfWidth(5.0f);
		p.SetHalfHeight(5.0f);
		p.SetCameraVertex(D3DXVECTOR3(0.0f, 1.0f, 0.0f), D3DXVECTOR3(1.0f, 0.0f, 0.0f)); // up=(0,5,0), cross=(5,0,0)
		p.SetPeriod(2.0f);
		p.SetAmplitude(2.0f);
		p.SetCurRadian(0.0f);
		p.SetActivate(true);

		const float dt = 1.0f / static_cast<float>(fps);
		const int steps = static_cast<int>(std::round(duration * static_cast<float>(fps)));
		const D3DXVECTOR3 center(0.0f, 0.0f, 0.0f);

		for (int i = 0; i < steps; ++i)
		{
			p.Update(dt, center);
		}
		return p.GetPosition();
	};

	const D3DXVECTOR3 pos30 = simulate_sway(30, 1.0f);
	const D3DXVECTOR3 pos60 = simulate_sway(60, 1.0f);
	const D3DXVECTOR3 pos144 = simulate_sway(144, 1.0f);

	// Ruch pionowy jest liniowy i scisle niezalezny od FPS: z = 2000 - 100 * 1.0 = 1900
	CHECK(pos30.z == doctest::Approx(1900.0f).epsilon(0.001f));
	CHECK(pos60.z == doctest::Approx(1900.0f).epsilon(0.001f));
	CHECK(pos144.z == doctest::Approx(1900.0f).epsilon(0.001f));

	// Kolysanie w osi X
	CHECK(std::abs(pos144.x) > 1.0f);
	const float rel_diff_60_144 = std::abs(pos144.x - pos60.x) / std::abs(pos144.x);
	const float rel_diff_30_144 = std::abs(pos144.x - pos30.x) / std::abs(pos144.x);

	// Roznica pomiedzy 60 FPS a 144 FPS < 1%
	CHECK(rel_diff_60_144 < 0.01f);

	// Roznica pomiedzy 30 FPS a 144 FPS < 2.5%
	CHECK(rel_diff_30_144 < 0.025f);
}

TEST_CASE("CSnowParticle - Deterministic Sway Under Identical Step Sequence")
{
	CSnowParticle p1;
	CSnowParticle p2;

	const D3DXVECTOR3 startPos(50.0f, -50.0f, 1500.0f);
	const D3DXVECTOR3 vel(0.0f, 0.0f, -120.0f);
	const D3DXVECTOR3 camUp(0.0f, 1.0f, 0.0f);
	const D3DXVECTOR3 camCross(1.0f, 0.0f, 0.0f);

	p1.SetPosition(startPos);
	p1.SetVelocity(vel);
	p1.SetHalfWidth(4.0f);
	p1.SetHalfHeight(4.0f);
	p1.SetCameraVertex(camUp, camCross);
	p1.SetPeriod(3.0f);
	p1.SetCurRadian(0.7f);
	p1.SetActivate(true);

	p2.SetPosition(startPos);
	p2.SetVelocity(vel);
	p2.SetHalfWidth(4.0f);
	p2.SetHalfHeight(4.0f);
	p2.SetCameraVertex(camUp, camCross);
	p2.SetPeriod(3.0f);
	p2.SetCurRadian(0.7f);
	p2.SetActivate(true);

	const float dtSteps[] = { 0.016f, 0.033f, 0.008f, 0.020f, 0.016f };
	const D3DXVECTOR3 center(0.0f, 0.0f, 0.0f);

	for (float dt : dtSteps)
	{
		p1.Update(dt, center);
		p2.Update(dt, center);

		CHECK(p1.GetPosition().x == doctest::Approx(p2.GetPosition().x));
		CHECK(p1.GetPosition().y == doctest::Approx(p2.GetPosition().y));
		CHECK(p1.GetPosition().z == doctest::Approx(p2.GetPosition().z));
		CHECK(p1.GetCurRadian() == doctest::Approx(p2.GetCurRadian()));
		CHECK(p1.GetRotation() == doctest::Approx(p2.GetRotation()));
	}
}

TEST_CASE("CSnowParticle - Smooth CurRadian Phase Advance")
{
	CSnowParticle p;
	p.SetPeriod(2.0f);
	p.SetCurRadian(0.0f);
	p.SetActivate(true);

	const D3DXVECTOR3 center(0.0f, 0.0f, 0.0f);
	p.Update(0.25f, center);
	CHECK(p.GetCurRadian() == doctest::Approx(0.5f));

	p.Update(0.25f, center);
	CHECK(p.GetCurRadian() == doctest::Approx(1.0f));
}

TEST_CASE("CSnowParticle - Deactivation and Boundaries")
{
	CSnowParticle p;
	p.SetPosition(D3DXVECTOR3(0.0f, 0.0f, 100.0f));
	p.SetVelocity(D3DXVECTOR3(0.0f, 0.0f, -650.0f));
	p.SetActivate(true);

	const D3DXVECTOR3 center(0.0f, 0.0f, 0.0f);
	// 1 second step: pos.z drops from 100 to -550, strictly below center.z - 500.0f (-500.0f)
	p.Update(1.0f, center);
	CHECK(p.IsActivate() == false);
}

TEST_CASE("CSnowParticle - Pool Allocation and Reuse")
{
	CSnowParticle::InitializePool(10);
	CSnowParticle* p1 = CSnowParticle::New();
	REQUIRE(p1 != nullptr);

	CSnowParticle::Delete(p1);
	CSnowParticle* p2 = CSnowParticle::New();
	CHECK(p2 == p1); // reused from pool
	CSnowParticle::Delete(p2);

	CSnowParticle::DestroyPool();
}
