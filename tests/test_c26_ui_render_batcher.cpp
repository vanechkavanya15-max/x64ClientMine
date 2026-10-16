#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest/doctest.h>
#include "../src/Client/UI/UIRenderBatcher.h"

using namespace Client::UI;

TEST_CASE("UIRenderBatcher - Initial State") {
    UIRenderBatcher batcher;
    CHECK(batcher.GetVertices().empty());
    CHECK(batcher.GetBatches().empty());
}

TEST_CASE("UIRenderBatcher - DrawRect Batches Geometry") {
    UIRenderBatcher batcher;
    Color red{255, 0, 0, 255};
    Color blue{0, 0, 255, 255};

    auto res = batcher.DrawRect(0, 0, 10, 10, red);
    REQUIRE(res.has_value());

    auto res2 = batcher.DrawRect(10, 10, 20, 20, blue);
    REQUIRE(res2.has_value());

    auto vertices = batcher.GetVertices();
    auto batches = batcher.GetBatches();

    CHECK(vertices.size() == 12);
    CHECK(batches.size() == 1); // Same primitive type and texture (none), so it should batch

    CHECK(batches[0].type == PrimitiveType::TriangleList);
    CHECK(batches[0].texture.id == 0);
    CHECK(batches[0].vertexCount == 12);
    CHECK(batches[0].vertexStart == 0);
}

TEST_CASE("UIRenderBatcher - DrawTexturedRect Creates New Batch On Texture Change") {
    UIRenderBatcher batcher;
    Color white{255, 255, 255, 255};
    TextureId tex1{1};
    TextureId tex2{2};

    auto res = batcher.DrawTexturedRect(0, 0, 10, 10, 0, 0, 1, 1, tex1, white);
    REQUIRE(res.has_value());

    auto res2 = batcher.DrawTexturedRect(10, 10, 20, 20, 0, 0, 1, 1, tex2, white);
    REQUIRE(res2.has_value());

    auto batches = batcher.GetBatches();
    REQUIRE(batches.size() == 2);

    CHECK(batches[0].texture == tex1);
    CHECK(batches[0].vertexCount == 6);
    CHECK(batches[0].vertexStart == 0);

    CHECK(batches[1].texture == tex2);
    CHECK(batches[1].vertexCount == 6);
    CHECK(batches[1].vertexStart == 6);
}

TEST_CASE("UIRenderBatcher - Different Primitives Create New Batch") {
    UIRenderBatcher batcher;
    Color white{255, 255, 255, 255};

    auto res = batcher.DrawRect(0, 0, 10, 10, white);
    REQUIRE(res.has_value());

    auto res2 = batcher.DrawLine(0, 0, 10, 10, white);
    REQUIRE(res2.has_value());

    auto batches = batcher.GetBatches();
    REQUIRE(batches.size() == 2);

    CHECK(batches[0].type == PrimitiveType::TriangleList);
    CHECK(batches[0].vertexCount == 6);

    CHECK(batches[1].type == PrimitiveType::LineList);
    CHECK(batches[1].vertexCount == 2);
}

TEST_CASE("UIRenderBatcher - Buffer Overflow Protection") {
    UIRenderBatcher batcher(10); // Max 10 vertices
    Color white{255, 255, 255, 255};

    auto res = batcher.DrawRect(0, 0, 10, 10, white); // 6 vertices
    REQUIRE(res.has_value());

    auto res2 = batcher.DrawRect(10, 10, 10, 10, white); // 6 vertices, should overflow
    REQUIRE(!res2.has_value());
    CHECK(res2.error() == UIBatchError::BufferOverflow);
}

TEST_CASE("UIRenderBatcher - Invalid Geometry/Texture") {
    UIRenderBatcher batcher;
    Color white{255, 255, 255, 255};

    auto res = batcher.DrawRect(0, 0, 0, 10, white);
    REQUIRE(!res.has_value());
    CHECK(res.error() == UIBatchError::InvalidGeometry);

    auto res2 = batcher.DrawTexturedRect(0, 0, 10, 10, 0, 0, 1, 1, TextureId{0}, white);
    REQUIRE(!res2.has_value());
    CHECK(res2.error() == UIBatchError::InvalidTexture);
}

TEST_CASE("UIRenderBatcher - Flush Clears Data") {
    UIRenderBatcher batcher;
    batcher.DrawRect(0, 0, 10, 10, Color{255, 255, 255, 255});
    
    REQUIRE(batcher.GetVertices().size() == 6);
    REQUIRE(batcher.GetBatches().size() == 1);

    batcher.Flush();

    CHECK(batcher.GetVertices().empty());
    CHECK(batcher.GetBatches().empty());
}
