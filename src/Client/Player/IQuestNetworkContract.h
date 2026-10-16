#pragma once

#include <cstdint>

namespace Client::Player {

    /**
     * @brief Interfejs kontraktu sieciowego dla operacji zwiazanych z zadaniami (questami).
     */
    class IQuestNetworkContract {
    public:
        virtual ~IQuestNetworkContract() = default;

        /**
         * @brief Wysyla odpowiedz na skrypt (np. po zamknieciu dialogu lub wybraniu opcji).
         * @param answerIndex Indeks odpowiedzi.
         */
        virtual void SendScriptAnswer(int32_t answerIndex) = 0;

        /**
         * @brief Wysyla odpowiedz na prosbe o potwierdzenie zadania.
         * @param answer Wartosc odpowiedzi (np. 1 - tak, 0 - nie).
         * @param requestPID Identyfikator proszacego procesu/gracza.
         */
        virtual void SendQuestConfirmAnswer(uint8_t answer, uint32_t requestPID) = 0;
    };

}
