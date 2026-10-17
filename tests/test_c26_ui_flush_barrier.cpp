#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "EterLib/Render/DynamicQuadBatcher.h"
#include "EterLib/RenderQueueManager.h"
#include "EterLib/Camera.h"

float CCamera::CAMERA_MAX_DISTANCE = 2500.0f;

using namespace EterLib::Render;

TEST_CASE("UIFlushBarrier - FlushPendingUIBehavior")
{
    DynamicQuadBatcher& batcher = DynamicQuadBatcher::Instance();
    CRenderQueueManager& queue = CRenderQueueManager::Instance();

    batcher.BeginFrame();
    queue.BeginFrame();

    // 1. Gdy nie ma wierzcholkow, FlushPendingUI nie tworzy pakietow
    batcher.FlushPendingUI(queue);
    CHECK(batcher.GetVertexCount() == 0u);
    CHECK(queue.GetUIQueueCount() == 0u);

    // 2. Dodajemy quady
    batcher.AddQuad(0.0f, 0.0f, 32.0f, 32.0f, 0xFFFFFFFF, 1);
    batcher.AddQuad(32.0f, 0.0f, 32.0f, 32.0f, 0xFFFFFFFF, 1);
    CHECK(batcher.GetVertexCount() == 8u);
    CHECK(batcher.GetBatchCount() == 1u);

    // 3. Wywolanie FlushPendingUI przesuwa wierzcholki do kolejki i wywoluje FlushUI
    batcher.FlushPendingUI(queue);
    CHECK(batcher.GetBatchCount() == 0u);
    CHECK(batcher.GetVertexCount() == 0u);
}

TEST_CASE("UIFlushBarrier - FlushFrameUI_ClearsBatcher")
{
    DynamicQuadBatcher& batcher = DynamicQuadBatcher::Instance();
    CRenderQueueManager& queue = CRenderQueueManager::Instance();

    batcher.BeginFrame();
    queue.BeginFrame();

    // Dodanie UI quadow
    batcher.AddQuad(10.0f, 10.0f, 50.0f, 50.0f, 0xFF00FF00, 2);
    CHECK(batcher.GetVertexCount() == 4u);

    // FlushToQueue
    batcher.FlushToQueue(queue);
    queue.FlushUI();

    CHECK(batcher.GetBatchCount() == 0u);
    CHECK(batcher.GetVertexCount() == 0u);
}
