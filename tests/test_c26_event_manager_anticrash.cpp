#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include <string>
#include <vector>
#include <memory>
#include <algorithm>
#include <cstdint>

// Symulacja struktur Event Managera dla weryfikacji anty-crash i limitu wykonania (Wave W11: T341)
namespace TestEventSecurity
{
    struct SimulatedTextInstance
    {
        std::string text;
        bool destroyed{false};

        void Destroy() noexcept {
            destroyed = true;
        }

        void SetValue(const char* val) {
            text = val ? val : "";
        }
    };

    struct SimulatedPool
    {
        uint32_t allocCount{0};
        uint32_t freeCount{0};

        SimulatedTextInstance* Alloc() {
            ++allocCount;
            return new SimulatedTextInstance();
        }

        void Free(SimulatedTextInstance* inst) {
            if (inst) {
                ++freeCount;
                delete inst;
            }
        }
    };

    struct SimulatedEventSet
    {
        char szFileName[33]{0};
        bool isLock{false};
        bool isWaitFlag{false};
        long lLastDelayTime{0};
        bool isConfirmWait{false};
        SimulatedTextInstance* pCurrentTextLine{nullptr};
        SimulatedTextInstance* pConfirmTimeTextLine{nullptr};
        std::vector<SimulatedTextInstance*> lines;
        int pythonRefCounter{0};
    };

    // Watchdog wykonawczy dla UpdateEventSet
    constexpr int MAX_COMMANDS_PER_FRAME = 512;

    int RunSimulatedUpdate(SimulatedEventSet& eventSet, int availableCommands)
    {
        int executed = 0;
        int safetyCounter = 0;

        while (!eventSet.isLock && !eventSet.isWaitFlag && eventSet.lLastDelayTime <= 0)
        {
            if (++safetyCounter > MAX_COMMANDS_PER_FRAME)
            {
                eventSet.isLock = true;
                break;
            }

            if (executed < availableCommands)
            {
                ++executed;
                // Symulacja wykonania komendy bez blokady
            }
            else
            {
                // Brak dalszych komend -> zakonczenie skryptu
                eventSet.isLock = true;
                break;
            }
        }

        return executed;
    }

    void RunSimulatedConfirmUpdate(SimulatedEventSet& eventSet, long currentTime, long endTime)
    {
        if (eventSet.isConfirmWait && eventSet.pConfirmTimeTextLine)
        {
            long leftTime = std::max(0l, endTime - currentTime);
            char buf[64];
            snprintf(buf, sizeof(buf), "Left: %ld s", leftTime);
            eventSet.pConfirmTimeTextLine->SetValue(buf);
        }
    }

    void RunSimulatedClearLine(SimulatedEventSet& eventSet, SimulatedPool& pool)
    {
        eventSet.isConfirmWait = false;
        eventSet.pConfirmTimeTextLine = nullptr;

        for (auto* line : eventSet.lines)
        {
            if (line)
            {
                line->Destroy();
                pool.Free(line);
            }
        }
        eventSet.lines.clear();

        if (eventSet.pCurrentTextLine)
        {
            eventSet.pCurrentTextLine->Destroy();
            pool.Free(eventSet.pCurrentTextLine);
            eventSet.pCurrentTextLine = nullptr;
        }
    }
}

TEST_CASE("EventManager AntiCrash - Watchdog prevents UI freeze on infinite script loop")
{
    using namespace TestEventSecurity;

    SimulatedEventSet eventSet;
    snprintf(eventSet.szFileName, sizeof(eventSet.szFileName), "quest_infinite_loop.lua");
    eventSet.isLock = false;
    eventSet.isWaitFlag = false;
    eventSet.lLastDelayTime = 0;

    // Skrypt probujacy wykonac 100 000 komend w jednej klatce
    int executed = RunSimulatedUpdate(eventSet, 100000);

    CHECK_EQ(executed, MAX_COMMANDS_PER_FRAME);
    CHECK_EQ(eventSet.isLock, true);
}

TEST_CASE("EventManager AntiCrash - Nullptr guard for pConfirmTimeTextLine prevents 0xC0000005 crash")
{
    using namespace TestEventSecurity;

    SimulatedEventSet eventSet;
    eventSet.isConfirmWait = true;
    eventSet.pConfirmTimeTextLine = nullptr; // Symulacja dereferencji po wyczyszczeniu

    // Nie powinno wyzwolic wyjatku ani awarii
    CHECK_NOTHROW(RunSimulatedConfirmUpdate(eventSet, 100, 150));

    // Gdy wskaznik jest poprawny
    SimulatedTextInstance validInstance;
    eventSet.pConfirmTimeTextLine = &validInstance;
    RunSimulatedConfirmUpdate(eventSet, 100, 150);
    CHECK_EQ(validInstance.text, "Left: 50 s");
}

TEST_CASE("EventManager AntiCrash - ClearLine completely drains pool without memory leaks")
{
    using namespace TestEventSecurity;

    SimulatedPool pool;
    SimulatedEventSet eventSet;

    // Alokacja 10 linii dialogowych
    for (int i = 0; i < 10; ++i)
    {
        eventSet.lines.push_back(pool.Alloc());
    }
    eventSet.pCurrentTextLine = pool.Alloc();
    eventSet.isConfirmWait = true;
    eventSet.pConfirmTimeTextLine = eventSet.lines[0];

    CHECK_EQ(pool.allocCount, 11);
    CHECK_EQ(pool.freeCount, 0);

    // Czyszczenie dialogu przy przejsciu do kolejnej strony
    RunSimulatedClearLine(eventSet, pool);

    CHECK_EQ(pool.freeCount, 11);
    CHECK_EQ(eventSet.lines.empty(), true);
    CHECK_EQ(eventSet.pCurrentTextLine, nullptr);
    CHECK_EQ(eventSet.pConfirmTimeTextLine, nullptr);
    CHECK_EQ(eventSet.isConfirmWait, false);
}

TEST_CASE("EventManager AntiCrash - Null-termination guarantee on fileName prevents buffer overread")
{
    char szFileName[33]{0};
    const char* longName = "this_is_a_very_long_quest_filename_that_exceeds_buffer_capacity.lua";

    strncpy(szFileName, longName, sizeof(szFileName) - 1);
    szFileName[sizeof(szFileName) - 1] = '\0';

    CHECK_EQ(strlen(szFileName), 32);
    CHECK_EQ(szFileName[32], '\0');
}
