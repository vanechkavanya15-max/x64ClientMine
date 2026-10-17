#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include <windows.h>
#include <d3d9.h>
#include <cstdint>

#include "EterLib/GrpBase.h"
#include "EterLib/GrpScreen.h"
#include "EterLib/ColorTransitionHelper.h"

// Stubs for external symbols pulled in when linking EterLib
#include "EterLib/Camera.h"
float CCamera::CAMERA_MAX_DISTANCE = 2500.0f;

class CSpeedTreeWrapper
{
public:
    void SetupBranchForTreeType(void) const;
    void SetupFrondForTreeType(void) const;
    void SetupLeafForTreeType(void) const;
    void EndLeafForTreeType(void);
    void RenderBranches(void) const;
    void RenderFronds(void) const;
    void RenderLeaves(void) const;
    void RenderBillboards(void) const;
};

void CSpeedTreeWrapper::SetupBranchForTreeType(void) const {}
void CSpeedTreeWrapper::SetupFrondForTreeType(void) const {}
void CSpeedTreeWrapper::SetupLeafForTreeType(void) const {}
void CSpeedTreeWrapper::EndLeafForTreeType(void) {}
void CSpeedTreeWrapper::RenderBranches(void) const {}
void CSpeedTreeWrapper::RenderFronds(void) const {}
void CSpeedTreeWrapper::RenderLeaves(void) const {}
void CSpeedTreeWrapper::RenderBillboards(void) const {}

TEST_SUITE("EterlibSmallFixes")
{
    TEST_CASE("CGraphicBase::GetColor - Boundaries and Clamping")
    {
        CGraphicBase gb;

        // Wartosci zerowe (0.0f)
        DWORD c0 = gb.GetColor(0.0f, 0.0f, 0.0f, 0.0f);
        CHECK(c0 == 0x00000000);

        // Wartosci pelne (1.0f)
        DWORD c1 = gb.GetColor(1.0f, 1.0f, 1.0f, 1.0f);
        CHECK(c1 == 0xFFFFFFFF);

        // Czyste kolory skladowe
        DWORD cRed = gb.GetColor(1.0f, 0.0f, 0.0f, 1.0f);
        CHECK(cRed == 0xFFFF0000);

        DWORD cGreen = gb.GetColor(0.0f, 1.0f, 0.0f, 1.0f);
        CHECK(cGreen == 0xFF00FF00);

        DWORD cBlue = gb.GetColor(0.0f, 0.0f, 1.0f, 1.0f);
        CHECK(cBlue == 0xFF0000FF);

        // Wartosci przekraczajace 1.0f (> 1) - powinny byc bezpiecznie sklampowane do 1.0f (255)
        DWORD cOver = gb.GetColor(2.5f, 5.0f, 10.0f, 1.5f);
        CHECK(cOver == 0xFFFFFFFF);

        // Wartosci ujemne (< 0) - powinny byc sklampowane do 0.0f (0)
        DWORD cUnder = gb.GetColor(-1.0f, -0.5f, -10.0f, -0.1f);
        CHECK(cUnder == 0x00000000);

        // Mieszane wartosci z clampem
        DWORD cMixed = gb.GetColor(1.5f, -0.2f, 0.5f, 1.0f);
        CHECK(((cMixed >> 24) & 0xFF) == 255); // Alpha = 1.0f -> 255
        CHECK(((cMixed >> 16) & 0xFF) == 255); // Red = 1.5f -> clamp 1.0f -> 255
        CHECK(((cMixed >> 8) & 0xFF) == 0);    // Green = -0.2f -> clamp 0.0f -> 0
        CHECK((cMixed & 0xFF) == 127);         // Blue = 0.5f -> 127
    }

    TEST_CASE("CColorTransitionHelper - Duration 0 immediate transition and clamping")
    {
        CColorTransitionHelper helper;
        helper.Clear(0.0f, 0.0f, 0.0f, 0.0f);

        helper.SetSrcColor(0.1f, 0.2f, 0.3f, 0.4f);
        // Transition z czasem trwania 0
        helper.SetTransition(1.0f, 0.0f, 0.5f, 1.0f, 0);
        helper.StartTransition();

        // Update powinien traktowac duration=0 jako natychmiastowe 100% (1.0f) i zakonczyc tranzycje
        bool isRunning = helper.Update();
        CHECK(isRunning == false);
        CHECK(helper.isTransitionStarted() == false);

        D3DCOLOR cur = helper.GetCurColor();
        CHECK(((cur >> 24) & 0xFF) == 255); // Alpha
        CHECK(((cur >> 16) & 0xFF) == 255); // Red
        CHECK(((cur >> 8) & 0xFF) == 0);    // Green
        CHECK((cur & 0xFF) == 127);         // Blue (0.5f * 255 = 127)

        // Test z przekraczajacymi zakres wartosciami docelowymi
        helper.SetSrcColor(0.0f, 0.0f, 0.0f, 0.0f);
        helper.SetTransition(2.0f, -1.0f, 1.5f, 3.0f, 0);
        helper.StartTransition();

        isRunning = helper.Update();
        CHECK(isRunning == false);
        D3DCOLOR curClamped = helper.GetCurColor();
        CHECK(((curClamped >> 24) & 0xFF) == 255); // 3.0f clamped -> 255
        CHECK(((curClamped >> 16) & 0xFF) == 255); // 2.0f clamped -> 255
        CHECK(((curClamped >> 8) & 0xFF) == 0);    // -1.0f clamped -> 0
        CHECK((curClamped & 0xFF) == 255);         // 1.5f clamped -> 255
    }

    TEST_CASE("CScreen::RenderCircle - Step count < 3 safety guard")
    {
        CScreen screen;

        // Bez zabezpieczenia 'if (iStep < 3) return;', ponizsze wywolania powodowaly
        // dzielenie przez zero w delta = 2 * PI / iStep, alokacje wektora o zerowym rozmiarze
        // i odwolanie do pts[iStep - 1] (indeks -1 / out-of-bounds UB / crash).
        CHECK_NOTHROW(screen.RenderCircle2d(0.0f, 0.0f, 0.0f, 10.0f, 0));
        CHECK_NOTHROW(screen.RenderCircle2d(10.0f, 20.0f, 0.0f, 15.0f, 1));
        CHECK_NOTHROW(screen.RenderCircle2d(50.0f, 50.0f, 0.0f, 25.0f, 2));

        // RenderCircle3d z iStep < 3 nie powinien probowac pobierac kamery ani rzucac wyjatkow/crashowac
        CHECK_NOTHROW(screen.RenderCircle3d(0.0f, 0.0f, 0.0f, 10.0f, 0));
        CHECK_NOTHROW(screen.RenderCircle3d(10.0f, 20.0f, 0.0f, 15.0f, 1));
        CHECK_NOTHROW(screen.RenderCircle3d(50.0f, 50.0f, 0.0f, 25.0f, 2));
    }
}
