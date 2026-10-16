#pragma once

#include <string>
#include "IQuestNetworkContract.h"
#include <Client/Core/ExpectedResult.h>

namespace Client::Player {

    enum class QuestManagerError : uint8_t {
        None,
        NotWaitingForConfirm,
        NullNetworkContract
    };

    /**
     * @brief Menadzer stanu zadan (questow) dla gracza.
     * Zarzadza interakcjami zwiazanymi z oknami potwierdzen, dialogami itp.
     */
    class PlayerQuestManager {
    public:
        explicit PlayerQuestManager(IQuestNetworkContract* networkContract);
        ~PlayerQuestManager() = default;

        // Zapobiegamy kopiowaniu
        PlayerQuestManager(const PlayerQuestManager&) = delete;
        PlayerQuestManager& operator=(const PlayerQuestManager&) = delete;
        
        // Zezwalamy na przenoszenie
        PlayerQuestManager(PlayerQuestManager&&) noexcept = default;
        PlayerQuestManager& operator=(PlayerQuestManager&&) noexcept = default;

        /**
         * @brief Przetwarza pakiet potwierdzenia zadania (przychodzi z sieci).
         * @param msg Tresc wiadomosci potwierdzenia.
         * @param timeout Czas oczekiwania na potwierdzenie.
         * @param requestPID ID proszacego gracza/procesu.
         */
        void OnQuestConfirm(const std::string& msg, int32_t timeout, uint32_t requestPID);

        /**
         * @brief Odpowiada na wczesniej otrzymane potwierdzenie zadania.
         * @param answer Wartosc odpowiedzi (np. 1 = zgoda, 0 = odmowa).
         * @return ExpectedResult void w przypadku sukcesu, lub QuestManagerError jesli brak biezacego potwierdzenia.
         */
        Core::ExpectedResult<void, QuestManagerError> AnswerQuestConfirm(uint8_t answer);

        /**
         * @brief Wysyla odpowiedz (indeks) z dialogu skryptu questa do serwera.
         * @param answerIndex Indeks wybranej odpowiedzi dialogowej.
         * @return ExpectedResult void w przypadku sukcesu, lub QuestManagerError przy braku kontraktu.
         */
        Core::ExpectedResult<void, QuestManagerError> AnswerScript(int32_t answerIndex);

        /**
         * @brief Sprawdza, czy menadzer aktualnie oczekuje na odpowiedz uzytkownika do potwierdzenia zadania.
         */
        [[nodiscard]] bool IsWaitingForConfirm() const noexcept;

        /**
         * @brief Zwraca PID zadania dla aktualnie oczekujacego potwierdzenia.
         */
        [[nodiscard]] uint32_t GetCurrentConfirmRequestPID() const noexcept;

    private:
        IQuestNetworkContract* networkContract_;
        
        bool isWaitingForConfirm_;
        uint32_t currentConfirmRequestPID_;
    };

}
