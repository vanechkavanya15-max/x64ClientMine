#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Skills/SkillTableRegistry.h"

TEST_CASE("SkillTableRegistry: Registration and retrieval") {
    Client::Skills::SkillTableRegistry reg;
    CHECK(reg.RegisterSkill(1, "Aura Miecza", 1, 50));
    auto s = reg.GetSkill(1);
    REQUIRE(s.has_value());
    CHECK(s->name == "Aura Miecza");
    CHECK(s->spCost == 50);
    CHECK(reg.Count() == 1);
}
