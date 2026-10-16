#include <doctest/doctest.h>
#include "Client/Skills/SkillMotionManager.h"
#include "Client/Core/Result.h"

using namespace Client::Skills;
using namespace Client::Core;

TEST_SUITE("SkillMotionManager") {
    TEST_CASE("Registration and Retrieval") {
        SkillMotionManager manager;
        SkillIndex skillId{1};
        SkillGrade grade{1};
        MotionIndex motionIdx{100};

        auto res = manager.RegisterMotion(skillId, grade, motionIdx);
        REQUIRE(res.has_value());

        auto motionRes = manager.GetSkillMotionIndex(skillId, grade);
        REQUIRE(motionRes.has_value());
        bool matches = motionRes.value() == motionIdx;
        CHECK(matches);
    }

    TEST_CASE("Update Existing Motion") {
        SkillMotionManager manager;
        SkillIndex skillId{2};
        SkillGrade grade{0};
        MotionIndex initialMotion{200};
        MotionIndex updatedMotion{201};

        manager.RegisterMotion(skillId, grade, initialMotion);
        manager.RegisterMotion(skillId, grade, updatedMotion);

        auto motionRes = manager.GetSkillMotionIndex(skillId, grade);
        REQUIRE(motionRes.has_value());
        bool matches = motionRes.value() == updatedMotion;
        CHECK(matches);
    }

    TEST_CASE("Fallback to Base Motion (Grade -1)") {
        SkillMotionManager manager;
        SkillIndex skillId{3};
        SkillGrade baseGrade{-1};
        SkillGrade requestedGrade{5};
        MotionIndex baseMotion{300};

        manager.RegisterMotion(skillId, baseGrade, baseMotion);

        // Should fallback to grade -1
        auto motionRes = manager.GetSkillMotionIndex(skillId, requestedGrade);
        REQUIRE(motionRes.has_value());
        bool matches = motionRes.value() == baseMotion;
        CHECK(matches);
    }

    TEST_CASE("Fallback Fails Without Base Motion") {
        SkillMotionManager manager;
        SkillIndex skillId{4};
        SkillGrade registeredGrade{1};
        SkillGrade requestedGrade{2};
        MotionIndex motion{400};

        manager.RegisterMotion(skillId, registeredGrade, motion);

        // Doesn't have base motion (-1) and grade 2 is not registered
        auto motionRes = manager.GetSkillMotionIndex(skillId, requestedGrade);
        REQUIRE(!motionRes.has_value());
        bool errorMatches = motionRes.error() == SkillMotionError::MotionNotFound;
        CHECK(errorMatches);
    }

    TEST_CASE("Skill Not Found") {
        SkillMotionManager manager;
        SkillIndex registeredSkillId{5};
        SkillIndex missingSkillId{6};
        SkillGrade grade{0};
        MotionIndex motion{500};

        manager.RegisterMotion(registeredSkillId, grade, motion);

        auto motionRes = manager.GetSkillMotionIndex(missingSkillId, grade);
        REQUIRE(!motionRes.has_value());
        bool errorMatches = motionRes.error() == SkillMotionError::SkillNotFound;
        CHECK(errorMatches);
    }

    TEST_CASE("Clear Skill Motions") {
        SkillMotionManager manager;
        SkillIndex skillId{7};
        SkillGrade grade1{0};
        SkillGrade grade2{1};
        MotionIndex motion1{700};
        MotionIndex motion2{701};

        manager.RegisterMotion(skillId, grade1, motion1);
        manager.RegisterMotion(skillId, grade2, motion2);

        manager.ClearSkillMotions(skillId);

        auto motionRes1 = manager.GetSkillMotionIndex(skillId, grade1);
        REQUIRE(!motionRes1.has_value());
        bool errorMatches = motionRes1.error() == SkillMotionError::SkillNotFound;
        CHECK(errorMatches);
    }
}
