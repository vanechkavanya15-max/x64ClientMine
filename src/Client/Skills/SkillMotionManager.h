#pragma once

#include "Client/Core/StrongTypes.h"
#include "Client/Core/Result.h"
#include <vector>
#include <cstdint>
#include <string_view>
#include <format>

namespace Client::Skills {

struct SkillGradeTag {};
using SkillGrade = Core::StrongType<SkillGradeTag, int32_t, -1>;

struct MotionIndexTag {};
using MotionIndex = Core::StrongType<MotionIndexTag, uint32_t, 0>;

enum class SkillMotionError : uint8_t {
    None = 0,
    SkillNotFound,
    MotionNotFound,
    InvalidGrade
};

[[nodiscard]] std::string_view to_string(SkillMotionError error) noexcept;

struct SkillMotionData {
    Core::SkillIndex skillId;
    SkillGrade grade;
    MotionIndex motionIndex;

    bool operator==(const SkillMotionData& other) const = default;
    auto operator<=>(const SkillMotionData& other) const = default;
};

class SkillMotionManager {
public:
    SkillMotionManager() = default;
    ~SkillMotionManager() = default;

    Core::Result<void, SkillMotionError> RegisterMotion(Core::SkillIndex skillId, SkillGrade grade, MotionIndex motionIndex);
    
    [[nodiscard]] Core::Result<MotionIndex, SkillMotionError> GetSkillMotionIndex(Core::SkillIndex skillId, SkillGrade grade = SkillGrade(-1)) const;
    
    void ClearSkillMotions(Core::SkillIndex skillId);

private:
    std::vector<SkillMotionData> motions_;
};

} // namespace Client::Skills

template <>
struct std::formatter<Client::Skills::SkillMotionError> : std::formatter<std::string_view> {
    auto format(Client::Skills::SkillMotionError err, std::format_context& ctx) const {
        return std::formatter<std::string_view>::format(Client::Skills::to_string(err), ctx);
    }
};

