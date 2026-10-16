#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Actors/ActorTcpStateManager.h"

TEST_CASE("ActorTcpStateManager: Mask operations and movement check") {
    using namespace Client::Actors;
    ActorTcpStateManager state;
    CHECK(state.CanMove());

    state.SetState(STATE_POISON);
    CHECK(state.HasState(STATE_POISON));
    CHECK(state.CanMove());

    state.SetState(STATE_STUN);
    CHECK(state.IsStunned());
    CHECK_FALSE(state.CanMove());

    state.ClearState(STATE_STUN);
    CHECK(state.CanMove());
}
