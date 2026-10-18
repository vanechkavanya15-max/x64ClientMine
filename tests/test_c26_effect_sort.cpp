#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include "EffectLib/EffectMesh.h"
#include "EffectLib/EffectData.h"
#include "EffectLib/EffectInstance.h"
#include "EffectLib/EffectManager.h"

#include <vector>
#include <algorithm>
#include <numeric>

#include "EterLib/Camera.h"
static CTimer s_timer;

TEST_CASE("EffectSortTest - Billboard Rotation Constant")
{
	// B5 C.10: stala kLegacyBillboardAllRotationRad zachowuje kompatybilnosc billboardow legacy (90.0 radianow)
	CHECK(kLegacyBillboardAllRotationRad == 90.0f);
}

TEST_CASE("EffectSortTest - CEffectData CRC and Accessors")
{
	CEffectData data;
	CHECK(data.GetCRC32() == 0);

	data.SetCRC32(0x12345678);
	CHECK(data.GetCRC32() == 0x12345678);

	data.Clear();
	CHECK(data.GetCRC32() == 0);
}

TEST_CASE("EffectSortTest - CEffectInstance LifeTime and CRC Accessors")
{
	CEffectInstance inst;
	CHECK(inst.GetLifeTime() == doctest::Approx(0.0f));
	CHECK(inst.GetEffectDataCRC() == 0);

	inst.SetLifeTime(2.75f);
	CHECK(inst.GetLifeTime() == doctest::Approx(2.75f));

	CEffectData data;
	data.SetCRC32(0xAABBCCDD);
	inst.SetEffectDataPointer(&data);

	CHECK(inst.GetEffectDataCRC() == 0xAABBCCDD);
	CHECK(inst.GetLifeTime() == doctest::Approx(0.0f)); // Reinitialized on SetEffectDataPointer
}

TEST_CASE("EffectSortTest - Deterministic Sorting By Effect CRC (Independent of Heap Addresses)")
{
	// S2.14 / T069: sortowanie instancji efektow po CRC zamiast wskaznikow pamieci this / m_pkEftData
	CEffectData dataA;
	dataA.SetCRC32(100);

	CEffectData dataB;
	dataB.SetCRC32(200);

	CEffectData dataC;
	dataC.SetCRC32(300);

	CEffectInstance instA;
	instA.SetEffectDataPointer(&dataA);

	CEffectInstance instB;
	instB.SetEffectDataPointer(&dataB);

	CEffectInstance instC;
	instC.SetEffectDataPointer(&dataC);

	CEffectManager_LessEffectInstancePtrRenderOrder comparator;

	// Weryfikacja bezposrednich relacji
	CHECK(comparator(&instA, &instB) == true);
	CHECK(comparator(&instB, &instA) == false);
	CHECK(comparator(&instB, &instC) == true);
	CHECK(comparator(&instC, &instB) == false);
	CHECK(comparator(&instA, &instC) == true);
	CHECK(comparator(&instC, &instA) == false);

	// Sprawdzenie wszystkich 6 permutacji poczatkowej kolejnosci 3 elementow
	std::vector<std::vector<CEffectInstance*>> permutations = {
		{ &instA, &instB, &instC },
		{ &instA, &instC, &instB },
		{ &instB, &instA, &instC },
		{ &instB, &instC, &instA },
		{ &instC, &instA, &instB },
		{ &instC, &instB, &instA }
	};

	for (auto& vec : permutations)
	{
		std::stable_sort(vec.begin(), vec.end(), comparator);
		CHECK(vec.size() == 3);
		CHECK(vec[0] == &instA);
		CHECK(vec[1] == &instB);
		CHECK(vec[2] == &instC);
		CHECK(vec[0]->GetEffectDataCRC() == 100);
		CHECK(vec[1]->GetEffectDataCRC() == 200);
		CHECK(vec[2]->GetEffectDataCRC() == 300);
	}
}

TEST_CASE("EffectSortTest - Deterministic Sorting By LifeTime (Same Effect CRC)")
{
	// Gdy CRC efektow jest identyczne (wiele instancji tego samego efektu), sortujemy po czasie zycia
	CEffectData sharedData;
	sharedData.SetCRC32(500);

	CEffectInstance instEarly;
	instEarly.SetEffectDataPointer(&sharedData);
	instEarly.SetLifeTime(0.5f);

	CEffectInstance instMid;
	instMid.SetEffectDataPointer(&sharedData);
	instMid.SetLifeTime(1.2f);

	CEffectInstance instLate;
	instLate.SetEffectDataPointer(&sharedData);
	instLate.SetLifeTime(3.8f);

	CEffectManager_LessEffectInstancePtrRenderOrder comparator;

	std::vector<CEffectInstance*> vec = { &instLate, &instEarly, &instMid };
	std::stable_sort(vec.begin(), vec.end(), comparator);

	CHECK(vec[0] == &instEarly);
	CHECK(vec[1] == &instMid);
	CHECK(vec[2] == &instLate);
	CHECK(vec[0]->GetLifeTime() == doctest::Approx(0.5f));
	CHECK(vec[1]->GetLifeTime() == doctest::Approx(1.2f));
	CHECK(vec[2]->GetLifeTime() == doctest::Approx(3.8f));
}

TEST_CASE("EffectSortTest - Heap Allocation Address Independence")
{
	// Tworzymy obiekty dynamicznie i sprawdzamy, czy kolejnosc adresow w pamieci nie wplywa na wynik
	auto* pDataHigh = new CEffectData();
	pDataHigh->SetCRC32(9999);

	auto* pDataLow = new CEffectData();
	pDataLow->SetCRC32(1111);

	auto* pInstHigh = new CEffectInstance();
	pInstHigh->SetEffectDataPointer(pDataHigh);

	auto* pInstLow = new CEffectInstance();
	pInstLow->SetEffectDataPointer(pDataLow);

	CEffectManager_LessEffectInstancePtrRenderOrder comparator;

	// Niezaleznie od tego, czy pInstHigh < pInstLow w sensie adresow wskaznikow:
	std::vector<CEffectInstance*> list1 = { pInstHigh, pInstLow };
	std::stable_sort(list1.begin(), list1.end(), comparator);
	CHECK(list1[0] == pInstLow);
	CHECK(list1[1] == pInstHigh);

	std::vector<CEffectInstance*> list2 = { pInstLow, pInstHigh };
	std::stable_sort(list2.begin(), list2.end(), comparator);
	CHECK(list2[0] == pInstLow);
	CHECK(list2[1] == pInstHigh);

	delete pInstHigh;
	delete pInstLow;
	delete pDataHigh;
	delete pDataLow;
}

TEST_CASE("EffectSortTest - Strict Weak Ordering and Null Safety")
{
	CEffectData data;
	data.SetCRC32(42);

	CEffectInstance inst;
	inst.SetEffectDataPointer(&data);

	CEffectManager_LessEffectInstancePtrRenderOrder comparator;

	// Irreflexivity: comp(x, x) == false
	CHECK(comparator(&inst, &inst) == false);

	// Null safety
	CHECK(comparator(nullptr, &inst) == true);
	CHECK(comparator(&inst, nullptr) == false);
	CHECK(comparator(nullptr, nullptr) == false);
}
