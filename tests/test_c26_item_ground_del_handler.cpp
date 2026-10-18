#include <vector>
#include <iostream>
#include <cassert>
#include <cstring>
#include <cstdint>

#include "Client/Network/Handlers/ItemGroundDelHandler.h"
#include "Client/Core/EventBus.h"

using namespace Client::Network::Handlers;

// Tracker for EventBus
struct TestTracker {
    int publish_count = 0;
    uint32_t last_vid = 0;
};
static TestTracker g_Tracker;

void test_buffer_underflow() {
    std::vector<uint8_t> buffer = { 0x01, 0x02, 0x03 }; 
    auto result = ItemGroundDelHandler::Handle(buffer);
    assert(!result.has_value());
    assert(result.error() == EterBase::PacketError::BufferUnderflow);
    std::cout << "[PASS] test_buffer_underflow\n";
}

void test_successful_parsing() {
    g_Tracker.publish_count = 0;

    // Subscribe to EventBus
    Client::Core::EventBus::GetInstance().Subscribe<ItemGroundDelEvent>([](const ItemGroundDelEvent& e) {
        g_Tracker.publish_count++;
        g_Tracker.last_vid = e.dropVid.value();
    });

    ItemGroundDelPacket pkt{};
    pkt.header = 0x55;
    pkt.itemVid = 12345;
    
    std::vector<uint8_t> buffer(sizeof(ItemGroundDelPacket));
    std::memcpy(buffer.data(), &pkt, sizeof(ItemGroundDelPacket));
    
    auto result = ItemGroundDelHandler::Handle(buffer);
    assert(result.has_value());
    assert(g_Tracker.publish_count == 1);
    assert(g_Tracker.last_vid == 12345);
    
    std::cout << "[PASS] test_successful_parsing\n";
}

int main() {
    std::cout << "Running tests for ItemGroundDelHandler...\n";
    test_buffer_underflow();
    test_successful_parsing();
    std::cout << "All tests passed.\n";
    return 0;
}
