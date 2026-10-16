#include "SkillMotionManager.h"
#include <algorithm>

namespace Client::Skills {

[[nodiscard]] std::string_view to_string(SkillMotionError error) noexcept {
    switch (error) {
        case SkillMotionError::None: return "SkillMotionError::None";
        case SkillMotionError::SkillNotFound: return "SkillMotionError::SkillNotFound";
        case SkillMotionError::MotionNotFound: return "SkillMotionError::MotionNotFound";
        case SkillMotionError::InvalidGrade: return "SkillMotionError::InvalidGrade";
        default: return "SkillMotionError::Unknown";
    }
}

Core::Result<void, SkillMotionError> SkillMotionManager::RegisterMotion(Core::SkillIndex skillId, SkillGrade grade, MotionIndex motionIndex) {
    auto it = std::ranges::lower_bound(motions_, std::pair{skillId.value(), grade.value()}, std::less<>{},
        [](const SkillMotionData& d) { return std::pair{d.skillId.value(), d.grade.value()}; });

    if (it != motions_.end() && it->skillId == skillId && it->grade == grade) {
        it->motionIndex = motionIndex;
    } else {
        motions_.insert(it, SkillMotionData{skillId, grade, motionIndex});
    }

    return {};
}

[[nodiscard]] Core::Result<MotionIndex, SkillMotionError> SkillMotionManager::GetSkillMotionIndex(Core::SkillIndex skillId, SkillGrade grade) const {
    auto itSpecific = std::ranges::lower_bound(motions_, std::pair{skillId.value(), grade.value()}, std::less<>{},
        [](const SkillMotionData& d) { return std::pair{d.skillId.value(), d.grade.value()}; });

    if (itSpecific != motions_.end() && itSpecific->skillId == skillId && itSpecific->grade == grade) {
        return itSpecific->motionIndex;
    }

    // Fallback logic
    if (grade.value() != -1) {
        auto itFallback = std::ranges::lower_bound(motions_, std::pair{skillId.value(), -1}, std::less<>{},
            [](const SkillMotionData& d) { return std::pair{d.skillId.value(), d.grade.value()}; });
            
        if (itFallback != motions_.end() && itFallback->skillId == skillId && itFallback->grade.value() == -1) {
            return itFallback->motionIndex;
        }
    }
    
    // Check if skill exists at all
    auto itSkill = std::ranges::lower_bound(motions_, skillId.value(), std::less<>{},
        [](const SkillMotionData& d) { return d.skillId.value(); });
        
    if (itSkill != motions_.end() && itSkill->skillId == skillId) {
        return std::unexpected(SkillMotionError::MotionNotFound);
    }

    return std::unexpected(SkillMotionError::SkillNotFound);
}

void SkillMotionManager::ClearSkillMotions(Core::SkillIndex skillId) {
    std::erase_if(motions_, [skillId](const SkillMotionData& data) {
        return data.skillId == skillId;
    });
}

} // namespace Client::Skills
