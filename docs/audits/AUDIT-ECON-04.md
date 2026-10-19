# AUDIT-ECON-04 — Private Shop & Tax Verification

## 1. Podsumowanie Architektury

Zbadane moduły (`src/UserInterface/Domain/Shop*`, `src/UserInterface/PythonNetworkStreamPhaseGameShop.cpp`, `src/UserInterface/PythonShop.cpp`, `src/Client/Network/ShopPacketCodec.cpp`, `src/UserInterface/Network/Handlers/*`) realizują funkcjonalności sklepu w kliencie gry, implementując odbiór pakietów sieciowych oraz synchronizację ze stanem interfejsu (w języku Python) i pamięcią natywną (w C++). 

Obserwowalny jest proces przechodzenia w stronę nowoczesnego C++ (C++20/C++23) - zauważyć można użycie `std::span`, `std::expected` (w `ShopPacketCodec` i `ShopInventoryModel`) oraz wprowadzenie Event Busa (`EventBus.h`) w celu de-coupling'u warstwy UI i logiki.
Z drugiej strony wciąż w kodzie znajdują się starsze mechanizmy oparte na języku C (surowe rzutowania `reinterpret_cast`, struktury POD kopiowane przez `memcpy`, niebezpieczne operatory C array i starsze typy zdefiniowane w Windows API typu `DWORD`, `BOOL`, `BYTE`).

## 2. Zestawienie Defektów i Niespójności

### A. Zarządzanie Zasobami i RAII (Memory Lifetime)
- **Plik**: `src/UserInterface/PythonNetworkStreamPhaseGameShop.cpp`
  - **Linia**: 66 (`for (unsigned char i = 0; i < shop_tab_count; i++) ...`) i pobieranie wskaźnika `pPackTab`.
  - **Opis**: Kopiowanie wartości ze wskaźnika na strukturę pakietu. Brak wykorzystania standardowych bezpiecznych rzutowań czy span-ów przy iterowaniu po strukturach danych zmiennej wielkości, w skutek czego występuje arytmetyka wskaźników z podatnością na UB.
  - **Wpływ**: Potencjalny błąd przekroczenia granicy (buffer overread), jeśli serwer prześle niekompletne dane co mogłoby spowodować wyciek danych lub błąd segfault.

- **Plik**: `src/UserInterface/PythonShop.cpp`
  - **Linia**: 91 (`for (int i = 0; i < SHOP_TAB_COUNT_MAX; i++) memset (m_aShoptabs[i].items, 0, sizeof(TShopItemData) * SHOP_HOST_ITEM_MAX_NUM);`)
  - **Opis**: Użycie `memset` na złożonej strukturze posiadającej niewiadomy (potencjalnie zmienny w przyszłości) typ wewnętrzny lub typy składowe, takie jak np. wektory.
  - **Wpływ**: Niszczenie obiektów lub naruszenie poprawności wirtualnych wskaźników VTable lub typów złożonych (std::string) prowadzi do wycieków pamięci lub błędów uszkodzenia sterty.

### B. Poprawność Współbieżności i Synchronizacja (Thread Safety)
- **Plik**: `src/UserInterface/PythonShop.cpp` & `src/UserInterface/Network/Handlers/ShopHandler.cpp`
  - **Opis**: Modyfikacja stanu singletona `CPythonShop::Instance()` bezpośrednio z przestrzeni handlerów sieciowych bez zabezpieczeń przed re-entrancy (muteksów, atomics).
  - **Wpływ**: Jeśli odbieranie pakietów jest realizowane w osobnym wątku sieciowym, a renderowanie GUI z PythonShop w wątku głównym, powoduje to data race. Nawet jeśli architektura jest jednowątkowa, rekurencyjne wywołania zwrotne z Pythona mogą wywołać niestabilny stan.

### C. Odporność Parsera (Protocol Stream Robustness)
- **Plik**: `src/UserInterface/PythonNetworkStreamPhaseGameShop.cpp`
  - **Linia**: 61 (`size_t read_point = sizeof(TPacketGCShopStartEx);`)
  - **Opis**: Parsowanie "zmiennej ilości zakładek". Obliczanie `read_point += sizeof(TPacketGCShopStartEx::TSubPacketShopTab)` bez weryfikacji wyrównania pamięci (alignment) ani użycia dedykowanych deserializatorów czy bounds-checked spanów.
  - **Wpływ**: Atak DoS polegający na manipulowaniu strukturą nagłówka ramki może spowodować panikę klienta (zbyt duży pakiet przekraczający bufor).
- **Plik**: `src/UserInterface/Network/Handlers/ShopSignHandler.cpp`
  - **Linia**: 18 (`std::string_view signView(packet->szSign, strnlen(packet->szSign, sizeof(packet->szSign)));`)
  - **Opis**: To jest poprawny wzorzec, jednakże w `PythonNetworkStreamPhaseGameShop.cpp` (starsza wersja kodu `HandleShopSign`) dalej istnieje kod: `if (0 == strlen(p.szSign))`. 
  - **Wpływ**: Jeśli ciąg znaków w surowym buforze z sieci nie jest zakończony bajtem null, `strlen` wejdzie w poza zaalokowaną pamięć bufora sieciowego (OOB read).

### D. Determinizm Logiki i Atomowość Operacji
- **Plik**: `src/UserInterface/PythonNetworkStreamPhaseGameShop.cpp`
  - **Linia**: 78 (`CPythonShop::Instance().SetItemData(i, j, *itemData);`)
  - **Opis**: Aktualizacja stanu po kolei, element po elemencie. Jeśli wystąpi wyjątek lub niepełna odpowiedź podczas iteracji, sklep zostanie wprowadzony w niespójny, pośrodkowy stan.
  - **Wpływ**: Niepełne dane inwentarza lub artefakty interfejsu (tzw. ghost items).

### E. Wydajność na Ścieżce Krytycznej (Performance)
- **Plik**: `src/UserInterface/PythonShop.cpp`
  - **Linia**: 32 (`std::sort(ItemStock.begin(), ItemStock.end(), ItemStockSortFunc());`)
  - **Opis**: Alokowanie i kopiowanie wektora lokalnego przy każdej budowie sklepu i to przy użyciu funktorów. 
  - **Wpływ**: Degradacja wydajności na ścieżce renderu klatki (jeśli jest to powiązane z renderem). Alokacja na stercie w pętli UI.

## 3. Wzorcowa Propozycja Refaktoryzacji (C++23)

Przejście z singletona modyfikowanego współbieżnie do nowoczesnego event-based approach ze state-machine przy użyciu strumieni `std::span` i `std::expected`.

```cpp
#include <span>
#include <vector>
#include <optional>
#include <expected>
#include <string_view>

// 1. Zabezpieczony Deserializator (Eliminacja memcpy / strlen)
template <typename T>
std::expected<T, std::string_view> SafeExtract(std::span<const uint8_t>& buffer) {
    if (buffer.size_bytes() < sizeof(T)) {
        return std::unexpected("Buffer underflow");
    }
    T result;
    // std::memcpy jest nadal bezpieczne dla trivially_copyable types,
    // lepsze by bylo uzycie std::bit_cast w przypadku statycznych rozmiarem struktur
    // jesli T posiada konstruktor lub vtable, to static_assert nie pozwoli
    static_assert(std::is_trivially_copyable_v<T>, "Type must be trivially copyable");
    std::memcpy(&result, buffer.data(), sizeof(T));
    buffer = buffer.subspan(sizeof(T));
    return result;
}

// 2. Transakcyjność - Budowa tymczasowego stanu sklepu przed commit'em do UI
std::expected<ShopStartEvent, std::string_view> ShopHandler::HandleShopStartEx(std::span<const uint8_t> payload) {
    auto startPacketRes = SafeExtract<TPacketGCShopStartEx>(payload);
    if (!startPacketRes) {
        return std::unexpected("Failed to extract TPacketGCShopStartEx");
    }
    const auto& startPacket = *startPacketRes;
    
    // Uzycie wektora w celu zebrania calej zawartosci przed podmiana stanu (atomowosc)
    std::vector<ShopTab> pendingTabs;
    pendingTabs.reserve(startPacket.shop_tab_count);

    for (uint8_t i = 0; i < startPacket.shop_tab_count; ++i) {
        auto tabRes = SafeExtract<TPacketGCShopStartEx::TSubPacketShopTab>(payload);
        if (!tabRes) {
            return std::unexpected("Tab extraction failed, packet fragmented");
        }
        
        ShopTab newTab;
        newTab.coinType = tabRes->coin_type;
        // Bezpieczne operacje na string_view zabezpieczajace przed brakiem NULL-terminatora
        newTab.name = std::string_view(tabRes->name, strnlen(tabRes->name, sizeof(tabRes->name)));

        for(const auto& itemData : tabRes->items) {
             newTab.items.push_back(itemData);
        }
        pendingTabs.push_back(std::move(newTab));
    }

    // Atomowa aktualizacja calego modelu
    auto& model = ShopInventoryModel::Instance();
    std::scoped_lock lock(model.GetMutex()); // Thread Safety
    model.SetTabs(std::move(pendingTabs));   // Brak niepotrzebnego kopiowania (Move Semantics)
    
    // Publikacja zdarzenia przez EventBus
    return ShopStartEvent{ startPacket.owner_vid };
}
```

Dalsze zalecenia:
- **Rust Integration**: Kod obsługujący pakiety i stany mógłby być przepisany w języku Rust, używając biblioteki `cxx` do eksponowania API dla klienta, gdzie sam Rust by gwarantował brak memory-leaków, buffer overflow i data race.
- **Odśmiecanie z `memset`**: Całkowicie wyeliminować `memset` z klas zawierających obiekty takie jak `std::string` na rzecz `std::ranges::fill` lub funkcji resetujących.
