#include "doctest/doctest.h"
#include "../src/EterLib/RenderQueueManager.h"
#include <thread>
#include <vector>

TEST_CASE("RenderQueueManager")
{
    // testing some invariants
    auto& mgr = CRenderQueueManager::Instance();
    mgr.BeginFrame();
    void* p1 = mgr.AllocateFrameData(10, 32);
    REQUIRE(reinterpret_cast<uintptr_t>(p1) % 32 == 0);
}
