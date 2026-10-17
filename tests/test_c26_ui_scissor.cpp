#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"

#include <windows.h>
#include <optional>

#include "EterLib/StdAfx.h"
#include "EterPythonLib/PythonWindow.h"

TEST_CASE("UI::ComputeChildScissor - Dziecko poza obszarem rodzica zwraca std::nullopt")
{
    // Rodzic na pozycji (100, 100) do (300, 300)
    RECT parentRect{ 100, 100, 300, 300 };

    SUBCASE("Dziecko calkowicie z prawej strony rodzica")
    {
        RECT childRect{ 350, 150, 450, 250 };
        auto result = UI::ComputeChildScissor(childRect, &parentRect);
        CHECK_FALSE(result.has_value());
    }

    SUBCASE("Dziecko stykajace sie prawa krawedzia (szerokosc przeciecia == 0)")
    {
        RECT childRect{ 300, 150, 400, 250 };
        auto result = UI::ComputeChildScissor(childRect, &parentRect);
        CHECK_FALSE(result.has_value());
    }

    SUBCASE("Dziecko calkowicie z lewej strony rodzica")
    {
        RECT childRect{ 10, 150, 90, 250 };
        auto result = UI::ComputeChildScissor(childRect, &parentRect);
        CHECK_FALSE(result.has_value());
    }

    SUBCASE("Dziecko stykajace sie lewa krawedzia (szerokosc przeciecia == 0)")
    {
        RECT childRect{ 50, 150, 100, 250 };
        auto result = UI::ComputeChildScissor(childRect, &parentRect);
        CHECK_FALSE(result.has_value());
    }

    SUBCASE("Dziecko calkowicie powyzej rodzica")
    {
        RECT childRect{ 150, 10, 250, 90 };
        auto result = UI::ComputeChildScissor(childRect, &parentRect);
        CHECK_FALSE(result.has_value());
    }

    SUBCASE("Dziecko stykajace sie gorna krawedzia (wysokosc przeciecia == 0)")
    {
        RECT childRect{ 150, 50, 250, 100 };
        auto result = UI::ComputeChildScissor(childRect, &parentRect);
        CHECK_FALSE(result.has_value());
    }

    SUBCASE("Dziecko calkowicie ponizej rodzica")
    {
        RECT childRect{ 150, 320, 250, 400 };
        auto result = UI::ComputeChildScissor(childRect, &parentRect);
        CHECK_FALSE(result.has_value());
    }

    SUBCASE("Dziecko stykajace sie dolna krawedzia (wysokosc przeciecia == 0)")
    {
        RECT childRect{ 150, 300, 250, 400 };
        auto result = UI::ComputeChildScissor(childRect, &parentRect);
        CHECK_FALSE(result.has_value());
    }

    SUBCASE("Dziecko zdegenerowane (pusty prostokat wlasny right <= left)")
    {
        RECT childRect{ 150, 150, 150, 250 };
        auto result = UI::ComputeChildScissor(childRect, &parentRect);
        CHECK_FALSE(result.has_value());
    }

    SUBCASE("Dziecko zdegenerowane (pusty prostokat wlasny bottom <= top)")
    {
        RECT childRect{ 150, 200, 250, 180 };
        auto result = UI::ComputeChildScissor(childRect, &parentRect);
        CHECK_FALSE(result.has_value());
    }
}

TEST_CASE("UI::ComputeChildScissor - Prawidlowe czesciowe i pelne zawieranie")
{
    RECT parentRect{ 100, 100, 300, 300 };

    SUBCASE("Dziecko calkowicie wewnatrz rodzica")
    {
        RECT childRect{ 120, 130, 250, 280 };
        auto result = UI::ComputeChildScissor(childRect, &parentRect);
        REQUIRE(result.has_value());
        CHECK(result->left == 120);
        CHECK(result->top == 130);
        CHECK(result->right == 250);
        CHECK(result->bottom == 280);
    }

    SUBCASE("Dziecko wystajace z czterech stron (obciecie do granic rodzica)")
    {
        RECT childRect{ 50, 50, 350, 350 };
        auto result = UI::ComputeChildScissor(childRect, &parentRect);
        REQUIRE(result.has_value());
        CHECK(result->left == 100);
        CHECK(result->top == 100);
        CHECK(result->right == 300);
        CHECK(result->bottom == 300);
    }

    SUBCASE("Dziecko wystajace w prawy dolny rog")
    {
        RECT childRect{ 200, 200, 400, 400 };
        auto result = UI::ComputeChildScissor(childRect, &parentRect);
        REQUIRE(result.has_value());
        CHECK(result->left == 200);
        CHECK(result->top == 200);
        CHECK(result->right == 300);
        CHECK(result->bottom == 300);
    }

    SUBCASE("Brak aktywnego rodzica (pActiveParent == nullptr)")
    {
        RECT childRect{ 50, 60, 200, 250 };
        auto result = UI::ComputeChildScissor(childRect, nullptr);
        REQUIRE(result.has_value());
        CHECK(result->left == 50);
        CHECK(result->top == 60);
        CHECK(result->right == 200);
        CHECK(result->bottom == 250);
    }
}

TEST_CASE("UI::ComputeChildScissor - Prawidlowe obcinanie na 3 poziomach zagniezdzenia")
{
    // Poziom 1: Dziadek (Grandparent)
    RECT grandparentRect{ 0, 0, 500, 500 };
    auto scissorL1 = UI::ComputeChildScissor(grandparentRect, nullptr);
    REQUIRE(scissorL1.has_value());
    CHECK(scissorL1->left == 0);
    CHECK(scissorL1->top == 0);
    CHECK(scissorL1->right == 500);
    CHECK(scissorL1->bottom == 500);

    // Poziom 2: Rodzic (Parent) czesciowo wychodzacy poza dziadka
    RECT parentRect{ 100, 100, 600, 400 };
    auto scissorL2 = UI::ComputeChildScissor(parentRect, &(*scissorL1));
    REQUIRE(scissorL2.has_value());
    // Rodzic zostaje obciety przez dziadka z prawej do 500
    CHECK(scissorL2->left == 100);
    CHECK(scissorL2->top == 100);
    CHECK(scissorL2->right == 500);
    CHECK(scissorL2->bottom == 400);

    SUBCASE("Poziom 3: Wnuk wewnatrz obcietego obszaru rodzica")
    {
        RECT childRect{ 150, 150, 350, 350 };
        auto scissorL3 = UI::ComputeChildScissor(childRect, &(*scissorL2));
        REQUIRE(scissorL3.has_value());
        CHECK(scissorL3->left == 150);
        CHECK(scissorL3->top == 150);
        CHECK(scissorL3->right == 350);
        CHECK(scissorL3->bottom == 350);
    }

    SUBCASE("Poziom 3: Wnuk wychodzacy poza prawa krawedz dziadka (500), ale w granicach rodzica (600)")
    {
        RECT childRect{ 400, 150, 580, 300 };
        auto scissorL3 = UI::ComputeChildScissor(childRect, &(*scissorL2));
        REQUIRE(scissorL3.has_value());
        // Wnuk musi zostac obciety do aktywnego obciecia przodka (500)
        CHECK(scissorL3->left == 400);
        CHECK(scissorL3->top == 150);
        CHECK(scissorL3->right == 500);
        CHECK(scissorL3->bottom == 300);
    }

    SUBCASE("Poziom 3: Wnuk w granicach rodzica (520-590), ale calkowicie poza dziadkiem (> 500)")
    {
        RECT childRect{ 520, 150, 590, 300 };
        auto scissorL3 = UI::ComputeChildScissor(childRect, &(*scissorL2));
        // Powinno zwrocic std::nullopt (pomijanie renderowania)
        CHECK_FALSE(scissorL3.has_value());
    }

    SUBCASE("Poziom 3: Wnuk calkowicie poza rodzicem (ponizej bottom=400), ale w granicach dziadka (bottom=500)")
    {
        RECT childRect{ 150, 420, 300, 480 };
        auto scissorL3 = UI::ComputeChildScissor(childRect, &(*scissorL2));
        // Powinno zwrocic std::nullopt (obciecie rodzica odrzuca wnuka)
        CHECK_FALSE(scissorL3.has_value());
    }
}

TEST_CASE("UI::ComputeChildScissor - Prawidlowa obsluga ujemnych wspolrzednych")
{
    SUBCASE("Rodzic i dziecko w obszarze ujemnym - przecinajace sie")
    {
        RECT parentRect{ -200, -200, -50, -50 };
        RECT childRect{ -150, -180, -20, -80 };

        auto result = UI::ComputeChildScissor(childRect, &parentRect);
        REQUIRE(result.has_value());
        CHECK(result->left == -150);
        CHECK(result->top == -180);
        CHECK(result->right == -50);
        CHECK(result->bottom == -80);
    }

    SUBCASE("Rodzic i dziecko w obszarze ujemnym - rozlaczne")
    {
        RECT parentRect{ -300, -300, -150, -150 };
        RECT childRect{ -100, -100, -20, -20 };

        auto result = UI::ComputeChildScissor(childRect, &parentRect);
        CHECK_FALSE(result.has_value());
    }

    SUBCASE("Przeciecie przechodzace przez zero (od ujemnych do dodatnich)")
    {
        RECT parentRect{ -100, -100, 200, 200 };
        RECT childRect{ -150, 50, 50, 250 };

        auto result = UI::ComputeChildScissor(childRect, &parentRect);
        REQUIRE(result.has_value());
        CHECK(result->left == -100);
        CHECK(result->top == 50);
        CHECK(result->right == 50);
        CHECK(result->bottom == 200);
    }

    SUBCASE("Dziecko w pelni ujemne, rodzic w pelni dodatni")
    {
        RECT parentRect{ 0, 0, 800, 600 };
        RECT childRect{ -200, -150, -10, -5 };

        auto result = UI::ComputeChildScissor(childRect, &parentRect);
        CHECK_FALSE(result.has_value());
    }

    SUBCASE("Ujemne wspolrzedne bez rodzica (pActiveParent == nullptr)")
    {
        RECT childRect{ -250, -180, -50, -20 };

        auto result = UI::ComputeChildScissor(childRect, nullptr);
        REQUIRE(result.has_value());
        CHECK(result->left == -250);
        CHECK(result->top == -180);
        CHECK(result->right == -50);
        CHECK(result->bottom == -20);
    }
}

TEST_CASE("UI::CWindow::ComputeChildScissor - Zgodnosc metody statycznej CWindow")
{
    RECT parentRect{ 50, 50, 200, 200 };
    RECT childRect{ 80, 30, 250, 150 };

    auto resUI = UI::ComputeChildScissor(childRect, &parentRect);
    auto resCWin = UI::CWindow::ComputeChildScissor(childRect, &parentRect);

    REQUIRE(resUI.has_value());
    REQUIRE(resCWin.has_value());
    CHECK(resCWin->left == resUI->left);
    CHECK(resCWin->top == resUI->top);
    CHECK(resCWin->right == resUI->right);
    CHECK(resCWin->bottom == resUI->bottom);
}
