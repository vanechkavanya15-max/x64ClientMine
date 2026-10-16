#pragma once

#include <string>
#include <vector>
#include <string_view>
#include <cstdint>
#include <expected>
#include "../../EterBase/Result.h"
#include "../Core/StrongTypes.h"
#include "../Core/IVfsProvider.h"

namespace Client::Skills {

struct SkillTableRecord
{
    Client::Core::SkillId dwVnum;
    std::string strName;
    uint8_t bType;
    uint8_t bLevelStep;
    uint8_t bMaxLevel;
    uint8_t bLevelLimit;
    int32_t iPointOn;
    std::string strPointPoly;
    std::string strSPCostPoly;
    std::string strDurationPoly;
    std::string strDurationSPCostPoly;
    std::string strCooldownPoly;
    std::string strMasterBonusPoly;
    std::string strAttackGradePoly;
    uint32_t dwFlag;
    uint32_t dwAffectFlag;
    int32_t iPointOn2;
    std::string strPointPoly2;
    std::string strDurationPoly2;
    uint32_t dwAffectFlag2;
    Client::Core::SkillId dwPrerequisiteSkillVnum;
    uint8_t bPrerequisiteSkillLevel;
    uint8_t bSkillType;
    uint8_t bMaxHit;
    std::string strSplashAroundDamageAdjustPoly;
    uint32_t dwTargetRange;
    uint32_t dwSplashRange;
};

class SkillTableLoader
{
public:
    static Client::Core::VfsResult<std::vector<SkillTableRecord>> LoadFromFile(Client::Core::IVfsProvider& vfs, std::string_view filePath);
    static EterBase::Result<std::vector<SkillTableRecord>> LoadFromMemory(std::string_view tsvData);
};

} // namespace Client::Skills
