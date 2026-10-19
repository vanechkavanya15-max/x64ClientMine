The goal is to replace `CPythonCharacterManager::Instance()` in physics and interactions with a reference to `ActorRegistry`.

Wait! Is there an existing file or test in `tests/` that is named something like `test_actor_registry.cpp`? Let me list the tests.
Ah, `ls -la tests/` output showed:
No `test_actor_registry.cpp` in `tests/` directory! Wait, `test_actor_registry.cpp` doesn't exist yet, I need to create it.
The prompt says: `check ctest -R test_actor_registry`. So the test target name will be `test_actor_registry`.

Where should I place the new code? "Nowy modul utworz w src/Client/<Podsystem>/, a test w tests/test_<modul>.cpp" -> I should probably create `src/Client/World/ActorInteractionManager.h` or something, but wait. If the task is "Zastapienie CPythonCharacterManager::Instance() w fizyce i interakcjach referencja do ActorRegistry" and the test name is `test_actor_registry`, it could literally be that I just need to create `test_actor_registry.cpp` for the existing `ActorRegistry.cpp`?
Wait, if the module is `ActorRegistry`, then the test would be `test_actor_registry.cpp`. But `ActorRegistry` already exists in `src/Client/World/ActorRegistry.cpp` and `src/Client/Actor/ActorRegistry.cpp`. Let's look at `src/Client/World/ActorRegistry.cpp`. Does it have interaction/physics logic? No.

Wait. The memory says: "In the Metin2 issue descriptions, 'Wydzielenie' refers to extracting or refactoring an existing component out of a monolith into a clean architecture module, not necessarily creating a feature from scratch. The target files may already exist and require dependency updates or business logic migration." But the title is "Zastapienie CPythonCharacterManager::Instance() w fizyce i interakcjach referencja do ActorRegistry".

So I should create `ActorRegistry.cpp`/`ActorRegistry.h`?
Wait! Look at the memory:
"In the Metin2 C++ modernization project, `Client::Player::PlayerItemUseHandler` (located in `src/Client/Player/`)..."
"In the Metin2 C++ modernization project, `Client::World::AreaCollisionMeshIndex` (located in `src/Client/World/`)..."
"In the Metin2 C++ modernization project, `Client::Actors::ActorTargetRuleValidator`..."

Let's check `UserInterface/PythonCharacterManager.cpp` again.
Is it possible that I should modify `ActorRegistry` to provide physics/interaction queries, or create a completely new service `ActorInteractionResolver`?
Let's see the title exactly: `Zastapienie CPythonCharacterManager::Instance() w fizyce i interakcjach referencja do ActorRegistry — owner worker-lane — check ctest -R test_actor_registry`.

It says "Zastapienie ... referencja do ActorRegistry".
This implies I should create or update `ActorRegistry` or `PhysicsInteractor` that takes `ActorRegistry` as dependency. Wait, if the test is `test_actor_registry`, then the module should be `ActorRegistry`.

Let's check `src/UserInterface/Physics/` or `src/UserInterface/Actors/Subsystems/ActorRegistry.h`.
In `src/UserInterface/Actors/Subsystems/ActorRegistry.h` there is `ActorRegistry`.
Could it be that the C++23 `ActorRegistry` in `src/Client/World/ActorRegistry.h` is the intended one, and we need to add methods like `FindNearbyActors`, `CheckCollision`, or similar physics/interaction queries there? Or perhaps we should modify `ActorCollisionComponent` to accept `ActorRegistry`?
Let's check `src/Client/World/ActorRegistry.h` again.
