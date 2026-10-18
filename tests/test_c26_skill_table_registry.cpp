#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Skills/SkillTableRegistry.h"
#include <vector>

TEST_CASE("SkillTableRegistry: Registration, retrieval and type mapping") {
    Client::Skills::SkillTableRegistry reg;
    
    std::vector<Client::Skills::SkillTableRecord> records;
    
    Client::Skills::SkillTableRecord activeSkill{};
    activeSkill.dwVnum = Client::Core::SkillId(1);
    activeSkill.strName = "Aura Miecza";
    activeSkill.bType = 1; // Active
    records.push_back(activeSkill);
    
    Client::Skills::SkillTableRecord passiveSkill{};
    passiveSkill.dwVnum = Client::Core::SkillId(2);
    passiveSkill.strName = "Gornictwo";
    passiveSkill.bType = 2; // Passive
    records.push_back(passiveSkill);
    
    Client::Skills::SkillTableRecord guildSkill{};
    guildSkill.dwVnum = Client::Core::SkillId(151);
    guildSkill.strName = "Smocza Krew";
    guildSkill.bType = 3; // Guild
    records.push_back(guildSkill);
    
    Client::Skills::SkillTableRecord horseSkill{};
    horseSkill.dwVnum = Client::Core::SkillId(137);
    horseSkill.strName = "Ciecie z Siodla";
    horseSkill.bType = 4; // Horse
    records.push_back(horseSkill);
    
    Client::Skills::SkillTableRecord unknownSkill{};
    unknownSkill.dwVnum = Client::Core::SkillId(999);
    unknownSkill.strName = "Nieznany Typ";
    unknownSkill.bType = 99; // Unknown
    records.push_back(unknownSkill);
    
    reg.Initialize(records);
    
    REQUIRE(reg.Count() == 5);

    SUBCASE("GetSkill returns correct values for existing skills") {
        auto s = reg.GetSkill(Client::Core::SkillId(1));
        REQUIRE(s.has_value());
        CHECK(s->strName == "Aura Miecza");
        CHECK(s->bType == 1);
    }
    
    SUBCASE("GetSkill returns nullopt for non-existent skill") {
        auto s = reg.GetSkill(Client::Core::SkillId(9999));
        CHECK_FALSE(s.has_value());
    }
    
    SUBCASE("GetSkillType returns correct mapped types") {
        auto t1 = reg.GetSkillType(Client::Core::SkillId(1));
        REQUIRE(t1.has_value());
        CHECK(t1.value() == Client::Skills::SkillType::Active);
        
        auto t2 = reg.GetSkillType(Client::Core::SkillId(2));
        REQUIRE(t2.has_value());
        CHECK(t2.value() == Client::Skills::SkillType::Passive);
        
        auto t3 = reg.GetSkillType(Client::Core::SkillId(151));
        REQUIRE(t3.has_value());
        CHECK(t3.value() == Client::Skills::SkillType::Guild);
        
        auto t4 = reg.GetSkillType(Client::Core::SkillId(137));
        REQUIRE(t4.has_value());
        CHECK(t4.value() == Client::Skills::SkillType::Horse);
    }
    
    SUBCASE("GetSkillType falls back to None for unknown bType values") {
        auto t999 = reg.GetSkillType(Client::Core::SkillId(999));
        REQUIRE(t999.has_value());
        CHECK(t999.value() == Client::Skills::SkillType::None);
    }
    
    SUBCASE("GetSkillType returns nullopt for non-existent skill") {
        auto tNonExistent = reg.GetSkillType(Client::Core::SkillId(9999));
        CHECK_FALSE(tNonExistent.has_value());
    }
}
