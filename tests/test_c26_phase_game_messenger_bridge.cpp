#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/Bridge/PhaseGameMessengerBridge.h"
#include "Client/Bridge/MockPythonRuntime.h"
#include "Client/Network/MessengerPacketCodec.h"
#include "Client/Network/Protocol/Protocol_GuildParty.h"
#include <vector>
#include <cstdint>
#include <cstring>
#include <string>

using namespace Client::Bridge;
using namespace Client::Network;

std::vector<uint8_t> CreateMessengerListPacket(const std::vector<std::pair<std::string, bool>>& friends) {
    std::vector<uint8_t> buffer(sizeof(TPacketGCMessenger));
    TPacketGCMessenger* header = reinterpret_cast<TPacketGCMessenger*>(buffer.data());
    header->header = 0x30; // HEADER_GC_MESSENGER (z beavium_gc_table.inl)
    header->subheader = MessengerSub::GC::LIST;
    
    for (const auto& [name, isOnline] : friends) {
        TPacketGCMessengerListOffline entry; // Struktura dla obu przypadkow ma ten sam rozmiar i pola (tylko connected sie zmienia)
        entry.connected = isOnline ? MESSENGER_CONNECTED_STATE_ONLINE : MESSENGER_CONNECTED_STATE_OFFLINE;
        entry.length = static_cast<uint8_t>(name.size());
        
        size_t offset = buffer.size();
        buffer.resize(offset + sizeof(entry) + name.size());
        std::memcpy(buffer.data() + offset, &entry, sizeof(entry));
        std::memcpy(buffer.data() + offset + sizeof(entry), name.data(), name.size());
    }
    
    header = reinterpret_cast<TPacketGCMessenger*>(buffer.data());
    header->length = static_cast<uint16_t>(buffer.size());
    return buffer;
}

std::vector<uint8_t> CreateMessengerLoginPacket(const std::string& name) {
    std::vector<uint8_t> buffer(sizeof(TPacketGCMessenger));
    TPacketGCMessenger* header = reinterpret_cast<TPacketGCMessenger*>(buffer.data());
    header->header = 0x30;
    header->subheader = MessengerSub::GC::LOGIN;
    
    TPacketGCMessengerLogin login;
    login.length = static_cast<uint8_t>(name.size());
    
    size_t offset = buffer.size();
    buffer.resize(offset + sizeof(login) + name.size());
    std::memcpy(buffer.data() + offset, &login, sizeof(login));
    std::memcpy(buffer.data() + offset + sizeof(login), name.data(), name.size());
    
    header = reinterpret_cast<TPacketGCMessenger*>(buffer.data());
    header->length = static_cast<uint16_t>(buffer.size());
    return buffer;
}

std::vector<uint8_t> CreateMessengerLogoutPacket(const std::string& name) {
    std::vector<uint8_t> buffer(sizeof(TPacketGCMessenger));
    TPacketGCMessenger* header = reinterpret_cast<TPacketGCMessenger*>(buffer.data());
    header->header = 0x30;
    header->subheader = MessengerSub::GC::LOGOUT;
    
    TPacketGCMessengerLogout logout;
    logout.length = static_cast<uint8_t>(name.size());
    
    size_t offset = buffer.size();
    buffer.resize(offset + sizeof(logout) + name.size());
    std::memcpy(buffer.data() + offset, &logout, sizeof(logout));
    std::memcpy(buffer.data() + offset + sizeof(logout), name.data(), name.size());
    
    header = reinterpret_cast<TPacketGCMessenger*>(buffer.data());
    header->length = static_cast<uint16_t>(buffer.size());
    return buffer;
}

std::vector<uint8_t> CreateMessengerRemovePacket(const std::string& name) {
    std::vector<uint8_t> buffer(sizeof(TPacketGCMessenger));
    TPacketGCMessenger* header = reinterpret_cast<TPacketGCMessenger*>(buffer.data());
    header->header = 0x30;
    header->subheader = MessengerSub::GC::REMOVE_FRIEND;
    
    uint8_t nameLength = static_cast<uint8_t>(name.size());
    
    size_t offset = buffer.size();
    buffer.resize(offset + sizeof(nameLength) + name.size());
    std::memcpy(buffer.data() + offset, &nameLength, sizeof(nameLength));
    std::memcpy(buffer.data() + offset + sizeof(nameLength), name.data(), name.size());
    
    header = reinterpret_cast<TPacketGCMessenger*>(buffer.data());
    header->length = static_cast<uint16_t>(buffer.size());
    return buffer;
}

TEST_CASE("PhaseGameMessengerBridge - LIST Packet") {
    MockPythonRuntime runtime;
    runtime.Initialize();
    PhaseGameMessengerBridge bridge(runtime);

    auto packet = CreateMessengerListPacket({{"FriendA", true}, {"FriendB", false}});
    
    bool result = bridge.HandleMessenger(packet);
    CHECK(result == true);

    auto history = runtime.GetHistory();
    REQUIRE(history.size() == 2);
    
    CHECK(history[0].moduleName == "messenger");
    CHECK(history[0].methodName == "OnFriendLogin");
    CHECK(history[0].args.size() == 1);
    CHECK(history[0].args[0] == "FriendA");
    
    CHECK(history[1].moduleName == "messenger");
    CHECK(history[1].methodName == "OnFriendLogout");
    CHECK(history[1].args.size() == 1);
    CHECK(history[1].args[0] == "FriendB");
}

TEST_CASE("PhaseGameMessengerBridge - LOGIN Packet") {
    MockPythonRuntime runtime;
    runtime.Initialize();
    PhaseGameMessengerBridge bridge(runtime);

    auto packet = CreateMessengerLoginPacket("TestFriend");
    
    bool result = bridge.HandleMessenger(packet);
    CHECK(result == true);

    auto history = runtime.GetHistory();
    REQUIRE(history.size() == 2);
    
    CHECK(history[0].moduleName == "messenger");
    CHECK(history[0].methodName == "OnFriendLogin");
    CHECK(history[0].args.size() == 1);
    CHECK(history[0].args[0] == "TestFriend");
    
    CHECK(history[1].moduleName == "game");
    CHECK(history[1].methodName == "RefreshTargetBoardByName");
    CHECK(history[1].args.size() == 1);
    CHECK(history[1].args[0] == "TestFriend");
}

TEST_CASE("PhaseGameMessengerBridge - LOGOUT Packet") {
    MockPythonRuntime runtime;
    runtime.Initialize();
    PhaseGameMessengerBridge bridge(runtime);

    auto packet = CreateMessengerLogoutPacket("TestFriend");
    
    bool result = bridge.HandleMessenger(packet);
    CHECK(result == true);

    auto history = runtime.GetHistory();
    REQUIRE(history.size() == 1);
    
    CHECK(history[0].moduleName == "messenger");
    CHECK(history[0].methodName == "OnFriendLogout");
    CHECK(history[0].args.size() == 1);
    CHECK(history[0].args[0] == "TestFriend");
}

TEST_CASE("PhaseGameMessengerBridge - REMOVE_FRIEND Packet") {
    MockPythonRuntime runtime;
    runtime.Initialize();
    PhaseGameMessengerBridge bridge(runtime);

    auto packet = CreateMessengerRemovePacket("TestFriend");
    
    bool result = bridge.HandleMessenger(packet);
    CHECK(result == true);

    auto history = runtime.GetHistory();
    REQUIRE(history.size() == 2);
    
    CHECK(history[0].moduleName == "messenger");
    CHECK(history[0].methodName == "RemoveFriend");
    CHECK(history[0].args.size() == 1);
    CHECK(history[0].args[0] == "TestFriend");
    
    CHECK(history[1].moduleName == "game");
    CHECK(history[1].methodName == "RefreshTargetBoardByName");
    CHECK(history[1].args.size() == 1);
    CHECK(history[1].args[0] == "TestFriend");
}

TEST_CASE("PhaseGameMessengerBridge - Buffer Underflow Handling") {
    MockPythonRuntime runtime;
    runtime.Initialize();
    PhaseGameMessengerBridge bridge(runtime);

    std::vector<uint8_t> shortPacket(sizeof(TPacketGCMessenger) - 1);
    
    bool result = bridge.HandleMessenger(shortPacket);
    CHECK(result == false);
    
    auto packet = CreateMessengerLoginPacket("TestFriend");
    packet.pop_back(); // Skroc pakiet
    
    result = bridge.HandleMessenger(packet);
    CHECK(result == false);
}
