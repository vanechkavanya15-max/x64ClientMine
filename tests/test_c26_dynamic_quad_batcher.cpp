#include <gtest/gtest.h>
#include "EterLib/Render/DynamicQuadBatcher.h"
#include "EterLib/RenderQueueManager.h"

using namespace EterLib::Render;

TEST(DynamicQuadBatcherTest, ZeroAllocationCapacityPreserved)
{
    DynamicQuadBatcher batcher(1024, 32);

    EXPECT_GE(batcher.GetVertexCapacity(), 1024u);
    EXPECT_EQ(batcher.GetVertexCount(), 0u);
    EXPECT_EQ(batcher.GetBatchCount(), 0u);

    // Dodanie quada (6 wierzcholkow)
    batcher.AddQuad(10.0f, 10.0f, 100.0f, 50.0f, 0xFFFFFFFF, 1);
    EXPECT_EQ(batcher.GetVertexCount(), 6u);
    EXPECT_EQ(batcher.GetBatchCount(), 1u);

    const size_t capBeforeClear = batcher.GetVertexCapacity();
    batcher.Clear();

    // Zero alokacji: pojemnosc zostaje zachowana
    EXPECT_EQ(batcher.GetVertexCount(), 0u);
    EXPECT_EQ(batcher.GetBatchCount(), 0u);
    EXPECT_EQ(batcher.GetVertexCapacity(), capBeforeClear);
}

TEST(DynamicQuadBatcherTest, AutomaticGroupingSameTextureAndScissor)
{
    DynamicQuadBatcher batcher(1024, 32);

    // Dodanie 3 quadow z ta sama tekstura i bez scissor
    batcher.AddQuad(0.0f, 0.0f, 32.0f, 32.0f, 0xFFFFFFFF, 100);
    batcher.AddQuad(32.0f, 0.0f, 32.0f, 32.0f, 0xFFFFFFFF, 100);
    batcher.AddQuad(64.0f, 0.0f, 32.0f, 32.0f, 0xFFFFFFFF, 100);

    EXPECT_EQ(batcher.GetBatchCount(), 1u);
    EXPECT_EQ(batcher.GetVertexCount(), 18u); // 3 * 6 wierzcholkow
}

TEST(DynamicQuadBatcherTest, AutomaticSplittingOnTextureChange)
{
    DynamicQuadBatcher batcher(1024, 32);

    // Quad z tekstura 10
    batcher.AddQuad(0.0f, 0.0f, 32.0f, 32.0f, 0xFFFFFFFF, 10);
    // Kolejny z tekstura 10 (ten sam batch)
    batcher.AddQuad(32.0f, 0.0f, 32.0f, 32.0f, 0xFFFFFFFF, 10);
    // Nowa tekstura 20 (nowy batch)
    batcher.AddQuad(64.0f, 0.0f, 32.0f, 32.0f, 0xFFFFFFFF, 20);

    EXPECT_EQ(batcher.GetBatchCount(), 2u);
    EXPECT_EQ(batcher.GetVertexCount(), 18u);
}

TEST(DynamicQuadBatcherTest, AutomaticSplittingOnScissorChange)
{
    DynamicQuadBatcher batcher(1024, 32);

    // Batch bez scissor
    batcher.AddQuad(0.0f, 0.0f, 100.0f, 100.0f, 0xFFFFFFFF, 5);

    // Wlaczenie scissor rect
    RECT scissor1{ 10, 10, 50, 50 };
    batcher.SetScissorRect(scissor1, true);
    batcher.AddQuad(10.0f, 10.0f, 40.0f, 40.0f, 0xFFFFFFFF, 5);

    // Zmiana prostokata scissor rect
    RECT scissor2{ 20, 20, 80, 80 };
    batcher.SetScissorRect(scissor2, true);
    batcher.AddQuad(20.0f, 20.0f, 40.0f, 40.0f, 0xFFFFFFFF, 5);

    // Wylaczenie scissor rect
    batcher.ResetScissorRect();
    batcher.AddQuad(0.0f, 0.0f, 100.0f, 100.0f, 0xFFFFFFFF, 5);

    // Kazda zmiana stanu scissor wygenerowala odrebny batch
    EXPECT_EQ(batcher.GetBatchCount(), 4u);
}

TEST(DynamicQuadBatcherTest, FlushToQueueSubmitsUIPackets)
{
    DynamicQuadBatcher batcher(1024, 32);
    CRenderQueueManager& queue = CRenderQueueManager::Instance();

    // Wypelnij batcher dwoma partiami
    batcher.AddBar2d(0.0f, 0.0f, 50.0f, 50.0f, 0xFF00FF00); // dwTextureID = 0

    RECT scissor{ 0, 0, 100, 100 };
    batcher.SetScissorRect(scissor, true);
    batcher.AddQuad(10.0f, 10.0f, 20.0f, 20.0f, 0xFFFFFFFF, 42);

    const size_t initialUIPackets = queue.GetUIPackets().size();

    batcher.FlushToQueue(queue);

    // Pakiety zostaly przekazane do RenderQueueManager
    const size_t newUIPackets = queue.GetUIPackets().size();
    EXPECT_EQ(newUIPackets - initialUIPackets, 2u);

    // Batcher zostal wyczyszczony po FlushToQueue
    EXPECT_EQ(batcher.GetVertexCount(), 0u);
    EXPECT_EQ(batcher.GetBatchCount(), 0u);
}

TEST(DynamicQuadBatcherTest, AddBarAndAddBoxGeometry)
{
    DynamicQuadBatcher batcher(1024, 32);

    // Bar = 1 quad = 6 wierzcholkow
    batcher.AddBar(0.0f, 0.0f, 100.0f, 20.0f, 0xFF112233);
    EXPECT_EQ(batcher.GetVertexCount(), 6u);

    // Box = 4 krawedzie (bary) = 4 quady = 24 wierzcholki
    batcher.AddBox(10.0f, 10.0f, 50.0f, 50.0f, 0xFF445566);
    EXPECT_EQ(batcher.GetVertexCount(), 30u);
}
