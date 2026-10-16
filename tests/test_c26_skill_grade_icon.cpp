#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Skills/SkillGradeIconManager.h"

TEST_CASE("SkillGradeIconManager: Icon path generation") {
    using namespace Client::Skills;
    CHECK(SkillGradeIconManager::GetIconPath(1, SkillGrade::Normal) == "d:/ymir work/ui/skill/warrior/1.sub");
    CHECK(SkillGradeIconManager::GetIconPath(1, SkillGrade::Master) == "d:/ymir work/ui/skill/warrior/1_m.sub");
    CHECK(SkillGradeIconManager::GetIconPath(1, SkillGrade::PerfectMaster) == "d:/ymir work/ui/skill/warrior/1_p.sub");
}
