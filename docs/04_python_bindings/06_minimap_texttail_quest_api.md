# Modul UserInterface: miniMap, textTail oraz quest

## 1. Cel Architektoniczny i Rola Modulu

W kliencie Metin2 podsystem interfejsu uzytkownika realizowany jest glownie poprzez biblioteke EterPythonLib we wspolpracy z C++ `UserInterface`. Trzy kluczowe moduly graficzno-informacyjne dla gracza to:

*   **miniMap (CPythonMiniMap)**: Modul renderujacy zarowno okragla minimape na ekranie (w tym znaczniki zalezne od typu, polozenia, rotacji), jak i pelnoekranowa mape (Atlas). Modul rzutuje pozycje z 3D (`CMapOutdoor`) do lokalnego ukladu na mapie, poslugujac sie shaderami lub podstawowymi transformacjami `CTerrain` do nakladania maski (MiniMapFilter).
*   **textTail (CPythonTextTail)**: System rzutujacy teksty i obiekty 2D nad instancjami 3D postaci i przedmiotow (tzw. "chmury" nad glowami). Zarzadza paskami zycia, nazwami gildii, tytułami (PK), nazwami NPC oraz dropnietymi przedmiotami. Opiera sie na funkcji rzutowania przez kamere z 3D na wspolrzedne ekranu i sortuje teksty, aby zapobiec nakladaniu sie.
*   **quest (CPythonQuest)**: Prosty model danych dzialajacy jako Single Source of Truth dla instancji questow z aktywnymi zegarami (odliczaniem w dol lub w gore). Wspolpracuje m.in. z modulu Python, dostarczajac narzedzi GUI do wyrysowania "Zwoju" lub ikon z lewej strony ekranu, ktory reaguje na zamykanie (Clear) czy tworzenie zadania. 

Zaleznosci to w duzej mierze podsystem C++ `EterLib` (dla klas bazowych, ladowania obrazow `CGraphicExpandedImageInstance`), `GameLib` (obiekty postaci), zas komunikacja z interpreterem Python zachodzi przy uzyciu `PyMethodDef` API (biblioteka standardowa C).

## 2. Diagram Architektury i Przeplywu Danych (Mermaid)

```mermaid
graph TD
    %% Component Definitions
    Python[Python UI Scripts]
    subgraph UI Modules
        MiniMap[CPythonMiniMap]
        TextTail[CPythonTextTail]
        Quest[CPythonQuest]
    end
    Network[Network / Packet Handler]
    GameLib[GameLib: Character/Item instances]
    EterLib[EterLib: CGraphicImage, TextInstance]
    MapOutdoor[MapOutdoor: CTerrain]
    
    %% Flows
    Python -->|Py_InitModule, C-API Calls| UI Modules
    Network -->|Add Waypoints, Quest Updates| UI Modules
    MiniMap -->|Get mini map texture| MapOutdoor
    TextTail -->|Transform 3D to 2D| GameLib
    TextTail -->|Pool Allocations| EterLib
    Quest -->|Get icon from ResourceMgr| EterLib
```

## 3. Rejestr Struktur Danych i Pamieci (Memory & Struct Layout)

### CPythonMiniMap

#### `TAtlasMarkInfo` (wlasciwosc CPythonMiniMap)
*   `m_byType` (`BYTE`): Typ znacznika (`TYPE_NPC`, `TYPE_MONSTER`, `TYPE_WAYPOINT`, etc.).
*   `m_dwID` (`DWORD`): Identyfikator wewnetrzny.
*   `m_fX`, `m_fY` (`float`): Pozycja globalna z serwera.
*   `m_fScreenX`, `m_fScreenY` (`float`): Przeliczona pozycja na ekranie do wyrenderowania Atlasu.
*   `m_fMiniMapX`, `m_fMiniMapY` (`float`): Przeliczona pozycja do renderowania na okraglej minimapie.
*   `m_dwChrVID` (`DWORD`): Obiekt VID przypisany do wpisu (do biezacej aktualizacji polozenia).
*   `m_strText` (`std::string`): Tekst dymku, tooltip markera.
*   *Wyrównanie*: 4-8 bytes w zaleznosci od kompilatora na x86, bez `pragma pack`.

#### `TGuildAreaInfo`
*   `dwGuildID` (`DWORD`), `lx`, `ly`, `lwidth`, `lheight` (`long`): Dane serwerowe strefy gildii (polozenie i szerokosc prostokata).
*   `fsxRender`, `fsyRender`, `fexRender`, `feyRender` (`float`): Uklad prostokata wyliczony i zcache'owany dla mapy (Atlas) - pozwala na szybki bounding box hit-test za pomoca kursora.

#### `SObserver`
*   `fCurX`, `fCurY`, `fSrcX`, `fSrcY`, `fDstX`, `fDstY` (`float`): Pola interpolacji ruchu obserwowanego.
*   `dwSrcTime`, `dwDstTime` (`DWORD`): Timestamps pobierane z `ELTimer_GetMSec()`.

### CPythonTextTail

#### `STextTail` (TTextTail)
*   `pTextInstance`, `pOwnerTextInstance`, `pGuildNameTextInstance`, `pTitleTextInstance`, `pLevelTextInstance` (`CGraphicTextInstance*`): Referencje rzutowanych 2D stringow.
*   `pMarkInstance` (`CGraphicMarkInstance*`): Znacznik ikony logotypu gildii.
*   `pOwner` (`CGraphicObjectInstance *`): Obiekt docelowy, od ktorego pobierane sa wspolrzedne 3D dla kamery.
*   `dwVirtualID` (`DWORD`): Unikalne ID w swiecie, do zarzadzania instancja w Mapach.
*   `x`, `y`, `z` (`float`): Cache biezacej docelowej pozycji rzutowanej.
*   `fDistanceFromPlayer` (`float`): Wplywa na glebokosc rysowania (Z-sorting).
*   `Color` (`D3DXCOLOR`): Barwa tekstu.
*   `bNameFlag` (`BOOL`): Informuje czy dla TextTail powiazanego z ChatTail'em wymagane jest dorysowanie nicku gracza nad wiadomoscia.
*   `xStart`, `yStart`, `xEnd`, `yEnd` (`float`): Bounding box kolizji (do wyliczania picking 2D kursorem myszy).
*   `LivingTime` (`DWORD`): Ile czasu pozostalo wiadomosci z chatu (jezeli dotyczy chatu), poczatkowo inicjowane globalnym `gs_TextTail_LivingTime`.
*   `fHeight` (`float`): Przesuniecie wyliczone w gore nad postacia.

### CPythonQuest

#### `SQuestInstance`
*   `dwIndex` (`DWORD`): Unikalny identyfikator klienta / serwera.
*   `strIconFileName`, `strTitle`, `strClockName`, `strCounterName` (`std::string`): Zmienne lancuchowe z tekstem i zasobem graficznym dla UI Python.
*   `iClockValue`, `iCounterValue` (`int`): Biezaca wartosc licznika i zegara (sekundy).
*   `iStartTime` (`int`): Pobierana ze zrodla `CTimer::Instance().GetCurrentSecond()`. Z tej wartosci odejmuje sie uplyw czasu, by zwrocic biezacy stan dla interfejsu (w funkcji `GetQuestLastTime`).

## 4. Rejestr Klas i Metod (API Reference)

### CPythonMiniMap

Klasa zarzadza rysowaniem UI MiniMap. Jest obiektem bazujacym na Singleton.

*   `SetCenterPosition(float fCenterX, float fCenterY)`: Uaktualnia centrum. Jesli sie ono zmienilo, wywoluje przeliczenie komorek i wciagniecie tekstur minimapy z `CTerrain` z `CMapOutdoor`.
*   `Update(float fCenterX, float fCenterY)`: Aktualizuje pozycje obiektow zaleznych od aktualnej pozycji glownego aktora. Czysci wektory wezlow widocznosci, uzupelniajc je (Warp, Monster, NPC, Party) posilkujac sie danymi `CPythonCharacterManager`.
*   `Render(float fScreenX, float fScreenY)`: Tworzy transformacje D3D8, aplikujac obrot i przesuniecie kamery. Nastepnie po ustawieniu odpowiednich kolorow za pomoca `STATEMANAGER.SetRenderState(D3DRS_TEXTUREFACTOR, color)` i maski tekstur z mapy outdoor, renderuje biale kwadraciki lub ikonki (`m_WhiteMark.Render()`) w miejscu, w ktorym zostaly wstepnie zainicjalizowane obiekty.
*   `RenderAtlas(float fScreenX, float fScreenY)`: Rysuje `m_AtlasImageInstance`. Na bierzaco iteruje zas `m_AtlasWarpInfoVector`, `m_AtlasNPCInfoVector`, `m_AtlasWayPointInfoVector`, oraz `m_GuildAreaInfoVector`.
*   `GetPickedInstanceInfo(...)`: Wykrywa czy gracz na okraglej minimapie kliknal gdzies swoim kursorem, zwracajac pozycje i kolor kliknietego wezla informacyjnego, jezeli byl.

### CPythonTextTail

Klasa realizuje zadania zwiazane ze stanowieniem "dymkow" i informacjami unoszacymi sie nad wlascicielami `CGraphicObjectInstance`.

*   `UpdateAllTextTail()`: Przebiega przez obiekty w Mapach `m_CharacterTextTailMap`, `m_ItemTextTailMap` i `m_ChatTailMap`. Za pomoca pozycji MainInstancePtr oraz `UpdateDistance`, wyliczana jest odleglosc `fDistanceFromPlayer` kazdego tekstowego dymku. 
*   `UpdateShowingTextTail()`: Realizuje samo uaktualnianie instancji do wyswietlenia oraz obsluguje usuniecie wiadomosci chat, jezeli ich timer (`LivingTime`) sie zakonczy (wartosc porownywana z `CTimer::Instance().GetCurrentMillisecond()`).
*   `Render()`: Przechodzi w petli i po wczesniejszym `Sort()` poprzez wlasciwosc Z (wlasnie the `fDistanceFromPlayer`), i renderuje obiekty GUI od najdalszego do najblizszego, uzywajac wczesniej uzyskanych polozen x,y rzutowanych przez klase `CCamera`.
*   `RegisterCharacterTextTail(DWORD dwGuildID, DWORD dwVirtualID, const D3DXCOLOR & c_rColor, float fAddHeight)`: Realizuje zarezerwowanie nowej struktury typu `STextTail` w alokatorze obiektu `m_TextTailPool`, nastepnie podpina ja do mapy. Oznacza tekst z konkretnym identyfikatorem z wirtualnego id postaci (dwVirtualID), ustawia tagowanie (tytul, gildia, itp).
*   `ArrangeTextTail()`: Glowna metoda chroniaca przed nakladaniem sie tekstow w 2D (over-drawing). Iteruje po TextTailList i sprawdza czy biezacy Bounding Box wchodzi w kolizje z poprzednim elementem - jezeli tak, to spycha go o wlasciwosc `fHeight` w dol, badz w gore.

### CPythonQuest

Klasa dzialajaca jako "magazyn danych".

*   `RegisterQuestInstance(const SQuestInstance & c_rQuestInstance)`: Otrzymujac wygenerowana z sieci (od pakietow z serwera) strukture questu, najpierw usuwa duplikat uzywajac `DeleteQuestInstance`, dodaje obiek do wektora z nowa instancja czasowa podana w `iStartTime = int(CTimer::Instance().GetCurrentSecond())`.
*   `SetQuestClockValue(DWORD dwIndex, int iClockValue)`: Zmienia wartosc zegara dla konkretnego `dwIndex` z mapowania. Wymaga przeliczenia `iStartTime` od zera na stan obecny, aby zegar poprawnie byl renderowany od momentu wyslania wiadomosci aktualizacji.
*   `GetQuestCount()`: Zwraca rozmiar glownego kontenera questow.

## 5. Punkty Styku (Cross-Subsystem Integration)

### Python (PyMethodDef)
Wszystkie trzy moduly wykorzystuja eksportowanie metod przez `PyMethodDef` i inicjalizacje `Py_InitModule`:
*   `PythonMiniMapModule.cpp`: Definiuje funkcje do sterowania takimi wlasciwosciami jak m.in.: `SetScale`, `miniMapSetCenterPosition`, `miniMapAddWayPoint`, `miniMapLoadAtlas`, `miniMapRender`. Wszystkie z nich na wejsciu dekoduja `PyTuple_GetInteger` badz inne zalezne struktury i odwoluja sie bezposrednio do instancji Singletonu `CPythonMiniMap::Instance()`.
*   `PythonTextTailModule.cpp`: Zewnetrznie naraza API poprzez np. `textTailClear`, `textTailUpdateAllTextTail`, `textTailShowCharacterTextTail`. Funkcja `textTailPick` uzywana jest bezposrednio przez GUI on-click do wykrycia, czy wlasnie wybrano napis od obiektu, ktory mial spadniety loot, by przeslac ten stan nastepnie poprzez siec za pomoca komendy item pickup.
*   `PythonQuest.cpp`: Oferuje funkcje `GetQuestCount`, `GetQuestData`, `GetQuestIndex`, `GetQuestLastTime`, w ktorych uzywane jest mapowanie wartosci przez `Py_BuildValue("sisi", ...)` (zbudowanie struktury z 4 argumentow: string, integer, string, integer). Funkcje nastepnie odczytywane sa z warstwy klienta Pythona w module `uiQuest.py`. 

### DirectX / Sprzet
*   MiniMap stosuje tricki wydajnosciowe podczas renderingu znacznikow modyfikujac bezposrednio `STATEMANAGER`. Posiada bufor obrotu `m_matMiniMapCover` jako `D3DXMATRIX` co pozwala z rotowac MiniMap bez obrotu calego UI 2D na warstwie.
*   TextTail nie rysuje tekstow od zera. Odwoluje sie zawsze po wstepne stringi do `CGraphicTextInstance`, ktore posiada ukryte bufory vertexow zarzadzane bezposrednio na karcie w atlasie wygenerowanych literek za pomoca EterLib, przyspieszajac operacje rysowania dziesiatek elementow podczas eventow. 

## 6. Pulapki, Antywzorce i Ograniczenia

1.  **Zlozonosc kwadratowa przy ukladaniu dymkow (ArrangeTextTail)**: Funkcja ta sprawdza ewentualne kolizje BB miedzy soba za pomoca pętli zagnieżdżonej, co daje zlozonosc w przyblizeniu `O(N^2)`. W miejscach o gigantycznej liczbie upuszczonych rzeczy na ziemi oraz tlumu mobow, spadek klatek na sekunde (FPS) spowodowany jest iterowaniem i modyfikowaniem pamieci wlasnie tego kodu (wymagany np. QuadTree albo spatial partitioning do 2D w przyszlosci).
2.  **Uzycie Timerow (Czas Systemowy)**: Klasy (MiniMap's SObserver oraz Quest) wykorzystuja pobranie czasu lokalnego z `CTimer::Instance()`. Prowadzi to do drobnych desynchronizacji podczas duzych opoznien (lagow) z serwera, gdzie odliczanie "Clock" bedzie wyprzedzac lub opozniac wlasciwy proces po stronie GameServer'a.
3.  **Brak Thread-Safety**: Zadna z map sluzacych do mapowania elementow np. `m_CharacterTextTailMap` (w TextTail) i `m_QuestInstanceContainer` (w Quest) nie jest zabezpieczona mutexami, wiec watek glowny i potencjalny watek sieciowy powninny unikac wspolzawodnictwa - wszystkie modyfikacje, typu pakiety sieciowe odbierane przez klienta, sa wywolywane z gwaranacji istnienia w glownej petli CMain.
4.  **Pamiec Poolowana vs Opoznienia usuniecia**: `CPythonTextTail` alokuje nowe zgloszenia (STextTail) do chmury tekstowej uzywajac `CDynamicPool`. Ogranicza to framerate hit przy tworzeniu wielu tysiecy stringow na sekunde, lecz w momencie, gdy z mapy kasuje sie 1000 itemow na raz (np. na skutek commandu '/ip'), moga wystapic tzw. "stutter spikes".
5.  **Polskie Znaki w Kodzie C++**: Plik np. Questow obok metody `__Initialize()` zawieral testowe opisy UTF-8/ANSI, ktore wskazuja na dawne pozostalosci debugowania. Dodatkowo sam klient Metin czesto odwoluje sie do pre-kompilowanych `CPythonQuest::SQuestInstance::strClockName`. Zaleca sie bezwglednie ich przesladowanie poza kod jako locale config.
