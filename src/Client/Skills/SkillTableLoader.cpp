#include "SkillTableLoader.h"
#include <charconv>
#include <string_view>
#include <array>
#include <algorithm>

namespace Client::Skills {

namespace {

// Helper to split a string_view by a character
std::vector<std::string_view> SplitString(std::string_view str, char delimiter) {
    std::vector<std::string_view> tokens;
    size_t start = 0;
    while (start < str.length()) {
        size_t end = str.find(delimiter, start);
        if (end == std::string_view::npos) {
            tokens.push_back(str.substr(start));
            break;
        }
        tokens.push_back(str.substr(start, end - start));
        start = end + 1;
    }
    // Handle trailing empty token if string ends with delimiter
    if (!str.empty() && str.back() == delimiter) {
        tokens.push_back("");
    }
    return tokens;
}

// Remove surrounding quotes if present
std::string_view Unquote(std::string_view str) {
    if (str.length() >= 2 && str.front() == '"' && str.back() == '"') {
        return str.substr(1, str.length() - 2);
    }
    return str;
}

// Helper to parse integers
template <typename T>
T ParseInt(std::string_view str, T defaultValue = 0) {
    T value;
    auto [ptr, ec] = std::from_chars(str.data(), str.data() + str.size(), value);
    if (ec == std::errc()) {
        return value;
    }
    return defaultValue;
}

} // namespace

EterBase::Result<std::vector<SkillTableRecord>> SkillTableLoader::LoadFromMemory(std::string_view tsvData)
{
    std::vector<SkillTableRecord> records;
    if (tsvData.empty()) {
        return records;
    }

    auto lines = SplitString(tsvData, '\n');
    if (lines.empty()) {
        return records;
    }

    // Skip header line (lines[0])
    for (size_t i = 1; i < lines.size(); ++i) {
        std::string_view line = lines[i];
        
        // Remove trailing \r if present
        if (!line.empty() && line.back() == '\r') {
            line = line.substr(0, line.length() - 1);
        }
        
        if (line.empty()) {
            continue;
        }

        auto columns = SplitString(line, '\t');
        
        if (columns.size() < 1) {
            continue; 
        }

        SkillTableRecord record{};
        
        auto getCol = [&](size_t idx) -> std::string_view {
            if (idx < columns.size()) {
                return Unquote(columns[idx]);
            }
            return "";
        };

        record.dwVnum = Client::Core::SkillId(ParseInt<uint32_t>(getCol(0)));
        record.strName = std::string(getCol(1));
        record.bType = ParseInt<uint8_t>(getCol(2));
        record.bLevelStep = ParseInt<uint8_t>(getCol(3));
        record.bMaxLevel = ParseInt<uint8_t>(getCol(4));
        record.bLevelLimit = ParseInt<uint8_t>(getCol(5));
        record.iPointOn = ParseInt<int32_t>(getCol(6));
        record.strPointPoly = std::string(getCol(7));
        record.strSPCostPoly = std::string(getCol(8));
        record.strDurationPoly = std::string(getCol(9));
        record.strDurationSPCostPoly = std::string(getCol(10));
        record.strCooldownPoly = std::string(getCol(11));
        record.strMasterBonusPoly = std::string(getCol(12));
        record.strAttackGradePoly = std::string(getCol(13));
        record.dwFlag = ParseInt<uint32_t>(getCol(14));
        record.dwAffectFlag = ParseInt<uint32_t>(getCol(15));
        record.iPointOn2 = ParseInt<int32_t>(getCol(16));
        record.strPointPoly2 = std::string(getCol(17));
        record.strDurationPoly2 = std::string(getCol(18));
        record.dwAffectFlag2 = ParseInt<uint32_t>(getCol(19));
        record.dwPrerequisiteSkillVnum = Client::Core::SkillId(ParseInt<uint32_t>(getCol(20)));
        record.bPrerequisiteSkillLevel = ParseInt<uint8_t>(getCol(21));
        record.bSkillType = ParseInt<uint8_t>(getCol(22));
        record.bMaxHit = ParseInt<uint8_t>(getCol(23));
        record.strSplashAroundDamageAdjustPoly = std::string(getCol(24));
        record.dwTargetRange = ParseInt<uint32_t>(getCol(25));
        record.dwSplashRange = ParseInt<uint32_t>(getCol(26));

        records.push_back(std::move(record));
    }

    return records;
}

} // namespace Client::Skills
