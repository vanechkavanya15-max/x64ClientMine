#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest/doctest.h>
#include <fstream>
#include <string>
#include <vector>
#include <regex>

static std::string ResolveFilePath(const std::string& relPath)
{
    std::vector<std::string> candidates = {
        "src/" + relPath,
        "../src/" + relPath,
        "../../src/" + relPath,
        "E:/m2dev client backlup/src/" + relPath
    };
    for (const auto& cand : candidates)
    {
        std::ifstream f(cand);
        if (f.is_open())
            return cand;
    }
    return relPath;
}

static bool CheckFileNoBareEndifTokens(const std::string& relPath)
{
    std::string filePath = ResolveFilePath(relPath);
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

static bool CheckFileNoMojibakeMarkers(const std::string& relPath)
{
    std::string filePath = ResolveFilePath(relPath);
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
        "EterLib/StateManager.h",
        "EterLib/GrpBase.h",
        "EterLib/RenderQueueManager.h",
        "EterLib/RenderConventions.h",
        "GameLib/MapOutdoor.h",
        "GameLib/MapOutdoorCharacterShadow.h",
        "GameLib/MapOutdoorWater.h",
        "UserInterface/PythonApplication.h"
    };

    for (const auto& path : headersToCheck)
    {
        CHECK(CheckFileNoBareEndifTokens(path));
    }
}

TEST_CASE("CommentCleanliness: Mojibake eliminated in PythonApplication and GrpBase")
{
    CHECK(CheckFileNoMojibakeMarkers("UserInterface/PythonApplication.cpp"));
    CHECK(CheckFileNoMojibakeMarkers("EterLib/GrpBase.cpp"));
    CHECK(CheckFileNoMojibakeMarkers("SpeedTreeLib/SpeedTreeForestDirectX.cpp"));
}
