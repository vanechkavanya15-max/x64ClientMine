#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include <cstdint>
#include <vector>
#include <cstring>
#include <string>
#include <span>
#include <unordered_map>

#include "EterBase/Result.h"
#include "Client/Network/Protocol/Protocol_GuildParty.h"
#include "Client/Network/GuildPacketCodec.h"
#include "Client/Bridge/PhaseGameGuildBridge.h"

// Mock CPythonNetworkStream dla sygnatury HandleGuild
class CPythonNetworkStream {};

// Tests
TEST_CASE("PhaseGameGuildBridge - Login") {
	CPythonNetworkStream stream;
	TPacketGCGuild packet{};
	packet.header = 1;
	packet.length = sizeof(packet);
	packet.subheader = GuildSub::GC::LOGIN;

	bool result = PhaseGameGuildBridge::HandleGuild(&stream, reinterpret_cast<const uint8_t*>(&packet), sizeof(packet));
	CHECK(result == true);
}

TEST_CASE("PhaseGameGuildBridge - Logout") {
	CPythonNetworkStream stream;
	TPacketGCGuild packet{};
	packet.header = 1;
	packet.length = sizeof(packet);
	packet.subheader = GuildSub::GC::LOGOUT;

	bool result = PhaseGameGuildBridge::HandleGuild(&stream, reinterpret_cast<const uint8_t*>(&packet), sizeof(packet));
	CHECK(result == true);
}

TEST_CASE("PhaseGameGuildBridge - Info") {
	TPacketGCGuildInfo info{};
	info.guild_id = 42;
	info.master_pid = 1337;
	info.level = 10;
	info.exp = 5000;
	info.member_count = 25;
	info.max_member_count = 50;
	info.gold = 1000000;
	info.hasLand = 1;
	std::strncpy(info.name, "TestGuild", 12);

	bool result = PhaseGameGuildBridge::HandleGuildSub_Info(info);
	CHECK(result == true);
}
