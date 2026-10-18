#include <gtest/gtest.h>
#include "../src/GameLib/MapOutdoor.h"
#include "../src/GameLib/TerrainPatch.h"
#include "../src/PRTerrainLib/Terrain.h"

// Note: Testing actual D3D code without a window requires mocking STATEMANAGER.
// For the bounds checks, we can test that calling out-of-bounds doesn't crash.

class MapOutdoorTest : public ::testing::Test {
protected:
    void SetUp() override {
        // Prepare mock environment if needed
    }

    void TearDown() override {
    }
};

TEST_F(MapOutdoorTest, BoundsCheckDrawWater) {
    CMapOutdoor mapOutdoor;
    // Uninitialized, m_wPatchCount is 0, lMaxPatches is 0
    // So DrawWater with negative should return safely
    EXPECT_NO_THROW(mapOutdoor.DrawWater(-1));
    // DrawWater with positive should return safely
    EXPECT_NO_THROW(mapOutdoor.DrawWater(100));
}

TEST_F(MapOutdoorTest, BoundsCheckDrawPatchAttr) {
    CMapOutdoor mapOutdoor;
    EXPECT_NO_THROW(mapOutdoor.DrawPatchAttr(-1));
    EXPECT_NO_THROW(mapOutdoor.DrawPatchAttr(100));
}

TEST_F(MapOutdoorTest, BoundsCheckDrawWireFrame) {
    CMapOutdoor mapOutdoor;
    EXPECT_NO_THROW(mapOutdoor.DrawWireFrame(-1, 0, D3DPT_TRIANGLELIST));
    EXPECT_NO_THROW(mapOutdoor.DrawWireFrame(100, 0, D3DPT_TRIANGLELIST));
}

