#pragma once

#include <d3d9.h>
#include <d3dx9.h>
#include <cstdint>
#include <vector>
#include <string>
#include "EterLib/GrpImage.h"
#include "EterLib/RenderPacketTypes.h"
#include "GltfTypes.h"

namespace EterModelLib {

class CGltfModel;
class CGltfModelInstance;

struct SGltfMaterialState {
    CGraphicImage::TRef roDiffuseImage;
    CGraphicImage::TRef roOpacityImage;
    bool isBlend{false};
    bool isTwoSided{false};
};

class GltfModelRenderer {
public:
    GltfModelRenderer() = default;
    ~GltfModelRenderer() = default;

    GltfModelRenderer(const GltfModelRenderer&) = delete;
    GltfModelRenderer& operator=(const GltfModelRenderer&) = delete;
    GltfModelRenderer(GltfModelRenderer&&) noexcept = default;
    GltfModelRenderer& operator=(GltfModelRenderer&&) noexcept = default;

    void Clear() noexcept;

    void InitMaterials(const CGltfModelInstance* pInstance);
    void SetMaterialImagePointer(
        const CGltfModelInstance* pInstance,
        const char* c_szImageName,
        CGraphicImage* pImage);

    [[nodiscard]] bool HaveBlendThing(const CGltfModelInstance* pInstance) const;

    bool EnsureStaticGpuBuffers(
        LPDIRECT3DDEVICE9 pDevice,
        const CGltfModelInstance* pInstance) const;

    [[nodiscard]] LPDIRECT3DVERTEXBUFFER9 GetStaticVertexBuffer(
        const CGltfModelInstance* pInstance) const noexcept;
    [[nodiscard]] LPDIRECT3DINDEXBUFFER9 GetStaticIndexBuffer(
        const CGltfModelInstance* pInstance) const noexcept;

    void Render(
        LPDIRECT3DDEVICE9 pDevice,
        const CGltfModelInstance* pInstance,
        const D3DXMATRIX& matWorld,
        uint32_t modelId,
        EModelPass pass);

    void ExecuteDrawItemD3D9(
        LPDIRECT3DDEVICE9 pDevice,
        const CGltfModelInstance* pInstance,
        const TModelDrawItem& item);

    [[nodiscard]] const std::vector<SGltfMaterialState>& GetMaterialStates() const noexcept {
        return m_materials;
    }

private:
    std::vector<SGltfMaterialState> m_materials;
};

} // namespace EterModelLib
