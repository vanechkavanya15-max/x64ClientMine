#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include <string>
#include <vector>
#include <iostream>
#include <cstring>
#include "Client/Data/ProtoJSONParserModern.h"

using namespace Client::Data;

TEST_CASE("ProtoJSON - Parse Valid ItemTable") {
    std::string validJSON = R"([
        {
            "dwVnum": 10,
            "dwVnumRange": 0,
            "szName": "Sword",
            "szLocaleName": "Miecz",
            "bType": 1,
            "bSubType": 2,
            "bWeight": 5,
            "bSize": 2,
            "dwAntiFlags": 0,
            "dwFlags": 0,
            "dwWearFlags": 16,
            "dwImmuneFlag": 0,
            "dwIBuyItemPrice": 100,
            "dwISellItemPrice": 50,
            "aLimits": [{"bType": 1, "lValue": 10}],
            "aApplies": [{"bType": 1, "lValue": 100}],
            "alValues": [10, 20, 30, 40, 50, 60],
            "alSockets": [0, 0, 0],
            "dwRefinedVnum": 11,
            "wRefineSet": 1,
            "bAlterToMagicItemPct": 5,
            "bSpecular": 10,
            "bGainSocketPct": 100
        }
    ])";

    auto res = ProtoJSONParserModern::ParseItemTable(validJSON);
    REQUIRE(res.has_value());
    REQUIRE(res->size() == 1);
    
    const auto& item = (*res)[0];
    CHECK(item.dwVnum == 10);
    CHECK(std::string(item.szName) == "Sword");
    CHECK(item.bType == 1);
    CHECK(item.alValues[0] == 10);
    CHECK(item.dwRefinedVnum == 11);
}

TEST_CASE("ProtoJSON - Parse Valid MobTable") {
    std::string validJSON = R"([
        {
            "dwVnum": 101,
            "szName": "Wild Dog",
            "szLocaleName": "Dziki Pies",
            "bType": 0,
            "bRank": 0,
            "bBattleType": 0,
            "bLevel": 1,
            "bSize": 1,
            "dwGoldMin": 10,
            "dwGoldMax": 20,
            "dwExp": 15,
            "dwMaxHP": 100,
            "bRegenCycle": 3,
            "bRegenPercent": 10,
            "wDef": 5,
            "dwAIFlag": 0,
            "dwRaceFlag": 0,
            "dwImmuneFlag": 0,
            "bStr": 5, "bDex": 5, "bCon": 5, "bInt": 5,
            "dwDamageRange": [10, 15],
            "sAttackSpeed": 100,
            "sMovingSpeed": 100,
            "bAggresiveHPPct": 0,
            "wAggressiveSight": 2000,
            "wAttackRange": 150,
            "cEnchants": [0,0,0,0,0,0],
            "cResists": [0,0,0,0,0,0,0,0,0,0,0],
            "dwResurrectionVnum": 0,
            "dwDropItemVnum": 0,
            "bMountCapacity": 0,
            "bOnClickType": 0,
            "bEmpire": 0,
            "szFolder": "wild_dog",
            "fDamMultiply": 1.0,
            "dwSummonVnum": 0,
            "dwDrainSP": 0,
            "dwMonsterColor": 0,
            "dwPolymorphItemVnum": 0,
            "Skills": [],
            "bBerserkPoint": 0,
            "bStoneSkinPoint": 0,
            "bGodSpeedPoint": 0,
            "bDeathBlowPoint": 0,
            "bRevivePoint": 0
        }
    ])";

    auto res = ProtoJSONParserModern::ParseMobTable(validJSON);
    REQUIRE(res.has_value());
    REQUIRE(res->size() == 1);

    const auto& mob = (*res)[0];
    CHECK(mob.dwVnum == 101);
    CHECK(std::string(mob.szName) == "Wild Dog");
    CHECK(mob.bLevel == 1);
    CHECK(mob.dwDamageRange[0] == 10);
    CHECK(mob.dwDamageRange[1] == 15);
}

TEST_CASE("ProtoJSON - Type Mismatch") {
    // bType should be an integer, we provide string
    std::string invalidJSON = R"([
        {
            "dwVnum": 10,
            "dwVnumRange": 0,
            "szName": "Sword",
            "szLocaleName": "Miecz",
            "bType": "not_an_int"
        }
    ])";

    auto res = ProtoJSONParserModern::ParseItemTable(invalidJSON);
    REQUIRE_FALSE(res.has_value());
    CHECK(res.error() == ProtoParseError::TypeMismatch);
}

TEST_CASE("ProtoJSON - Missing Field") {
    // Missing dwVnumRange
    std::string invalidJSON = R"([
        {
            "dwVnum": 10,
            "szName": "Sword"
        }
    ])";

    auto res = ProtoJSONParserModern::ParseItemTable(invalidJSON);
    REQUIRE_FALSE(res.has_value());
    CHECK(res.error() == ProtoParseError::MissingField);
}

TEST_CASE("ProtoJSON - Round Trip Export ItemTable") {
    CItemData::TItemTable item = {};
    item.dwVnum = 12345;
    std::strncpy(item.szName, "TestItem", ITEM_NAME_MAX_LEN);
    item.bType = 2;
    item.alValues[0] = 999;
    
    std::vector<CItemData::TItemTable> vec = { item };

    auto exportedResult = ProtoJSONParserModern::ExportItemTableToJSON(vec);
    REQUIRE(exportedResult.has_value());
    
    auto parsedResult = ProtoJSONParserModern::ParseItemTable(exportedResult.value());
    REQUIRE(parsedResult.has_value());
    REQUIRE(parsedResult->size() == 1);

    const auto& parsedItem = (*parsedResult)[0];
    CHECK(parsedItem.dwVnum == 12345);
    CHECK(std::string(parsedItem.szName) == "TestItem");
    CHECK(parsedItem.bType == 2);
    CHECK(parsedItem.alValues[0] == 999);
}
