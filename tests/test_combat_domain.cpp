#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "vendor/doctest/doctest.h"
#include "src/Client/Combat/CombatTargetingLogic.h"
#include "src/Client/Contracts/IPlayerDomainContract.h"
#include <vector>
#include <cstdlib>
#include <new>

// Memory simulation for std::bad_alloc testing
static bool g_simulateBadAlloc = false;

void* operator new(std::size_t size)
{
    if (g_simulateBadAlloc)
        throw std::bad_alloc();
    void* ptr = std::malloc(size);
    if (!ptr)
        throw std::bad_alloc();
    return ptr;
}

void* operator new[](std::size_t size)
{
    if (g_simulateBadAlloc)
        throw std::bad_alloc();
    void* ptr = std::malloc(size);
    if (!ptr)
        throw std::bad_alloc();
    return ptr;
}

void operator delete(void* ptr) noexcept
{
    std::free(ptr);
}

void operator delete[](void* ptr) noexcept
{
    std::free(ptr);
}

void operator delete(void* ptr, std::size_t) noexcept
{
    std::free(ptr);
}

void operator delete[](void* ptr, std::size_t) noexcept
{
    std::free(ptr);
}

class MockPlayerDomainContract : public Client::Contracts::IPlayerDomainContract
{
public:
    uint32_t GetPlayerVID() const noexcept override { return 1001; }
    std::string_view GetPlayerName() const noexcept override { return "TestPlayer"; }
    int GetPlayerLevel() const noexcept override { return 50; }
    int64_t GetCurrentHP() const noexcept override { return 1000; }
    int64_t GetMaxHP() const noexcept override { return 1000; }
    int64_t GetCurrentSP() const noexcept override { return 500; }
    int64_t GetMaxSP() const noexcept override { return 500; }
    int64_t GetYang() const noexcept override { return 0; }
    int64_t GetExp() const noexcept override { return 0; }
    Client::Contracts::PlayerCoordinates GetCoordinates() const noexcept override { return {0.0f, 0.0f, 0.0f, 0.0f}; }
    uint32_t GetTargetVID() const noexcept override { return m_targetVid; }
    bool IsInCombat() const noexcept override { return true; }
    
    Client::Contracts::PlayerStatusSnapshot CaptureSnapshot() const override { return {}; }
    
    void SetTargetVID(uint32_t vid) { m_targetVid = vid; }

private:
    uint32_t m_targetVid{0};
};

TEST_CASE("CombatTargetingLogic - GetCurrentTarget")
{
    MockPlayerDomainContract mockPlayer;
    Client::Combat::CombatTargetingLogic logic(mockPlayer);

    SUBCASE("No target")
    {
        mockPlayer.SetTargetVID(0);
        auto result = logic.GetCurrentTarget();
        CHECK(!result.has_value());
        CHECK(result.error() == Client::Combat::CombatTargetingError::TargetNotFound);
    }

    SUBCASE("Has target")
    {
        mockPlayer.SetTargetVID(2002);
        auto result = logic.GetCurrentTarget();
        CHECK(result.has_value());
        CHECK(result.value() == 2002);
    }
}

TEST_CASE("CombatTargetingLogic - ValidateTarget")
{
    MockPlayerDomainContract mockPlayer;
    Client::Combat::CombatTargetingLogic logic(mockPlayer);

    SUBCASE("Invalid target")
    {
        auto result = logic.ValidateTarget(0, 1000.0f);
        CHECK(!result.has_value());
        CHECK(result.error() == Client::Combat::CombatTargetingError::TargetNotFound);
    }

    SUBCASE("Valid target")
    {
        auto result = logic.ValidateTarget(2002, 1000.0f);
        CHECK(result.has_value());
        CHECK(result.value().vid == 2002);
    }
}

TEST_CASE("CombatTargetingLogic - GetTargetsInRange")
{
    MockPlayerDomainContract mockPlayer;
    Client::Combat::CombatTargetingLogic logic(mockPlayer);

    SUBCASE("Success")
    {
        std::vector<uint32_t> potentialTargets = {1001, 2002, 0, 3003}; // 1001 is player, 0 is invalid
        auto result = logic.GetTargetsInRange(potentialTargets, 1000.0f);
        CHECK(result.has_value());
        CHECK(result.value().size() == 2);
        CHECK(result.value()[0] == 2002);
        CHECK(result.value()[1] == 3003);
    }

    SUBCASE("Out of Memory")
    {
        std::vector<uint32_t> potentialTargets = {2002, 3003, 4004};
        
        g_simulateBadAlloc = true;
        auto result = logic.GetTargetsInRange(potentialTargets, 1000.0f);
        g_simulateBadAlloc = false;

        CHECK(!result.has_value());
        CHECK(result.error() == Client::Combat::CombatTargetingError::AllocationFailed);
    }
}
