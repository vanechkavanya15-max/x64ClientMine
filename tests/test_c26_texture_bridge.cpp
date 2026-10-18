#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include <windows.h>
#include <vector>
#include <cstdint>

#include "EterLib/TextureUploadBridge.h"
#include "EterLib/RenderOwnership.h"
#include "EterLib/WgpuModule.h"

TEST_CASE("TextureUploadBridge: Argument validation for Texture2DArray")
{
    CTextureUploadBridge& bridge = CTextureUploadBridge::Instance();

    // 1. dwTextureID == 0 musi zwrocic false
    uint8_t dummyData[16] = {0};
    bool resZeroId = bridge.RegisterTextureArray(0, 128, 128, 4, 0, dummyData, sizeof(dummyData));
    CHECK(resZeroId == false);

    // 2. layers == 0 musi zwrocic false
    bool resZeroLayers = bridge.RegisterTextureArray(1001, 128, 128, 0, 0, dummyData, sizeof(dummyData));
    CHECK(resZeroLayers == false);

    // 3. dwTextureID == 0 w UpdateTextureArrayLayer musi zwrocic false
    bool resZeroUpdate = bridge.UpdateTextureArrayLayer(0, 0, dummyData, sizeof(dummyData));
    CHECK(resZeroUpdate == false);

    // 4. Overload z const void* dla dwTextureID == 0
    const void* pVoidData = dummyData;
    bool resVoidZero = bridge.RegisterTextureArray(0, 64, 64, 2, 0, pVoidData, sizeof(dummyData));
    CHECK(resVoidZero == false);

    bool resVoidZeroUpdate = bridge.UpdateTextureArrayLayer(0, 1, pVoidData, sizeof(dummyData));
    CHECK(resVoidZeroUpdate == false);
}

TEST_CASE("TextureUploadBridge: IsWgpuActive and Configuration State")
{
    CTextureUploadBridge& bridge = CTextureUploadBridge::Instance();

    bool prevConfig = RenderOwnership::IsWgpuConfigured();

    // Wymuszenie stanu nieaktywnego WGPU (jezeli modul DLL nie jest zaladowany)
    RenderOwnership::SetWgpuConfigured(false);
    if (!CWgpuModule::Instance().IsLoaded())
    {
        CHECK(bridge.IsWgpuActive() == false);
    }

    // Wymuszenie stanu aktywnego WGPU
    RenderOwnership::SetWgpuConfigured(true);
    CHECK(bridge.IsWgpuActive() == true);

    // Przywrocenie stanu poprzedniego
    RenderOwnership::SetWgpuConfigured(prevConfig);
}

TEST_CASE("TextureUploadBridge: Counter and Lifecycle")
{
    CTextureUploadBridge& bridge = CTextureUploadBridge::Instance();
    CHECK(bridge.GetRegisteredCount() >= 0);

    bridge.Shutdown();
    CHECK(bridge.GetRegisteredCount() == 0);
}
