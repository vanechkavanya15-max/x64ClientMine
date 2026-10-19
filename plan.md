1. **Remove `d3dx9math.h` from `src/SphereLib/frustum.h` and `src/SphereLib/vector.h`**:
   - Replace `<d3dx9math.h>` with `#include "../Client/Actors/CollisionMathClean.h"` and `#include "../Client/Math/PODMath.h"`.
   - Update `Vector3d` to inherit from `Client::Collision::CleanVec3` or use it directly instead of `D3DXVECTOR3`. Since it's currently derived from `D3DXVECTOR3`, we'll derive it from `Client::Collision::CleanVec3` to maintain compatibility if possible, or alias `Vector3d` to `Client::Collision::CleanVec3`. Given the requirements to remove DirectX dependencies, we'll replace `D3DXVECTOR3` with `Client::Collision::CleanVec3` in `Vector3d` definition.
   - For `frustum.h` and `frustum.cpp`, replace `D3DXMATRIX` with `Client::Math::Mat4` and `D3DXPLANE` with a custom plane struct `CleanPlane` inside `CollisionMathClean.h` or within the `Frustum` class if it doesn't exist, to fully isolate it. Wait, `D3DXPLANE` has `a,b,c,d`. We can define `CleanPlane` in `CollisionMathClean.h`.
   - Modify `Frustum::ExtractPlanes` and `Frustum::BuildViewFrustum` to use `Client::Math::Mat4` and `CleanPlane` / `Client::Collision::CleanVec3`.
   - Ensure the implementations in `frustum.cpp` do not use `D3DXPlaneNormalize`, `D3DXPlaneDotCoord`, etc., and replace them with standard C++ math logic.

2. **Add missing structures to `CollisionMathClean.h`**:
   - Add `CleanPlane` struct with `a,b,c,d`.
   - Add a normalization function for `CleanPlane`.
   - Add dot coordinate function for `CleanPlane` and `CleanVec3`.

3. **Verify tests using `ctest -R test_sphere_tree`**:
   - Since `tests/test_sphere_tree` will be run, check how `Frustum` and `Vector3d` are used in tests. Wait, the requirement says "Nowy modul utworz w src/Client/<Podsystem>/, a test w tests/test_<modul>.cpp". But the test we need to pass is `test_sphere_tree`. The instruction asks to remove the header and pass `test_sphere_tree`.

4. **Verify changes and complete pre-commit steps**:
   - Run tests, check for clean architecture.

