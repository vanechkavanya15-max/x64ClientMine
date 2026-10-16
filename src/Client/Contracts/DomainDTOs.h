#pragma once

#include <cstdint>
#include <cstddef>
#include <type_traits>

/**
 * ============================================================================
 * DOMAIN DTOs (C++23 ZERO-SDK POD CONTRACTS)
 * Metin2 Client 2026 - Czyste Kontrakty Domenowe Zero-SDK
 *
 * Wszystkie struktury w tym pliku sa scisle POD (Plain Old Data),
 * standard-layout i trivially copyable. Nie posiadaja zaleznosci od
 * Windows API (Win32), Pythona (CPython API) ani DirectX (D3D9).
 * Przeznaczone do uzycia w silniku bezokienkowym (Headless),
 * mostku MCP, serializacji sieciowej oraz komunikacji miedzyprocesowej IPC.
 * ============================================================================
 */

namespace Client::Contracts
{
#pragma pack(push, 1)

    /**
     * @brief Zrzut stanu aktora / encji w swiecie gry.
     * Scisly rozmiar 32 bajtow dla zero-copy FFI oraz serializacji sieciowej.
     */
    struct ActorSnapshot
    {
        uint32_t vid{0};       // 0..3 (4B): Unikalny identyfikator wirtualny encji (VID)
        uint32_t vnum{0};      // 4..7 (4B): Identyfikator typu encji w bazie (VNUM z mob_proto)
        float x{0.0f};         // 8..11 (4B): Wspolrzedna pozycji X w swiecie gry
        float y{0.0f};         // 12..15 (4B): Wspolrzedna pozycji Y w swiecie gry
        float z{0.0f};         // 16..19 (4B): Wysokosc pozycji Z (wysokosc terenu)
        int32_t hp{0};         // 20..23 (4B): Aktualne punkty zycia (HP)
        int32_t max_hp{0};     // 24..27 (4B): Maksymalne punkty zycia (Max HP)
        uint8_t type{0};       // 28 (1B): Typ encji (0=mob, 1=npc, 2=metin/stone, 6=player)
        uint8_t is_dead{0};    // 29 (1B): Flaga stanu zycia (0=zywy, 1=martwy)
        uint16_t _reserved{0}; // 30..31 (2B): Wyrownanie do 32 bajtow (granica 4-bajtowa)
    };

    /**
     * @brief Pojedynczy atrybut / bonus przedmiotu (gniazdo bonusa).
     * Scisly rozmiar 8 bajtow, wyrownany do naturalnej granicy 4B.
     */
    struct ItemAttributeDTO
    {
        int16_t type{0};   // 0..1 (2B): Identyfikator typu bonusu (np. MAX_HP, ATT_SPEED)
        int16_t _pad{0};   // 2..3 (2B): Wyrownanie do 4 bajtow
        int32_t value{0};  // 4..7 (4B): Wartosc liczbowa bonusu
    };

    /**
     * @brief Stan przedmiotu w slocie ekwipunku, wyposazenia lub magazynu.
     * Czysta reprezentacja POD bez dynamicznych lancuchow i wektorow.
     * Scisly rozmiar 104 bajty.
     */
    struct ItemSlotDTO
    {
        uint32_t vnum{0};                 // 0..3 (4B): Identyfikator przedmiotu w bazie (item_proto)
        uint32_t count{1};                // 4..7 (4B): Liczba sztuk przedmiotu w stosie
        uint32_t flags{0};                // 8..11 (4B): Flagi zachowania i wlasciwosci przedmiotu
        uint32_t anti_flags{0};           // 12..15 (4B): Antyflagi (blokady klas, plci, handlu)
        uint16_t cell{0};                 // 16..17 (2B): Indeks slotu w wybranym oknie
        uint8_t window_type{1};           // 18 (1B): Typ okna (1=Inventory, 2=Equipment, 10=Belt, 11=DragonSoul)
        uint8_t is_equipped{0};           // 19 (1B): Flaga zalozenia (1=zalozony na postac, 0=w plecaku)
        uint8_t is_locked{0};             // 20 (1B): Flaga blokady (1=zablokowany np. podczas handlu)
        uint8_t _reserved[3]{0, 0, 0};    // 21..23 (3B): Wyrownanie do granicy 4 bajtow
        int32_t sockets[6]{0};            // 24..47 (24B): Sloty kamieni dusz, przetopow lub czasu trwania
        ItemAttributeDTO attributes[7]{}; // 48..103 (56B): 7 gniazd bonusow (1-5 standard, 6-7 rzadkie)
    };

    /**
     * @brief Polecenie wykonania akcji bojowej przez silnik klienta.
     * Wykorzystywane przez sztuczna inteligencje, mostek MCP i system automatyzacji.
     * Scisly rozmiar 28 bajtow.
     */
    struct CombatCommandDTO
    {
        uint32_t target_vid{0};   // 0..3 (4B): VID atakowanego celu (potwor, boss, inny gracz)
        uint32_t skill_vnum{0};   // 4..7 (4B): VNUM umiejetnosci (0 = zwykly atak bronia/piescia)
        uint8_t attack_type{0};   // 8 (1B): Rodzaj akcji bojowej (0=Melee, 1=Skill, 2=Combo, 3=Cancel)
        uint8_t combo_index{0};   // 9 (1B): Indeks kroku sekwencji combo (0..5)
        uint8_t motion_index{0};  // 10 (1B): Indeks animacji/ruchu bojowego
        uint8_t flags{0};         // 11 (1B): Flagi ataku (bit 0=AutoFace, bit 1=Dash, bit 2=ForceRange)
        float target_x{0.0f};     // 12..15 (4B): Opcjonalna docelowa wspolrzedna X celu
        float target_y{0.0f};     // 16..19 (4B): Opcjonalna docelowa wspolrzedna Y celu
        float target_z{0.0f};     // 20..23 (4B): Opcjonalna docelowa wspolrzedna Z celu
        float range_limit{0.0f};  // 24..27 (4B): Maksymalny dystans ataku (0.0f = domyslny dla typu broni)
    };

    /**
     * @brief Punkt trasy (waypoint) dla kontrolera nawigacji i pathfindingu.
     * Pozbawiony zaleznosci od systemu grafiki i fizyki kolizji.
     * Scisly rozmiar 28 bajtow.
     */
    struct NavigationWaypointDTO
    {
        float x{0.0f};                  // 0..3 (4B): Docelowa wspolrzedna pozycji X w swiecie gry
        float y{0.0f};                  // 4..7 (4B): Docelowa wspolrzedna pozycji Y w swiecie gry
        float z{0.0f};                  // 8..11 (4B): Docelowa wspolrzedna pozycji Z (wysokosc terenu)
        float acceptance_radius{50.0f}; // 12..15 (4B): Promien tolerancji dotarcia do punktu
        uint32_t wait_time_ms{0};       // 16..19 (4B): Czas postoju w punkcie w milisekundach
        uint32_t action_id{0};          // 20..23 (4B): Identyfikator akcji po dotarciu (np. dialog z NPC)
        uint8_t movement_mode{1};       // 24 (1B): Tryb poruszania (0=Stop, 1=Walk, 2=Run, 3=Mount, 4=Warp)
        uint8_t flags{0};               // 25 (1B): Flagi nawigacji (bit 0=IgnoreMobs, bit 1=StrictPath)
        uint16_t _reserved{0};          // 26..27 (2B): Wyrownanie do 4-bajtowej granicy pamieci
    };

#pragma pack(pop)

    // ========================================================================
    // STATYCZNE ASERCJE C++23 POD, STANDARD LAYOUT I ROZMIAROW
    // ========================================================================
    static_assert(sizeof(ActorSnapshot) == 32, "ActorSnapshot musi miec dokladnie 32 bajty dla C-ABI");
    static_assert(std::is_standard_layout_v<ActorSnapshot>, "ActorSnapshot musi byc standard layout");
    static_assert(std::is_trivially_copyable_v<ActorSnapshot>, "ActorSnapshot musi byc trivially copyable");

    static_assert(sizeof(ItemAttributeDTO) == 8, "ItemAttributeDTO musi miec dokladnie 8 bajtow");
    static_assert(sizeof(ItemSlotDTO) == 104, "ItemSlotDTO musi miec dokladnie 104 bajty");
    static_assert(std::is_standard_layout_v<ItemSlotDTO>, "ItemSlotDTO musi byc standard layout");
    static_assert(std::is_trivially_copyable_v<ItemSlotDTO>, "ItemSlotDTO musi byc trivially copyable");

    static_assert(sizeof(CombatCommandDTO) == 28, "CombatCommandDTO musi miec dokladnie 28 bajtow");
    static_assert(std::is_standard_layout_v<CombatCommandDTO>, "CombatCommandDTO musi byc standard layout");
    static_assert(std::is_trivially_copyable_v<CombatCommandDTO>, "CombatCommandDTO musi byc trivially copyable");

    static_assert(sizeof(NavigationWaypointDTO) == 28, "NavigationWaypointDTO musi miec dokladnie 28 bajtow");
    static_assert(std::is_standard_layout_v<NavigationWaypointDTO>, "NavigationWaypointDTO musi byc standard layout");
    static_assert(std::is_trivially_copyable_v<NavigationWaypointDTO>, "NavigationWaypointDTO musi byc trivially copyable");

} // namespace Client::Contracts
