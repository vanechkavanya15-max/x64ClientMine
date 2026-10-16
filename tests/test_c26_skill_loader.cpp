#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/Skills/SkillTableLoader.h"
#include "Client/Core/IVfsProvider.h"
#include <string>

using namespace Client::Skills;

class MockVfsProvider : public Client::Core::IVfsProvider {
public:
    Client::Core::VfsResult<std::vector<uint8_t>> ReadFile(std::string_view path) override {
        if (path == "locale/pl/skilltable.txt") {
            std::string tsvData = "VNUM\tNAME\n100\t\"MockSkill\"\n";
            return std::vector<uint8_t>(tsvData.begin(), tsvData.end());
        }
        return std::unexpected(Client::Core::VfsError::FileNotFound);
    }

    Client::Core::VfsResult<std::span<const uint8_t>> GetCachedFile(std::string_view path) override {
        return std::unexpected(Client::Core::VfsError::FileNotFound);
    }

    bool Exists(std::string_view path) const override {
        return path == "locale/pl/skilltable.txt";
    }
};

TEST_CASE("SkillTableLoader - Parse valid TSV record")
{
    std::string tsvData = "VNUM\tNAME\tTYPE\tLEVEL_STEP\tMAX_LEVEL\tLEVEL_LIMIT\tPOINT_ON\tPOINT_POLY\tSP_COST_POLY\tDURATION_POLY\tDURATION_SP_COST_POLY\tCOOLDOWN_POLY\tMASTER_BONUS_POLY\tATTACK_GRADE_POLY\tFLAG\tAFFECT_FLAG\tPOINT_ON2\tPOINT_POLY2\tDURATION_POLY2\tAFFECT_FLAG2\tPREREQUISITE_SKILL_VNUM\tPREREQUISITE_SKILL_LEVEL\tSKILL_TYPE\tMAX_HIT\tSPLASH_AROUND_DAMAGE_ADJUST_POLY\tTARGET_RANGE\tSPLASH_RANGE\n"
                          "1\t\"SkillName\"\t1\t1\t20\t0\t10\t10+20*k\t10\t0\t0\t10\t0\t0\t0\t0\t0\t0\t0\t0\t0\t0\t0\t1\t0\t1000\t0\n";
                          
    auto result = SkillTableLoader::LoadFromMemory(tsvData);
    REQUIRE(result.has_value());
    REQUIRE(result->size() == 1);
    
    const auto& record = result->at(0);
    CHECK(record.dwVnum == 1);
    CHECK(record.strName == "SkillName"); // quotes should be unquoted
    CHECK(record.bType == 1);
    CHECK(record.bLevelStep == 1);
    CHECK(record.bMaxLevel == 20);
    CHECK(record.bLevelLimit == 0);
    CHECK(record.iPointOn == 10);
    CHECK(record.strPointPoly == "10+20*k");
    CHECK(record.strSPCostPoly == "10");
    CHECK(record.dwTargetRange == 1000);
}

TEST_CASE("SkillTableLoader - Parse empty and malformed data")
{
    // Empty data
    auto resEmpty = SkillTableLoader::LoadFromMemory("");
    REQUIRE(resEmpty.has_value());
    CHECK(resEmpty->empty());

    // Only header
    auto resHeader = SkillTableLoader::LoadFromMemory("VNUM\tNAME\n");
    REQUIRE(resHeader.has_value());
    CHECK(resHeader->empty());

    // Trailing \r
    std::string tsvCRLF = "VNUM\r\n2\r\n";
    auto resCRLF = SkillTableLoader::LoadFromMemory(tsvCRLF);
    REQUIRE(resCRLF.has_value());
    REQUIRE(resCRLF->size() == 1);
    CHECK(resCRLF->at(0).dwVnum == 2);
}

TEST_CASE("SkillTableLoader - Missing columns")
{
    // Partial columns
    std::string tsvPartial = "VNUM\tNAME\n3\tPartialSkill";
    auto resPartial = SkillTableLoader::LoadFromMemory(tsvPartial);
    REQUIRE(resPartial.has_value());
    REQUIRE(resPartial->size() == 1);
    CHECK(resPartial->at(0).dwVnum == 3);
    CHECK(resPartial->at(0).strName == "PartialSkill");
    CHECK(resPartial->at(0).bType == 0); // Defaults to 0
    CHECK(resPartial->at(0).strPointPoly == ""); // Defaults to empty
}

TEST_CASE("SkillTableLoader - Skip empty rows")
{
    std::string tsvEmptyRow = "VNUM\n4\n\n5\n";
    auto resEmptyRow = SkillTableLoader::LoadFromMemory(tsvEmptyRow);
    REQUIRE(resEmptyRow.has_value());
    REQUIRE(resEmptyRow->size() == 2);
    CHECK(resEmptyRow->at(0).dwVnum == 4);
    CHECK(resEmptyRow->at(1).dwVnum == 5);
}

TEST_CASE("SkillTableLoader - LoadFromFile success")
{
    MockVfsProvider mockVfs;
    auto result = SkillTableLoader::LoadFromFile(mockVfs, "locale/pl/skilltable.txt");
    REQUIRE(result.has_value());
    REQUIRE(result->size() == 1);
    CHECK(result->at(0).dwVnum == 100);
    CHECK(result->at(0).strName == "MockSkill");
}

TEST_CASE("SkillTableLoader - LoadFromFile not found")
{
    MockVfsProvider mockVfs;
    auto result = SkillTableLoader::LoadFromFile(mockVfs, "missing.txt");
    REQUIRE(!result.has_value());
    CHECK(result.error() == Client::Core::VfsError::FileNotFound);
}
