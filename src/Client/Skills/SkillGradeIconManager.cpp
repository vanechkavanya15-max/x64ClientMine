#include "SkillGradeIconManager.h"

namespace Client::Skills {

std::string SkillGradeIconManager::GetIconPath(uint32_t skillVnum, SkillGrade grade) {
    std::string base = "d:/ymir work/ui/skill/warrior/" + std::to_string(skillVnum);
    switch (grade) {
        case SkillGrade::Master: return base + "_m.sub";
        case SkillGrade::GrandMaster: return base + "_g.sub";
        case SkillGrade::PerfectMaster: return base + "_p.sub";
        default: return base + ".sub";
    }
}

} // namespace Client::Skills
