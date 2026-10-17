#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "EterLib/Render/DynamicQuadBatcher.h"
#include "EterLib/RenderQueueManager.h"
#include "EterLib/Camera.h"

float CCamera::CAMERA_MAX_DISTANCE = 2500.0f;

using namespace EterLib::Render;

TEST_CASE("DynamicQuadBatcher - ZeroAllocationCapacityPreserved")
{
    DynamicQuadBatcher batcher(1024, 32);

    CHECK(batcher.GetVertexCapacity() >= 1024u);
    CHECK(batcher.GetVertexCount() == 0u);
    CHECK(batcher.GetBatchCount() == 0u);

    // Dodanie quada (4 wierzcholki dla indeksowanego quada)
    batcher.AddQuad(10.0f, 10.0f, 100.0f, 50.0f, 0xFFFFFFFF, 1);
    CHECK(batcher.GetVertexCount() == 4u);
    CHECK(batcher.GetBatchCount() == 1u);

    const size_t capBeforeClear = batcher.GetVertexCapacity();
    batcher.Clear();

    // Zero alokacji: pojemnosc zostaje zachowana
    CHECK(batcher.GetVertexCount() == 0u);
    CHECK(batcher.GetBatchCount() == 0u);
    CHECK(batcher.GetVertexCapacity() == capBeforeClear);
}

TEST_CASE("DynamicQuadBatcher - AutomaticGroupingSameTextureAndScissor")
{
    DynamicQuadBatcher batcher(1024, 32);

    // Dodanie 3 quadow z ta sama tekstura i bez scissor
    batcher.AddQuad(0.0f, 0.0f, 32.0f, 32.0f, 0xFFFFFFFF, 100);
    batcher.AddQuad(32.0f, 0.0f, 32.0f, 32.0f, 0xFFFFFFFF, 100);
    batcher.AddQuad(64.0f, 0.0f, 32.0f, 32.0f, 0xFFFFFFFF, 100);

    CHECK(batcher.GetBatchCount() == 1u);
    CHECK(batcher.GetVertexCount() == 12u); // 3 quady * 4 wierzcholki
}

TEST_CASE("DynamicQuadBatcher - AutomaticSplittingOnTextureChange")
{
    DynamicQuadBatcher batcher(1024, 32);

    // Quad z tekstura 10
    batcher.AddQuad(0.0f, 0.0f, 32.0f, 32.0f, 0xFFFFFFFF, 10);
    // Kolejny z tekstura 10 (ten sam batch)
    batcher.AddQuad(32.0f, 0.0f, 32.0f, 32.0f, 0xFFFFFFFF, 10);
    // Nowa tekstura 20 (nowy batch)
    batcher.AddQuad(64.0f, 0.0f, 32.0f, 32.0f, 0xFFFFFFFF, 20);

    CHECK(batcher.GetBatchCount() == 2u);
    CHECK(batcher.GetVertexCount() == 12u);
}

TEST_CASE("DynamicQuadBatcher - AutomaticSplittingOnScissorChange")
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
    CHECK(batcher.GetBatchCount() == 4u);
}

TEST_CASE("DynamicQuadBatcher - FlushToQueueSubmitsUIPackets")
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
    CHECK(newUIPackets - initialUIPackets == 2u);

    // Batcher zostal wyczyszczony po FlushToQueue
    CHECK(batcher.GetVertexCount() == 0u);
    CHECK(batcher.GetBatchCount() == 0u);
}

TEST_CASE("DynamicQuadBatcher - AddBarAndAddBoxGeometry")
{
    DynamicQuadBatcher batcher(1024, 32);

    // Bar = 1 quad = 4 wierzcholki
    batcher.AddBar(0.0f, 0.0f, 100.0f, 20.0f, 0xFF112233);
    CHECK(batcher.GetVertexCount() == 4u);

    // Box = 4 krawedzie (bary) = 4 quady = 16 wierzcholkow
    batcher.AddBox(10.0f, 10.0f, 50.0f, 50.0f, 0xFF445566);
    CHECK(batcher.GetVertexCount() == 20u); // 4 + 16 = 20
}

TEST_CASE("DynamicQuadBatcher - LargeBatchSplittingAt16380Quads")
{
    // Test wymogu F-NEW-018: 20000 quadow z podzialem partii przy 16380
    // 20000 quadow = 80000 wierzcholkow
    DynamicQuadBatcher batcher(100000, 64);

    for (int i = 0; i < 20000; ++i)
    {
        batcher.AddQuad(static_cast<float>(i % 100), static_cast<float>(i / 100), 10.0f, 10.0f, 0xFFFFFFFF, 77);
    }

    CHECK(batcher.GetOverflowQuads() == 0u);
    CHECK(batcher.GetVertexCount() == 80000u);
    // Partia powinna byc automatycznie podzielona na 2:
    // Partia 0: 16380 quadow = 65520 wierzcholkow
    // Partia 1: 3620 quadow = 14480 wierzcholkow
    CHECK(batcher.GetBatchCount() == 2u);

    CRenderQueueManager& queue = CRenderQueueManager::Instance();
    const size_t beforePackets = queue.GetUIPackets().size();
    batcher.FlushToQueue(queue);
    const size_t afterPackets = queue.GetUIPackets().size();

    CHECK(afterPackets - beforePackets == 2u);
    const auto& p0 = queue.GetUIPackets()[afterPackets - 2];
    const auto& p1 = queue.GetUIPackets()[afterPackets - 1];

    CHECK(p0.dwVertexCount == 16380u * 4u);
    CHECK(p1.dwVertexCount == 3620u * 4u);
    CHECK(p0.dwTextureID == 77);
    CHECK(p1.dwTextureID == 77);
    CHECK(p0.pVertices != nullptr);
    CHECK(p1.pVertices == p0.pVertices + (16380u * 4u));
}

TEST_CASE("DynamicQuadBatcher - NoMidFrameResizeAvoidsUseAfterFree")
{
    // Pojemnosc dokladnie na 10 quadow (40 wierzcholkow)
    DynamicQuadBatcher batcher(40, 16);
    batcher.BeginFrame();

    for (int i = 0; i < 10; ++i)
    {
        batcher.AddQuad(0.0f, 0.0f, 1.0f, 1.0f, 0xFFFFFFFF, 1);
    }

    CHECK(batcher.GetVertexCount() == 40u);
    CHECK(batcher.GetOverflowQuads() == 0u);
    const TUIVertex* pInitialData = batcher.GetVertexBufferData();
    const size_t initialCapacity = batcher.GetVertexCapacity();

    // Dodanie 11. quada przy pelnym buforze NIE MOZE wykonac resize (zakaz mid-frame resize)
    batcher.AddQuad(0.0f, 0.0f, 1.0f, 1.0f, 0xFFFFFFFF, 1);

    CHECK(batcher.GetVertexCapacity() == initialCapacity);
    CHECK(batcher.GetVertexBufferData() == pInitialData); // Wskaznik nie ulegl zmianie (brak UAF)
    CHECK(batcher.GetOverflowQuads() == 1u);
    CHECK(batcher.GetOverflowVertices() == 4u);
}

TEST_CASE("DynamicQuadBatcher - FrameLifecycleAndRingBuffer")
{
    DynamicQuadBatcher batcher(1000, 32);
    CRenderQueueManager& queue = CRenderQueueManager::Instance();

    batcher.BeginFrame();
    batcher.AddQuad(0.0f, 0.0f, 10.0f, 10.0f, 0xFFFFFFFF, 1);

    const size_t before = queue.GetUIPackets().size();
    batcher.FlushToQueue(queue);
    CHECK(queue.GetUIPackets().size() - before == 1u);

    // Po FlushToQueue licznik aktywnych wierzcholkow = 0, ale offset bufora wewnetrznego nie jest czyszczony
    CHECK(batcher.GetVertexCount() == 0u);
    CHECK(batcher.GetTotalVertexCount() == 4u);

    // Kolejny quad w tej samej klatce dopisuje sie na nowej pozycji w buforze
    batcher.AddQuad(10.0f, 10.0f, 10.0f, 10.0f, 0xFFFFFFFF, 2);
    CHECK(batcher.GetVertexCount() == 4u);
    CHECK(batcher.GetTotalVertexCount() == 8u);

    batcher.FlushToQueue(queue);
    CHECK(queue.GetUIPackets().size() - before == 2u);

    const auto& pkt0 = queue.GetUIPackets()[before];
    const auto& pkt1 = queue.GetUIPackets()[before + 1];
    CHECK(pkt1.pVertices == pkt0.pVertices + 4);

    // Dopiero BeginFrame resetuje caly bufor klatki
    batcher.BeginFrame();
    CHECK(batcher.GetTotalVertexCount() == 0u);
    CHECK(batcher.GetVertexCount() == 0u);
}
