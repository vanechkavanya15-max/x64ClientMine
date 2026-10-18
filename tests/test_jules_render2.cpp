#include "doctest/doctest.h"
#include "../src/EterLib/RenderQueueManager.h"

TEST_CASE("RenderQueueManager")
{
    auto& mgr = CRenderQueueManager::Instance();
    mgr.BeginFrame();
    
    mgr.EndFrame();
}
