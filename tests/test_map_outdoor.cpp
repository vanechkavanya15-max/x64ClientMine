#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include <windows.h>
#include <d3d9.h>
#include <d3dx9.h>
#include "../src/GameLib/MapOutdoor.h"
#include "../src/GameLib/TerrainPatch.h"
#include "../src/PRTerrainLib/Terrain.h"

class CPythonSystem
{
public:
    int GetFogLevel();
};
int CPythonSystem::GetFogLevel() { return 0; }

// Note: Testing actual D3D code without a window requires mocking STATEMANAGER.
// For the bounds checks, we can test that calling out-of-bounds doesn't crash.

class TestableMapOutdoor : public CMapOutdoor
{
public:
    using CMapOutdoor::DrawWater;
    using CMapOutdoor::DrawPatchAttr;
    using CMapOutdoor::DrawWireFrame;
};

TEST_CASE("MapOutdoor - BoundsCheckDrawWater") {
    TestableMapOutdoor mapOutdoor;
    // Uninitialized, m_wPatchCount is 0, lMaxPatches is 0
    // So DrawWater with negative should return safely
    CHECK_NOTHROW(mapOutdoor.DrawWater(-1));
    // DrawWater with positive should return safely
    CHECK_NOTHROW(mapOutdoor.DrawWater(100));
}

TEST_CASE("MapOutdoor - BoundsCheckDrawPatchAttr") {
    TestableMapOutdoor mapOutdoor;
    CHECK_NOTHROW(mapOutdoor.DrawPatchAttr(-1));
    CHECK_NOTHROW(mapOutdoor.DrawPatchAttr(100));
}

TEST_CASE("MapOutdoor - BoundsCheckDrawWireFrame") {
    TestableMapOutdoor mapOutdoor;
    CHECK_NOTHROW(mapOutdoor.DrawWireFrame(-1, 0, D3DPT_TRIANGLELIST));
    CHECK_NOTHROW(mapOutdoor.DrawWireFrame(100, 0, D3DPT_TRIANGLELIST));
}
