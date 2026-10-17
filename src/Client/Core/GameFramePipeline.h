#pragma once

#include <cstdint>
#include <string>
#include <string_view>
#include <chrono>
#include <atomic>
#include <memory>

namespace Client::Core
{
    /**
     * @brief Fazy cyklu zycia klatki gry (Game Frame Lifecycle Phases).
     * Definiuje scisly, deterministyczny porzadek przetwarzania klatki gry dla agentow AI.
     */
    enum class FramePhase : uint8_t
    {
        IpcIngest = 0,       ///< Faza 1: Odbior i obsluga zadan JSON-RPC z Named Pipe (MCP/AI)
        InputUpdate,         ///< Faza 2: Synchronizacja peryferiow OS i wirtualnych kontrolerow AI
        NetworkPump,         ///< Faza 3: Przetwarzanie strumieni TCP (pakiety gry, auth, guild)
        WorldSimulation,     ///< Faza 4: Symulacja swiata (pozycja gracza, interpolacja, fizyka, efekty)
        UiSynchronization,   ///< Faza 5: Aktualizacja interfejsu uzytkownika i zadan Pythona
        TelemetryRecord      ///< Faza 6: Rejestracja telemetrii i pomiarow czasu trwania klatki
    };

    [[nodiscard]] constexpr std::string_view FramePhaseToString(FramePhase phase) noexcept
    {
        switch (phase)
        {
            case FramePhase::IpcIngest:          return "IPC_INGEST";
            case FramePhase::InputUpdate:        return "INPUT_UPDATE";
            case FramePhase::NetworkPump:        return "NETWORK_PUMP";
            case FramePhase::WorldSimulation:    return "WORLD_SIMULATION";
            case FramePhase::UiSynchronization:  return "UI_SYNCHRONIZATION";
            case FramePhase::TelemetryRecord:    return "TELEMETRY_RECORD";
            default:                             return "UNKNOWN";
        }
    }

    /**
     * @brief Struktura telemetrii czasowej i stanu klatki logiki gry dla modeli AI.
     */
    struct FrameTelemetryData
    {
        uint64_t frameIndex{0};
        uint64_t currentTick{0};
        float deltaTimeSec{0.0166f};

        // Czasy wykonania poszczegolnych faz w mikrosekundach (us)
        uint32_t ipcDurationUs{0};
        uint32_t inputDurationUs{0};
        uint32_t networkDurationUs{0};
        uint32_t simulationDurationUs{0};
        uint32_t uiDurationUs{0};
        uint32_t totalFrameDurationUs{0};

        // Flagi kontrolera
        bool isTestMode{false};
        bool isFrozen{false};
        bool isHeadless{false};

        // Podsumowanie swiata
        uint32_t aliveActorsCount{0};
        std::string networkPhaseName{"Unknown"};
        std::string activeProtocolDriver{"standard_x64"};
    };

    /**
     * @brief Centralny zarzadca potoku wykonawczego klatki logiki gry (GameFramePipeline).
     * 
     * Odpowiada za spojna, czytelna orkiestracje wszystkich podsystemow gry
     * z pominieciem fizycznego renderera (Direct3D 9 / WGPU pozostaja nienaruszone).
     */
    class GameFramePipeline
    {
    public:
        static GameFramePipeline& Instance() noexcept;

        GameFramePipeline() noexcept = default;
        ~GameFramePipeline() = default;

        GameFramePipeline(const GameFramePipeline&) = delete;
        GameFramePipeline& operator=(const GameFramePipeline&) = delete;

        // --- Zdarzenia poczatku i konca fazy ---
        void BeginPhase(FramePhase phase) noexcept;
        void EndPhase(FramePhase phase) noexcept;

        // --- Rejestracja i zrzut telemetrii ---
        void RecordFrameComplete(uint64_t frameIndex, uint64_t tick, float deltaSec) noexcept;
        void UpdateWorldMetrics(uint32_t aliveActors, std::string_view netPhase, std::string_view driver) noexcept;
        void SetEngineModeFlags(bool testMode, bool frozen, bool headless) noexcept;

        [[nodiscard]] FrameTelemetryData GetTelemetry() const noexcept;
        [[nodiscard]] std::string GetTelemetryJson() const;

        // Reset licznikow
        void Reset() noexcept;

    private:
        mutable std::atomic<uint64_t> m_frameCounter{0};
        mutable std::atomic<uint64_t> m_tickCounter{0};
        mutable std::atomic<float> m_lastDeltaSec{0.0166f};

        std::chrono::steady_clock::time_point m_phaseStartTimes[6]{};
        std::atomic<uint32_t> m_phaseDurationsUs[6]{};

        std::atomic<bool> m_isTestMode{false};
        std::atomic<bool> m_isFrozen{false};
        std::atomic<bool> m_isHeadless{false};
        std::atomic<uint32_t> m_aliveActorsCount{0};

        char m_networkPhaseName[64]{"OffLine"};
        char m_activeProtocolDriver[64]{"standard_x64"};
    };

    /**
     * @brief Pomocniczy RAII guard do automatycznego pomiaru czasu trwania fazy klatki.
     */
    class ScopedPhaseTimer
    {
    public:
        explicit ScopedPhaseTimer(FramePhase phase) noexcept
            : m_phase(phase)
        {
            GameFramePipeline::Instance().BeginPhase(m_phase);
        }

        ~ScopedPhaseTimer()
        {
            GameFramePipeline::Instance().EndPhase(m_phase);
        }

        ScopedPhaseTimer(const ScopedPhaseTimer&) = delete;
        ScopedPhaseTimer& operator=(const ScopedPhaseTimer&) = delete;

    private:
        FramePhase m_phase;
    };

} // namespace Client::Core
