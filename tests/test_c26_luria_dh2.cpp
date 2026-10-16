#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/Network/Protocol/LuriaDH2.h"

TEST_CASE("LuriaDH2 ModPow Consistency Test") {
    bool result = Luria::TestModPowConsistency();
    CHECK(result == true);
}
