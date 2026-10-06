# INSTRUKCJE DLA AGENTOW AI (JULES SWARM)

## 1. ZASADY GLOWNE I PRIORYTETY
1. **GRAFIKI NIE RUSZAMY NA RAZIE:** Zgodnie z wytycznymi, podsystemy graficzne (DirectX, renderery, modele) pozostaja nienaruszone.
2. **AUDIOLIB WYTNIETY:** `AudioLib` zostal zastapiony lekka, bezkosztowa atrapa (stub). Zadne moduly dzwieku (miniaudio, Miles) nie sa kompilowane ani inicjalizowane.
3. **GLOWNY CEL NA ROK 2026:** Porzadek, modularyzacja, refaktoryzacja do standardu C++20 oraz stworzenie **maksymalnej czytelnosci dla agentow AI** w kodzie sieci, logiki i stanu gry.

---

## 2. KOMPENDIUM WIEDZY: KATALOG `docs/`
W katalogu `docs/` znajduje sie kompletna dekonstrukcja architektury klienta:
* `docs/CLIENT_SYSTEM_MAP.md` - centralna mapa systemu, cykl zycia i petla glowna.
* `docs/01_architecture/` - gleboka analiza warstwowa i integracja z serwerem.
* `docs/02_subsystems/` - dekonstrukcja bibliotek pomocniczych.
* `docs/03_network_protocol/`:
  * `01_network_stream_core_handshake.md` - handshake, time-sync, ping-pong.
  * `02_phases_login_select_loading.md` - przebieg faz logowania i wyboru postaci.
  * `03_phase_game_actors_items.md` - obsluga ruchu, encji i przedmiotow.
  * `04_packets_cg_client_to_server.md` - KOMPLETNY rejestr pakietow wychodzacych.
  * `05_packets_gc_server_to_client.md` - KOMPLETNY rejestr pakietow przychodzacych.
* `docs/04_python_bindings/` - powiazania C++ do modulow Pythona (`player`, `chrmgr`, `item`).

ZANIM zaczniesz zmieniac dany pakiet lub modul, przeczytaj odpowiedni plik w `docs/`!

---

## 3. KLUCZOWA PRAWDA ARCHITEKTONICZNA: STAN GRY ZYJE W C++
Python w kliencie Metin2 jest **wylacznie warstwa widoku/interfejsu (View)**. Caly krytyczny stan gry zyje w czystym kodzie C++:
* **`CPythonPlayer`** -> przechowuje tablice ekwipunku, aktualne HP/MP/EXP, sloty umiejetnosci i cele ataku.
* **`CPythonCharacterManager`** -> trzyma instancje wszystkich potworow, graczy i NPC w zasiegu widzenia (`CInstanceBase`).
* **`CPythonItem`** -> zarzadza przedmiotami lezacymi na ziemi.

Oznacza to, ze bot i agenci AI operuja bezposrednio na strukturach C++, bez koniecznosci polegania na skryptach graficznych UI.

---

## 4. STANDARDY MODERNIZACJI NA ROK 2026 (AI READABILITY)

### A. Bezpieczne odpiecie od GUI (UI Decoupling)
Handlery w `PythonNetworkStreamPhaseGame.cpp` nie moga bezwarunkowo wywolywac metod okienek Pythona:
```cpp
// ZLE (Ryzyko crasha, gdy okno nie jest aktywne):
PyCallClassMemberFunc(m_apoPhaseWnd[PHASE_WINDOW_GAME], "RefreshAlignment", Py_BuildValue("()"));

// PRAWIDLOWO (Standard 2026):
if (m_apoPhaseWnd[PHASE_WINDOW_GAME]) {
    PyCallClassMemberFunc(m_apoPhaseWnd[PHASE_WINDOW_GAME], "RefreshAlignment", Py_BuildValue("()"));
}
```

### B. Standard Typow C++20
* Zastapic archaiczne `DWORD`, `WORD`, `BYTE`, `BOOL` przez `uint32_t`, `uint16_t`, `uint8_t`, `bool`.
* Likwidowac notacje wegierska w nowym kodzie (`dwVID` -> `vid`, `pkInst` -> `instance`).
* Uzywac `std::string_view` i `std::span` zamiast surowych tablic wskaznikowych.

### C. Zastapienie monolitycznych Switch-Case tablicami dispatcherow
Zamiast 2500 linijek gigantycznego `switch(header)` w `PythonNetworkStreamPhaseGame.cpp`, docelowo wprowadzamy czytelny rejestr pakietow (Packet Dispatch Table), gdzie kazdy pakiet ma przypisany naglowek, rozmiar i wskaznik na funkcje handlera.
Dla agenta AI oznacza to mozliwosc modyfikacji pojedynczego pakietu w 3 linijkach kodu.

---

## 5. ZASADY PRACY W REPOZYTORIUM
* Ignorujemy `m2dev-client-main/`, `build/`, `bin/`.
* Kazdy agent skupia sie na czytelnosci, modularnosci i testowalnosci kodu.
* Glowny interfejs bota opiera sie na `src/UserInterface/BotApi.h`.
