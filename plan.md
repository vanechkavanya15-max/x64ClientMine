1. Zaktualizować użycie silnych typów `EntityVid` i `ItemVnum` (zdefiniowanych w `src/Client/Core/StrongTypes.h`) w strukturach zdarzeń domenowych emitowanych przez `NetCombatRouter` (w `src/UserInterface/Network/Routers/NetCombatRouter.h`).
   - `CombatAttackDomainEvent`: zmienić `uint32_t` na `Client::Core::EntityVid` dla `dwAttackerVID` i `dwVictimVID`.
   - `CombatDamageInfoDomainEvent`: zmienić `uint32_t` na `Client::Core::EntityVid` dla `dwVictimVID` i `dwAttackerVID`.
   - `CombatFlyDomainEvent`: zmienić `uint32_t` na `Client::Core::EntityVid` dla `dwStartVID` i `dwEndVID`.
   - `CombatFlyTargetingDomainEvent`: zmienić `uint32_t` na `Client::Core::EntityVid` dla `dwShooterVID` i `dwTargetVID`.

2. Zaktualizować użycie silnych typów `EntityVid` i `ItemVnum` w strukturach zdarzeń domenowych emitowanych przez `NetItemRouter` (w `src/UserInterface/Network/Routers/NetItemRouter.h`).
   - `ItemSetDomainEvent`: zmienić `uint32_t` na `Client::Core::ItemVnum` dla `dwVnum`.
   - `ItemGroundAddDomainEvent`: zmienić `uint32_t` na `Client::Core::EntityVid` dla `dwVID` oraz `uint32_t` na `Client::Core::ItemVnum` dla `dwVnum`.
   - `ItemGroundDelDomainEvent`: zmienić `uint32_t` na `Client::Core::EntityVid` dla `dwVID`.

3. Zaktualizować kod w `NetCombatRouter.cpp` i `NetItemRouter.cpp`, aby przekazywał wartości owrapowane w silne typy podczas konstruowania obiektów zdarzeń domenowych.
   - Np. zamiast `packet.dwVID` przekazać `Client::Core::EntityVid{packet.dwVID}`.

4. W `IGameEvents.h`, zmienić deklarację `VID` i `ItemID` lub zmienić użycie typów w strukturach eventowych:
   - Ale uwaga! `IGameEvents.h` definiuje `using VID = uint32_t;`. Ponieważ zadanie to "Wdrożenie StrongTypes (EntityVid, ItemVnum) w NetCombatRouter i NetItemRouter", powinniśmy skupić się na zmianie `DomainEvents` definiowanych w tych routerach, ewentualnie zmienić to też w `IGameEvents.h` jeśli dotyczy interfejsu `IGameEventSink`, który też jest używany w tych routerach.
   - W `IGameEvents.h` możemy zostawić `VID` jako alias, ale lepiej zaktualizować zdarzenia by po prostu przypisywały `EntityVid` jeśli struktury w `IGameEvents.h` się nie zmieniły (przypomnijmy, zdarzenia kontraktowe w `IGameEvents.h` używają nadal `VID`, więc może trzeba zaktualizować tylko zdarzenia domenowe, lub też te w `IGameEvents.h` na `Client::Core::EntityVid`). W zasadzie możemy zmienić to w `IGameEvents.h`:
     ```cpp
     #include "Client/Core/StrongTypes.h"
     using VID = Client::Core::EntityVid;
     using ItemID = Client::Core::ItemVnum;
     ```
     Bądź zaktualizować w obu miejscach. Aby nie ruszać globalnego `IGameEvents.h` ponad to, co w zadaniu, wystarczy zaktualizować zdarzenia wysyłane w samych routerach, tzn. `CombatAttackDomainEvent` itp. (a do `IGameEvents.h` puszczamy wartości po wyciągnięciu `.get()` albo możemy zaktualizować obydwa).

Sprawdźmy powiązania.
