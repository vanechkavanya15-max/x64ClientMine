#pragma once
#include <string>

namespace Client::Skills {

class SkillDescriptionParser {
public:
    SkillDescriptionParser() = default;
    ~SkillDescriptionParser() = default;

    [[nodiscard]] static std::string FormatFormula(const std::string& rawDesc, float skillLevel, float playerInt);
};

} // namespace Client::Skills
