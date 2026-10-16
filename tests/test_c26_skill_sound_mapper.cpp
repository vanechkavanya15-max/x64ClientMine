#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include "Client/Skills/SkillSoundMapper.h"
#include "Client/Core/StrongTypes.h"
#include "Client/Core/DomainErrors.h"
#include "Client/Core/Result.h"

using namespace Client::Skills;
using namespace Client::Core;

TEST_SUITE("SkillSoundMapper") {

    TEST_CASE("Register and Get Sound Path") {
        SkillSoundMapper mapper;
        SkillIndex skillId(1);
        std::string soundPath = "sound/skill/warrior/slash.wav";

        auto registerResult = mapper.RegisterSound(skillId, soundPath);
        CHECK(registerResult.has_value() == true);
        CHECK(mapper.GetMappedCount() == 1);

        auto getResult = mapper.GetSoundPath(skillId);
        CHECK(getResult.has_value() == true);
        CHECK(getResult.value() == soundPath);
    }

    TEST_CASE("Register empty sound path fails") {
        SkillSoundMapper mapper;
        SkillIndex skillId(2);

        auto registerResult = mapper.RegisterSound(skillId, "");
        CHECK(registerResult.has_value() == false);
        CHECK(registerResult.error() == SkillError::RequirementNotMet);
        CHECK(mapper.GetMappedCount() == 0);
    }

    TEST_CASE("Get non-existent sound path fails") {
        SkillSoundMapper mapper;
        SkillIndex skillId(3);

        auto getResult = mapper.GetSoundPath(skillId);
        CHECK(getResult.has_value() == false);
        CHECK(getResult.error() == SkillError::SkillNotFound);
    }

    TEST_CASE("Unregister Sound") {
        SkillSoundMapper mapper;
        SkillIndex skillId(4);
        std::string soundPath = "sound/skill/sura/flame.wav";

        mapper.RegisterSound(skillId, soundPath);
        CHECK(mapper.GetMappedCount() == 1);

        auto unregisterResult = mapper.UnregisterSound(skillId);
        CHECK(unregisterResult.has_value() == true);
        CHECK(mapper.GetMappedCount() == 0);

        auto getResult = mapper.GetSoundPath(skillId);
        CHECK(getResult.has_value() == false);
        CHECK(getResult.error() == SkillError::SkillNotFound);
    }

    TEST_CASE("Unregister non-existent sound fails") {
        SkillSoundMapper mapper;
        SkillIndex skillId(5);

        auto unregisterResult = mapper.UnregisterSound(skillId);
        CHECK(unregisterResult.has_value() == false);
        CHECK(unregisterResult.error() == SkillError::SkillNotFound);
    }

    TEST_CASE("Clear removes all registered sounds") {
        SkillSoundMapper mapper;
        SkillIndex skillId1(10);
        SkillIndex skillId2(20);

        mapper.RegisterSound(skillId1, "sound1.wav");
        mapper.RegisterSound(skillId2, "sound2.wav");
        CHECK(mapper.GetMappedCount() == 2);

        mapper.Clear();
        CHECK(mapper.GetMappedCount() == 0);

        auto getResult1 = mapper.GetSoundPath(skillId1);
        CHECK(getResult1.has_value() == false);
        auto getResult2 = mapper.GetSoundPath(skillId2);
        CHECK(getResult2.has_value() == false);
    }
}
