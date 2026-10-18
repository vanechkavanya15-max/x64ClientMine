#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#define ETERBASE_STDAFX_H // Disable stdafx include
#include <cstdint>
#include <vector>
#include <cstring>

// Mocks for testing without UserInterface/Packet.h
namespace GC {
    constexpr uint16_t TIME = 0x0B13;
    constexpr uint16_t DUNGEON = 0x0B11;
    constexpr uint16_t FISHING = 0x0B10;
    constexpr uint16_t CHANNEL = 0x0B14;
}

namespace CG {
    constexpr uint16_t FISHING = 0x0B01;
}

#pragma pack(push, 1)

typedef struct SPacketGCTime
{
    uint16_t	header;
    uint16_t	length;
    long      time; // Use long instead of time_t to avoid <ctime> differences
} TPacketGCTime;

typedef struct packet_dungeon
{
	uint16_t	header;
	uint16_t	length;
    uint8_t		subheader;
} TPacketGCDungeon;

typedef struct packet_fishing
{
    uint16_t	header;
    uint16_t	length;
    uint8_t subheader;
    uint32_t info;
    uint8_t dir;
} TPacketGCFishing;

typedef struct packet_channel
{
    uint16_t	header;
    uint16_t	length;
    uint8_t channel;
} TPacketGCChannel;

typedef struct command_fishing
{
    uint16_t	header;
    uint16_t	length;
    uint8_t dir;
} TPacketCGFishing;

#pragma pack(pop)

// Providing our own mock structs to WorldPacketCodec.h
// so we define a macro to skip including Packet.h and PacketHeader.h
#define _USERINTERFACE_PACKET_H_
#define _USERINTERFACE_PACKETS_PACKETHEADER_H_

#include "../src/Client/Network/WorldPacketCodec.h"
#include "../src/Client/Network/WorldPacketCodec.cpp"

using namespace Client::Network;
using EterBase::PacketError;

TEST_CASE("WorldPacketCodecTest: DecodeTime_Success") {
    TPacketGCTime expectedPacket{};
    expectedPacket.header = GC::TIME;
    expectedPacket.length = sizeof(TPacketGCTime);
    expectedPacket.time = 123456789;

    std::vector<uint8_t> buffer(sizeof(TPacketGCTime));
    std::memcpy(buffer.data(), &expectedPacket, sizeof(TPacketGCTime));

    auto result = WorldPacketCodec::DecodeTime(buffer);
    REQUIRE(result.has_value());
    CHECK(result.value().header == expectedPacket.header);
    CHECK(result.value().length == expectedPacket.length);
    CHECK(result.value().time == expectedPacket.time);
}

TEST_CASE("WorldPacketCodecTest: DecodeTime_BufferTooSmall") {
    std::vector<uint8_t> buffer(sizeof(TPacketGCTime) - 1);
    auto result = WorldPacketCodec::DecodeTime(buffer);
    CHECK_FALSE(result.has_value());
    CHECK(result.error() == PacketError::BufferUnderflow);
}

TEST_CASE("WorldPacketCodecTest: DecodeDungeon_Success") {
    TPacketGCDungeon expectedPacket{};
    expectedPacket.header = GC::DUNGEON;
    expectedPacket.length = sizeof(TPacketGCDungeon);
    expectedPacket.subheader = 1;

    std::vector<uint8_t> buffer(sizeof(TPacketGCDungeon));
    std::memcpy(buffer.data(), &expectedPacket, sizeof(TPacketGCDungeon));

    auto result = WorldPacketCodec::DecodeDungeon(buffer);
    REQUIRE(result.has_value());
    CHECK(result.value().header == expectedPacket.header);
    CHECK(result.value().length == expectedPacket.length);
    CHECK(result.value().subheader == expectedPacket.subheader);
}

TEST_CASE("WorldPacketCodecTest: DecodeDungeon_BufferTooSmall") {
    std::vector<uint8_t> buffer(sizeof(TPacketGCDungeon) - 1);
    auto result = WorldPacketCodec::DecodeDungeon(buffer);
    CHECK_FALSE(result.has_value());
    CHECK(result.error() == PacketError::BufferUnderflow);
}

TEST_CASE("WorldPacketCodecTest: DecodeFishing_Success") {
    TPacketGCFishing expectedPacket{};
    expectedPacket.header = GC::FISHING;
    expectedPacket.length = sizeof(TPacketGCFishing);
    expectedPacket.subheader = 2;
    expectedPacket.info = 3;
    expectedPacket.dir = 4;

    std::vector<uint8_t> buffer(sizeof(TPacketGCFishing));
    std::memcpy(buffer.data(), &expectedPacket, sizeof(TPacketGCFishing));

    auto result = WorldPacketCodec::DecodeFishing(buffer);
    REQUIRE(result.has_value());
    CHECK(result.value().header == expectedPacket.header);
    CHECK(result.value().length == expectedPacket.length);
    CHECK(result.value().subheader == expectedPacket.subheader);
    CHECK(result.value().info == expectedPacket.info);
    CHECK(result.value().dir == expectedPacket.dir);
}

TEST_CASE("WorldPacketCodecTest: DecodeFishing_BufferTooSmall") {
    std::vector<uint8_t> buffer(sizeof(TPacketGCFishing) - 1);
    auto result = WorldPacketCodec::DecodeFishing(buffer);
    CHECK_FALSE(result.has_value());
    CHECK(result.error() == PacketError::BufferUnderflow);
}

TEST_CASE("WorldPacketCodecTest: DecodeChannel_Success") {
    TPacketGCChannel expectedPacket{};
    expectedPacket.header = GC::CHANNEL;
    expectedPacket.length = sizeof(TPacketGCChannel);
    expectedPacket.channel = 5;

    std::vector<uint8_t> buffer(sizeof(TPacketGCChannel));
    std::memcpy(buffer.data(), &expectedPacket, sizeof(TPacketGCChannel));

    auto result = WorldPacketCodec::DecodeChannel(buffer);
    REQUIRE(result.has_value());
    CHECK(result.value().header == expectedPacket.header);
    CHECK(result.value().length == expectedPacket.length);
    CHECK(result.value().channel == expectedPacket.channel);
}

TEST_CASE("WorldPacketCodecTest: DecodeChannel_BufferTooSmall") {
    std::vector<uint8_t> buffer(sizeof(TPacketGCChannel) - 1);
    auto result = WorldPacketCodec::DecodeChannel(buffer);
    CHECK_FALSE(result.has_value());
    CHECK(result.error() == PacketError::BufferUnderflow);
}

TEST_CASE("WorldPacketCodecTest: EncodeFishing_Success") {
    int32_t rotation = 20;
    auto buffer = WorldPacketCodec::EncodeFishing(rotation);
    
    REQUIRE(buffer.size() == sizeof(TPacketCGFishing));
    
    TPacketCGFishing decodedPacket;
    std::memcpy(&decodedPacket, buffer.data(), sizeof(TPacketCGFishing));
    
    CHECK(decodedPacket.header == CG::FISHING);
    CHECK(decodedPacket.length == sizeof(TPacketCGFishing));
    CHECK(decodedPacket.dir == rotation / 5);
}
