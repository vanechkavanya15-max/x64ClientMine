#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/Terrain/TerrainEnvironmentConfigManager.h"
#include <string_view>
#include <cmath>

TEST_CASE("TerrainEnvironmentConfigManager: Configuration parsing") {
    using namespace Client::Terrain;
    TerrainEnvironmentConfigManager mgr;

    SUBCASE("Valid complete configuration") {
        std::string_view validConfig = R"(
            ScriptType EnvironmentData
            
            # Fog configuration
            FogEnable 1
            FogStart 1500.5
            FogEnd 12000.0
            FogDensity 0.005
            FogColor 0.8 0.9 1.0
            
            // Sun configuration
            SunEnable 1
            SunDir 0.0 -1.0 0.5
            SunColor 1.0 1.0 0.8
            
            AmbientColor 0.3 0.3 0.35
            
            NightMode 1
            DayCycleSpeed 2.5
        )";

        auto result = mgr.LoadFromMemory(validConfig);
        CHECK(result.has_value());

        auto params = mgr.GetParams();
        
        // Fog checks
        CHECK(params.fog.enabled == true);
        CHECK(std::abs(params.fog.start - 1500.5f) < 0.0001f);
        CHECK(std::abs(params.fog.end - 12000.0f) < 0.0001f);
        CHECK(std::abs(params.fog.density - 0.005f) < 0.0001f);
        CHECK(std::abs(params.fog.colorR - 0.8f) < 0.0001f);
        CHECK(std::abs(params.fog.colorG - 0.9f) < 0.0001f);
        CHECK(std::abs(params.fog.colorB - 1.0f) < 0.0001f);
        
        // Sun checks
        CHECK(params.sun.enabled == true);
        CHECK(std::abs(params.sun.dirX - 0.0f) < 0.0001f);
        CHECK(std::abs(params.sun.dirY - (-1.0f)) < 0.0001f);
        CHECK(std::abs(params.sun.dirZ - 0.5f) < 0.0001f);
        CHECK(std::abs(params.sun.colorR - 1.0f) < 0.0001f);
        CHECK(std::abs(params.sun.colorG - 1.0f) < 0.0001f);
        CHECK(std::abs(params.sun.colorB - 0.8f) < 0.0001f);

        // Ambient checks
        CHECK(std::abs(params.ambient.colorR - 0.3f) < 0.0001f);
        CHECK(std::abs(params.ambient.colorG - 0.3f) < 0.0001f);
        CHECK(std::abs(params.ambient.colorB - 0.35f) < 0.0001f);

        // Day cycle checks
        CHECK(params.dayCycle.nightMode == true);
        CHECK(std::abs(params.dayCycle.cycleSpeed - 2.5f) < 0.0001f);
    }

    SUBCASE("Empty configuration") {
        std::string_view emptyConfig = "   \n \t  ";
        auto result = mgr.LoadFromMemory(emptyConfig);
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == EnvConfigError::EmptyConfiguration);
    }

    SUBCASE("Invalid signature") {
        std::string_view badSig1 = "ScriptType WrongData";
        auto res1 = mgr.LoadFromMemory(badSig1);
        CHECK_FALSE(res1.has_value());
        CHECK(res1.error() == EnvConfigError::InvalidSignature);

        std::string_view badSig2 = "WrongType EnvironmentData";
        auto res2 = mgr.LoadFromMemory(badSig2);
        CHECK_FALSE(res2.has_value());
        CHECK(res2.error() == EnvConfigError::InvalidSignature);
    }

    SUBCASE("Missing values") {
        std::string_view missing = R"(
            ScriptType EnvironmentData
            FogColor 0.8 0.9
        )";
        auto result = mgr.LoadFromMemory(missing);
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == EnvConfigError::MissingValue);
    }

    SUBCASE("Invalid float format") {
        std::string_view invalidFloat = R"(
            ScriptType EnvironmentData
            FogStart not_a_number
        )";
        auto result = mgr.LoadFromMemory(invalidFloat);
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == EnvConfigError::InvalidFloatFormat);
    }

    SUBCASE("Invalid int format") {
        std::string_view invalidInt = R"(
            ScriptType EnvironmentData
            FogEnable true
        )";
        auto result = mgr.LoadFromMemory(invalidInt);
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == EnvConfigError::InvalidIntFormat);
    }

    SUBCASE("Unknown property") {
        std::string_view unknown = R"(
            ScriptType EnvironmentData
            UnknownProperty 123.0
        )";
        auto result = mgr.LoadFromMemory(unknown);
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == EnvConfigError::UnknownProperty);
    }

    SUBCASE("Default initialization check") {
        EnvironmentParams defaultParams = mgr.GetParams();
        CHECK(defaultParams.fog.enabled == false);
        CHECK(defaultParams.sun.enabled == false);
        CHECK(defaultParams.dayCycle.nightMode == false);
    }
}