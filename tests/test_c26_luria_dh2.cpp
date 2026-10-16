#include "doctest.h"
#include "src/Client/Network/Protocol/LuriaDH2.h"

TEST_CASE("LuriaDH2 ModPow Consistency Test") {
    bool result = Luria::TestModPowConsistency();
    CHECK(result == true);
}
