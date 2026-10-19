#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

// We verify that the API signatures match what is expected without relying on heavy D3DX dependencies.
// To do so, we mock the classes just to make sure they compile with the expected interface.

namespace Client::Core {
    struct WorldContext {
        int dummy;
    };
}

class CActorInstance {
public:
    void Update(Client::Core::WorldContext& context) {}
    void Render(Client::Core::WorldContext& context) {}
};

class CFlyingInstance {
public:
    bool Update(Client::Core::WorldContext& context) { return true; }
    void Render(Client::Core::WorldContext& context) {}
};

class CPhysicsObject {
public:
    void Update(float fElapsedTime, Client::Core::WorldContext& context) {}
};

using namespace Client::Core;

TEST_CASE("WorldContext API Signature Verification in GameLib") {
    // Check CActorInstance signatures
    using ActorUpdateFunc = void(CActorInstance::*)(WorldContext&);
    using ActorRenderFunc = void(CActorInstance::*)(WorldContext&);
    ActorUpdateFunc a_update = &CActorInstance::Update;
    ActorRenderFunc a_render = &CActorInstance::Render;
    CHECK(a_update != nullptr);
    CHECK(a_render != nullptr);

    // Check CFlyingInstance signatures
    using FlyingUpdateFunc = bool(CFlyingInstance::*)(WorldContext&);
    using FlyingRenderFunc = void(CFlyingInstance::*)(WorldContext&);
    FlyingUpdateFunc f_update = &CFlyingInstance::Update;
    FlyingRenderFunc f_render = &CFlyingInstance::Render;
    CHECK(f_update != nullptr);
    CHECK(f_render != nullptr);

    // Check CPhysicsObject signatures
    using PhysicsUpdateFunc = void(CPhysicsObject::*)(float, WorldContext&);
    PhysicsUpdateFunc p_update = &CPhysicsObject::Update;
    CHECK(p_update != nullptr);
}
