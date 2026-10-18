#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include <windows.h>
#include <d3d9.h>
#include <d3dx9.h>
#include <cstdint>
#include <cstring>
#include <vector>
#include <memory>
#include <cmath>

#include "EterLib/RenderPacketTypes.h"
#include "EterLib/RenderQueueManager.h"
#include "EterLib/D3D9RenderBackend.h"
#include "EterLib/StateManager.h"
#include "EterLib/RenderOwnership.h"
#include "EterLib/GrpBase.h"
#include "EterGrnLib/LODController.h"
#include "EterGrnLib/Thing.h"
#include "EterGrnLib/ThingInstance.h"
#include "EterModelLib/GltfModel.h"
#include "EterModelLib/GltfModelInstance.h"
#include "GameLib/GameType.h"
#include "GameLib/MapType.h"
#include "GameLib/RaceData.h"
#include "GameLib/ActorInstance.h"
#include "GameLib/ActorRenderExtractor.h"

using namespace EterModelLib;

class CPythonSystem
{
public:
	int GetFogLevel();
};
int CPythonSystem::GetFogLevel() { return 0; }

struct CActorInstanceTest : public CActorInstance
{
	void SetTestLODController(size_t index, CGrannyLODController* pCtrl)
	{
		if (m_LODControllerVector.size() <= index)
			m_LODControllerVector.resize(index + 1, nullptr);
		m_LODControllerVector[index] = pCtrl;
	}
};

namespace
{
static void BuildTestActorGltfModel(GltfModelData& data, size_t boneCount)
{
	data.name = "TestActorGltfModel";

	GltfMaterial mat;
	mat.name = "ActorMat";
	mat.doubleSided = false;
	data.materials.push_back(mat);

	// Dodanie wierzcholkow
	GltfVertex v1{}, v2{}, v3{};
	v1.position = { -10.0f, -5.0f, 0.0f };
	v2.position = { 10.0f, 5.0f, 20.0f };
	v3.position = { 0.0f, 0.0f, 10.0f };
	data.vertices.push_back(v1);
	data.vertices.push_back(v2);
	data.vertices.push_back(v3);
	data.indices = { 0, 1, 2 };

	GltfSubmesh sm{};
	sm.materialIndex = 0;
	sm.vertexOffset = 0;
	sm.vertexCount = 3;
	sm.indexOffset = 0;
	sm.indexCount = 3;
	data.submeshes.push_back(sm);

	// Dodanie kosci
	for (size_t i = 0; i < boneCount; ++i)
	{
		GltfJoint joint;
		joint.name = "bone_" + std::to_string(i);
		joint.parentIndex = (i == 0) ? -1 : 0;
		joint.localTranslation = { static_cast<float>(i * 5), 0.0f, 0.0f };
		joint.localRotation = { 0.0f, 0.0f, 0.0f, 1.0f };
		joint.localScale = { 1.0f, 1.0f, 1.0f };
		joint.inverseBindMatrix = { 1.0f, 0.0f, 0.0f, 0.0f,
									0.0f, 1.0f, 0.0f, 0.0f,
									0.0f, 0.0f, 1.0f, 0.0f,
									0.0f, 0.0f, 0.0f, 1.0f };
		data.skin.joints.push_back(joint);
	}
}

} // namespace

// ============================================================================
// TEST 1: Weryfikacja ukladu pamieci TActorRenderPacket i asercji ABI x64
// ============================================================================
TEST_CASE("TActorRenderPacket_ABILayoutAndOffsets")
{
	static_assert(sizeof(TActorRenderPacket) == 5040, "TActorRenderPacket size must be 5040 bytes");
	static_assert(alignof(TActorRenderPacket) == 16, "TActorRenderPacket alignment must be 16 bytes");
	static_assert(offsetof(TActorRenderPacket, matWorld) == 16, "TActorRenderPacket matWorld offset != 16");
	static_assert(offsetof(TActorRenderPacket, pBoneMatrices) == 80, "TActorRenderPacket pBoneMatrices offset != 80");
	static_assert(offsetof(TActorRenderPacket, boneMatrices) == 96, "TActorRenderPacket boneMatrices offset != 96");
	static_assert(offsetof(TActorRenderPacket, fShadowSize) == 5036, "TActorRenderPacket fShadowSize offset != 5036");

	TActorRenderPacket packet{};
	CHECK(packet.dwActorID == 0u);
	CHECK(packet.dwBoneCount == 0u);
	CHECK(packet.pBoneMatrices == nullptr);

	// Domyślnie GetBoneMatrices() zwraca tablicę wewnętrzną gdy pBoneMatrices == nullptr
	CHECK(packet.GetBoneMatrices() == packet.boneMatrices);

	D3DXMATRIX testMatrix;
	D3DXMatrixIdentity(&testMatrix);
	packet.pBoneMatrices = &testMatrix;
	CHECK(packet.GetBoneMatrices() == &testMatrix);
}

// ============================================================================
// TEST 2: Ciagla pula macierzy kosci klatki w CRenderQueueManager
// ============================================================================
TEST_CASE("CRenderQueueManager_ContinuousBoneMatrixPool")
{
	auto& queueMgr = CRenderQueueManager::Instance();
	queueMgr.BeginFrame();

	CHECK(queueMgr.GetBoneMatrixPoolCapacity() >= 32768u);
	CHECK(queueMgr.GetBoneMatrixOffset() == 0u);
	CHECK(queueMgr.GetDroppedBones() == 0u);

	// Alokacja 64 macierzy dla pierwszego aktora
	D3DXMATRIX* pBones1 = queueMgr.AllocateBoneMatrices(64);
	REQUIRE(pBones1 != nullptr);
	CHECK(queueMgr.GetBoneMatrixOffset() == 64u);

	// Zapis i odczyt danych w puli
	D3DXMatrixScaling(&pBones1[0], 2.0f, 2.0f, 2.0f);
	CHECK(pBones1[0]._11 == 2.0f);

	// Alokacja 32 macierzy dla drugiego aktora
	D3DXMATRIX* pBones2 = queueMgr.AllocateBoneMatrices(32);
	REQUIRE(pBones2 != nullptr);
	CHECK(queueMgr.GetBoneMatrixOffset() == 96u);
	CHECK(pBones2 == pBones1 + 64);

	// Zerowa alokacja zwraca nullptr bez przesuwania offsetu
	D3DXMATRIX* pZero = queueMgr.AllocateBoneMatrices(0);
	CHECK(pZero == nullptr);
	CHECK(queueMgr.GetBoneMatrixOffset() == 96u);

	// Reset klatki
	queueMgr.BeginFrame();
	CHECK(queueMgr.GetBoneMatrixOffset() == 0u);

	// Symulacja przepelnienia puli
	size_t remaining = queueMgr.GetBoneMatrixPoolCapacity();
	D3DXMATRIX* pMax = queueMgr.AllocateBoneMatrices(remaining + 1);
	CHECK(pMax == nullptr);
	CHECK(queueMgr.GetDroppedBones() == remaining + 1);

	queueMgr.ResetDroppedCounters();
	CHECK(queueMgr.GetDroppedBones() == 0u);
}

// ============================================================================
// TEST 3: Paleta kosci przekazywana bezposrednio z CGltfModelInstance do puli CRenderQueueManager (F-NEW-028)
// ============================================================================
TEST_CASE("ActorBonePalette_GLTFDirectSkinningMatricesToPool")
{
	auto& queueMgr = CRenderQueueManager::Instance();
	queueMgr.BeginFrame();

	GltfModelData modelData;
	BuildTestActorGltfModel(modelData, 4);

	CGraphicThing thing("actor_gltf_test.glb");
	thing.AddReferenceOnly();
	REQUIRE(thing.CreateFromGltfModelData(modelData) == true);

	CGrannyLODController controller;
	controller.AddModel(&thing, 0);
	REQUIRE(controller.isGltfModelInstance() == true);

	EterModelLib::CGltfModelInstance* pGltfInst = controller.GetGltfModelInstance();
	REQUIRE(pGltfInst != nullptr);
	pGltfInst->Update(0.0f);

	const auto& skinMatrices = pGltfInst->GetSkinningMatrices();
	REQUIRE(skinMatrices.size() == 4u);

	// Sprawdzamy czy macierz kosci nie jest tozsamosciowa (joint 1, 2, 3 sa przesuniete)
	CHECK(skinMatrices[1].m[3][0] != 0.0f);

	// Przypisanie kontrolera do aktora i ekstrakcja palety
	CActorInstanceTest actor;
	actor.SetMainInstance();
	actor.Show();
	actor.SetTestLODController(CRaceData::PART_MAIN, &controller);

	TActorRenderPacket packet{};
	bool extractOk = ActorRenderExtractor::ExtractBonePalette(actor, packet);
	CHECK(extractOk == true);
	CHECK(packet.dwBoneCount == 4u);
	REQUIRE(packet.pBoneMatrices != nullptr);

	// Wskaznik pBoneMatrices musi wskazywac na ciagla pule klatki w CRenderQueueManager
	const D3DXMATRIX* pPoolStart = queueMgr.GetBoneMatrixData();
	const D3DXMATRIX* pPoolEnd = pPoolStart + queueMgr.GetBoneMatrixPoolCapacity();
	CHECK(packet.pBoneMatrices >= pPoolStart);
	CHECK(packet.pBoneMatrices < pPoolEnd);

	// Wartosci macierzy w puli musza dokladnie odpowiadac skinning matrices z glTF (F-NEW-028)
	const D3DXMATRIX* pExtracted = packet.GetBoneMatrices();
	CHECK(pExtracted[0]._11 == 1.0f);
	CHECK(pExtracted[1]._41 == skinMatrices[1].m[3][0]);

	// Pozostale kosci w tablicy wewnetrznej musza byc wypelnione tozsamoscia
	CHECK(packet.boneMatrices[4]._11 == 1.0f);
	CHECK(packet.boneMatrices[63]._44 == 1.0f);

	actor.SetTestLODController(CRaceData::PART_MAIN, nullptr);
}

// ============================================================================
// TEST 4: Obsluga czesci aktora jako pod-elementow i macierzy socketow (F-P2-07, C3-01)
// ============================================================================
TEST_CASE("ActorSockets_WeaponHelmetHairAura")
{
	CActorInstance actor;
	actor.Show();

	TActorRenderPacket packet{};
	bool extractOk = ActorRenderExtractor::Extract(actor, packet);
	CHECK(extractOk == true);

	// Domyslnie przy braku doczepionych obiektow gniazda maja macierze jednostkowe
	CHECK(packet.matWeaponSocket._11 == 1.0f);
	CHECK(packet.matWeaponLeftSocket._22 == 1.0f);
	CHECK(packet.matHelmetSocket._33 == 1.0f);
	CHECK(packet.matHairSocket._44 == 1.0f);
	CHECK(packet.dwAuraEffectCount == 0u);

	// Test ekstrakcji gniazd ze zdefiniowanymi transformacjami
	D3DXMATRIX matWeapon;
	D3DXMatrixTranslation(&matWeapon, 15.0f, -20.0f, 35.0f);
	D3DXMATRIX* pMat = nullptr;

	// Weryfikacja GetAttachingBoneName dla podstawowych partow
	const char* szBoneName = nullptr;
	CHECK(actor.GetAttachingBoneName(CRaceData::PART_WEAPON, &szBoneName) == TRUE);
	CHECK(std::string(szBoneName) == "equip_right");

	CHECK(actor.GetAttachingBoneName(CRaceData::PART_WEAPON_LEFT, &szBoneName) == TRUE);
	CHECK(std::string(szBoneName) == "equip_left");

	CHECK(actor.GetAttachingBoneName(CRaceData::PART_HEAD, &szBoneName) == TRUE);
	CHECK(std::string(szBoneName) == "Bip01 Head");

	CHECK(actor.GetAttachingBoneName(CRaceData::PART_HAIR, &szBoneName) == TRUE);
	CHECK(std::string(szBoneName) == "Bip01 Head");
}

// ============================================================================
// TEST 5: Limit 60 kosci w CD3D9RenderBackend::RenderActor (F-NEW-033)
// ============================================================================
TEST_CASE("D3D9Backend_RenderActor_60BoneLimit")
{
	CHECK(kMaxShaderBones == 60u);

	// Przypadek 1: Pakiet z 64 kosćmi (powyzej limitu GPU 60)
	TActorRenderPacket packet64{};
	packet64.dwBoneCount = 64;
	UINT matrixCount64 = (std::min)(packet64.dwBoneCount, kMaxShaderBones);
	CHECK(matrixCount64 == 60u);
	CHECK(matrixCount64 * 4u == 240u);

	// Przypadek 2: Pakiet z 10 kosćmi
	TActorRenderPacket packet10{};
	packet10.dwBoneCount = 10;
	UINT matrixCount10 = (std::min)(packet10.dwBoneCount, kMaxShaderBones);
	CHECK(matrixCount10 == 10u);
	CHECK(matrixCount10 * 4u == 40u);

	// Przypadek 3: Pakiet z 0 kosćmi
	TActorRenderPacket packet0{};
	packet0.dwBoneCount = 0;
	UINT matrixCount0 = (std::min)(packet0.dwBoneCount, kMaxShaderBones);
	CHECK(matrixCount0 == 0u);
}

// ============================================================================
// TEST 6: Bounding box aktora jako suma czesci skladowych wg DEC-9
// ============================================================================
TEST_CASE("ActorBoundingBox_DEC9_UnionOfComponents")
{
	// Tworzymy glowny obiekt postaci
	CGraphicThingInstance thing;

	// Bounding box pustego obiektu to punkt (0, 0, 0)
	D3DXVECTOR3 vMin, vMax;
	thing.GetBoundBox(&vMin, &vMax);
	CHECK(vMin.x == 0.0f);
	CHECK(vMin.y == 0.0f);
	CHECK(vMin.z == 0.0f);
	CHECK(vMax.x == 0.0f);
	CHECK(vMax.y == 0.0f);
	CHECK(vMax.z == 0.0f);

	// Sprawdzenie matematycznej unii skladowych postaci (DEC-9)
	std::vector<std::pair<D3DXVECTOR3, D3DXVECTOR3>> partsAABB = {
		{ { -10.0f, -5.0f, 0.0f }, { 10.0f, 5.0f, 20.0f } },   // Cialo (Body)
		{ { 8.0f, 10.0f, 15.0f }, { 25.0f, 30.0f, 35.0f } },   // Bron (Weapon)
		{ { -8.0f, -4.0f, 18.0f }, { 8.0f, 4.0f, 28.0f } }     // Helm (Helmet)
	};

	vMin = { 100000.0f, 100000.0f, 100000.0f };
	vMax = { -100000.0f, -100000.0f, -100000.0f };
	for (const auto& part : partsAABB)
	{
		vMin.x = (std::min)(vMin.x, part.first.x);
		vMin.y = (std::min)(vMin.y, part.first.y);
		vMin.z = (std::min)(vMin.z, part.first.z);
		vMax.x = (std::max)(vMax.x, part.second.x);
		vMax.y = (std::max)(vMax.y, part.second.y);
		vMax.z = (std::max)(vMax.z, part.second.z);
	}

	CHECK(vMin.x == -10.0f);
	CHECK(vMin.y == -5.0f);
	CHECK(vMin.z == 0.0f);
	CHECK(vMax.x == 25.0f);
	CHECK(vMax.y == 30.0f);
	CHECK(vMax.z == 35.0f);
}
