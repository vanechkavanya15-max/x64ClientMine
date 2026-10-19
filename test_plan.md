1. Zmodyfikuj `src/SphereLib/vector.h`:
    - Usuń `#include <d3dx9math.h>`.
    - Dodaj `#include "Client/Actors/CollisionMathClean.h"`.
    - Zmień dziedziczenie klasy `Vector3d`: zamiast `D3DXVECTOR3`, użyj dziedziczenia po `Client::Collision::CleanVec3`.
    - Usuń konstruktor kopiujący przyjmujący `D3DXVECTOR3` na rzecz kompatybilności.
2. Zmodyfikuj `src/SphereLib/frustum.h`:
    - Usuń `#include <d3dx9math.h>`.
    - Dodaj `#include "Client/Actors/CollisionMathClean.h"`.
    - Dodaj `#include "Client/Math/PODMath.h"`.
    - Zamień `D3DXMATRIX` na `Client::Math::Mat4`.
    - Ponieważ musimy zachować strukturę płaszczyzny (a,b,c,d), dodajmy nową czystą strukturę `CleanPlane` w `frustum.h` (albo `CollisionMathClean.h` ale instruakcja mówi o bezwzględnym zakazie modyfikacji plików współdzielonych jeśli to nie jest konieczne). Utworzę ją w `frustum.h`.
    - Zastąp `D3DXVECTOR3` klasą `Client::Collision::CleanVec3` w całej definicji klasy `Frustum`.
3. Zmodyfikuj `src/SphereLib/frustum.cpp`:
    - Zastąp wywołania takie jak `D3DXPlaneNormalize`, `D3DXPlaneDotCoord` manualnymi obliczeniami w czystym C++.
    - Zastąp wywołania do normalizacji wektorów (`D3DXVec3Normalize`) metodą z `CleanVec3`.
4. Skompiluj `test_c26_sphere_tree_bvh.cpp` i inne zależne pliki (z wyłączonymi zależnościami d3d9) aby upewnić się, że g++ przetestuje z zero wycieków RAII (`-fsanitize=address`).
5. Ukończ instrukcje przed commitem.
