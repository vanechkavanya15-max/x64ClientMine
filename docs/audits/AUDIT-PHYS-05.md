# AUDIT-PHYS-05: Ground Drop Gravity Physics

## 1. Podsumowanie architektury zbadanych modułów

Moduł `GroundDrop` (znajdujący się w `src/UserInterface/GroundDrop/`) odpowiada za logikę biznesową, renderowanie oraz symulację fizyki dla przedmiotów upuszczanych na ziemię.
Architektura opiera się na wydzielonych implementacjach interfejsu `IDropPhysicsSimulator` (np. `DropPhys_GravityStep.cpp`, `DropPhys_GroundBounce.cpp`, `DropPhys_ScatterRing.cpp`), które symulują trajektorie, oraz `IGroundDropBatchRenderer`, które zarządzają stanem obiektów renderowalnych na ziemi (np. `GroundDrop_Add.cpp`, `GroundDrop_Remove.cpp`, `GroundDrop_MeshInstancing.cpp`, `GroundDrop_FrustumCull.cpp`).

Komunikacja między komponentami fizyki i logiki UI realizowana jest asynchronicznie za pomocą szyny zdarzeń (`EventBus`). Zastosowano także mechanizmy zliczania czasu życia dropów (`GroundDrop_ExpirationTimer.cpp`). Kod posługuje się w wielu miejscach semantyką z C++23, np. `std::expected` (aliasy `EterBase::Result`, `EterBase::PacketResult`) oraz sztywnym typowaniem ID za pomocą `EterBase::EntityId` dla identyfikatorów z wirtualnej reprezentacji (virtualId).

## 2. Zestawienie zidentyfikowanych defektów kodu i niespójności

### Defekt 1: Podział stanu powodujący niespójności
* **Plik:** `src/UserInterface/GroundDrop/DropPhys_GravityStep.cpp`, `src/UserInterface/GroundDrop/DropPhys_GroundBounce.cpp`, `src/UserInterface/GroundDrop/DropPhys_ScatterRing.cpp`
* **Linia:** Deklaracje map przechowujących stany (np. `std::unordered_map<EterBase::EntityId, DropTrajectory> trajectories_`)
* **Mechanizm:** Poszczególne instancje fizyki (`IDropPhysicsSimulator`) wewnętrznie śledzą stan fizyczny używając słownika. Równolegle obiekty te są śledzone przez instancje implementacji `IGroundDropBatchRenderer` (np. `std::unordered_map<uint32_t, GroundDropItemData> drops_;` w `GroundDrop_FrustumCull.cpp`, czy wektory w innych wariantach).
* **Wpływ na stabilność (Atomowość i determinizm):** Stan przedmiotu na ziemi jest zduplikowany w kilku różnych podsystemach bez jasnego, centralnego SSOT (Single Source Of Truth). Prowadzi to do niespójności (np. ghost items) po asynchronicznej manipulacji lub rozłączeniu, oraz błędów przy czyszczeniu (ktoś może zapomnieć wywołać `ClearAll` w jednym z menedżerów).

### Defekt 2: Wycieki logiki w operacjach pętli update
* **Plik:** `src/UserInterface/GroundDrop/GroundDrop_ExpirationTimer.cpp`
* **Linia:** 92-105
* **Mechanizm:** Usuwanie z `m_timers` polega na zebraniu ID zgasłych timerów do tymczasowego wektora, a potem zastosowaniu `std::remove_if` i usunięciu ich z `m_timers`. Następnie wykonywana jest publikacja zdarzeń w pętli opierającej się na tymczasowym wektorze wygasłych ID.
* **Wpływ na wydajność (Alokacje):** Tworzenie wektora `expiredIds` w każdej klatce update'u, w której jakiś obiekt wygasa, wymusza niepotrzebną alokację dynamiczną na stercie w pętli krytycznej klatki.
* **Rozwiązanie (Wydajność w pętli krytycznej):** Proces wygasania można zaimplementować jednoprzebiegowo z użyciem semantyki in-place `std::erase_if`, przetwarzając callback/zdarzenie w trakcie ewaluacji, chociaż tu z uwagi na Zero-Conflict z `EventBus` można posłużyć się strukturami typu flat map/vektor o wstępnej pojemności alokacji. Można też połączyć odliczanie z główną strukturą.

### Defekt 3: Zduplikowany zarządca renderera a `std::vector`
* **Plik:** `src/UserInterface/GroundDrop/GroundDrop_Add.cpp`, `GroundDrop_Remove.cpp`, `GroundDrop_MeshInstancing.cpp`, etc.
* **Linia:** Zależna od pliku, np. `m_drops.push_back(item);` kontra w innych `m_drops[item.virtualId] = item;`
* **Mechanizm:** Zidentyfikowano rozrzucenie logiki operacji na `GroundDropItemData` poprzez wiele implementacji implementujących `IGroundDropBatchRenderer`, tworzonych z użyciem wzorca strategii i wyodrębnienia (jak np. `CreateGroundDropBatchRendererAddImpl`, `CreateGroundDropBatchRendererRemoveImpl`).
* **Wpływ na bezpieczeństwo współbieżności i wydajność:** O ile `GroundDrop_Remove.cpp` zamyka się w w muteksie (`std::lock_guard<std::mutex> lock(m_mutex);`), inne implementacje (np. w `GroundDrop_Add.cpp` czy `GroundDrop_MeshInstancing.cpp`) już tego nie robią. Ponieważ jest to system modułowy pracujący pomiędzy wątkiem sieci i główną pętlą (Network thread / Renderer thread), takie mieszanie powoduje **Race Conditions** w wektorach pod spodem. Złożoność jest w niektórych przypadkach O(N) podczas usuwania przez `virtualId` wektorów per klatka.

### Defekt 4: Niekonsystentne rzutowanie i sprawdzanie granic typów
* **Plik:** `src/UserInterface/GroundDrop/GroundDrop_GroundSnap.cpp`, `DropPhys_ParticlePuff.cpp` i inne
* **Mechanizm:** Zdarzenia i menedżery przesyłają surowe `uint32_t virtualId`, a inne silnie typowane `EterBase::EntityId virtualId`. Podczas odbierania lub serializacji po stronie parsera / EventBusa nie upewnia się granic ani przepełnień. Wyciąganie wartości jako `id.value()` lub iteracja `auto& [id, traj]` wymusza niekontrolowane niejawne lub jawne kastowania pomiędzy warstwą fizyki a widoku.
* **Wpływ na stabilność (Odporność na fragmentację):** Błędne parsowanie identyfikatorów połączone z ucinaniem wartości (signed/unsigned mismatches w koordynatach `int32_t` w sieci, a `float` w fizyce). Brak rygoru C++23.

---

## 3. Wzorcowa propozycja refaktoryzacji do nowoczesnego C++23 / Rust

### Strategia do wdrożenia w C++23
Głównym problemem jest rozwarstwienie dostępu do stanu (State Duplication) i brak ochrony (Data Races / Thread Safety).
Należy to naprawić, stosując paradygmat ECS (Entity Component System) lub jedno scentralizowane zablokowane/bezlokowe źródło prawdy oraz wydajne iteracje.

**1. Zarządzanie Pamięcią i State Determinism (C++23):**
Użyjmy `std::flat_map` (C++23) lub scentralizowanego wektora przechowującego pełen stan elementu, co sprzyja Data-Oriented Design (Cache-friendly optymalizacja):
```cpp
// Scentralizowany komponent stanu przedmiotu upuszczonego
struct GroundDropComponent {
    EterBase::EntityId virtualId;
    EterBase::ItemVnum itemVnum;
    float posX, posY, posZ;
    float velX, velY, velZ;
    float expirationTime;
    bool isSettled;
    uint32_t ownerVid;
};
```
Eliminacja podwójnych słowników w renderze i w fizyce. Menedżer (System w ECS) iteruje po jednej domenie (np. `std::span<GroundDropComponent>`).

**2. Poprawność współbieżności i synchronizacja (Thread Safety):**
Zamiast rozrzucać Mutexy, zaleca się wprowadzenie w modelu Single Responsibility wydzielonej struktury wymiany `Lock-Free Queue` na wiadomości z wątku sieci, aplikowane jednokrotnie przez wątek główny przed wywołaniem pętli aktualizacji fizyki i grafiki:
```cpp
// Przekaz danych do logiki z watku sieci na watek logiki, uzywajac np. EterBase::FastLockFreeRingBuffer
// Eliminacja deadlockow, nie trzeba dodawac Mutexu w GroundDrop_Remove.cpp, GroundDrop_Add.cpp.
```

**3. Wydajność na ścieżce krytycznej (Performance Optimization - SIMD):**
Kalkulacja Grawitacji może zostać zwektoryzowana:
Zamiast aktualizować pozycje sekwencyjnie w skalarach float:
```cpp
traj.posX += traj.velX * deltaTime;
```
Można użyć wbudowanego w C++ modułu SIMD lub bibliotek dołączonych (`MathSIMD.h`), aby przetworzyć np. 4-8 trajektorii naraz:
```cpp
// Pseudo-wektoryzacja SIMD (np. przy uzyciu std::experimental::simd / C++26 albo intrinsic w <immintrin.h>)
// _mm256_fmadd_ps pozwala wymnozyc i dodac deltaTime do calej paczki predkosci obiektow na ziemi naraz.
```

### Strategia przepisania do języka Rust (Alternatywa)
Przepisanie tego fragmentu w RUST rozwiązuje problem z rzutowaniami typu, współbieżnością poprzez model Ownership i typowaniem rygorystycznym.
```rust
use rayon::prelude::*;
use nalgebra::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct GroundDropEntity {
    pub virtual_id: u32,
    pub position: Vector3<f32>,
    pub velocity: Vector3<f32>,
    pub is_settled: bool,
    pub time_remaining: f32,
}

pub struct GroundDropSystem {
    drops: Vec<GroundDropEntity>,
}

impl GroundDropSystem {
    // We update physics securely in parallel without data races
    pub fn step_physics(&mut self, delta_time: f32, ground_height: f32) {
        let gravity = Vector3::new(0.0, 0.0, -981.0);
        
        // Rayon parallel iteration cache-friendly update
        self.drops.par_iter_mut().for_each(|drop| {
            if drop.is_settled {
                return;
            }
            drop.velocity += gravity * delta_time;
            drop.position += drop.velocity * delta_time;
            
            if drop.position.z <= ground_height {
                drop.position.z = ground_height;
                drop.velocity = Vector3::zeros();
                drop.is_settled = true;
                // Emit event to a crossbeam-channel to decoupled systems safely
            }
        });
    }
}
```
Powyższy kod rozwiązuje braki atomowości operacji (stan uaktualniany w całości w tablicy SOA/AOS, zero ryzyka Ghost-items w różnych modułach), brak niepotrzebnych alokacji dynamicznych w pętli aktualizacji (`Vec` działa in-place, lub za pomocą `retain()`) i wprowadza wektoryzację po stronie kompilatora poprzez `par_iter_mut()` działający thread-safe na wydzielonych danych, czego nie zapewniał luźny zbiór klas C++ współużytkujących muteksy.
