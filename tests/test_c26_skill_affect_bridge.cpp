#include "doctest.h"
#include "../src/Client/Skills/SkillAffectBridge.h"
#include "../src/Client/Skills/ISkillPlayerAdapter.h"
#include "../src/Client/Bridge/IPythonRuntime.h"
#include <string>
#include <vector>

using namespace Client::Skills;
using namespace Client::Bridge;

// Mock PythonRuntime
class MockPythonRuntime : public IPythonRuntime {
public:
    std::string lastModuleName;
    std::string lastMethodName;
    std::vector<std::string> lastArgs;
    bool shouldFail = false;

    void Initialize() noexcept override {}
    void Shutdown() noexcept override {}

    EterBase::Result<std::string, PythonRuntimeError> ExecuteMethod(
        std::string_view moduleName, 
        std::string_view methodName, 
        std::span<const std::string> args = {}) override 
    {
        lastModuleName = std::string(moduleName);
        lastMethodName = std::string(methodName);
        lastArgs.assign(args.begin(), args.end());

        if (shouldFail) {
            return std::unexpected(static_cast<PythonRuntimeError>(1));
        }
        return "success";
    }
};

// Mock PlayerAdapter
class MockPlayerAdapter : public ISkillPlayerAdapter {
public:
    bool resetCalled = false;

    void ResetAffectState() noexcept override {
        resetCalled = true;
    }
};

TEST_CASE("SkillAffectBridge - AddAffect") {
    MockPythonRuntime mockRuntime;
    MockPlayerAdapter mockAdapter;
    SkillAffectBridge bridge(&mockRuntime, &mockAdapter);

    SUBCASE("Successfully invokes python method") {
        auto result = bridge.AddAffect(10, 1, 100, 300, 0);
        
        CHECK(result.has_value() == true);
        CHECK(mockRuntime.lastModuleName == "game");
        CHECK(mockRuntime.lastMethodName == "BINARY_NEW_AddAffect");
        CHECK(mockRuntime.lastArgs.size() == 5);
        CHECK(mockRuntime.lastArgs[0] == "10");
        CHECK(mockRuntime.lastArgs[1] == "1");
        CHECK(mockRuntime.lastArgs[2] == "100");
        CHECK(mockRuntime.lastArgs[3] == "300");
        CHECK(mockRuntime.lastArgs[4] == "0");
    }

    SUBCASE("Handles python runtime failure") {
        mockRuntime.shouldFail = true;
        auto result = bridge.AddAffect(10, 1, 100, 300, 0);
        
        CHECK(result.has_value() == false);
        CHECK(result.error() == SkillAffectError::AddAffectFailed);
    }
    
    SUBCASE("Handles null python runtime") {
        SkillAffectBridge noRuntimeBridge(nullptr, &mockAdapter);
        auto result = noRuntimeBridge.AddAffect(10, 1, 100, 300, 0);
        
        CHECK(result.has_value() == false);
        CHECK(result.error() == SkillAffectError::PythonRuntimeNotAvailable);
    }
}

TEST_CASE("SkillAffectBridge - RemoveAffect") {
    MockPythonRuntime mockRuntime;
    MockPlayerAdapter mockAdapter;
    SkillAffectBridge bridge(&mockRuntime, &mockAdapter);

    SUBCASE("Successfully invokes python method and resets state") {
        auto result = bridge.RemoveAffect(10, 1);
        
        CHECK(result.has_value() == true);
        CHECK(mockRuntime.lastModuleName == "game");
        CHECK(mockRuntime.lastMethodName == "BINARY_NEW_RemoveAffect");
        CHECK(mockRuntime.lastArgs.size() == 2);
        CHECK(mockRuntime.lastArgs[0] == "10");
        CHECK(mockRuntime.lastArgs[1] == "1");
        CHECK(mockAdapter.resetCalled == true);
    }

    SUBCASE("Handles python runtime failure") {
        mockRuntime.shouldFail = true;
        auto result = bridge.RemoveAffect(10, 1);
        
        CHECK(result.has_value() == false);
        CHECK(result.error() == SkillAffectError::RemoveAffectFailed);
        CHECK(mockAdapter.resetCalled == false); // Should not reset if python fails
    }
    
    SUBCASE("Handles null python runtime") {
        SkillAffectBridge noRuntimeBridge(nullptr, &mockAdapter);
        auto result = noRuntimeBridge.RemoveAffect(10, 1);
        
        CHECK(result.has_value() == false);
        CHECK(result.error() == SkillAffectError::PythonRuntimeNotAvailable);
    }

    SUBCASE("Handles null adapter") {
        SkillAffectBridge noAdapterBridge(&mockRuntime, nullptr);
        auto result = noAdapterBridge.RemoveAffect(10, 1);
        
        CHECK(result.has_value() == false);
        CHECK(result.error() == SkillAffectError::AdapterNotAvailable);
    }
}
