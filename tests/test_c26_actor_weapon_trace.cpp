#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/Actors/ActorWeaponTraceRenderer.h"
#include <vector>

using namespace Client::Actors;
using namespace Client::Core;

class MockWeaponTraceBackend : public IWeaponTraceBackend {
public:
    struct RenderCall {
        TraceId id;
        std::vector<TraceVertex> vertices;
    };
    
    std::vector<RenderCall> calls;

    void RenderTrace(TraceId id, const std::vector<TraceVertex>& vertices) override {
        calls.push_back({id, vertices});
    }
};

TEST_CASE("ActorWeaponTraceRenderer") {
    MockWeaponTraceBackend backend;
    ActorWeaponTraceRenderer renderer(&backend);
    
    SUBCASE("StartTrace") {
        TraceId id(1);
        auto res = renderer.StartTrace(id);
        CHECK(res.has_value());
        
        auto res2 = renderer.StartTrace(id);
        CHECK(!res2.has_value());
        CHECK(res2.error() == TraceError::TraceAlreadyExists);
    }
    
    SUBCASE("StopTrace") {
        TraceId id(1);
        renderer.StartTrace(id);
        
        auto res = renderer.StopTrace(id);
        CHECK(res.has_value());
        
        auto res2 = renderer.StopTrace(id);
        CHECK(!res2.has_value());
        CHECK(res2.error() == TraceError::TraceNotFound);
    }
    
    SUBCASE("AddPoint updates trace correctly and tests fading") {
        TraceId id(1);
        renderer.StartTrace(id);
        
        Vec3 p0(0, 0, 0);
        Vec3 p1(1, 1, 1);
        
        // Need at least 2 points for a render (fallback linear strip)
        renderer.AddPoint(id, p0, p1);
        renderer.AddPoint(id, p0, p1);
        
        renderer.Update(0.1f);
        CHECK(backend.calls.size() == 1);
        CHECK(backend.calls[0].id == id);
        CHECK(backend.calls[0].vertices.size() == 4); // 2 points * 2 vertices per point
        CHECK(backend.calls[0].vertices[0].alpha > 0.0f); // Should be fading out slightly
        CHECK(backend.calls[0].vertices[0].alpha < 1.0f);
        
        backend.calls.clear();
        
        // Wait 1.0s to let the trace fade out fully and points to be removed
        renderer.Update(1.0f);
        CHECK(backend.calls.empty()); // No more points = no render call
    }
    
    SUBCASE("CubicSpline generation kicks in with >= 4 points") {
        TraceId id(2);
        renderer.StartTrace(id);
        
        renderer.AddPoint(id, Vec3(0,0,0), Vec3(0,1,0));
        renderer.AddPoint(id, Vec3(1,0,0), Vec3(1,1,0));
        renderer.AddPoint(id, Vec3(2,0,0), Vec3(2,1,0));
        renderer.AddPoint(id, Vec3(3,0,0), Vec3(3,1,0));
        
        renderer.Update(0.0f); // 0.0s so no fading happens yet
        CHECK(backend.calls.size() == 1);
        
        // We have 4 points. The loop runs from i=1 to i<2 (so 1 iteration).
        // Each iteration produces 4 segments * 2 vertices = 8 vertices.
        // Plus 2 vertices for the final point. Total = 10 vertices.
        CHECK(backend.calls[0].vertices.size() == 10);
    }
}
