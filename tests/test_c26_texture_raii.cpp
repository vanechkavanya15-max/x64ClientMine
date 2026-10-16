#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include "../src/Client/Graphics/D3DTextureResource.h"

// Define the mock structure explicitly for testing, matching the COM layout
struct IUnknown
{
    virtual long QueryInterface(const void* riid, void** ppvObj) = 0;
    virtual unsigned long AddRef() = 0;
    virtual unsigned long Release() = 0;
};
struct IDirect3DResource9 : public IUnknown {};
struct IDirect3DBaseTexture9 : public IDirect3DResource9 {};
struct IDirect3DTexture9 : public IDirect3DBaseTexture9 {};

using namespace Client::Graphics;

class MockD3DTexture : public IDirect3DTexture9
{
public:
    int releaseCount = 0;

    long QueryInterface(const void* riid, void** ppvObj) override { return 0; }
    unsigned long AddRef() override { return 1; }

    unsigned long Release() override
    {
        releaseCount++;
        return 0;
    }
};

TEST_CASE("ResourceHandle basic RAII operations") {
    SUBCASE("Default constructor creates empty handle") {
        D3DTextureHandle handle;
        CHECK(handle.IsValid() == false);
        CHECK(handle.Get() == nullptr);
        CHECK(static_cast<bool>(handle) == false);
    }

    SUBCASE("Handle takes ownership of pointer") {
        MockD3DTexture tex;
        {
            D3DTextureHandle handle(&tex);
            CHECK(handle.IsValid() == true);
            CHECK(handle.Get() == &tex);
            CHECK(tex.releaseCount == 0);
        }
        CHECK(tex.releaseCount == 1);
    }
}

TEST_CASE("ResourceHandle move semantics") {
    SUBCASE("Move constructor transfers ownership") {
        MockD3DTexture tex;
        {
            D3DTextureHandle h1(&tex);
            D3DTextureHandle h2(std::move(h1));
            
            CHECK(h1.IsValid() == false);
            CHECK(h1.Get() == nullptr);
            
            CHECK(h2.IsValid() == true);
            CHECK(h2.Get() == &tex);
            CHECK(tex.releaseCount == 0);
        }
        CHECK(tex.releaseCount == 1);
    }

    SUBCASE("Move assignment transfers ownership and releases old resource") {
        MockD3DTexture tex1;
        MockD3DTexture tex2;
        
        {
            D3DTextureHandle h1(&tex1);
            D3DTextureHandle h2(&tex2);
            
            h2 = std::move(h1);
            
            CHECK(tex2.releaseCount == 1); // h2 old resource released
            
            CHECK(h1.IsValid() == false);
            CHECK(h2.IsValid() == true);
            CHECK(h2.Get() == &tex1);
            CHECK(tex1.releaseCount == 0);
        }
        CHECK(tex1.releaseCount == 1);
    }
}

TEST_CASE("ResourceHandle reset and release") {
    SUBCASE("Reset with no args releases resource and clears handle") {
        MockD3DTexture tex;
        D3DTextureHandle handle(&tex);
        
        handle.Reset();
        CHECK(tex.releaseCount == 1);
        CHECK(handle.IsValid() == false);
    }

    SUBCASE("Reset with new resource releases old and takes new") {
        MockD3DTexture tex1;
        MockD3DTexture tex2;
        D3DTextureHandle handle(&tex1);
        
        handle.Reset(&tex2);
        CHECK(tex1.releaseCount == 1);
        CHECK(handle.IsValid() == true);
        CHECK(handle.Get() == &tex2);
        CHECK(tex2.releaseCount == 0);
    }

    SUBCASE("Release returns pointer and clears handle without calling Release") {
        MockD3DTexture tex;
        IDirect3DTexture9* rawPtr = nullptr;
        {
            D3DTextureHandle handle(&tex);
            rawPtr = handle.Release();
            
            CHECK(rawPtr == &tex);
            CHECK(handle.IsValid() == false);
        }
        CHECK(tex.releaseCount == 0); // Not released!
    }
}

TEST_CASE("ResourceHandle GetAddressOf") {
    SUBCASE("Returns address of internal pointer") {
        D3DTextureHandle handle;
        IDirect3DTexture9** ptr = handle.GetAddressOf();
        CHECK(ptr != nullptr);
        CHECK(*ptr == nullptr);
        
        MockD3DTexture tex;
        *ptr = &tex;
        CHECK(handle.Get() == &tex);
        
        handle.Reset(); // clean up to avoid memory issues with real COM objects, though mock just incs counter
    }
}

TEST_CASE("ResourceHandle smart pointer semantics") {
    SUBCASE("operator-> returns internal pointer") {
        MockD3DTexture tex;
        D3DTextureHandle handle(&tex);
        CHECK(handle.operator->() == &tex);
    }
    
    SUBCASE("operator* returns reference to object") {
        MockD3DTexture tex;
        D3DTextureHandle handle(&tex);
        CHECK(&handle.operator*() == &tex);
    }
}

TEST_CASE("ResourceHandle comparison operators") {
    MockD3DTexture tex1;
    MockD3DTexture tex2;
    D3DTextureHandle h1(&tex1);
    D3DTextureHandle h2(&tex2);
    D3DTextureHandle h3(&tex1);
    
    SUBCASE("operator== and operator!= with other handles") {
        CHECK(h1 == h3);
        CHECK(h1 != h2);
    }
    
    SUBCASE("operator== and operator!= with pointers") {
        CHECK(h1 == &tex1);
        CHECK(h1 != &tex2);
        CHECK(h1 != nullptr);
        
        D3DTextureHandle empty;
        CHECK(empty == nullptr);
    }
}

TEST_CASE("ResourceHandle swap") {
    MockD3DTexture tex1;
    MockD3DTexture tex2;
    D3DTextureHandle h1(&tex1);
    D3DTextureHandle h2(&tex2);
    
    SUBCASE("Swap member function") {
        h1.Swap(h2);
        CHECK(h1.Get() == &tex2);
        CHECK(h2.Get() == &tex1);
        CHECK(tex1.releaseCount == 0);
        CHECK(tex2.releaseCount == 0);
    }
    
    SUBCASE("swap free function") {
        using std::swap;
        swap(h1, h2);
        CHECK(h1.Get() == &tex2);
        CHECK(h2.Get() == &tex1);
    }
}

TEST_CASE("ResourceHandle prevents self-assignment leak via m_ptr check") {
    MockD3DTexture tex;
    D3DTextureHandle handle(&tex);
    handle.Reset(&tex);
    CHECK(tex.releaseCount == 0);
    CHECK(handle.Get() == &tex);
}
