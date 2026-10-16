#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Input/IMECompositionManager.h"

TEST_CASE("IMECompositionManager: Composition string and caret management") {
    Client::Input::IMECompositionManager ime;
    CHECK_FALSE(ime.HasComposition());
    ime.SetCompositionString("hangul", 6);
    CHECK(ime.HasComposition());
    CHECK(ime.GetString() == "hangul");
    CHECK(ime.GetCaretPos() == 6);
    ime.Clear();
    CHECK_FALSE(ime.HasComposition());
}
