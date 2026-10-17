#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include <type_traits>
#include <windows.h>
#include <d3d9.h>
#include <d3dx9.h>

#include "EterGrnLib/LODController.h"
#include "EterLib/GrpTextInstance.h"
#include "EterLib/Render/DynamicQuadBatcher.h"
#include "EterLib/RenderQueueManager.h"

using namespace EterLib::Render;

TEST_SUITE("S8.1_NamingConventions")
{
    TEST_CASE("CModelLODController - TypeAliasAndPolymorphicCompatibility")
    {
        // 1. Sprawdzenie zgodnosci typow na poziomie kompilacji
        static_assert(std::is_same_v<CModelLODController, CGrannyLODController>,
                      "CModelLODController musi byc dokladnym aliasem dla CGrannyLODController");

        // 2. Sprawdzenie instancjonowania oraz zgodnosci wskaznikow
        CModelLODController controller;
        controller.SetModelID(42001);
        CHECK(controller.GetModelID() == 42001);

        CGrannyLODController* pGranny = &controller;
        CHECK(pGranny->GetModelID() == 42001);

        CModelLODController* pModel = pGranny;
        CHECK(pModel->GetModelID() == 42001);

        // 3. Sprawdzenie stanu poczatkowego LOD i metod
        CHECK(controller.GetLODLevel() == 0);
        CHECK(controller.GetAttachedParentModel() == nullptr);
        CHECK(controller.isModelInstance() == FALSE);
        CHECK(controller.isGltfModelInstance() == false);
    }

    TEST_CASE("GrpTextInstance - SetHorizontalAlignAndSetHorizonalAlignWrapper")
    {
        CGraphicTextInstance textInst;

        // Domyslny align po __Initialize() to HORIZONTAL_ALIGN_LEFT
        CHECK(textInst.GetHorizontalAlign() == CGraphicTextInstance::HORIZONTAL_ALIGN_LEFT);

        // Test nowej metody SetHorizontalAlign
        textInst.SetHorizontalAlign(CGraphicTextInstance::HORIZONTAL_ALIGN_CENTER);
        CHECK(textInst.GetHorizontalAlign() == CGraphicTextInstance::HORIZONTAL_ALIGN_CENTER);

        textInst.SetHorizontalAlign(CGraphicTextInstance::HORIZONTAL_ALIGN_RIGHT);
        CHECK(textInst.GetHorizontalAlign() == CGraphicTextInstance::HORIZONTAL_ALIGN_RIGHT);

        // Test istniejacej metody SetHorizonalAlign jako inline wrapper
        textInst.SetHorizonalAlign(CGraphicTextInstance::HORIZONTAL_ALIGN_CENTER);
        CHECK(textInst.GetHorizontalAlign() == CGraphicTextInstance::HORIZONTAL_ALIGN_CENTER);

        textInst.SetHorizonalAlign(CGraphicTextInstance::HORIZONTAL_ALIGN_LEFT);
        CHECK(textInst.GetHorizontalAlign() == CGraphicTextInstance::HORIZONTAL_ALIGN_LEFT);
    }

    TEST_CASE("DynamicQuadBatcher - SubmitToQueueAlias")
    {
        DynamicQuadBatcher batcher(1024, 32);
        CRenderQueueManager& queue = CRenderQueueManager::Instance();

        const size_t initialPackets = queue.GetUIPackets().size();

        // Dodanie geometrii (AddBar tworzy 1 quad = 4 wierzcholki)
        batcher.AddBar(10.0f, 20.0f, 100.0f, 50.0f, 0xFF55AAFF);
        CHECK(batcher.GetVertexCount() == 4u);
        CHECK(batcher.GetBatchCount() == 1u);

        // SubmitToQueue() wywoluje Flush() -> FlushToQueue(CRenderQueueManager::Instance())
        batcher.SubmitToQueue();

        // Po wywolaniu SubmitToQueue: wierzcholki i batche wyczyszczone, pakiet przekazany do kolejki
        CHECK(batcher.GetVertexCount() == 0u);
        CHECK(batcher.GetBatchCount() == 0u);
        CHECK(queue.GetUIPackets().size() == initialPackets + 1u);
    }
}
