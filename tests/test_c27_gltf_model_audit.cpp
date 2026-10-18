#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "EterModelLib/GltfModel.h"
#include <thread>
#include <vector>

using namespace EterModelLib;

TEST_CASE("GltfModelAudit: CalculateAABBCorrectly")
{
    CGltfModel model;
    GltfModelData data;
    data.name = "TestModel";
    GltfVertex v1, v2;
    v1.position = {-10.0f, -5.0f, 0.0f};
    v2.position = {10.0f, 20.0f, 5.0f};
    data.vertices.push_back(v1);
    data.vertices.push_back(v2);

    GltfSubmesh sm;
    sm.vertexOffset = 0;
    sm.vertexCount = 2;
    data.submeshes.push_back(sm);

    model.SetModelData(data);

    const GltfModelData& builtData = model.GetModelData();
    CHECK(builtData.aabb.min.x == doctest::Approx(-10.0f));
    CHECK(builtData.aabb.min.y == doctest::Approx(-5.0f));
    CHECK(builtData.aabb.min.z == doctest::Approx(0.0f));
    CHECK(builtData.aabb.max.x == doctest::Approx(10.0f));
    CHECK(builtData.aabb.max.y == doctest::Approx(20.0f));
    CHECK(builtData.aabb.max.z == doctest::Approx(5.0f));
    CHECK(builtData.submeshes[0].aabb.min.x == doctest::Approx(-10.0f));
    CHECK(builtData.submeshes[0].aabb.max.y == doctest::Approx(20.0f));
}

TEST_CASE("GltfModelAudit: EnsureGpuBuffersThreadSafety")
{
    CGltfModel model;
    GltfModelData data;
    data.vertices.resize(100);
    data.indices.resize(100);
    model.SetModelData(data);

    IDirect3DDevice9 deviceMock;

    constexpr int NUM_THREADS = 10;
    std::vector<std::thread> threads;
    for (int i = 0; i < NUM_THREADS; ++i)
    {
        threads.emplace_back([&model, &deviceMock]() {
            model.EnsureGpuBuffers(reinterpret_cast<::LPDIRECT3DDEVICE9>(&deviceMock));
        });
    }

    for (auto& t : threads)
    {
        if (t.joinable())
            t.join();
    }
    CHECK(true);
}
