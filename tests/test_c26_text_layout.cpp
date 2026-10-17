#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "../src/Client/Text/TextLayoutEngine.h"
#include <string_view>

using namespace Client::Text;

class MockFontMetrics : public IFontMetricsProvider {
public:
    float GetGlyphAdvance(char32_t codepoint) const override {
        if (codepoint == 'W') return 15.0f;
        if (codepoint == 'i') return 5.0f;
        if (codepoint == 'l') return 5.0f;
        return 10.0f; // Default 10px per char
    }
    float GetLineHeight() const override {
        return 20.0f;
    }
};

TEST_SUITE("TextLayoutEngine") {
    TEST_CASE("Basic text layout without tags") {
        MockFontMetrics metrics;
        LayoutConfig config;
        
        auto result = TextLayoutEngine::BuildLayout("Hello", metrics, config);
        
        REQUIRE(result.has_value());
        // H(10) e(10) l(5) l(5) o(10) = 40.0f
        CHECK(result->width == 40.0f);
        CHECK(result->height == 20.0f);
        REQUIRE(result->lines.size() == 1);
        CHECK(result->lines[0].glyphs.size() == 5);
        CHECK(result->lines[0].glyphs[0].content == "H");
        CHECK(result->lines[0].glyphs[0].x == 0.0f);
        CHECK(result->lines[0].glyphs[4].x == 30.0f); // 10+10+5+5 = 30
    }
    
    TEST_CASE("Text layout with color tags") {
        MockFontMetrics metrics;
        LayoutConfig config;
        config.defaultColor = 0xFFFFFFFF;
        
        auto result = TextLayoutEngine::BuildLayout("A|cFFFF0000B|rC", metrics, config);
        
        REQUIRE(result.has_value());
        REQUIRE(result->lines.size() == 1);
        REQUIRE(result->lines[0].glyphs.size() == 3);
        
        CHECK(result->lines[0].glyphs[0].content == "A");
        CHECK(result->lines[0].glyphs[0].color == 0xFFFFFFFF);
        
        CHECK(result->lines[0].glyphs[1].content == "B");
        CHECK(result->lines[0].glyphs[1].color == 0xFFFF0000);
        
        CHECK(result->lines[0].glyphs[2].content == "C");
        CHECK(result->lines[0].glyphs[2].color == 0xFFFFFFFF);
    }
    
    TEST_CASE("Text layout with hyperlink") {
        MockFontMetrics metrics;
        LayoutConfig config;
        
        auto result = TextLayoutEngine::BuildLayout("Click |Hitem:123|hhere|h!", metrics, config);
        
        REQUIRE(result.has_value());
        REQUIRE(result->hyperlinks.size() == 1);
        
        CHECK(result->hyperlinks[0].text == "item:123");
        // C(10) l(5) i(5) c(10) k(10)  (10) = 50.0f
        CHECK(result->hyperlinks[0].sx == 50.0f);
        // h(10) e(10) r(10) e(10) = 40.0f -> ex = 90.0f
        CHECK(result->hyperlinks[0].ex == 90.0f);
        CHECK(result->hyperlinks[0].sy == 0.0f);
        CHECK(result->hyperlinks[0].ey == 20.0f);
    }
    
    TEST_CASE("Multiline wrapping") {
        MockFontMetrics metrics;
        LayoutConfig config;
        config.multiLine = true;
        config.limitWidth = 45.0f; // Max 4 chars per line (40px)
        
        auto result = TextLayoutEngine::BuildLayout("123456789", metrics, config);
        
        REQUIRE(result.has_value());
        REQUIRE(result->lines.size() == 3);
        
        CHECK(result->lines[0].glyphs.size() == 4); // "1234"
        CHECK(result->lines[1].glyphs.size() == 4); // "5678"
        CHECK(result->lines[2].glyphs.size() == 1); // "9"
        
        CHECK(result->width == 40.0f);
        CHECK(result->height == 60.0f);
        
        // Check Y coordinates
        CHECK(result->lines[0].glyphs[0].y == 0.0f);
        CHECK(result->lines[1].glyphs[0].y == 20.0f);
        CHECK(result->lines[2].glyphs[0].y == 40.0f);
    }
    
    TEST_CASE("Alignment") {
        MockFontMetrics metrics;
        
        SUBCASE("Center Horizontal") {
            LayoutConfig config;
            config.horizontalAlign = HorizontalAlignment::Center;
            config.limitWidth = 100.0f;
            
            auto result = TextLayoutEngine::BuildLayout("Hello", metrics, config); // 40px width
            REQUIRE(result.has_value());
            // offsetX should be (100 - 40) / 2 = 30px
            CHECK(result->lines[0].glyphs[0].x == 30.0f);
        }
        
        SUBCASE("Right Horizontal") {
            LayoutConfig config;
            config.horizontalAlign = HorizontalAlignment::Right;
            config.limitWidth = 100.0f;
            
            auto result = TextLayoutEngine::BuildLayout("Hello", metrics, config); // 40px width
            REQUIRE(result.has_value());
            // offsetX should be 100 - 40 = 60px
            CHECK(result->lines[0].glyphs[0].x == 60.0f);
        }
        
        SUBCASE("Center Vertical") {
            LayoutConfig config;
            config.verticalAlign = VerticalAlignment::Center;
            
            auto result = TextLayoutEngine::BuildLayout("A", metrics, config); // 20px height
            REQUIRE(result.has_value());
            // startY should be -20 / 2 = -10px
            CHECK(result->lines[0].glyphs[0].y == -10.0f);
        }
        
        SUBCASE("Bottom Vertical") {
            LayoutConfig config;
            config.verticalAlign = VerticalAlignment::Bottom;
            
            auto result = TextLayoutEngine::BuildLayout("A", metrics, config); // 20px height
            REQUIRE(result.has_value());
            // startY should be -20px
            CHECK(result->lines[0].glyphs[0].y == -20.0f);
        }
    }
    
    TEST_CASE("RTL Direction") {
        MockFontMetrics metrics;
        LayoutConfig config;
        config.textDirection = TextDirection::RTL;
        config.limitWidth = 100.0f;
        
        auto result = TextLayoutEngine::BuildLayout("AB", metrics, config); // 20px width
        REQUIRE(result.has_value());
        
        CHECK(result->isRtlComputed == true);
        
        // offsetX should be 100 - 20 = 80px (for Left align in RTL with limit width)
        // A is placed at 80 + 20 - 10 = 90
        // B is placed at 80 + 20 - 10 - 10 = 80
        CHECK(result->lines[0].glyphs[0].content == "A");
        CHECK(result->lines[0].glyphs[0].x == 90.0f);
        CHECK(result->lines[0].glyphs[1].content == "B");
        CHECK(result->lines[0].glyphs[1].x == 80.0f);
    }
}

#include "EterLib/GrpTextInstance.h"
#include "EterLib/RenderOwnership.h"
#include <algorithm>

class TestGraphicTextInstance : public CGraphicTextInstance
{
public:
    TestGraphicTextInstance()
    {
        m_isUpdate = true;
    }

    void AddMockChar(short width, short height, float advance, float bearingX,
                     float left, float top, float right, float bottom,
                     DWORD color = 0xFFFFFFFF, float kern = 0.0f)
    {
        auto* pInfo = new CGraphicFontTexture::TCharacterInfomation();
        pInfo->index = -1;
        pInfo->width = width;
        pInfo->height = height;
        pInfo->advance = advance;
        pInfo->bearingX = bearingX;
        pInfo->left = left;
        pInfo->top = top;
        pInfo->right = right;
        pInfo->bottom = bottom;

        m_mockInfos.push_back(pInfo);
        m_pCharInfoVector.push_back(pInfo);
        m_dwColorInfoVector.push_back(color);
        m_kernVector.push_back(kern);

        m_textWidth += static_cast<WORD>(advance + kern);
        m_textHeight = (std::max)(m_textHeight, static_cast<WORD>(height));
    }

    void SetMockTextSize(WORD w, WORD h)
    {
        m_textWidth = w;
        m_textHeight = h;
    }

    void SetComputedRTL(bool rtl)
    {
        m_computedRTL = rtl;
    }

    ~TestGraphicTextInstance() override
    {
        for (auto* p : m_mockInfos)
            delete p;
        m_mockInfos.clear();
        m_pCharInfoVector.clear();
    }

private:
    std::vector<CGraphicFontTexture::TCharacterInfomation*> m_mockInfos;
};

TEST_SUITE("CGraphicTextInstance_LayoutGlyphs") {
    TEST_CASE("LayoutGlyphs - PlainText") {
        TestGraphicTextInstance inst;
        inst.SetPosition(100.0f, 200.0f);

        inst.AddMockChar(10, 16, 10.0f, 0.0f, 0.1f, 0.1f, 0.2f, 0.2f, 0xFFFFFFFF);
        inst.AddMockChar(10, 16, 10.0f, 0.0f, 0.2f, 0.1f, 0.3f, 0.2f, 0xFFFFFFFF);
        inst.AddMockChar(10, 16, 10.0f, 0.0f, 0.3f, 0.1f, 0.4f, 0.2f, 0xFFFFFFFF);

        const auto& quads = inst.LayoutGlyphs();

        REQUIRE(quads.size() == 3);
        for (const auto& q : quads)
        {
            CHECK(q.layer == ETextQuadLayer::Main);
            CHECK(q.color == 0xFFFFFFFF);
            CHECK(q.w == 10.0f);
            CHECK(q.h == 16.0f);
        }

        // Base fStanX = 100, fStanY = 200 + 1.0f = 201.0f
        // fFontSx = fCurX + bearingX - 0.5f = 99.5f
        // fFontSy = fCurY - 0.5f = 200.5f
        CHECK(quads[0].x == doctest::Approx(99.5f));
        CHECK(quads[0].y == doctest::Approx(200.5f));
        CHECK(quads[0].u0 == doctest::Approx(0.1f));

        CHECK(quads[1].x == doctest::Approx(109.5f));
        CHECK(quads[1].y == doctest::Approx(200.5f));

        CHECK(quads[2].x == doctest::Approx(119.5f));
        CHECK(quads[2].y == doctest::Approx(200.5f));
    }

    TEST_CASE("LayoutGlyphs - ColoredText") {
        TestGraphicTextInstance inst;
        inst.SetPosition(0.0f, 0.0f);

        inst.AddMockChar(8, 14, 8.0f, 0.0f, 0.0f, 0.0f, 0.1f, 0.1f, 0xFFFF0000);
        inst.AddMockChar(8, 14, 8.0f, 0.0f, 0.1f, 0.0f, 0.2f, 0.1f, 0xFF00FF00);
        inst.AddMockChar(8, 14, 8.0f, 0.0f, 0.2f, 0.0f, 0.3f, 0.1f, 0xFF0000FF);

        const auto& quads = inst.LayoutGlyphs();

        REQUIRE(quads.size() == 3);
        CHECK(quads[0].color == 0xFFFF0000);
        CHECK(quads[1].color == 0xFF00FF00);
        CHECK(quads[2].color == 0xFF0000FF);
    }

    TEST_CASE("LayoutGlyphs - AlignmentHorizontalAndVertical") {
        SUBCASE("Center Horizontal Alignment") {
            TestGraphicTextInstance inst;
            inst.SetPosition(100.0f, 200.0f);
            inst.SetHorizonalAlign(CGraphicTextInstance::HORIZONTAL_ALIGN_CENTER);

            inst.AddMockChar(10, 16, 10.0f, 0.0f, 0.0f, 0.0f, 0.1f, 0.1f);
            inst.AddMockChar(10, 16, 10.0f, 0.0f, 0.1f, 0.0f, 0.2f, 0.1f);
            inst.AddMockChar(10, 16, 10.0f, 0.0f, 0.2f, 0.0f, 0.3f, 0.1f);
            // textWidth = 30 -> offset -15
            const auto& quads = inst.LayoutGlyphs();
            REQUIRE(quads.size() == 3);
            CHECK(quads[0].x == doctest::Approx(99.5f - 15.0f));
        }

        SUBCASE("Right Horizontal Alignment") {
            TestGraphicTextInstance inst;
            inst.SetPosition(100.0f, 200.0f);
            inst.SetHorizonalAlign(CGraphicTextInstance::HORIZONTAL_ALIGN_RIGHT);

            inst.AddMockChar(10, 16, 10.0f, 0.0f, 0.0f, 0.0f, 0.1f, 0.1f);
            inst.AddMockChar(10, 16, 10.0f, 0.0f, 0.1f, 0.0f, 0.2f, 0.1f);
            inst.AddMockChar(10, 16, 10.0f, 0.0f, 0.2f, 0.0f, 0.3f, 0.1f);
            // textWidth = 30 -> offset -30
            const auto& quads = inst.LayoutGlyphs();
            REQUIRE(quads.size() == 3);
            CHECK(quads[0].x == doctest::Approx(99.5f - 30.0f));
        }

        SUBCASE("Center Vertical Alignment") {
            TestGraphicTextInstance inst;
            inst.SetPosition(100.0f, 200.0f);
            inst.SetVerticalAlign(CGraphicTextInstance::VERTICAL_ALIGN_CENTER);

            inst.AddMockChar(10, 20, 10.0f, 0.0f, 0.0f, 0.0f, 0.1f, 0.1f);
            // textHeight = 20 -> offset -10
            const auto& quads = inst.LayoutGlyphs();
            REQUIRE(quads.size() == 1);
            CHECK(quads[0].y == doctest::Approx(200.5f - 10.0f));
        }

        SUBCASE("Bottom Vertical Alignment") {
            TestGraphicTextInstance inst;
            inst.SetPosition(100.0f, 200.0f);
            inst.SetVerticalAlign(CGraphicTextInstance::VERTICAL_ALIGN_BOTTOM);

            inst.AddMockChar(10, 20, 10.0f, 0.0f, 0.0f, 0.0f, 0.1f, 0.1f);
            // textHeight = 20 -> offset -20
            const auto& quads = inst.LayoutGlyphs();
            REQUIRE(quads.size() == 1);
            CHECK(quads[0].y == doctest::Approx(200.5f - 20.0f));
        }

        SUBCASE("RTL Direction Alignment") {
            TestGraphicTextInstance inst;
            inst.SetPosition(100.0f, 200.0f);
            inst.SetComputedRTL(true);
            inst.SetHorizonalAlign(CGraphicTextInstance::HORIZONTAL_ALIGN_LEFT);

            inst.AddMockChar(10, 16, 10.0f, 0.0f, 0.0f, 0.0f, 0.1f, 0.1f);
            inst.AddMockChar(10, 16, 10.0f, 0.0f, 0.1f, 0.0f, 0.2f, 0.1f);
            // In RTL, HORIZONTAL_ALIGN_LEFT offsets by -textWidth (-20)
            const auto& quads = inst.LayoutGlyphs();
            REQUIRE(quads.size() == 2);
            CHECK(quads[0].x == doctest::Approx(99.5f - 20.0f));
        }
    }

    TEST_CASE("LayoutGlyphs - OutlineGeneration") {
        TestGraphicTextInstance inst;
        inst.SetPosition(100.0f, 200.0f);
        inst.SetOutline(true);
        inst.SetOutLineColor(0xFF334455);

        inst.AddMockChar(10, 16, 10.0f, 0.0f, 0.1f, 0.1f, 0.2f, 0.2f);
        inst.AddMockChar(10, 16, 10.0f, 0.0f, 0.2f, 0.1f, 0.3f, 0.2f);

        const auto& quads = inst.LayoutGlyphs();

        // 2 chars * 4 outline directions = 8 outline quads + 2 main quads = 10 quads
        REQUIRE(quads.size() == 10);

        for (size_t i = 0; i < 8; ++i)
        {
            CHECK(quads[i].layer == ETextQuadLayer::Outline);
            CHECK(quads[i].color == 0xFF334455);
            CHECK(quads[i].w == 10.0f);
            CHECK(quads[i].h == 16.0f);
        }

        // First glyph outline directions: Left (-1, 0), Right (+1, 0), Top (0, -1), Bottom (0, +1)
        // base fFontSx = 99.5f, fFontSy = 200.5f
        CHECK(quads[0].x == doctest::Approx(98.5f));
        CHECK(quads[0].y == doctest::Approx(200.5f));

        CHECK(quads[1].x == doctest::Approx(100.5f));
        CHECK(quads[1].y == doctest::Approx(200.5f));

        CHECK(quads[2].x == doctest::Approx(99.5f));
        CHECK(quads[2].y == doctest::Approx(199.5f));

        CHECK(quads[3].x == doctest::Approx(99.5f));
        CHECK(quads[3].y == doctest::Approx(201.5f));

        // Main text quads at the end
        CHECK(quads[8].layer == ETextQuadLayer::Main);
        CHECK(quads[9].layer == ETextQuadLayer::Main);
    }

    TEST_CASE("LayoutGlyphs - SelectionAndCursor") {
        TestGraphicTextInstance inst;
        inst.SetPosition(50.0f, 60.0f);

        inst.AddMockChar(10, 16, 10.0f, 0.0f, 0.0f, 0.0f, 0.1f, 0.1f);
        inst.AddMockChar(10, 16, 10.0f, 0.0f, 0.1f, 0.0f, 0.2f, 0.1f);

        inst.SetSelection(0, 2);
        inst.ShowCursor();

        const auto& quads = inst.LayoutGlyphs();

        REQUIRE(quads.size() >= 4);

        // First quad should be Selection
        CHECK(quads[0].layer == ETextQuadLayer::Selection);
        CHECK(quads[0].color == 0x80339CFF);
        CHECK(quads[0].w == 20.0f);
        CHECK(quads[0].h == 16.0f);
        CHECK(quads[0].x == doctest::Approx(50.0f));
        CHECK(quads[0].y == doctest::Approx(60.0f));

        // Last quad should be Cursor
        CHECK(quads.back().layer == ETextQuadLayer::Cursor);
    }
}

