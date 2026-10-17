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
 * mostku MCP, serializacji sieciowej, komunikacji miedzyprocesowej IPC
 * oraz bezposredniej integracji FFI z jezykiem Rust (#[repr(C, packed)]).
 * ============================================================================
 */

namespace Client::Contracts
{
#pragma pack(push, 1)

    // ========================================================================
    // DOMENA 1: AKTORZY I POSTACIE SWIATA (ACTOR & ENTITY STATE)
    // ========================================================================

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
     * @brief Stan wygladu i wyposazenia wizualnego aktora.
     * Scisly rozmiar 32 bajty.
     */
    struct ActorAppearanceDTO
    {
        uint32_t vid{0};          // 0..3 (4B): VID aktora
        uint32_t armor_vnum{0};   // 4..7 (4B): VNUM zbroi / kostiumu
        uint32_t weapon_vnum{0};  // 8..11 (4B): VNUM broni / nakladki
        uint32_t hair_vnum{0};    // 12..15 (4B): VNUM fryzury
        uint32_t acce_vnum{0};    // 16..19 (4B): VNUM szarfy
        uint32_t mount_vnum{0};   // 20..23 (4B): VNUM konia / wierzchowca
        int16_t alignment{0};     // 24..25 (2B): Punkty rangi / karmy
        uint8_t pk_mode{0};       // 26 (1B): Tryb PK (0=Peace, 1=Revenge, 2=Free, 3=Guild)
        uint8_t guild_rank{0};    // 27 (1B): Ranga w gildii (0..15)
        uint32_t guild_id{0};     // 28..31 (4B): Identyfikator gildii
    };

    /**
     * @brief Stan animacji, ruchu i fizyki postaci.
     * Scisly rozmiar 24 bajty.
     */
    struct ActorMotionStateDTO
    {
        uint32_t vid{0};           // 0..3 (4B): VID aktora
        uint16_t motion_mode{0};   // 4..5 (2B): Tryb ruchu (0=Stand, 1=Walk, 2=Run, 3=Wait, 4=Damage)
        uint16_t motion_index{0};  // 6..7 (2B): Indeks animacji z pliku msa
        float rotation_deg{0.0f};  // 8..11 (4B): Aktualny kat obrotu w stopniach (0..360)
        float move_speed{100.0f};  // 12..15 (4B): Szybkosc poruszania sie
        float attack_speed{100.0f};// 16..19 (4B): Szybkosc animacji ataku
        uint8_t is_combo{0};       // 20 (1B): Flaga trwania sekwencji combo
        uint8_t is_loop{0};        // 21 (1B): Flaga zapetlenia animacji
        uint16_t _reserved{0};     // 22..23 (2B): Wyrownanie pamieciowe
    };

    // ========================================================================
    // DOMENA 2: GRACZ I PUNKTY STATUSU (PLAYER CORE STATE)
    // ========================================================================

    /**
     * @brief Glowny stan statystyk i zasobow gracza.
     * Scisly rozmiar 72 bajty.
     */
    struct PlayerStatsDTO
    {
        uint32_t vid{0};         // 0..3 (4B): VID glownej postaci
        int32_t level{1};        // 4..7 (4B): Poziom postaci
        int64_t hp{0};           // 8..15 (8B): Aktualne punkty PŻ
        int64_t max_hp{0};       // 16..23 (8B): Maksymalne punkty PŻ
        int64_t sp{0};           // 24..31 (8B): Aktualne punkty PE
        int64_t max_sp{0};       // 32..39 (8B): Maksymalne punkty PE
        int64_t yang{0};         // 40..47 (8B): Waluta Yang / zloto (64-bit)
        int64_t exp{0};          // 48..55 (8B): Punkty doswiadczenia
        int64_t next_exp{0};     // 56..63 (8B): Wymagane doswiadczenie na nastepny poziom
        int32_t cheque{0};       // 64..67 (4B): Waluta Won / czeki
        int32_t gem{0};          // 68..71 (4B): Waluta Gaya / klejnoty
    };

    /**
     * @brief Cechy podstawowe i parametry bojowe postaci.
     * Scisly rozmiar 32 bajty.
     */
    struct PlayerAttributesDTO
    {
        int16_t str{0};           // 0..1 (2B): Sila (STR)
        int16_t dex{0};           // 2..3 (2B): Zrecznosc (DEX)
        int16_t con{0};           // 4..5 (2B): Witalnosc (CON)
        int16_t int_pt{0};        // 6..7 (2B): Inteligencja (INT)
        int16_t min_atk{0};       // 8..9 (2B): Minimalna wartosc ataku
        int16_t max_atk{0};       // 10..11 (2B): Maksymalna wartosc ataku
        int16_t def{0};           // 12..13 (2B): Obrona fizyczna
        int16_t magic_def{0};     // 14..15 (2B): Obrona magiczna
        int16_t atk_speed{100};   // 16..17 (2B): Szybkosc ataku
        int16_t move_speed{100};  // 18..19 (2B): Szybkosc ruchu
        int16_t cast_speed{100};  // 20..21 (2B): Szybkosc rzucania czarow
        uint16_t status_flags{0}; // 22..23 (2B): Flagi afektow (otrucie, spowolnienie, omdlenie)
        uint32_t play_time_sec{0};// 24..27 (4B): Czas gry w sekundach
        uint32_t _reserved{0};    // 28..31 (4B): Wyrownanie do 32B
    };

    /**
     * @brief Pojedynczy slot paska szybkiego dostepu (Quickslot 1..32).
     * Scisly rozmiar 8 bajtow.
     */
    struct PlayerQuickslotDTO
    {
        uint8_t slot_type{0};   // 0 (1B): Typ slotu (0=Pusty, 1=Item, 2=Skill, 3=Command)
        uint8_t window_type{0}; // 1 (1B): Typ powiazanego okna
        uint16_t slot_index{0}; // 2..3 (2B): Indeks slotu na pasku
        uint32_t vnum{0};       // 4..7 (4B): VNUM przedmiotu lub umiejetnosci
    };

    // ========================================================================
    // DOMENA 3: EKWIPUNEK I PRZEDMIOTY (INVENTORY & ITEM STATE)
    // ========================================================================

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
     * @brief Podsumowanie siatki i zajetosci stron ekwipunku (bitmaski komorek).
     * Scisly rozmiar 32 bajty.
     */
    struct InventoryGridSummaryDTO
    {
        uint16_t total_slots{90};        // 0..1 (2B): Laczna liczba slotow ekwipunku
        uint16_t occupied_slots{0};      // 2..3 (2B): Liczba zajetych slotow
        uint32_t total_items_count{0};   // 4..7 (4B): Suma ilosci wszystkich przedmiotow
        uint64_t page_masks[3]{0, 0, 0}; // 8..31 (24B): Bitmaski zajetosci komorek dla stron 0, 1, 2 (po 64 bity)
    };

    // ========================================================================
    // DOMENA 4: UMIEJETNOSCI (SKILL STATE)
    // ========================================================================

    /**
     * @brief Stan pojedynczej umiejetnosci postaci.
     * Scisly rozmiar 24 bajty.
     */
    struct SkillSlotDTO
    {
        uint32_t vnum{0};                  // 0..3 (4B): VNUM umiejetnosci (np. 1 = Trzystronne Ciecie)
        uint8_t level{0};                  // 4 (1B): Poziom umiejetnosci (0..40)
        uint8_t master_type{0};            // 5 (1B): Stopien zaawansowania (0=Base, 1=Master, 2=GrandMaster, 3=Perfect)
        uint8_t can_use{1};                // 6 (1B): Czy skill moze byc obecnie aktywowany
        uint8_t is_toggled{0};             // 7 (1B): Czy skill ciagly/aura jest aktywny
        uint32_t sp_cost{0};               // 8..11 (4B): Koszt punktow many (PE)
        uint32_t cooldown_remaining_ms{0}; // 12..15 (4B): Pozostaly czas odnowienia w ms
        uint32_t cooldown_total_ms{0};     // 16..19 (4B): Calkowity czas odnowienia w ms
        uint32_t last_cast_time_ms{0};     // 20..23 (4B): Sygnatura czasowa ostatniego uzycia
    };

    // ========================================================================
    // DOMENA 5: SIEC I PROTOKOL (NETWORK SESSION & PACKETS)
    // ========================================================================

    /**
     * @brief Zunifikowany naglowek pakietu sieciowego.
     * Scisly rozmiar 8 bajtow.
     */
    struct PacketHeaderDTO
    {
        uint16_t opcode{0};   // 0..1 (2B): Kod operacji pakietu (Opcode)
        uint16_t sequence{0}; // 2..3 (2B): Numer sekwencyjny pakietu
        uint32_t size{0};     // 4..7 (4B): Calkowity rozmiar tresci pakietu w bajtach
    };

    /**
     * @brief Telemetria i stan polaczenia sieciowego klienta.
     * Scisly rozmiar 32 bajty.
     */
    struct NetworkSessionDTO
    {
        uint32_t phase{0};              // 0..3 (4B): Faza polaczenia (0=Off, 1=Handshake, 2=Login, 3=Select, 4=Load, 5=Game)
        uint32_t ping_ms{0};            // 4..7 (4B): Ostatni zmierzony czas RTT w ms
        uint32_t last_recv_time_ms{0};  // 8..11 (4B): Czas ostatniego odebranego bajtu
        uint32_t last_send_time_ms{0};  // 12..15 (4B): Czas ostatniego wyslanego bajtu
        uint64_t bytes_received{0};     // 16..23 (8B): Laczna liczba odebranych bajtow
        uint64_t bytes_sent{0};         // 24..31 (8B): Laczna liczba wyslanych bajtow
    };

    // ========================================================================
    // DOMENA 6: AKCJE BOJOWE I NAWIGACJA (COMBAT & NAVIGATION)
    // ========================================================================

    /**
     * @brief Polecenie wykonania akcji bojowej przez silnik klienta.
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

    // ========================================================================
    // DOMENA 7: TEREN, SWIAT I FIZYKA (WORLD & TERRAIN STATE)
    // ========================================================================

    /**
     * @brief Wynik probkowania fizyki i wysokosci terenu w danym punkcie swiata.
     * Scisly rozmiar 32 bajty.
     */
    struct TerrainQueryDTO
    {
        float world_x{0.0f};     // 0..3 (4B): Wspolrzedna X punktu
        float world_y{0.0f};     // 4..7 (4B): Wspolrzedna Y punktu
        float height_z{0.0f};    // 8..11 (4B): Wyliczona wysokosc terenu (wysokosc siatki)
        float normal_x{0.0f};    // 12..15 (4B): Wektor normalny powierzchni X
        float normal_y{0.0f};    // 16..19 (4B): Wektor normalny powierzchni Y
        float normal_z{1.0f};    // 20..23 (4B): Wektor normalny powierzchni Z
        uint32_t attr_flags{0};  // 24..27 (4B): Atrybuty terenu (1=Block, 2=Water, 4=NoPK, 8=Bridge)
        uint8_t is_walkable{1};  // 28 (1B): Flaga czy punkt jest osiagalny dla pieszego ruchu
        uint8_t _pad[3]{0, 0, 0};// 29..31 (3B): Wyrownanie do granicy 4 bajtow
    };

    /**
     * @brief Stan oswietlenia, pory dnia i srodowiska mapy.
     * Scisly rozmiar 64 bajty.
     */
    struct WorldEnvironmentDTO
    {
        uint32_t map_id{0};           // 0..3 (4B): Identyfikator mapy (hash nazwy folderu)
        float time_of_day{12.0f};     // 4..7 (4B): Pora dnia w godzinach (0.0 .. 24.0)
        float fog_near{2400.0f};      // 8..11 (4B): Dystans poczatku mgly
        float fog_far{12000.0f};      // 12..15 (4B): Dystans konca mgly
        float sun_direction[3]{0.0f, 0.0f, -1.0f}; // 16..27 (12B): Wektor kierunku slonca (XYZ)
        float ambient_color[4]{0.4f, 0.4f, 0.4f, 1.0f}; // 28..43 (16B): Kolor swiatla rozproszonego (RGBA)
        float sun_color[4]{1.0f, 1.0f, 1.0f, 1.0f};     // 44..59 (16B): Kolor swiatla kierunkowego (RGBA)
        uint32_t _reserved{0};        // 60..63 (4B): Wyrownanie pamieciowe do 64B
    };

    // ========================================================================
    // DOMENA 8: OKNA I GEOMETRIA GUI (WINDOW MANAGER STATE)
    // ========================================================================

    /**
     * @brief Geometria, hierarchia i widocznosc pojedynczego okna interfejsu.
     * Scisly rozmiar 32 bajty.
     */
    struct WindowGeometryDTO
    {
        uint32_t window_id{0};    // 0..3 (4B): Unikalny uchwyt / ID okna w CWindowManager
        uint32_t parent_id{0};    // 4..7 (4B): ID okna nadrzednego (0 = RootWindow)
        int32_t left{0};          // 8..11 (4B): Wspolrzedna lewej krawedzi
        int32_t top{0};           // 12..15 (4B): Wspolrzedna gornej krawedzi
        int32_t right{0};         // 16..19 (4B): Wspolrzedna prawej krawedzi
        int32_t bottom{0};        // 20..23 (4B): Wspolrzedna dolnej krawedzi
        uint32_t flags{0};        // 24..27 (4B): Flagi zachowania okna (Movable, Limit, Float itp.)
        uint8_t is_show{0};       // 28 (1B): Czy okno jest widoczne (Show/Hide)
        uint8_t is_rendering{0};  // 29 (1B): Czy okno jest aktualnie renderowane
        uint8_t window_type{0};   // 30 (1B): Typ kontrolki (0=Base, 1=Button, 2=TextLine, 3=Slot)
        uint8_t _pad{0};          // 31 (1B): Wyrownanie do 32 bajtow
    };

    // ========================================================================
    // DOMENA 9: ASYNCHRONICZNA SZYNA ZDARZEN (GAME EVENT BUS)
    // ========================================================================

    /**
     * @brief Uniwersalny rekord zdarzenia gry przekazywany do szyny EventBus.
     * Umozliwia odseparowanie logiki gry od interpretera Pythona.
     * Scisly rozmiar 32 bajty.
     */
    struct GameEventDTO
    {
        uint32_t event_type{0};   // 0..3 (4B): Typ zdarzenia (1=Damage, 2=Exp, 3=ItemAdd, 4=ItemDel, 5=Chat, 6=Quest)
        uint32_t timestamp_ms{0}; // 4..7 (4B): Sygnatura czasowa w milisekundach
        uint32_t source_vid{0};   // 8..11 (4B): VID nadawcy / zrodla zdarzenia
        uint32_t target_vid{0};   // 12..15 (4B): VID odbiorcy / celu
        int64_t value1{0};        // 16..23 (8B): Glowna wartosc liczbowa (np. ilosc obrazen, Yang, Exp)
        int32_t value2{0};        // 24..27 (4B): Pomocnicza wartosc liczbowa (np. flaga krytyka, typ ataku)
        uint32_t flag{0};         // 28..31 (4B): Dodatkowe binarne flagi zdarzenia
    };

#pragma pack(pop)

    // ========================================================================
    // STATYCZNE ASERCJE C++23 POD, STANDARD LAYOUT I ROZMIAROW (C-ABI GUARANTEE)
    // ========================================================================

    static_assert(sizeof(ActorSnapshot) == 32, "ActorSnapshot musi miec dokladnie 32 bajty");
    static_assert(std::is_standard_layout_v<ActorSnapshot>, "ActorSnapshot musi byc standard layout");
    static_assert(std::is_trivially_copyable_v<ActorSnapshot>, "ActorSnapshot musi byc trivially copyable");

    static_assert(sizeof(ActorAppearanceDTO) == 32, "ActorAppearanceDTO musi miec dokladnie 32 bajty");
    static_assert(std::is_standard_layout_v<ActorAppearanceDTO>, "ActorAppearanceDTO musi byc standard layout");
    static_assert(std::is_trivially_copyable_v<ActorAppearanceDTO>, "ActorAppearanceDTO musi byc trivially copyable");

    static_assert(sizeof(ActorMotionStateDTO) == 24, "ActorMotionStateDTO musi miec dokladnie 24 bajty");
    static_assert(std::is_standard_layout_v<ActorMotionStateDTO>, "ActorMotionStateDTO musi byc standard layout");
    static_assert(std::is_trivially_copyable_v<ActorMotionStateDTO>, "ActorMotionStateDTO musi byc trivially copyable");

    static_assert(sizeof(PlayerStatsDTO) == 72, "PlayerStatsDTO musi miec dokladnie 72 bajty");
    static_assert(std::is_standard_layout_v<PlayerStatsDTO>, "PlayerStatsDTO musi byc standard layout");
    static_assert(std::is_trivially_copyable_v<PlayerStatsDTO>, "PlayerStatsDTO musi byc trivially copyable");

    static_assert(sizeof(PlayerAttributesDTO) == 32, "PlayerAttributesDTO musi miec dokladnie 32 bajty");
    static_assert(std::is_standard_layout_v<PlayerAttributesDTO>, "PlayerAttributesDTO musi byc standard layout");
    static_assert(std::is_trivially_copyable_v<PlayerAttributesDTO>, "PlayerAttributesDTO musi byc trivially copyable");

    static_assert(sizeof(PlayerQuickslotDTO) == 8, "PlayerQuickslotDTO musi miec dokladnie 8 bajtow");
    static_assert(std::is_standard_layout_v<PlayerQuickslotDTO>, "PlayerQuickslotDTO musi byc standard layout");
    static_assert(std::is_trivially_copyable_v<PlayerQuickslotDTO>, "PlayerQuickslotDTO musi byc trivially copyable");

    static_assert(sizeof(ItemAttributeDTO) == 8, "ItemAttributeDTO musi miec dokladnie 8 bajtow");
    static_assert(std::is_standard_layout_v<ItemAttributeDTO>, "ItemAttributeDTO musi byc standard layout");
    static_assert(std::is_trivially_copyable_v<ItemAttributeDTO>, "ItemAttributeDTO musi byc trivially copyable");

    static_assert(sizeof(ItemSlotDTO) == 104, "ItemSlotDTO musi miec dokladnie 104 bajty");
    static_assert(std::is_standard_layout_v<ItemSlotDTO>, "ItemSlotDTO musi byc standard layout");
    static_assert(std::is_trivially_copyable_v<ItemSlotDTO>, "ItemSlotDTO musi byc trivially copyable");

    static_assert(sizeof(InventoryGridSummaryDTO) == 32, "InventoryGridSummaryDTO musi miec dokladnie 32 bajty");
    static_assert(std::is_standard_layout_v<InventoryGridSummaryDTO>, "InventoryGridSummaryDTO musi byc standard layout");
    static_assert(std::is_trivially_copyable_v<InventoryGridSummaryDTO>, "InventoryGridSummaryDTO musi byc trivially copyable");

    static_assert(sizeof(SkillSlotDTO) == 24, "SkillSlotDTO musi miec dokladnie 24 bajty");
    static_assert(std::is_standard_layout_v<SkillSlotDTO>, "SkillSlotDTO musi byc standard layout");
    static_assert(std::is_trivially_copyable_v<SkillSlotDTO>, "SkillSlotDTO musi byc trivially copyable");

    static_assert(sizeof(PacketHeaderDTO) == 8, "PacketHeaderDTO musi miec dokladnie 8 bajtow");
    static_assert(std::is_standard_layout_v<PacketHeaderDTO>, "PacketHeaderDTO musi byc standard layout");
    static_assert(std::is_trivially_copyable_v<PacketHeaderDTO>, "PacketHeaderDTO musi byc trivially copyable");

    static_assert(sizeof(NetworkSessionDTO) == 32, "NetworkSessionDTO musi miec dokladnie 32 bajty");
    static_assert(std::is_standard_layout_v<NetworkSessionDTO>, "NetworkSessionDTO musi byc standard layout");
    static_assert(std::is_trivially_copyable_v<NetworkSessionDTO>, "NetworkSessionDTO musi byc trivially copyable");

    static_assert(sizeof(CombatCommandDTO) == 28, "CombatCommandDTO musi miec dokladnie 28 bajtow");
    static_assert(std::is_standard_layout_v<CombatCommandDTO>, "CombatCommandDTO musi byc standard layout");
    static_assert(std::is_trivially_copyable_v<CombatCommandDTO>, "CombatCommandDTO musi byc trivially copyable");

    static_assert(sizeof(NavigationWaypointDTO) == 28, "NavigationWaypointDTO musi miec dokladnie 28 bajtow");
    static_assert(std::is_standard_layout_v<NavigationWaypointDTO>, "NavigationWaypointDTO musi byc standard layout");
    static_assert(std::is_trivially_copyable_v<NavigationWaypointDTO>, "NavigationWaypointDTO musi byc trivially copyable");

    static_assert(sizeof(TerrainQueryDTO) == 32, "TerrainQueryDTO musi miec dokladnie 32 bajty");
    static_assert(std::is_standard_layout_v<TerrainQueryDTO>, "TerrainQueryDTO musi byc standard layout");
    static_assert(std::is_trivially_copyable_v<TerrainQueryDTO>, "TerrainQueryDTO musi byc trivially copyable");

    static_assert(sizeof(WorldEnvironmentDTO) == 64, "WorldEnvironmentDTO musi miec dokladnie 64 bajty");
    static_assert(std::is_standard_layout_v<WorldEnvironmentDTO>, "WorldEnvironmentDTO musi byc standard layout");
    static_assert(std::is_trivially_copyable_v<WorldEnvironmentDTO>, "WorldEnvironmentDTO musi byc trivially copyable");

    static_assert(sizeof(WindowGeometryDTO) == 32, "WindowGeometryDTO musi miec dokladnie 32 bajty");
    static_assert(std::is_standard_layout_v<WindowGeometryDTO>, "WindowGeometryDTO musi byc standard layout");
    static_assert(std::is_trivially_copyable_v<WindowGeometryDTO>, "WindowGeometryDTO musi byc trivially copyable");

    static_assert(sizeof(GameEventDTO) == 32, "GameEventDTO musi miec dokladnie 32 bajty");
    static_assert(std::is_standard_layout_v<GameEventDTO>, "GameEventDTO musi byc standard layout");
    static_assert(std::is_trivially_copyable_v<GameEventDTO>, "GameEventDTO musi byc trivially copyable");

} // namespace Client::Contracts
