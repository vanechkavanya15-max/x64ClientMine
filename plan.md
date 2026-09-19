1. **Analiza Modulów:** Zbadanie struktury `CPythonPlayer` i `CPythonSkill` (architektura, zaleznosci, powiazania, cykl zycia, zarzadzanie pamiecia).
2. **Budowanie Dokumentacji:** Stworzenie pliku `docs/04_python_bindings/02_player_and_skill_api.md` z podzialem na:
    * **Cel Architektoniczny:** Funkcja `player` i `skill` w integracji UI <-> Engine.
    * **Diagramy:** Mermaid (graph TD) obrazujacy powiazania `CPythonPlayer`, `CPythonSkill`, `CPythonCharacterManager`, `CPythonItem`, bindowanie w Pythonie i pakiety z serwerem.
    * **Struktury pamieci:** Rejestr enumaratorów i struktur (np. `TSkillInstance`, `TPlayerStatus`, `SAutoPotionInfo`, `SSkillData`, `TAffectData` itp) wraz z polozeniem, przeznaczeniem i wyrównaniem.
    * **API Reference:** Opis zaimplementowanych metod (od C++ do Pythona m.in. `SetItemData`, `GetStatus`, `UseSkill`, `SendClickItemPacket`, `AffectIndexToSkillIndex`) oraz metod z `skill_methods`.
    * **Integracje:** Python bindings (np. `playerSetItemData`, `skillGetSkillName`), DirectX (GUI/Rendering z instancjami postaci) oraz pakiety sieciowe.
    * **Problemy i antywzorce:** Ograniczenia `CPythonPlayer`, luki i bezpieczenstwo.
3. **Pre commit:** Wykonanie narzuconych w AGENTS.md procedur testowych (upewnienie sie co do poprawnosci kodu, review, checki).
4. **Zatwierdzenie:** Stworzenie commita przez `submit`.
