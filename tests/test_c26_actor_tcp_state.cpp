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

TEST_CASE("ActorTcpStateManager: Network Update and Jitter Buffer") {
    using namespace Client::Actors;
    ActorTcpStateManager state;
    
    StateUpdateCommand cmd1{100, 1000, STATE_SLOW};
    auto res1 = state.PushNetworkUpdate(cmd1);
    CHECK(res1.has_value());
    
    // Stale update
    StateUpdateCommand cmd_stale{99, 900, STATE_NONE};
    auto res_stale = state.PushNetworkUpdate(cmd_stale);
    CHECK(!res_stale.has_value());
    CHECK(res_stale.error() == StateManagerError::StaleUpdate);
    
    // Duplicate update
    StateUpdateCommand cmd_dup{100, 1000, STATE_SLOW};
    auto res_dup = state.PushNetworkUpdate(cmd_dup);
    CHECK(!res_dup.has_value());
    CHECK(res_dup.error() == StateManagerError::DuplicateUpdate);
    
    // Out of order insertion
    StateUpdateCommand cmd3{102, 1200, STATE_STUN};
    auto res3 = state.PushNetworkUpdate(cmd3);
    CHECK(res3.has_value());
    
    StateUpdateCommand cmd2{101, 1100, STATE_SLOW | STATE_POISON};
    auto res2 = state.PushNetworkUpdate(cmd2);
    CHECK(res2.has_value());
    
    // Playout Time 1050 (should process cmd1)
    state.ProcessPlayout(1050);
    CHECK(state.HasState(STATE_SLOW));
    CHECK_FALSE(state.HasState(STATE_POISON));
    CHECK_FALSE(state.IsStunned());
    
    // Playout Time 1250 (should process cmd2 and cmd3)
    state.ProcessPlayout(1250);
    CHECK_FALSE(state.HasState(STATE_SLOW));
    CHECK_FALSE(state.HasState(STATE_POISON));
    CHECK(state.IsStunned()); // Overridden to STUN in cmd3
}

TEST_CASE("ActorTcpStateManager: Network Playout correct replace check") {
    using namespace Client::Actors;
    ActorTcpStateManager state;
    
    state.PushNetworkUpdate({1, 100, STATE_SLOW});
    state.PushNetworkUpdate({2, 200, STATE_SLOW | STATE_POISON});
    state.PushNetworkUpdate({3, 300, STATE_STUN});
    
    state.ProcessPlayout(150);
    CHECK(state.HasState(STATE_SLOW));
    CHECK_FALSE(state.HasState(STATE_POISON));
    
    state.ProcessPlayout(250);
    CHECK(state.HasState(STATE_SLOW));
    CHECK(state.HasState(STATE_POISON));
    
    state.ProcessPlayout(350);
    CHECK(state.HasState(STATE_STUN));
    CHECK_FALSE(state.HasState(STATE_SLOW)); // Entire mask is replaced
    CHECK_FALSE(state.HasState(STATE_POISON)); // Entire mask is replaced
}

TEST_CASE("ActorTcpStateManager: Local Prediction") {
    using namespace Client::Actors;
    ActorTcpStateManager state;
    
    state.SetState(STATE_POISON);
    CHECK(state.HasState(STATE_POISON));
    
    // Predict stun
    uint32_t pid = state.PredictStateLocal(1000, STATE_STUN, 0);
    CHECK(state.HasState(STATE_POISON));
    CHECK(state.HasState(STATE_STUN));
    
    // Rollback
    auto res = state.RollbackLocalPrediction(pid);
    CHECK(res.has_value());
    CHECK(state.HasState(STATE_POISON));
    CHECK_FALSE(state.HasState(STATE_STUN));
    
    // Confirm
    uint32_t pid2 = state.PredictStateLocal(1100, STATE_SLOW, STATE_POISON);
    CHECK(state.HasState(STATE_SLOW));
    CHECK_FALSE(state.HasState(STATE_POISON)); // POISON should be cleared in effective mask
    
    auto res2 = state.ConfirmLocalPrediction(pid2);
    CHECK(res2.has_value());
    CHECK_FALSE(state.HasState(STATE_POISON)); // Confirmed cleared POISON
    CHECK(state.HasState(STATE_SLOW)); // Confirmed set SLOW
}

TEST_CASE("ActorTcpStateManager: History Buffer and Lookups") {
    using namespace Client::Actors;
    ActorTcpStateManager state;
    
    state.PushNetworkUpdate({1, 1000, STATE_NONE});
    state.PushNetworkUpdate({2, 1100, STATE_POISON});
    state.PushNetworkUpdate({3, 1200, STATE_POISON | STATE_SLOW});
    state.PushNetworkUpdate({4, 1300, STATE_STUN});
    
    state.ProcessPlayout(1500); // Process all
    
    auto optState = state.GetStateAtTimestamp(900);
    CHECK(optState.has_value());
    CHECK(*optState == STATE_NONE);
    
    auto optState2 = state.GetStateAtTimestamp(1050);
    CHECK(optState2.has_value());
    CHECK(*optState2 == STATE_NONE);
    
    auto optState3 = state.GetStateAtTimestamp(1150);
    CHECK(optState3.has_value());
    CHECK(*optState3 == STATE_POISON);
    
    auto optState4 = state.GetStateAtTimestamp(1250);
    CHECK(optState4.has_value());
    CHECK(*optState4 == (STATE_POISON | STATE_SLOW));
    
    auto optState5 = state.GetStateAtTimestamp(1400);
    CHECK(optState5.has_value());
    CHECK(*optState5 == STATE_STUN);
    
    // HadStateInWindow
    CHECK(state.HadStateInWindow(1150, 1250, STATE_POISON));
    CHECK(state.HadStateInWindow(1150, 1250, STATE_SLOW)); // at 1200 it became slow
    CHECK_FALSE(state.HadStateInWindow(1000, 1150, STATE_SLOW));
    CHECK(state.HadStateInWindow(1250, 1350, STATE_STUN)); // at 1300 became stun
}

TEST_CASE("ActorTcpStateManager: Cleanup Stale Predictions") {
    using namespace Client::Actors;
    ActorTcpStateManager state;
    
    state.PredictStateLocal(1000, STATE_STUN, 0);
    state.PredictStateLocal(1500, STATE_SLOW, 0);
    
    CHECK(state.HasState(STATE_STUN));
    CHECK(state.HasState(STATE_SLOW));
    
    // Time 1300, timeout 200 -> predictions before 1100 should be removed.
    state.CleanupStalePredictions(1300, 200);
    
    CHECK_FALSE(state.HasState(STATE_STUN)); // Removed
    CHECK(state.HasState(STATE_SLOW)); // Kept
}
