#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "SpeedTreeForest.h"
#include "EterLib/Camera.h"
#include <cmath>

float CCamera::CAMERA_MAX_DISTANCE = 2500.0f;

class CTestSpeedTreeForest : public CSpeedTreeForest
{
public:
	mutable int m_uploadCount = 0;
	mutable int m_renderCount = 0;

	void UploadWindMatrix(unsigned int uiLocation, const float* pMatrix) const override
	{
		m_uploadCount++;
	}

	void Render(unsigned long ulRenderBitVector) override
	{
		m_renderCount++;
	}
};

TEST_CASE("CSpeedTreeForest - Member Variables Initialization")
{
	CTestSpeedTreeForest forest;

	CHECK(forest.GetWindStrength() == doctest::Approx(0.0f));
	CHECK(forest.GetAccumTime() == doctest::Approx(0.0f));
	CHECK(forest.GetLastUpdateTime() == doctest::Approx(0.0f));
	CHECK(forest.IsWindMatricesCached() == false);

	for (int i = 0; i < 4; ++i)
	{
		CHECK(forest.GetMatrixTime(i) == doctest::Approx(0.0f));
	}
}

TEST_CASE("CSpeedTreeForest - Single UpdateSystem per Frame")
{
	CTestSpeedTreeForest forest;

	// Pierwsze wywolanie ustala baze czasowa (dt = 0) bez znieksztalcen i skokow
	forest.UpdateSystem(100.0f);
	CHECK(forest.GetLastUpdateTime() == doctest::Approx(100.0f));
	CHECK(forest.GetAccumTime() == doctest::Approx(0.0f));
	CHECK(forest.IsWindMatricesCached() == true);

	// Przy braku wiatru macierze wiatru sa macierzami jednostkowymi
	const float* pMatrix0 = forest.GetCachedWindMatrix(0);
	REQUIRE(pMatrix0 != nullptr);
	CHECK(pMatrix0[0] == doctest::Approx(1.0f));
	CHECK(pMatrix0[5] == doctest::Approx(1.0f));
	CHECK(pMatrix0[10] == doctest::Approx(1.0f));
	CHECK(pMatrix0[15] == doctest::Approx(1.0f));
	CHECK(pMatrix0[1] == doctest::Approx(0.0f));
	CHECK(pMatrix0[4] == doctest::Approx(0.0f));

	// Kolejna klatka: czas plynie o 16.6 ms (odpowiednik ~60 FPS)
	forest.UpdateSystem(100.0166f);
	CHECK(forest.GetLastUpdateTime() == doctest::Approx(100.0166f));
	CHECK(forest.GetAccumTime() == doctest::Approx(0.0166f));
}

TEST_CASE("CSpeedTreeForest - Redundant Duplicate Calls in Same Frame Ignored")
{
	CTestSpeedTreeForest forest;

	forest.UpdateSystem(50.0f);
	forest.UpdateSystem(50.0333f);

	const float accumAfterFirst = forest.GetAccumTime();
	CHECK(accumAfterFirst == doctest::Approx(0.0333f));

	// Symulacja wielokrotnych wywolan w tej samej klatce (np. MapOutdoorUpdate + wielokrotne OnRender)
	for (int i = 0; i < 20; ++i)
	{
		forest.UpdateSystem(50.0333f);
	}

	// Czas akumulowany nie moze ulec podwojeniu ani znieksztalceniu
	CHECK(forest.GetAccumTime() == doctest::Approx(accumAfterFirst));
	CHECK(forest.GetLastUpdateTime() == doctest::Approx(50.0333f));
}

TEST_CASE("CSpeedTreeForest - Wind Strength and Matrix Stability Under Duplicate Calls")
{
	CTestSpeedTreeForest forest;
	forest.SetWindStrength(0.8f);
	CHECK(forest.GetWindStrength() == doctest::Approx(0.8f));

	float currentTime = 10.0f;
	forest.UpdateSystem(currentTime);

	// Symulacja 30 klatek, w kazdej klatce 5 zbednych duplikatow z identycznym czasem
	for (int frame = 0; frame < 30; ++frame)
	{
		currentTime += 0.016f;

		// Glowne wywolanie z MapOutdoorUpdate
		forest.UpdateSystem(currentTime);
		const float accumExpected = forest.GetAccumTime();

		// Duplikaty z tym samym czasem
		for (int dup = 0; dup < 5; ++dup)
		{
			forest.UpdateSystem(currentTime);
		}

		CHECK(forest.GetAccumTime() == doctest::Approx(accumExpected));

		// Weryfikacja ze wszystkie macierze wiatru sa poprawnymi liczbami skonczonymi
		for (int m = 0; m < 4; ++m)
		{
			const float* pMat = forest.GetCachedWindMatrix(m);
			REQUIRE(pMat != nullptr);
			for (int elem = 0; elem < 16; ++elem)
			{
				CHECK(!std::isnan(pMat[elem]));
				CHECK(!std::isinf(pMat[elem]));
			}
			CHECK(pMat[15] == doctest::Approx(1.0f));
		}
	}
}

TEST_CASE("CSpeedTreeForest - Backwards Time Jump (Clock Reset / Warp Safety)")
{
	CTestSpeedTreeForest forest;

	forest.UpdateSystem(200.0f);
	forest.UpdateSystem(200.05f);

	const float accBeforeJump = forest.GetAccumTime();
	CHECK(accBeforeJump == doctest::Approx(0.05f));

	// Zegar cofnal sie (np. reset timera systemowego)
	forest.UpdateSystem(150.0f);

	// Nie powinno nastapic odjecie czasu anomalia ujemna
	CHECK(forest.GetAccumTime() == doctest::Approx(accBeforeJump));

	// Nastepny prawidlowy krok
	forest.UpdateSystem(150.02f);
	CHECK(forest.GetAccumTime() == doctest::Approx(accBeforeJump + 0.02f));
}

TEST_CASE("CSpeedTreeForest - Instance Isolation (No Shared Static State)")
{
	CTestSpeedTreeForest forest1;
	CTestSpeedTreeForest forest2;

	// Inicjalizacja obu instancji roznymi czasami
	forest1.UpdateSystem(10.0f);
	forest2.UpdateSystem(500.0f);

	// Przejscie czasu w forest1
	forest1.UpdateSystem(10.5f);
	CHECK(forest1.GetAccumTime() == doctest::Approx(0.5f));
	CHECK(forest1.GetLastUpdateTime() == doctest::Approx(10.5f));

	// forest2 nie moze byc dotkniete przez zmiany w forest1
	CHECK(forest2.GetAccumTime() == doctest::Approx(0.0f));
	CHECK(forest2.GetLastUpdateTime() == doctest::Approx(500.0f));

	// Przejscie czasu w forest2
	forest2.UpdateSystem(500.25f);
	CHECK(forest2.GetAccumTime() == doctest::Approx(0.25f));
	CHECK(forest2.GetLastUpdateTime() == doctest::Approx(500.25f));

	// forest1 pozostaje nienaruszone
	CHECK(forest1.GetAccumTime() == doctest::Approx(0.5f));
	CHECK(forest1.GetLastUpdateTime() == doctest::Approx(10.5f));
}
