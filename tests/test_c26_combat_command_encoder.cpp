#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/Network/CombatCommandEncoder.h"
#include "Client/Network/Protocol/Protocol.h"
#include "Client/Core/DomainCommands.h"

TEST_CASE("CombatCommandEncoder - EncodeAttackCommand Ymir Framing (1B)") {
    Client::Core::AttackCommand cmd;
    cmd.targetVid = Client::Core::EntityVid(12345);
    cmd.attackType = 2;

    auto result = Client::Network::CombatCommandEncoder::EncodeAttackCommand(cmd, true);
    REQUIRE(result.has_value());
    const auto& buffer = *result;

    CHECK(buffer.size() == 8);
    CHECK(buffer[0] == static_cast<uint8_t>(CG::ATTACK & 0xFF)); 
    CHECK(buffer[1] == 2);    
    
    uint32_t decodedVid = buffer[2] | (buffer[3] << 8) | (buffer[4] << 16) | (buffer[5] << 24);
    CHECK(decodedVid == 12345);
}

TEST_CASE("CombatCommandEncoder - EncodeAttackCommand m2dev Framing (4B)") {
    Client::Core::AttackCommand cmd;
    cmd.targetVid = Client::Core::EntityVid(54321);
    cmd.attackType = 1; 

    auto result = Client::Network::CombatCommandEncoder::EncodeAttackCommand(cmd, false);
    REQUIRE(result.has_value());
    const auto& buffer = *result;

    CHECK(buffer.size() == sizeof(TPacketCGAttack));
    
    const TPacketCGAttack* decoded = reinterpret_cast<const TPacketCGAttack*>(buffer.data());
    CHECK(decoded->header == CG::ATTACK);
    CHECK(decoded->length == sizeof(TPacketCGAttack));
    CHECK(decoded->bType == 1);
    CHECK(decoded->dwVictimVID == 54321);
}

TEST_CASE("CombatCommandEncoder - EncodeShootCommand Ymir Framing (1B)") {
    Client::Core::ShootCommand cmd;
    cmd.targetVid = Client::Core::EntityVid(999);
    cmd.skillVnum = 15; 

    auto result = Client::Network::CombatCommandEncoder::EncodeShootCommand(cmd, true);
    REQUIRE(result.has_value());
    const auto& buffer = *result;

    CHECK(buffer.size() == 2);
    CHECK(buffer[0] == static_cast<uint8_t>(CG::SHOOT & 0xFF));
    CHECK(buffer[1] == 15);
}

TEST_CASE("CombatCommandEncoder - EncodeShootCommand m2dev Framing (4B)") {
    Client::Core::ShootCommand cmd;
    cmd.targetVid = Client::Core::EntityVid(888);
    cmd.skillVnum = 12;

    auto result = Client::Network::CombatCommandEncoder::EncodeShootCommand(cmd, false);
    REQUIRE(result.has_value());
    const auto& buffer = *result;

    CHECK(buffer.size() == sizeof(TPacketCGShoot));
    
    const TPacketCGShoot* decoded = reinterpret_cast<const TPacketCGShoot*>(buffer.data());
    CHECK(decoded->header == CG::SHOOT);
    CHECK(decoded->length == sizeof(TPacketCGShoot));
    CHECK(decoded->bType == 12);
}
