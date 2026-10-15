1. Utworzyć `src/Client/Network/MessengerPacketCodec.h` i `src/Client/Network/MessengerPacketCodec.cpp`:
   - Dodamy `struct MessengerFriendEvent { std::string name; bool isOnline; };` oraz `struct MessengerDomainEvent { uint8_t subheader; std::vector<MessengerFriendEvent> friends; };`.
   - Klasa `MessengerPacketCodec` będzie miała metodę: `static EterBase::PacketResult<MessengerDomainEvent> DecodeMessenger(std::span<const uint8_t> buffer);`.
   - Zaimplementować parser, używający `std::span` i weryfikujący `buffer.size()`. Chroni przed BufferUnderflow (wynik to `std::unexpected(EterBase::PacketError::BufferUnderflow)`). 

2. Utworzyć `src/Client/Bridge/PhaseGameMessengerBridge.h` i `src/Client/Bridge/PhaseGameMessengerBridge.cpp`:
   - Zależność `IPythonRuntime& m_pythonRuntime;`
   - Metoda `bool HandleMessenger(std::span<const uint8_t> buffer);` która:
     - Wywołuje `Client::Network::MessengerPacketCodec::DecodeMessenger(buffer)`.
     - Jeśli błąd, zwraca false.
     - Iteruje po `event.friends`:
       - `subh == LIST`: jeśli `f.isOnline`, woła `m_pythonRuntime.Invoke("messenger", "OnFriendLogin", f.name);`, inaczej `OnFriendLogout`.
       - `subh == LOGIN`: woła `"messenger"`, `"OnFriendLogin"`, i `"game"`, `"RefreshTargetBoardByName"`.
       - `subh == LOGOUT`: woła `"messenger"`, `"OnFriendLogout"`.
       - `subh == REMOVE_FRIEND`: woła `"messenger"`, `"RemoveFriend"`, i `"game"`, `"RefreshTargetBoardByName"`.

3. Utworzyć testy jednostkowe `tests/test_c26_phase_game_messenger_bridge.cpp`:
   - Użyć frameworku `doctest.h`
   - `MockPythonRuntime` by rejestrować wywołania (czy na pewno mamy nagrać `MockPythonRuntime.h`? Sprwadźmy `src/Client/Bridge/MockPythonRuntime.h`).
   - Mockowanie span'a dla pakietu LIST (2 znajomych: 1 online, 1 offline).
   - Mockowanie span'a dla LOGIN.
   - Mockowanie span'a dla LOGOUT.
   - Mockowanie span'a dla REMOVE_FRIEND.
   - Sprawdzenie logiki `MessengerPacketCodec` (np. za krótkie bufory).

4. Kompletacja instrukcji `pre_commit_instructions` (aby zaliczyć plan).
