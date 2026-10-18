#include "doctest/doctest.h"
#include "../src/EterLib/RenderQueueManager.h"
#include <thread>
#include <vector>

TEST_CASE("RenderQueueManager - AllocationAlignment")
{
    auto& mgr = CRenderQueueManager::Instance();
    mgr.BeginFrame();
    void* p1 = mgr.AllocateFrameData(1, 16);
    void* p2 = mgr.AllocateFrameData(1, 16);
    REQUIRE(reinterpret_cast<uintptr_t>(p1) % 16 == 0);
    REQUIRE(reinterpret_cast<uintptr_t>(p2) % 16 == 0);
}
