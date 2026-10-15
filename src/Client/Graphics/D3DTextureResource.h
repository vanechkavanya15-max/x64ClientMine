#pragma once

#include "ResourceHandle.h"
#include <cstdint>

// Forward declare D3D-like interfaces for mock/headless compatibility
struct IDirect3DTexture9;

namespace Client::Graphics
{

struct D3DTextureReleasePolicy
{
    using PointerType = IDirect3DTexture9*;

    static void Release(PointerType ptr) noexcept;
    static constexpr PointerType InvalidValue() noexcept { return nullptr; }
};

using D3DTextureHandle = ResourceHandle<D3DTextureReleasePolicy>;

} // namespace Client::Graphics
