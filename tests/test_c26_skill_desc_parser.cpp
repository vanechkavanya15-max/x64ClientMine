#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Skills/SkillDescriptionParser.h"

TEST_CASE("SkillDescriptionParser: Formula formatting") {
    std::string raw = "Obrazenia: {VAL} pkt";
    std::string formatted = Client::Skills::SkillDescriptionParser::FormatFormula(raw, 20.0f, 90.0f);
    // 20*10 + 90*1.5 = 200 + 135 = 335
    CHECK(formatted == "Obrazenia: 335 pkt");
}
