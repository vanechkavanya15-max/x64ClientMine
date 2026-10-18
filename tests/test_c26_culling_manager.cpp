#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest/doctest.h>

#include "EterLib/CullingManager.h"
#include "EterLib/GrpObjectInstance.h"

class DummyGraphicObject : public CGraphicObjectInstance {
public:
    DummyGraphicObject() {
        m_isVisible = true;
    }
    void OnInitialize() override {}
    void OnClear() override {}
    void OnUpdate() override {}
    void OnDeform() override {}
    void OnRender() override {}
    void OnBlendRender() override {}
    void OnRenderToShadowMap() override {}
    void OnRenderShadow() override {}
    void OnRenderPCBlocker() override {}
    void OnUpdateCollisionData(const CStaticCollisionDataVector*) override {}
    void OnUpdateHeighInstance(CAttributeInstance*) override {}
    bool OnGetObjectHeight(float, float, float*) override { return false; }
};

TEST_CASE("CullingManager Lifecycle and Logic") {
    CCullingManager cullingMgr;
    DummyGraphicObject obj1;
    obj1.SetPosition(100.0f, 100.0f, 0.0f);
    obj1.UpdateBoundingSphere(); // updates local bounds
    auto handle1 = cullingMgr.Register(&obj1);
    CHECK(handle1 != nullptr);

    SUBCASE("FindRange Tests") {
        cullingMgr.FindRange(Vector3d(100.0f, 100.0f, 0.0f), 50.0f);
        bool found = false;
        for (auto* item : cullingMgr) {
            if (item == &obj1) found = true;
        }
        CHECK(found == true);

        // Outside range
        cullingMgr.FindRange(Vector3d(500.0f, 500.0f, 0.0f), 50.0f);
        found = false;
        for (auto* item : cullingMgr) {
            if (item == &obj1) found = true;
        }
        CHECK(found == false);
    }

    SUBCASE("FindRayDistance Tests") {
        // Pointing right at the object
        cullingMgr.FindRayDistance(Vector3d(0.0f, 100.0f, 0.0f), Vector3d(1.0f, 0.0f, 0.0f), 500.0f);
        bool found = false;
        for (auto* item : cullingMgr) {
            if (item == &obj1) found = true;
        }
        CHECK(found == true);
        
        // Ray that hits but is limited by distance
        cullingMgr.FindRayDistance(Vector3d(0.0f, 100.0f, 0.0f), Vector3d(1.0f, 0.0f, 0.0f), 50.0f); // too short, object is at 100,0
        found = false;
        for (auto* item : cullingMgr) {
            if (item == &obj1) found = true;
        }
        CHECK(found == false);
    }

    cullingMgr.Unregister(handle1);
}

static std::vector<CGraphicObjectInstance*> s_RayTestResults;
struct MockCallbackFunction {
    void operator()(CGraphicObjectInstance* obj) {
        s_RayTestResults.push_back(obj);
    }
};

TEST_CASE("RangeTester actual distance logic") {
    CCullingManager cullingMgr;
    DummyGraphicObject obj;
    obj.SetPosition(100.0f, 0.0f, 0.0f);
    obj.UpdateBoundingSphere();
    auto handle = cullingMgr.Register(&obj);

    s_RayTestResults.clear();
    MockCallbackFunction callback;
    cullingMgr.ForInRayDistance(Vector3d(0.0f, 0.0f, 0.0f), Vector3d(1.0f, 0.0f, 0.0f), 50.0f, &callback);
    CHECK(s_RayTestResults.empty());

    s_RayTestResults.clear();
    cullingMgr.ForInRayDistance(Vector3d(0.0f, 0.0f, 0.0f), Vector3d(1.0f, 0.0f, 0.0f), 150.0f, &callback);
    CHECK(!s_RayTestResults.empty());
    CHECK(s_RayTestResults[0] == &obj);

    cullingMgr.Unregister(handle);
}
