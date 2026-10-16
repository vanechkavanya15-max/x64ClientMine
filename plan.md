1.  **Stworzenie `src/Client/Player/PlayerPartyManager.h`**
    - Użyję narzędzia `run_in_bash_session` z poleceniem `cat << 'EOF' > ...` aby utworzyć plik nagłówkowy C++23.
    - Zdefiniuję klasę `PlayerPartyManager` w namespace `Client::Player` z zależnościami do `std::shared_ptr<Client::Gameplay::SocialManager>`.
    - Zadeklaruję enumerację `PartyError` (np. `None, NotInParty, AlreadyInParty, NotLeader, TargetInvalid`).
    - Zadeklaruję metody zwracające `Client::Core::Result<std::vector<uint8_t>, PartyError>` takie jak `InviteToParty(uint32_t targetVid)`, `AcceptInvite(uint32_t leaderVid)`, `DeclineInvite(uint32_t leaderVid)`, `LeaveParty()` i `RemoveMember(uint32_t pid)` oraz `SetDistributeMode(uint8_t mode)`.

2.  **Stworzenie `src/Client/Player/PlayerPartyManager.cpp`**
    - Użyję narzędzia `run_in_bash_session` z poleceniem `cat << 'EOF' > ...` aby zaimplementować metody z `PlayerPartyManager.h`.
    - Wcześniej sprawdziłem pełny plik SocialDomain.h (cat src/Client/Gameplay/SocialDomain.h src/Client/Network/Handlers/PartyPacketDomainHandler.h w czwartej iteracji bash). SocialManager posiada:
        `std::shared_ptr<Party> GetParty();`
        `void CreateParty();`
        `void LeaveParty();`
    - Użyję metod statycznych `EncodePartyInvite`, `EncodePartyInviteAnswer`, `EncodePartyRemove`, `EncodePartyParameter` z `Client::Network::Handlers::PartyPacketDomainHandler`.

3.  **Stworzenie `tests/test_c26_player_party_manager.cpp`**
    - Użyję narzędzia `run_in_bash_session` z poleceniem `cat << 'EOF' > ...` do stworzenia pliku testowego `doctest`.
    - Stworzę przypadki testowe dla `InviteToParty`, `AcceptInvite`, `DeclineInvite`, `LeaveParty` i `RemoveMember`.
    - Będę rzutował `pid` na `EterBase::EntityId` dla kompatybilności z `PartyMember`. W tym celu dołączę `EterBase/StrongTypes.h`.

4.  **Weryfikacja stworzonych plików**
    - Użyję narzędzia `run_in_bash_session` używając polecenia `ls -la` oraz `cat` by zweryfikować czy pliki `src/Client/Player/PlayerPartyManager.h`, `src/Client/Player/PlayerPartyManager.cpp` oraz `tests/test_c26_player_party_manager.cpp` zostały poprawnie utworzone.

5.  **Uruchomienie testów i kompilacja**
    - Użyję narzędzia `run_in_bash_session` by skompilować testy pod C++23 używając kompilatora g++:
      `g++ -std=c++23 tests/test_main.cpp tests/test_c26_player_party_manager.cpp src/Client/Player/PlayerPartyManager.cpp src/Client/Network/Handlers/PartyPacketDomainHandler.cpp src/Client/Gameplay/SocialDomain.cpp -I./tests/mock_includes -I./src -o test_runner && ./test_runner`
    - UWAGA: z poprzedniego polecenia usunięto zależność StrongTypes.cpp (z testów ręcznych nie ma pliku src/Client/Core/StrongTypes.cpp, za to jest EterBase/StrongTypes.h zdefiniowane np. w src/EterBase/StrongTypes.h, o czym wiem, bo mam informacje w pamięci i wyświetlało mi sie to w pliku źródłowym SocialDomain.h). Jednakże zgodnie z memory, "When compiling headless unit tests that utilize C++20 strong types from `src/Client/Core/StrongTypes.h` (such as `MapCoords` or `TerrainWidth`), explicitly include `src/Client/Core/StrongTypes.cpp` in the `g++` compilation command". Zrobię polecenie tak:
      `g++ -std=c++23 tests/test_main.cpp tests/test_c26_player_party_manager.cpp src/Client/Player/PlayerPartyManager.cpp src/Client/Network/Handlers/PartyPacketDomainHandler.cpp src/Client/Gameplay/SocialDomain.cpp src/Client/Core/StrongTypes.cpp src/EterBase/StrongTypes.cpp -I./tests/mock_includes -I./src -o test_runner && ./test_runner || g++ ... bez EterBase/StrongTypes.cpp`

6.  **Pre-commit steps**
    - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

7.  **Zatwierdzenie zmian (Submit)**
    - Wykonam użycie narzędzia submit podając commit message, tytuł, opis i branch.
