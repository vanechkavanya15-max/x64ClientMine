#include "SkillDescriptionParser.h"
#include <regex>

namespace Client::Skills {

std::string SkillDescriptionParser::FormatFormula(const std::string& rawDesc, float skillLevel, float playerInt) {
    std::string res = rawDesc;
    int val = static_cast<int>(skillLevel * 10.0f + playerInt * 1.5f);
    // Zastap placeholder {VAL} obliczona wartoscia
    size_t pos = res.find("{VAL}");
    if (pos != std::string::npos) {
        res.replace(pos, 5, std::to_string(val));
    }
    return res;
}

} // namespace Client::Skills
