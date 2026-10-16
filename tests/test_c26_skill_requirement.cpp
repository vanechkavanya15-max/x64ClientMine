#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Skills/SkillRequirementEvaluator.h"

TEST_CASE("SkillRequirementEvaluator: Cast validation") {
    using namespace Client::Skills;
    SkillUsageContext ctx{100, 50, 10.0f, 50.0f, true, false};
    CHECK(SkillRequirementEvaluator::CanCast(ctx));

    ctx.currentSP = 30; // Not enough SP
    CHECK_FALSE(SkillRequirementEvaluator::CanCast(ctx));
    ctx.currentSP = 100;

    ctx.isCooling = true; // On cooldown
    CHECK_FALSE(SkillRequirementEvaluator::CanCast(ctx));
}
