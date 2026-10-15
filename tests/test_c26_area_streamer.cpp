#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "../src/Client/World/AreaObjectStreamer.h"

using namespace Client::World;
using namespace Client::Core;

TEST_CASE("AreaObjectStreamer tests: RegisterAndUnregister") {
    AreaObjectStreamer streamer;
    
    AreaObjectRecord record1{AreaObjectId{1}, 10.0f, 20.0f, 0.0f, 100};
    
    auto result1 = streamer.RegisterObject(record1);
    CHECK(result1.has_value());
    CHECK_EQ(streamer.GetRegisteredCount(), 1);
    
    auto result2 = streamer.RegisterObject(record1);
    CHECK_FALSE(result2.has_value());
    CHECK(result2.error() == AreaStreamerError::AlreadyExists);
    
    auto result3 = streamer.UnregisterObject(AreaObjectId{1});
    CHECK(result3.has_value());
    CHECK_EQ(streamer.GetRegisteredCount(), 0);
    
    auto result4 = streamer.UnregisterObject(AreaObjectId{2});
    CHECK_FALSE(result4.has_value());
    CHECK(result4.error() == AreaStreamerError::NotFound);
}

TEST_CASE("AreaObjectStreamer tests: StreamInOutByDistance") {
    AreaObjectStreamer streamer;
    
    AreaObjectRecord record1{AreaObjectId{1}, 10.0f, 10.0f, 0.0f, 100}; // Dist ~14.1
    AreaObjectRecord record2{AreaObjectId{2}, 100.0f, 100.0f, 0.0f, 101}; // Dist ~141.4
    
    streamer.RegisterObject(record1);
    streamer.RegisterObject(record2);
    
    int streamInCalls = 0;
    int streamOutCalls = 0;
    AreaObjectId lastStreamedIn{0};
    AreaObjectId lastStreamedOut{0};
    
    auto onStreamIn = [&](const AreaObjectRecord& rec) {
        streamInCalls++;
        lastStreamedIn = rec.id;
    };
    
    auto onStreamOut = [&](AreaObjectId id) {
        streamOutCalls++;
        lastStreamedOut = id;
    };
    
    // Viewer at (0, 0), radius 20
    streamer.UpdateViewerPosition(0.0f, 0.0f, 20.0f, onStreamIn, onStreamOut);
    
    CHECK_EQ(streamInCalls, 1);
    CHECK_EQ(streamOutCalls, 0);
    CHECK(lastStreamedIn == AreaObjectId{1});
    CHECK(streamer.IsVisible(AreaObjectId{1}));
    CHECK_FALSE(streamer.IsVisible(AreaObjectId{2}));
    
    streamInCalls = 0;
    lastStreamedIn = AreaObjectId{0};
    
    // Viewer moves to (90, 90), radius 20
    streamer.UpdateViewerPosition(90.0f, 90.0f, 20.0f, onStreamIn, onStreamOut);
    
    CHECK_EQ(streamInCalls, 1);
    CHECK_EQ(streamOutCalls, 1);
    CHECK(lastStreamedIn == AreaObjectId{2});
    CHECK(lastStreamedOut == AreaObjectId{1});
    CHECK_FALSE(streamer.IsVisible(AreaObjectId{1}));
    CHECK(streamer.IsVisible(AreaObjectId{2}));
    
    // Test unregistering visible object
    streamer.UnregisterObject(AreaObjectId{2});
    CHECK_FALSE(streamer.IsVisible(AreaObjectId{2}));
}

TEST_CASE("AreaObjectStreamer tests: InvalidRadius") {
    AreaObjectStreamer streamer;
    AreaObjectRecord record1{AreaObjectId{1}, 0.0f, 0.0f, 0.0f, 100};
    streamer.RegisterObject(record1);
    
    int calls = 0;
    streamer.UpdateViewerPosition(0.0f, 0.0f, -10.0f, [&](const AreaObjectRecord&) { calls++; }, [&](AreaObjectId) { calls++; });
    
    CHECK_EQ(calls, 0);
    CHECK_FALSE(streamer.IsVisible(AreaObjectId{1}));
}

TEST_CASE("AreaObjectStreamer tests: Clear") {
    AreaObjectStreamer streamer;
    AreaObjectRecord record1{AreaObjectId{1}, 0.0f, 0.0f, 0.0f, 100};
    streamer.RegisterObject(record1);
    streamer.UpdateViewerPosition(0.0f, 0.0f, 10.0f, nullptr, nullptr);
    
    CHECK_EQ(streamer.GetRegisteredCount(), 1);
    CHECK_EQ(streamer.GetVisibleObjects().size(), 1);
    
    streamer.Clear();
    
    CHECK_EQ(streamer.GetRegisteredCount(), 0);
    CHECK_EQ(streamer.GetVisibleObjects().size(), 0);
}
