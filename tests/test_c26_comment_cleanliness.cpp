#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest/doctest.h>
#include <fstream>
#include <string>
#include <vector>
#include <regex>

static bool CheckFileNoBareEndifTokens(const std::string& filePath)
{
    std::ifstream file(filePath);
    if (!file.is_open())
        return true;

    std::string line;
    std::regex bareEndif(R"(^\s*#endif\s+[a-zA-Z0-9_]+)");
    while (std::getline(file, line))
    {
        if (std::regex_search(line, bareEndif))
            return false;
    }
    return true;
}

static bool CheckFileNoMojibakeMarkers(const std::string& filePath)
{
    std::ifstream file(filePath, std::ios::binary);
    if (!file.is_open())
        return true;

    std::string content((std::istreambuf_iterator<char>(file)), std::istreambuf_iterator<char>());
    // Sprawdzanie znanych ciagow uszkodzonego mojibake CP949 z inwentarza auditowego
    if (content.find("!@# Alt+Tab") != std::string::npos && content.find("levites") != std::string::npos)
        return false;
    if (content.find("10000 \xC6\xFA\xB8\xAE\xB0\xEF") != std::string::npos)
        return false;

    return true;
}

TEST_CASE("CommentCleanliness: No bare endif tokens in key headers")
{
    std::vector<std::string> headersToCheck = {
        "../src/EterLib/StateManager.h",
        "../src/EterLib/GrpBase.h",
        "../src/EterLib/RenderQueueManager.h",
        "../src/EterLib/RenderConventions.h",
        "../src/GameLib/MapOutdoor.h",
        "../src/GameLib/MapOutdoorCharacterShadow.h",
        "../src/GameLib/MapOutdoorWater.h",
        "../src/UserInterface/PythonApplication.h"
    };

    for (const auto& path : headersToCheck)
    {
        CHECK(CheckFileNoBareEndifTokens(path));
    }
}

TEST_CASE("CommentCleanliness: Mojibake eliminated in PythonApplication and GrpBase")
{
    CHECK(CheckFileNoMojibakeMarkers("../src/UserInterface/PythonApplication.cpp"));
    CHECK(CheckFileNoMojibakeMarkers("../src/EterLib/GrpBase.cpp"));
    CHECK(CheckFileNoMojibakeMarkers("../src/SpeedTreeLib/SpeedTreeForestDirectX.cpp"));
}
