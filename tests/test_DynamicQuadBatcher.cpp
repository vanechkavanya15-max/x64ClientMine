#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "EterLib/Render/DynamicQuadBatcher.h"

using namespace EterLib::Render;

TEST_CASE("DynamicQuadBatcher - Initialization & Reservation")
{
    DynamicQuadBatcher batcher(100, 10);
    CHECK(batcher.GetVertexCapacity() >= 100);
    CHECK(batcher.GetBatchCapacity() >= 10);

    SUBCASE("Reserve mid-frame shouldn't resize")
    {
        batcher.BeginFrame();
        batcher.AddQuad(0.0f, 0.0f, 10.0f, 10.0f, 0xFFFFFFFF);
        size_t capBefore = batcher.GetVertexCapacity();
        batcher.Reserve(capBefore + 1000);
        CHECK(batcher.GetVertexCapacity() == capBefore);
    }
}

TEST_CASE("DynamicQuadBatcher - Batch Generation & Scissor Rects")
{
    DynamicQuadBatcher batcher;
    batcher.BeginFrame();

    SUBCASE("Adding identical geometry does not spawn new batches")
    {
        batcher.AddQuad(0, 0, 10, 10, 0xFFFFFFFF, 1);
        batcher.AddQuad(10, 10, 10, 10, 0xFFFFFFFF, 1);
        CHECK(batcher.GetBatchCount() == 1);
        CHECK(batcher.GetVertexCount() == 8);
    }

    SUBCASE("Changing state spawns new batch lazily")
    {
        batcher.AddQuad(0, 0, 10, 10, 0xFFFFFFFF, 1);
        
        RECT r = {0, 0, 100, 100};
        batcher.SetScissorRect(r, true);
        CHECK(batcher.GetBatchCount() == 1); // Not spawned immediately

        batcher.AddQuad(10, 10, 10, 10, 0xFFFFFFFF, 1);
        CHECK(batcher.GetBatchCount() == 2); // Spawned lazily
    }
    
    SUBCASE("Empty batch is reused")
    {
        batcher.AddQuad(0, 0, 10, 10, 0xFFFFFFFF, 1);
        // Force a new empty batch 
        RECT r1 = {0, 0, 10, 10};
        batcher.SetScissorRect(r1, true);
        batcher.SetScissorRect(r1, false);
        batcher.AddQuad(0, 0, 10, 10, 0xFFFFFFFF, 2);
        CHECK(batcher.GetBatchCount() == 2);
    }
}

TEST_CASE("DynamicQuadBatcher - Zero Capacity Vector Fix")
{
    DynamicQuadBatcher batcher(0, 0); // Start with 0 capacity
    batcher.BeginFrame();

    batcher.AddQuad(0, 0, 10, 10, 0xFFFFFFFF, 1);
    CHECK(batcher.GetBatchCount() == 1);
    CHECK(batcher.GetBatchCapacity() >= 1);
}

TEST_CASE("DynamicQuadBatcher - Winding Order Fix")
{
    DynamicQuadBatcher batcher;
    batcher.BeginFrame();

    batcher.AddLine2d(0, 0, 10, 10, 0xFFFFFFFF);
    
    CHECK(batcher.GetVertexCount() == 4);
    
    // Test that vertex 1 & 2 swap matches normal quad ordering
    const TUIVertex* vertices = batcher.GetVertexBufferData();
    CHECK(vertices[1].position.x > vertices[2].position.x);
}
