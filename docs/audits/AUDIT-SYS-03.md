# RAPORT Z AUDYTU JAKOSCI: AUDIT-SYS-03 (CoreEventBus & Thread Safety)

## 1. Podsumowanie architektury zbadanych modulow
Zbadane moduly w katalogu `src/Client/Core/` koncentruja sie na zarzadzaniu stanem swiata (`WorldContext`), asynchronicznym dystrybuowaniu zdarzen (`CoreEventBus`, `QueuedEventDispatcher`), oraz podstawowych operacjach bezpiecznego dostepu do pamieci strumieni binarnych (`ByteSpan`).
- **CoreEventBus**: Szyna zdarzen implementujaca wzorzec Publisher-Subscriber. Kod wykorzystuje kanaly przypisane do typow zdarzen z limitem subskrybentow opartym o struktury `std::array`.
- **QueuedEventDispatcher**: Kolejka do przerzucania zadan (w postaci `std::function`) miedzy watkami (np. z watku sieci na glowny watek silnika).
- **WorldContext**: Glowne centrum integracji domen gry (Inwentarz, Statystyki, Umiejetnosci, Spatial Hash Grid). Zapewnia ochronę wspolbieznego dostepu uzywajac masywnego `std::shared_mutex`.
- **ByteSpan / SIMDBoundsChecker**: Warstwa narzedziowa implementujaca odczyty w postaci *zero-copy* (`std::span`, `std::bit_cast`) oraz wektoryzowane kalkulacje geometryczne.

## 2. Zestawienie zidentyfikowanych defektow kodu i niespojnosci

### A. Wiszace referencje i brak cyklu zycia subskrybenta (RAII & Memory Lifetime)
- **Plik**: `src/Client/Core/CoreEventBus.h` (Linie 93, 115)
- **Opis mechanizmu**: System subskrypcji przechowuje surowe wskazniki (`IEventHandler<EventType>* handler`) jako wlascicielskie/slabe bez zadnego zmechanizowanego zarzadzania cyklem zycia. Jesli obiekt subskrybenta (np. element UI) ulegnie destrukcji bez uprzedniego wywolania `Unsubscribe`, w systemie pozostanie wiszaca referencja. Zostanie ona zderferencjowana w metodzie `Publish`, naruszajac pamiec.
- **Wplyw na stabilnosc**: KRYTYCZNY. Bardzo wysokie ryzyko awarii typu Segmentation Fault w dynamicznym srodowisku (np. przelaczanie map).

### B. Ryzyko Re-entrancy w wywolaniach zwrotnych (Thread Safety & Synchronization)
- **Plik**: `src/Client/Core/CoreEventBus.h` (Linia 166-170)
- **Opis mechanizmu**: `Publish` tworzy lokalna kopie tablicy subskrybentow pod blokada muteksu, aby pominac dead-lock podczas iteracji. Jeśli podpiety `OnEvent()` wykona re-subskrypcję lub modyfikację (re-entrancy), zmodyfikuje wylacznie strukture macierzysta, ale proces iteracji na buforze lokalnym moze wykonac destrukcje, co w skrajnym przypadku wciaz moze prowadzic do niespojnosci logiki gry.
- **Wplyw na stabilnosc**: SREDNI.

### C. Zbedne alokacje dynamiczne na sciezce krytycznej (Performance Optimization)
- **Plik**: `src/Client/Core/QueuedEventDispatcher.h` (Linia 53, 93)
- **Opis mechanizmu**: `EnqueueTask` uzywa `std::function` jako elementow bufora `std::vector`, co moze powodowac ukryte alokacje (type-erasure heap allocation, jesli domkniecie lambda jest duze). Przetwarzanie wywoluje blokade i relokacje kontenerow w glownej petli.
- **Wplyw na stabilnosc**: SREDNI. Spadki framerate'u (stuttering) w przypadku zmasowanego naplywu danych z watku sieci.

### D. Brak granularnosci blokad w kontekscie stanow (Thread Safety & State Determinism)
- **Plik**: `src/Client/Core/WorldContext.cpp` (Linie 47, 85, 236)
- **Opis mechanizmu**: Klasa chroniona jest jednym potężnym `std::shared_mutex m_contextMutex`. Kiedy uzytkownik podnosi przedmiot (`BindQuickslotItem`), wywolywany jest system Ekwipunku (`inventory.GetItem`), który moze uzywac wlasnych wewnetrznych blokad. Stwarza to ryzyko Deadlocka (inwersja blokad) i paraliżuje watek renderera przy dluzszym query na pozycjach sfery przestrzennej.
- **Wplyw na stabilnosc**: KRYTYCZNY. Stan gry traci na spojnosci; w wypadku obciazenia wydajnosc wielowatkowa drastycznie spada.

### E. Odpornosc parsera na architektury docelowe (Protocol Stream Robustness)
- **Plik**: `src/Client/Core/ByteSpan.h` (Linia 46, 83)
- **Opis mechanizmu**: Uzyskiwanie prymitywow realizowane jest bezpiecznym `std::bit_cast<T>`. Jednak parser nie weryfikuje endianness (kolejnosci bajtow). Jesli infrastruktura docelowa (serwer/klient) różni sie kolejnoscią, odczyty `uint32_t` spowodują ogromne wartości (signed/unsigned mismatches, partial packet truncation) psując determinizm.
- **Wplyw na stabilnosc**: SREDNI.

## 3. Wzorcowa propozycja refaktoryzacji (Rust & C++23)

### Eliminacja surowych wskaznikow - mechanizm RAII (Rust / C++23)
W celu zapobiegniecia wiszacym referencjom w zdarzeniach, powinnismy odwrocic zaleznosci uzywajac inteligentnych uchwytow, tzw. `SubscriptionHandle`. Gdy wlasciciel (UI/Gra) ulega destrukcji, uchwyt rowniez ulega destrukcji, co pociaga za soba automatyczne wylogowanie z szyny (RAII).

**Propozycja (Rust):**
```rust
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

type EventHandler<T> = Box<dyn Fn(&T) + Send + Sync>;

pub struct EventChannel<T> {
    subscribers: Arc<RwLock<HashMap<u32, EventHandler<T>>>>,
}

pub struct SubscriptionHandle<T> {
    channel: Arc<RwLock<HashMap<u32, EventHandler<T>>>>,
    id: u32,
}

impl<T> Drop for SubscriptionHandle<T> {
    fn drop(&mut self) {
        if let Ok(mut subs) = self.channel.write() {
            subs.remove(&self.id);
        }
    }
}
```

**Odpowiednik (C++23):**
Zastapic `IEventHandler*` przez `std::weak_ptr<IEventHandler>`. Zanim wykona sie wywolanie w lokalnej pętli, bus sprawdza waznosc slabej referencji:
```cpp
if (auto lockedHandler = localSubscribers[i].handler.lock()) {
    lockedHandler->OnEvent(event);
}
```

### Optymalizacja alokacji dyspozytora zdarzen (Wektoryzacja & Ring Buffer)
Dla `QueuedEventDispatcher` nalezaloby zrezygnowac z ukrytych alokacji sterty:
1. Wprowadzic lock-free SPSC / MPSC ring buffer (np. z uzyciem zmiennych atomowych `std::atomic<size_t>`).
2. Zadania o powtarzajacym sie charakterze realizowac przez wzorzec Command z wlasnym wewnetrznym alokatorem pamieci basenowej (Pool Allocator) zamiast `std::function`.
3. Podzial muteksu `WorldContext` - wprowadzic osobny zamek r/w (RWLock) do siatki przestrzennej i osobny do logiki ekwipunku, eliminujac blokady calego serwera podczas obrabiania statystyk gracza.
