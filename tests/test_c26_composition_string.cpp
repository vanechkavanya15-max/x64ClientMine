#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest/doctest.h>
#include "Client/Input/CompositionStringHandler.h"

using namespace Client::Input;

TEST_CASE("CompositionStringHandler - UpdateComposition and FinalizeResult") {
    CompositionStringHandler handler;
    
    SUBCASE("Update composition string successfully") {
        std::wstring comp = L"test";
        std::vector<uint8_t> attrs = {1, 1, 1, 1};
        auto result = handler.UpdateComposition(comp, attrs, 2);
        
        CHECK(result.has_value() == true);
        const auto& state = handler.GetState();
        CHECK(state.current_composition == L"test");
        CHECK(state.attributes.size() == 4);
        CHECK(state.cursor_position.value() == 2);
        CHECK(state.result_string.empty());
    }

    SUBCASE("Update composition fails due to attribute length mismatch") {
        std::wstring comp = L"test";
        std::vector<uint8_t> attrs = {1, 1}; // Invalid length
        auto result = handler.UpdateComposition(comp, attrs, 2);
        
        CHECK(result.has_value() == false);
        CHECK(result.error() == CompositionError::InvalidAttributeLength);
    }
    
    SUBCASE("Update composition fails due to invalid cursor position") {
        std::wstring comp = L"test";
        std::vector<uint8_t> attrs = {1, 1, 1, 1};
        auto result = handler.UpdateComposition(comp, attrs, 10); // Out of bounds
        
        CHECK(result.has_value() == false);
        CHECK(result.error() == CompositionError::InvalidCursorPosition);
    }
    
    SUBCASE("FinalizeResult resets composition and sets result") {
        std::wstring comp = L"test";
        std::vector<uint8_t> attrs = {1, 1, 1, 1};
        handler.UpdateComposition(comp, attrs, 2);
        
        auto result = handler.FinalizeResult(L"final_result");
        CHECK(result.has_value() == true);
        
        const auto& state = handler.GetState();
        CHECK(state.result_string == L"final_result");
        CHECK(state.current_composition.empty());
        CHECK(state.attributes.empty());
        CHECK(state.cursor_position.has_value() == false);
    }
    
    SUBCASE("Clear resets entire state") {
        handler.UpdateComposition(L"test", {1, 1, 1, 1}, 2);
        handler.FinalizeResult(L"result");
        
        handler.Clear();
        const auto& state = handler.GetState();
        CHECK(state.current_composition.empty());
        CHECK(state.result_string.empty());
        CHECK(state.attributes.empty());
        CHECK(state.cursor_position.has_value() == false);
    }
}
