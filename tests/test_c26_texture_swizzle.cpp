#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include <windows.h>
#include <vector>
#include <cstdint>

#include "EterLib/GrpTexture.h"

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

TEST_SUITE("TextureSwizzle")
{
    TEST_CASE("SwizzleRgbaToBgra - Row pitch != width * 4 (pitch padding preserved)")
    {
        const size_t width = 5;
        const size_t height = 4;
        const size_t rowBytes = width * 4; // 20 bytes
        const size_t dstPitch = 32;        // 12 bytes padding per row
        const uint8_t canary = 0xCD;

        std::vector<uint8_t> src(rowBytes * height);
        for (size_t y = 0; y < height; ++y)
        {
            for (size_t x = 0; x < width; ++x)
            {
                size_t idx = (y * width + x) * 4;
                src[idx + 0] = static_cast<uint8_t>(x * 20 + y * 5 + 1); // R
                src[idx + 1] = static_cast<uint8_t>(x * 10 + y * 7 + 2); // G
                src[idx + 2] = static_cast<uint8_t>(x * 30 + y * 9 + 3); // B
                src[idx + 3] = static_cast<uint8_t>(200 + y);            // A
            }
        }

        std::vector<uint8_t> dst(dstPitch * height, canary);

        SwizzleRgbaToBgra(dst.data(), dstPitch, src.data(), width, height);

        for (size_t y = 0; y < height; ++y)
        {
            for (size_t x = 0; x < width; ++x)
            {
                size_t srcIdx = (y * width + x) * 4;
                size_t dstIdx = y * dstPitch + x * 4;

                // BGRA output from RGBA input
                CHECK(dst[dstIdx + 0] == src[srcIdx + 2]); // B == src.B
                CHECK(dst[dstIdx + 1] == src[srcIdx + 1]); // G == src.G
                CHECK(dst[dstIdx + 2] == src[srcIdx + 0]); // R == src.R
                CHECK(dst[dstIdx + 3] == src[srcIdx + 3]); // A == src.A
            }

            // Verify padding bytes are untouched
            for (size_t p = rowBytes; p < dstPitch; ++p)
            {
                CHECK(dst[y * dstPitch + p] == canary);
            }
        }
    }

    TEST_CASE("SwizzleRgbaToBgra - SIMD width multiple of 4 with row padding")
    {
        const size_t width = 16;
        const size_t height = 3;
        const size_t rowBytes = width * 4; // 64 bytes
        const size_t dstPitch = 80;        // 16 bytes padding
        const uint8_t canary = 0xEF;

        std::vector<uint8_t> src(rowBytes * height);
        for (size_t i = 0; i < src.size(); ++i)
        {
            src[i] = static_cast<uint8_t>(i & 0xFF);
        }

        std::vector<uint8_t> dst(dstPitch * height, canary);

        SwizzleRgbaToBgra(dst.data(), dstPitch, src.data(), width, height);

        for (size_t y = 0; y < height; ++y)
        {
            for (size_t x = 0; x < width; ++x)
            {
                size_t srcIdx = (y * width + x) * 4;
                size_t dstIdx = y * dstPitch + x * 4;

                CHECK(dst[dstIdx + 0] == src[srcIdx + 2]);
                CHECK(dst[dstIdx + 1] == src[srcIdx + 1]);
                CHECK(dst[dstIdx + 2] == src[srcIdx + 0]);
                CHECK(dst[dstIdx + 3] == src[srcIdx + 3]);
            }

            for (size_t p = rowBytes; p < dstPitch; ++p)
            {
                CHECK(dst[y * dstPitch + p] == canary);
            }
        }
    }

    TEST_CASE("SwizzleRgbaToBgra - Mixed SIMD and scalar (width not multiple of 4)")
    {
        const size_t width = 19;           // 16 SIMD + 3 scalar
        const size_t height = 5;
        const size_t rowBytes = width * 4; // 76 bytes
        const size_t dstPitch = 96;        // 20 bytes padding
        const uint8_t canary = 0xBA;

        std::vector<uint8_t> src(rowBytes * height);
        for (size_t i = 0; i < src.size(); ++i)
        {
            src[i] = static_cast<uint8_t>((i * 7 + 13) & 0xFF);
        }

        std::vector<uint8_t> dst(dstPitch * height, canary);

        SwizzleRgbaToBgra(dst.data(), dstPitch, src.data(), width, height);

        for (size_t y = 0; y < height; ++y)
        {
            for (size_t x = 0; x < width; ++x)
            {
                size_t srcIdx = (y * width + x) * 4;
                size_t dstIdx = y * dstPitch + x * 4;

                CHECK(dst[dstIdx + 0] == src[srcIdx + 2]);
                CHECK(dst[dstIdx + 1] == src[srcIdx + 1]);
                CHECK(dst[dstIdx + 2] == src[srcIdx + 0]);
                CHECK(dst[dstIdx + 3] == src[srcIdx + 3]);
            }

            for (size_t p = rowBytes; p < dstPitch; ++p)
            {
                CHECK(dst[y * dstPitch + p] == canary);
            }
        }
    }

    TEST_CASE("SwizzleRgbaToBgra - Small widths (1x1, 2x2, 3x3 scalar only)")
    {
        SUBCASE("1x1")
        {
            const size_t width = 1;
            const size_t height = 1;
            const size_t dstPitch = 16;
            const uint8_t canary = 0x55;

            uint8_t src[4] = { 10, 20, 30, 40 };
            std::vector<uint8_t> dst(dstPitch, canary);

            SwizzleRgbaToBgra(dst.data(), dstPitch, src, width, height);

            CHECK(dst[0] == 30);
            CHECK(dst[1] == 20);
            CHECK(dst[2] == 10);
            CHECK(dst[3] == 40);

            for (size_t p = 4; p < dstPitch; ++p)
            {
                CHECK(dst[p] == canary);
            }
        }

        SUBCASE("3x3")
        {
            const size_t width = 3;
            const size_t height = 3;
            const size_t rowBytes = 12;
            const size_t dstPitch = 32;
            const uint8_t canary = 0x77;

            std::vector<uint8_t> src(rowBytes * height);
            for (size_t i = 0; i < src.size(); ++i)
                src[i] = static_cast<uint8_t>(i + 1);

            std::vector<uint8_t> dst(dstPitch * height, canary);

            SwizzleRgbaToBgra(dst.data(), dstPitch, src.data(), width, height);

            for (size_t y = 0; y < height; ++y)
            {
                for (size_t x = 0; x < width; ++x)
                {
                    size_t srcIdx = (y * width + x) * 4;
                    size_t dstIdx = y * dstPitch + x * 4;
                    CHECK(dst[dstIdx + 0] == src[srcIdx + 2]);
                    CHECK(dst[dstIdx + 1] == src[srcIdx + 1]);
                    CHECK(dst[dstIdx + 2] == src[srcIdx + 0]);
                    CHECK(dst[dstIdx + 3] == src[srcIdx + 3]);
                }
                for (size_t p = rowBytes; p < dstPitch; ++p)
                {
                    CHECK(dst[y * dstPitch + p] == canary);
                }
            }
        }
    }

    TEST_CASE("SwizzleRgbaToBgra - Tightly packed buffer (dstPitch == width * 4)")
    {
        const size_t width = 8;
        const size_t height = 8;
        const size_t rowBytes = width * 4;
        const size_t dstPitch = rowBytes;

        std::vector<uint8_t> src(rowBytes * height);
        for (size_t i = 0; i < src.size(); ++i)
            src[i] = static_cast<uint8_t>(i & 0xFF);

        std::vector<uint8_t> dst(dstPitch * height, 0);

        SwizzleRgbaToBgra(dst.data(), dstPitch, src.data(), width, height);

        for (size_t y = 0; y < height; ++y)
        {
            for (size_t x = 0; x < width; ++x)
            {
                size_t srcIdx = (y * width + x) * 4;
                size_t dstIdx = y * dstPitch + x * 4;
                CHECK(dst[dstIdx + 0] == src[srcIdx + 2]);
                CHECK(dst[dstIdx + 1] == src[srcIdx + 1]);
                CHECK(dst[dstIdx + 2] == src[srcIdx + 0]);
                CHECK(dst[dstIdx + 3] == src[srcIdx + 3]);
            }
        }
    }

    TEST_CASE("SwizzleRgbaToBgra - Custom srcPitch > width * 4")
    {
        const size_t width = 6;
        const size_t height = 3;
        const size_t rowBytes = width * 4; // 24
        const size_t srcPitch = 32;        // 8 bytes padding
        const size_t dstPitch = 40;        // 16 bytes padding
        const uint8_t canary = 0x33;

        std::vector<uint8_t> src(srcPitch * height, 0x00);
        for (size_t y = 0; y < height; ++y)
        {
            for (size_t x = 0; x < width; ++x)
            {
                size_t idx = y * srcPitch + x * 4;
                src[idx + 0] = static_cast<uint8_t>(x + 10);
                src[idx + 1] = static_cast<uint8_t>(y + 20);
                src[idx + 2] = static_cast<uint8_t>(x + y + 30);
                src[idx + 3] = 255;
            }
        }

        std::vector<uint8_t> dst(dstPitch * height, canary);

        SwizzleRgbaToBgra(dst.data(), dstPitch, src.data(), width, height, srcPitch);

        for (size_t y = 0; y < height; ++y)
        {
            for (size_t x = 0; x < width; ++x)
            {
                size_t srcIdx = y * srcPitch + x * 4;
                size_t dstIdx = y * dstPitch + x * 4;
                CHECK(dst[dstIdx + 0] == src[srcIdx + 2]);
                CHECK(dst[dstIdx + 1] == src[srcIdx + 1]);
                CHECK(dst[dstIdx + 2] == src[srcIdx + 0]);
                CHECK(dst[dstIdx + 3] == src[srcIdx + 3]);
            }

            for (size_t p = rowBytes; p < dstPitch; ++p)
            {
                CHECK(dst[y * dstPitch + p] == canary);
            }
        }
    }

    TEST_CASE("SwizzleRgbaToBgra - Null and zero bounds safety")
    {
        uint8_t buf[16] = { 0 };

        // Should return without crashing or modifying buffers
        SwizzleRgbaToBgra(nullptr, 16, buf, 4, 1);
        SwizzleRgbaToBgra(buf, 16, nullptr, 4, 1);
        SwizzleRgbaToBgra(buf, 16, buf, 0, 1);
        SwizzleRgbaToBgra(buf, 16, buf, 4, 0);

        // dstPitch smaller than width * 4 should be safely ignored
        uint8_t canaries[16] = { 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA,
                                 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA };
        SwizzleRgbaToBgra(canaries, 8, buf, 4, 1); // 4*4 = 16 > 8
        for (int i = 0; i < 16; ++i)
        {
            CHECK(canaries[i] == 0xAA);
        }
    }

    TEST_CASE("CGraphicTexture static wrapper functions")
    {
        const size_t width = 4;
        const size_t height = 2;
        const size_t rowBytes = 16;
        const size_t dstPitch = 24;
        const uint8_t canary = 0x88;

        std::vector<uint8_t> src(rowBytes * height);
        for (size_t i = 0; i < src.size(); ++i)
            src[i] = static_cast<uint8_t>(i);

        std::vector<uint8_t> dst(dstPitch * height, canary);

        CGraphicTexture::SwizzleRgbaToBgra(dst.data(), dstPitch, src.data(), width, height);

        for (size_t y = 0; y < height; ++y)
        {
            for (size_t x = 0; x < width; ++x)
            {
                size_t srcIdx = (y * width + x) * 4;
                size_t dstIdx = y * dstPitch + x * 4;
                CHECK(dst[dstIdx + 0] == src[srcIdx + 2]);
                CHECK(dst[dstIdx + 1] == src[srcIdx + 1]);
                CHECK(dst[dstIdx + 2] == src[srcIdx + 0]);
                CHECK(dst[dstIdx + 3] == src[srcIdx + 3]);
            }
            for (size_t p = rowBytes; p < dstPitch; ++p)
            {
                CHECK(dst[y * dstPitch + p] == canary);
            }
        }

        // Test CreateTextureFromRGBA null parameter rejection
        LPDIRECT3DTEXTURE9 pTex = reinterpret_cast<LPDIRECT3DTEXTURE9>(0x1234);
        bool result = CGraphicTexture::CreateTextureFromRGBA(nullptr, 16, 16, D3DFMT_A8R8G8B8, src.data(), 16 * 4, &pTex);
        CHECK_FALSE(result);
        CHECK(pTex == reinterpret_cast<LPDIRECT3DTEXTURE9>(0x1234));

        result = CGraphicTexture::CreateTextureFromRGBA(nullptr, 0, 0, D3DFMT_A8R8G8B8, nullptr, 0, nullptr);
        CHECK_FALSE(result);
    }
}
