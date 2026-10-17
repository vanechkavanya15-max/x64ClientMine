#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include <windows.h>
#include <d3d9.h>
#include <vector>
#include <cstdint>

#include "EterLib/GrpTexture.h"
#include "EterLib/GrpImage.h"
#include "EterLib/GrpImageTexture.h"
#include "EterLib/GrpSubImage.h"
#include "EterLib/GrpFontTexture.h"
#include "EterLib/TextureUploadBridge.h"

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

// Minimal Mock for IDirect3DTexture9 with reference counting
class MockDirect3DTexture9 : public IDirect3DTexture9
{
public:
    ULONG refCount;
    bool deleted;

    MockDirect3DTexture9() : refCount(1), deleted(false) {}

    // IUnknown methods
    HRESULT STDMETHODCALLTYPE QueryInterface(REFIID riid, void** ppvObj) override
    {
        if (!ppvObj)
            return E_POINTER;
        *ppvObj = nullptr;
        return E_NOINTERFACE;
    }

    ULONG STDMETHODCALLTYPE AddRef() override
    {
        return ++refCount;
    }

    ULONG STDMETHODCALLTYPE Release() override
    {
        if (refCount > 0)
        {
            --refCount;
            if (refCount == 0)
            {
                deleted = true;
                return 0;
            }
        }
        return refCount;
    }

    // IDirect3DResource9 methods
    HRESULT STDMETHODCALLTYPE GetDevice(IDirect3DDevice9** ppDevice) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetPrivateData(REFGUID refguid, const void* pData, DWORD SizeOfData, DWORD Flags) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetPrivateData(REFGUID refguid, void* pData, DWORD* pSizeOfData) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE FreePrivateData(REFGUID refguid) override { return D3D_OK; }
    DWORD STDMETHODCALLTYPE SetPriority(DWORD PriorityNew) override { return 0; }
    DWORD STDMETHODCALLTYPE GetPriority() override { return 0; }
    void STDMETHODCALLTYPE PreLoad() override {}
    D3DRESOURCETYPE STDMETHODCALLTYPE GetType() override { return D3DRTYPE_TEXTURE; }

    // IDirect3DBaseTexture9 methods
    DWORD STDMETHODCALLTYPE SetLOD(DWORD LODNew) override { return 0; }
    DWORD STDMETHODCALLTYPE GetLOD() override { return 0; }
    DWORD STDMETHODCALLTYPE GetLevelCount() override { return 1; }
    HRESULT STDMETHODCALLTYPE SetAutoGenFilterType(D3DTEXTUREFILTERTYPE FilterType) override { return D3D_OK; }
    D3DTEXTUREFILTERTYPE STDMETHODCALLTYPE GetAutoGenFilterType() override { return D3DTEXF_NONE; }
    void STDMETHODCALLTYPE GenerateMipSubLevels() override {}

    // IDirect3DTexture9 methods
    HRESULT STDMETHODCALLTYPE GetLevelDesc(UINT Level, D3DSURFACE_DESC *pDesc) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetSurfaceLevel(UINT Level, IDirect3DSurface9** ppSurfaceLevel) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE LockRect(UINT Level, D3DLOCKED_RECT* pLockedRect, const RECT* pRect, DWORD Flags) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE UnlockRect(UINT Level) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE AddDirtyRect(const RECT* pDirtyRect) override { return D3D_OK; }
};

// Derived helper to inspect and set protected members of CGraphicTexture
class TestGraphicTexture : public CGraphicTexture
{
public:
    void SetTestD3DTexture(LPDIRECT3DTEXTURE9 pTex, int w = 64, int h = 64)
    {
        m_lpd3dTexture = pTex;
        m_width = w;
        m_height = h;
        m_bEmpty = (pTex == nullptr);
    }

    void InvokeDestroy()
    {
        Destroy();
    }
};

class TestGraphicImageTexture : public CGraphicImageTexture
{
public:
    void SetTestD3DTexture(LPDIRECT3DTEXTURE9 pTex, int w = 64, int h = 64)
    {
        m_lpd3dTexture = pTex;
        m_width = w;
        m_height = h;
        m_bEmpty = (pTex == nullptr);
    }
};

class TestGraphicImage : public CGraphicImage
{
public:
    TestGraphicImage(const char* c_szFileName) : CGraphicImage(c_szFileName) {}
    ~TestGraphicImage() override = default;

    void OnConstruct() override {}
    void OnSelfDestruct() override { delete this; }

    void SetTestTexture(LPDIRECT3DTEXTURE9 pTex, int w = 64, int h = 64)
    {
        static_cast<TestGraphicImageTexture*>(&m_imageTexture)->SetTestD3DTexture(pTex, w, h);
        m_rect.left = 0;
        m_rect.top = 0;
        m_rect.right = w;
        m_rect.bottom = h;
    }
};

class TestGraphicSubImage : public CGraphicSubImage
{
public:
    TestGraphicSubImage(const char* c_szFileName) : CGraphicSubImage(c_szFileName) {}
    ~TestGraphicSubImage() override = default;

    void OnConstruct() override {}
    void OnSelfDestruct() override { delete this; }
};

TEST_SUITE("TextureOwnership")
{
    TEST_CASE("CGraphicTexture - Default ownership and mutator")
    {
        TestGraphicTexture tex;
        CHECK(tex.DoesOwnTexture() == true);

        tex.SetOwnsTexture(false);
        CHECK(tex.DoesOwnTexture() == false);

        tex.SetOwnsTexture(true);
        CHECK(tex.DoesOwnTexture() == true);
    }

    TEST_CASE("CGraphicTexture - Destroy releases texture only when owned")
    {
        // 1. Owning texture releases D3D resource on Destroy
        {
            MockDirect3DTexture9 mockTex;
            CHECK(mockTex.refCount == 1);
            CHECK(mockTex.deleted == false);

            {
                TestGraphicTexture owningTex;
                CHECK(owningTex.DoesOwnTexture() == true);
                owningTex.SetTestD3DTexture(&mockTex);

                owningTex.InvokeDestroy();
                CHECK(mockTex.refCount == 0);
                CHECK(mockTex.deleted == true);
                CHECK(owningTex.GetD3DTexture() == nullptr);
            }
        }

        // 2. Non-owning texture does NOT release D3D resource on Destroy
        {
            MockDirect3DTexture9 mockTex;
            CHECK(mockTex.refCount == 1);
            CHECK(mockTex.deleted == false);

            {
                TestGraphicTexture nonOwningTex;
                nonOwningTex.SetOwnsTexture(false);
                CHECK(nonOwningTex.DoesOwnTexture() == false);
                nonOwningTex.SetTestD3DTexture(&mockTex);

                nonOwningTex.InvokeDestroy();
                // refCount remains unchanged because non-owner must not release
                CHECK(mockTex.refCount == 1);
                CHECK(mockTex.deleted == false);
                CHECK(nonOwningTex.GetD3DTexture() == nullptr);
            }

            // Cleanup mockTex manually
            mockTex.Release();
            CHECK(mockTex.deleted == true);
        }
    }

    TEST_CASE("CGraphicImageTexture - CreateFromTexturePointer sets non-owning borrow")
    {
        MockDirect3DTexture9 mockTex;

        TestGraphicTexture srcTex;
        srcTex.SetTestD3DTexture(&mockTex, 128, 128);
        CHECK(srcTex.DoesOwnTexture() == true);

        {
            CGraphicImageTexture subTex;
            CHECK(subTex.DoesOwnTexture() == true);

            subTex.CreateFromTexturePointer(&srcTex);
            CHECK(subTex.DoesOwnTexture() == false);
            CHECK(subTex.GetD3DTexture() == &mockTex);
            CHECK(subTex.GetWidth() == 128);
            CHECK(subTex.GetHeight() == 128);

            // Destructor of subTex should NOT release mockTex
        }

        CHECK(mockTex.refCount == 1);
        CHECK(mockTex.deleted == false);

        // When source texture is destroyed, it releases the mock
        srcTex.InvokeDestroy();
        CHECK(mockTex.refCount == 0);
        CHECK(mockTex.deleted == true);
    }

    TEST_CASE("CGraphicSubImage - SubImage destruction does not release source image texture")
    {
        MockDirect3DTexture9 mockTex;

        CRef<TestGraphicImage> roSrcImage(new TestGraphicImage("source_image.dds"));
        roSrcImage->SetTestTexture(&mockTex, 256, 256);
        CHECK(roSrcImage->GetTexturePointer()->DoesOwnTexture() == true);
        CHECK(roSrcImage->GetD3DTexture() == &mockTex);

        // Scope containing CGraphicSubImage
        {
            CRef<TestGraphicSubImage> roSubImage(new TestGraphicSubImage("sub_image.sub"));
            CHECK(roSubImage->GetTexturePointer()->DoesOwnTexture() == false);

            roSubImage->SetImagePointer(roSrcImage.GetPointer());
            CHECK(roSubImage->GetTexturePointer()->DoesOwnTexture() == false);
            CHECK(roSubImage->GetD3DTexture() == &mockTex);
            CHECK(mockTex.refCount == 1);

            // Explicitly call DestroyDeviceObjects on subImage
            roSubImage->DestroyDeviceObjects();
            CHECK(mockTex.refCount == 1);
            CHECK(mockTex.deleted == false);

            roSubImage.Clear();
        }

        // After subImage is destroyed, source texture remains valid and not deleted
        CHECK(mockTex.refCount == 1);
        CHECK(mockTex.deleted == false);
        CHECK(roSrcImage->GetD3DTexture() == &mockTex);

        // Now release source image
        roSrcImage.Clear();
        CHECK(mockTex.refCount == 0);
        CHECK(mockTex.deleted == true);
    }

    TEST_CASE("CGraphicFontTexture - Initialized with non-owning alias")
    {
        CGraphicFontTexture fontTex;
        CHECK(fontTex.DoesOwnTexture() == false);

        fontTex.Destroy();
        CHECK(fontTex.DoesOwnTexture() == false);
    }

    TEST_CASE("CTextureUploadBridge - Registered texture count stability on sub-image destroy")
    {
        uint64_t initialCount = CTextureUploadBridge::Instance().GetRegisteredCount();

        MockDirect3DTexture9 mockTex;
        CRef<TestGraphicImage> roSrcImage(new TestGraphicImage("source_bridge.dds"));
        roSrcImage->SetTestTexture(&mockTex, 64, 64);

        {
            CRef<TestGraphicSubImage> roSubImage(new TestGraphicSubImage("sub_bridge.sub"));
            roSubImage->SetImagePointer(roSrcImage.GetPointer());
            roSubImage.Clear();
        }

        // SubImage destruction does not affect registered texture count
        CHECK(CTextureUploadBridge::Instance().GetRegisteredCount() == initialCount);

        // Explicit unregister does not throw or crash
        CTextureUploadBridge::Instance().UnregisterTexture(0);
        CHECK(CTextureUploadBridge::Instance().GetRegisteredCount() == initialCount);

        roSrcImage.Clear();
    }
}
