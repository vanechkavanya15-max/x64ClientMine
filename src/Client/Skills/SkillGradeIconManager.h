#pragma once
#include <cstdint>
#include <string>

namespace Client::Skills {

enum class SkillGrade : uint8_t { Normal = 0, Master = 1, GrandMaster = 2, PerfectMaster = 3 };

class SkillGradeIconManager {
public:
    SkillGradeIconManager() = default;
    ~SkillGradeIconManager() = default;

    [[nodiscard]] static std::string GetIconPath(uint32_t skillVnum, SkillGrade grade);
};

} // namespace Client::Skills
