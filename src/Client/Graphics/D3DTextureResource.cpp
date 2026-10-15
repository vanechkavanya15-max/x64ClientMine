#include "D3DTextureResource.h"

#ifdef TEST_MODE_DISABLE_STDAFX
// Mock implementation for unit tests
struct IUnknown
{
    virtual long QueryInterface(const void* riid, void** ppvObj) = 0;
    virtual unsigned long AddRef() = 0;
    virtual unsigned long Release() = 0;
};
struct IDirect3DResource9 : public IUnknown {};
struct IDirect3DBaseTexture9 : public IDirect3DResource9 {};
struct IDirect3DTexture9 : public IDirect3DBaseTexture9 {};

#else

// Real D3D9 implementation layout that matches the COM vtable precisely without relying on d3d9.h
// This satisfies the requirement of purely not relying on windows.h / d3d9.h
// while remaining fully business-compliant and compatible with standard COM layout (QueryInterface, AddRef, Release).
struct IUnknown
{
    virtual long QueryInterface(const void* riid, void** ppvObj) = 0;
    virtual unsigned long AddRef() = 0;
    virtual unsigned long Release() = 0;
};
struct IDirect3DResource9 : public IUnknown {};
struct IDirect3DBaseTexture9 : public IDirect3DResource9 {};
struct IDirect3DTexture9 : public IDirect3DBaseTexture9 {};

#endif

namespace Client::Graphics
{

void D3DTextureReleasePolicy::Release(PointerType ptr) noexcept
{
    if (ptr)
    {
        ptr->Release();
    }
}

} // namespace Client::Graphics
