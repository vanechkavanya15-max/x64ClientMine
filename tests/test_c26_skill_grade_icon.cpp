#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Skills/SkillGradeIconManager.h"
#include <memory>

using namespace Client::Skills;
using namespace Client::Core;

TEST_CASE("SkillGradeIconManager: Podstawowe generowanie sciezek i cachowanie") {
    auto generator = std::make_unique<DefaultSkillPathGenerator>();
    auto cache = std::make_unique<DefaultSkillIconCache>();
    SkillGradeIconManager manager(std::move(generator), std::move(cache));

    SUBCASE("Normal grade") {
        auto result = manager.ResolveIconPath(SkillIndex(1), SkillGrade::Normal);
        REQUIRE(result.has_value());
        CHECK(result.value() == "d:/ymir work/ui/skill/warrior/1.sub");
    }

    SUBCASE("Master grade") {
        auto result = manager.ResolveIconPath(SkillIndex(1), SkillGrade::Master);
        REQUIRE(result.has_value());
        CHECK(result.value() == "d:/ymir work/ui/skill/warrior/1_m.sub");
    }
    
    SUBCASE("GrandMaster grade") {
        auto result = manager.ResolveIconPath(SkillIndex(1), SkillGrade::GrandMaster);
        REQUIRE(result.has_value());
        CHECK(result.value() == "d:/ymir work/ui/skill/warrior/1_g.sub");
    }

    SUBCASE("PerfectMaster grade") {
        auto result = manager.ResolveIconPath(SkillIndex(1), SkillGrade::PerfectMaster);
        REQUIRE(result.has_value());
        CHECK(result.value() == "d:/ymir work/ui/skill/warrior/1_p.sub");
    }
    
    SUBCASE("Sprawdzenie czy po wygenerowaniu sciezka trafia do cache") {
        auto res1 = manager.ResolveIconPath(SkillIndex(5), SkillGrade::Master);
        REQUIRE(res1.has_value());
        CHECK(res1.value() == "d:/ymir work/ui/skill/warrior/5_m.sub");
        
        // Powinno odczytac z cache'u, wynik ten sam
        auto res2 = manager.ResolveIconPath(SkillIndex(5), SkillGrade::Master);
        REQUIRE(res2.has_value());
        CHECK(res2.value() == "d:/ymir work/ui/skill/warrior/5_m.sub");
    }
}

TEST_CASE("SkillGradeIconManager: Przypadki brzegowe i bledy") {
    auto generator = std::make_unique<DefaultSkillPathGenerator>();
    auto cache = std::make_unique<DefaultSkillIconCache>();
    SkillGradeIconManager manager(std::move(generator), std::move(cache));

    SUBCASE("Invalid VNUM (0)") {
        auto result = manager.ResolveIconPath(SkillIndex(0), SkillGrade::Normal);
        REQUIRE_FALSE(result.has_value());
        CHECK(result.error() == SkillIconError::InvalidVnum);
    }
    
    SUBCASE("Invalid Grade (cast)") {
        SkillGrade invalidGrade = static_cast<SkillGrade>(99);
        auto result = manager.ResolveIconPath(SkillIndex(10), invalidGrade);
        REQUIRE_FALSE(result.has_value());
        CHECK(result.error() == SkillIconError::InvalidGrade);
    }
}

TEST_CASE("SkillGradeIconManager: Manualne nadpisania (Overrides)") {
    auto generator = std::make_unique<DefaultSkillPathGenerator>();
    auto cache = std::make_unique<DefaultSkillIconCache>();
    SkillGradeIconManager manager(std::move(generator), std::move(cache));

    SUBCASE("Dodanie poprawnego nadpisania") {
        auto regResult = manager.RegisterOverride(SkillIndex(100), SkillGrade::PerfectMaster, "custom/path/icon.sub");
        REQUIRE(regResult.has_value());
        
        auto res = manager.ResolveIconPath(SkillIndex(100), SkillGrade::PerfectMaster);
        REQUIRE(res.has_value());
        CHECK(res.value() == "custom/path/icon.sub");
    }
    
    SUBCASE("Proba nadpisania pustym stringiem zwraca blad") {
        auto regResult = manager.RegisterOverride(SkillIndex(20), SkillGrade::Normal, "");
        REQUIRE_FALSE(regResult.has_value());
        CHECK(regResult.error() == SkillIconError::InvalidArgument);
    }
}

TEST_CASE("SkillGradeIconManager: Czyszczenie Cache") {
    auto generator = std::make_unique<DefaultSkillPathGenerator>();
    auto cache = std::make_unique<DefaultSkillIconCache>();
    SkillGradeIconManager manager(std::move(generator), std::move(cache));
    
    (void)manager.RegisterOverride(SkillIndex(999), SkillGrade::Normal, "test.sub");
    auto res1 = manager.ResolveIconPath(SkillIndex(1), SkillGrade::Normal);
    REQUIRE(res1.has_value());
    
    manager.ClearAllCaches();
    
    // Po usunieciu cache i overrides, override znika i wracamy do domyslnego generatora
    auto res2 = manager.ResolveIconPath(SkillIndex(999), SkillGrade::Normal);
    REQUIRE(res2.has_value());
    CHECK(res2.value() == "d:/ymir work/ui/skill/warrior/999.sub");
}
