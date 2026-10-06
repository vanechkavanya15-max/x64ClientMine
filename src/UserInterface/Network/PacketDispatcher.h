#pragma once

#include <cstdint>
#include <span>
#include <array>
#include <functional>

namespace Network
{
    /**
     * @brief Sygnatura handlera pakietu w architekturze Zero-Alloc C++20.
     */
    using PacketHandlerFn = bool (*)(std::span<const uint8_t>);

    /**
     * @brief Nowoczesny dispatcher pakietow gry Metin2 x64 (Standard 2026).
     * 
     * Zastepuje monolityczny switch-case w PythonNetworkStreamPhaseGame.cpp
     * tablica funkcyjna O(1) pozbawiona alokacji pamieci (Zero-GC, Zero-Alloc).
     */
    class PacketDispatcher
    {
    public:
        static PacketDispatcher& Instance();

        /**
         * @brief Rejestruje handler dla danego naglowka (opcode) pakietu.
         */
        void RegisterHandler(uint8_t opcode, PacketHandlerFn handler);

        /**
         * @brief Dysponuje surowy bufor pakietu do zarejestrowanego handlera domeny.
         * @param opcode Naglowek pakietu.
         * @param payload Bufor z zawartoscia pakietu.
         * @return true jesli pakiet zostal pomyslnie przetworzony.
         */
        bool Dispatch(uint8_t opcode, std::span<const uint8_t> payload);

        /**
         * @brief Rejestruje komplet wszystkich domyslnych handlerow Fali 1.
         */
        void RegisterDefaultHandlers();

        /**
         * @brief Sprawdza czy dany opcode posiada zarejestrowany handler.
         */
        bool HasHandler(uint8_t opcode) const;

    private:
        PacketDispatcher();
        ~PacketDispatcher() = default;

        std::array<PacketHandlerFn, 256> m_handlers{};
    };
}
