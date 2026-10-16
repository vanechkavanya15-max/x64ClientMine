#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest/doctest.h>
#include "Client/Player/PlayerPartyManager.h"
#include "Client/Gameplay/SocialDomain.h"
#include "Client/Network/Handlers/PartyPacketDomainHandler.h"

using namespace Client::Player;
using namespace Client::Gameplay;
using namespace Client::Network::Handlers;

TEST_SUITE("C26 PlayerPartyManager") {
    TEST_CASE("InviteToParty - Success") {
        auto socialManager = std::make_shared<SocialManager>();
        PlayerPartyManager manager(socialManager);

        auto result = manager.InviteToParty(EterBase::EntityId{1001});
        REQUIRE(result.has_value());
    }

    TEST_CASE("AcceptInvite - Fails if already in party") {
        auto socialManager = std::make_shared<SocialManager>();
        socialManager->CreateParty();
        PlayerPartyManager manager(socialManager);

        auto result = manager.AcceptInvite(EterBase::EntityId{1002});
        REQUIRE_FALSE(result.has_value());
        REQUIRE(result.error() == PartyError::AlreadyInParty);
    }

    TEST_CASE("AcceptInvite - Success if not in party") {
        auto socialManager = std::make_shared<SocialManager>();
        PlayerPartyManager manager(socialManager);

        auto result = manager.AcceptInvite(EterBase::EntityId{1002});
        REQUIRE(result.has_value());
    }

    TEST_CASE("DeclineInvite - Fails if already in party") {
        auto socialManager = std::make_shared<SocialManager>();
        socialManager->CreateParty();
        PlayerPartyManager manager(socialManager);

        auto result = manager.DeclineInvite(EterBase::EntityId{1002});
        REQUIRE_FALSE(result.has_value());
        REQUIRE(result.error() == PartyError::AlreadyInParty);
    }

    TEST_CASE("DeclineInvite - Success if not in party") {
        auto socialManager = std::make_shared<SocialManager>();
        PlayerPartyManager manager(socialManager);

        auto result = manager.DeclineInvite(EterBase::EntityId{1002});
        REQUIRE(result.has_value());
    }

    TEST_CASE("LeaveParty - Fails if not in party") {
        auto socialManager = std::make_shared<SocialManager>();
        PlayerPartyManager manager(socialManager);

        auto result = manager.LeaveParty();
        REQUIRE_FALSE(result.has_value());
        REQUIRE(result.error() == PartyError::NotInParty);
    }

    TEST_CASE("LeaveParty - Success if in party") {
        auto socialManager = std::make_shared<SocialManager>();
        socialManager->CreateParty();
        PlayerPartyManager manager(socialManager);

        auto result = manager.LeaveParty();
        REQUIRE(result.has_value());
    }

    TEST_CASE("RemoveMember - Fails if not in party") {
        auto socialManager = std::make_shared<SocialManager>();
        PlayerPartyManager manager(socialManager);

        auto result = manager.RemoveMember(EterBase::EntityId{2001});
        REQUIRE_FALSE(result.has_value());
        REQUIRE(result.error() == PartyError::NotInParty);
    }

    TEST_CASE("RemoveMember - Success if in party") {
        auto socialManager = std::make_shared<SocialManager>();
        socialManager->CreateParty();
        PlayerPartyManager manager(socialManager);

        auto result = manager.RemoveMember(EterBase::EntityId{2001});
        REQUIRE(result.has_value());
    }

    TEST_CASE("SetDistributeMode - Fails if not in party") {
        auto socialManager = std::make_shared<SocialManager>();
        PlayerPartyManager manager(socialManager);

        auto result = manager.SetDistributeMode(1);
        REQUIRE_FALSE(result.has_value());
        REQUIRE(result.error() == PartyError::NotInParty);
    }

    TEST_CASE("SetDistributeMode - Success if in party") {
        auto socialManager = std::make_shared<SocialManager>();
        socialManager->CreateParty();
        PlayerPartyManager manager(socialManager);

        auto result = manager.SetDistributeMode(1);
        REQUIRE(result.has_value());
    }
}
