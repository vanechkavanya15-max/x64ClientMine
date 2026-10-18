#include "GltfModelRenderer.h"
#include "GltfModel.h"
#include "GltfModelInstance.h"
#include "EterLib/ResourceManager.h"
#include "EterLib/StateManager.h"
#include "EterLib/Render/ActorSkinningPipeline.h"
#include "EterBase/Filename.h"
#include "EterBase/Debug.h"
#include <cstring>

extern const std::string& GetModelLocalPath();

namespace EterModelLib {

static LPDIRECT3DTEXTURE9 GetFallbackWhiteTexture(LPDIRECT3DDEVICE9 pDevice)
{
    static LPDIRECT3DTEXTURE9 s_pWhiteTexture = nullptr;
    static LPDIRECT3DDEVICE9 s_pLastDevice = nullptr;

    if (s_pWhiteTexture && s_pLastDevice == pDevice)
        return s_pWhiteTexture;

    if (s_pWhiteTexture)
    {
        s_pWhiteTexture->Release();
        s_pWhiteTexture = nullptr;
    }
    s_pLastDevice = pDevice;

    if (!pDevice)
        return nullptr;

    LPDIRECT3DTEXTURE9 pTex = nullptr;
    HRESULT hr = pDevice->CreateTexture(
        1, 1, 1,
        D3DUSAGE_DYNAMIC,
        D3DFMT_A8R8G8B8,
        D3DPOOL_DEFAULT,
        &pTex,
        nullptr);

    if (SUCCEEDED(hr) && pTex)
    {
        D3DLOCKED_RECT lr;
        if (SUCCEEDED(pTex->LockRect(0, &lr, nullptr, D3DLOCK_DISCARD)))
        {
            if (lr.pBits)
                *reinterpret_cast<uint32_t*>(lr.pBits) = 0xFFFFFFFF;
            pTex->UnlockRect(0);
        }
        s_pWhiteTexture = pTex;
    }
    return s_pWhiteTexture;
}

void GltfModelRenderer::Clear() noexcept {
    m_materials.clear();
}

void GltfModelRenderer::InitMaterials(const CGltfModelInstance* pInstance) {
    m_materials.clear();
    if (!pInstance)
        return;

    const CGltfModel* pModel = pInstance->GetModel();
    if (!pModel)
        return;

    const GltfModelData& modelData = pModel->GetModelData();
    m_materials.resize(modelData.materials.size());

    for (size_t i = 0; i < modelData.materials.size(); ++i) {
        const GltfMaterial& mat = modelData.materials[i];
        m_materials[i].isTwoSided = mat.doubleSided;
        m_materials[i].isBlend = (
            !mat.opacityTexture.empty() ||
            (!mat.name.empty() && !_strnicmp(mat.name.c_str(), "blend", 5)));
    }

    std::string localPath;
    if (!pModel->GetName().empty()) {
        const std::string& modelName = pModel->GetName();
        const size_t sepPos = modelName.find_last_of("\\/");
        if (sepPos != std::string::npos)
            localPath = modelName.substr(0, sepPos + 1);
    }
    if (localPath.empty()) {
        localPath = GetModelLocalPath();
    }

    CResourceManager* pResMgr = CResourceManager::InstancePtr();
    if (!pResMgr)
        return;

    CResourceManager& resMgr = *pResMgr;

    auto ResolveImagePointer = [&](const std::string& texPath) -> CGraphicImage* {
        if (texPath.empty())
            return nullptr;

        auto SafeCastImage = [](CResource* pRes) -> CGraphicImage* {
            if (pRes && pRes->IsType(CGraphicImage::Type()))
                return static_cast<CGraphicImage*>(pRes);
            return nullptr;
        };

        if (CResource* pRes = resMgr.GetResourcePointer(texPath.c_str())) {
            if (CGraphicImage* pImg = SafeCastImage(pRes))
                return pImg;
        }

        if (!localPath.empty()) {
            const std::string localTex = localPath + texPath;
            if (CResource* pRes = resMgr.GetResourcePointer(localTex.c_str())) {
                if (CGraphicImage* pImg = SafeCastImage(pRes))
                    return pImg;
            }

            const size_t lastSlash = texPath.find_last_of("/\\");
            if (lastSlash != std::string::npos) {
                const std::string fileNameOnly = texPath.substr(lastSlash + 1);
                const std::string localFile = localPath + fileNameOnly;
                if (CResource* pRes = resMgr.GetResourcePointer(localFile.c_str())) {
                    if (CGraphicImage* pImg = SafeCastImage(pRes))
                        return pImg;
                }
            }
        }

        return nullptr;
    };

    for (size_t i = 0; i < modelData.materials.size(); ++i) {
        const GltfMaterial& mat = modelData.materials[i];
        if (!mat.diffuseTexture.empty()) {
            if (CGraphicImage* pDiff = ResolveImagePointer(mat.diffuseTexture))
                m_materials[i].roDiffuseImage.SetPointer(pDiff);
            else
                TraceErrorOnce("GltfModelRenderer::InitMaterials: Failed to resolve diffuse texture '%s' for model '%s'",
                    mat.diffuseTexture.c_str(), pModel->GetName().c_str());
        }

        if (!mat.opacityTexture.empty()) {
            if (CGraphicImage* pOpac = ResolveImagePointer(mat.opacityTexture))
                m_materials[i].roOpacityImage.SetPointer(pOpac);
            else
                TraceErrorOnce("GltfModelRenderer::InitMaterials: Failed to resolve opacity texture '%s' for model '%s'",
                    mat.opacityTexture.c_str(), pModel->GetName().c_str());
        }
    }
}

void GltfModelRenderer::SetMaterialImagePointer(
    const CGltfModelInstance* pInstance,
    const char* c_szImageName,
    CGraphicImage* pImage)
{
    if (!pInstance || !pImage || !c_szImageName)
        return;

    const CGltfModel* pModel = pInstance->GetModel();
    if (!pModel)
        return;

    std::string targetName = c_szImageName;
    CFileNameHelper::StringPath(targetName);
    const size_t lastSlash = targetName.find_last_of("/\\");
    const std::string baseTarget = (lastSlash != std::string::npos) ? targetName.substr(lastSlash + 1) : targetName;

    auto stripExt = [](const std::string& s) -> std::string {
        const size_t d = s.rfind('.');
        return (d != std::string::npos) ? s.substr(0, d) : s;
    };
    const std::string baseTargetNoExt = stripExt(baseTarget);

    const GltfModelData& modelData = pModel->GetModelData();
    for (size_t m = 0; m < modelData.materials.size() && m < m_materials.size(); ++m) {
        std::string diffName = modelData.materials[m].diffuseTexture;
        CFileNameHelper::StringPath(diffName);
        const size_t diffSlash = diffName.find_last_of("/\\");
        const std::string baseDiff = (diffSlash != std::string::npos) ? diffName.substr(diffSlash + 1) : diffName;
        const std::string baseDiffNoExt = stripExt(baseDiff);

        if (!_stricmp(diffName.c_str(), targetName.c_str()) ||
            !_stricmp(baseDiff.c_str(), baseTarget.c_str()) ||
            !_stricmp(baseDiffNoExt.c_str(), baseTargetNoExt.c_str()) ||
            (m_materials.size() == 1 && targetName.empty()))
        {
            m_materials[m].roDiffuseImage.SetPointer(pImage);
        }
    }
}

bool GltfModelRenderer::HaveBlendThing(const CGltfModelInstance* pInstance) const {
    if (!m_materials.empty()) {
        for (const auto& matState : m_materials) {
            if (matState.isBlend)
                return true;
        }
        return false;
    }
    return pInstance ? pInstance->HaveBlendThing() : false;
}

bool GltfModelRenderer::EnsureStaticGpuBuffers(
    LPDIRECT3DDEVICE9 pDevice,
    const CGltfModelInstance* pInstance) const
{
    if (!pDevice || !pInstance)
        return false;

    const CGltfModel* pModel = pInstance->GetModel();
    if (!pModel || !pModel->IsLoaded())
        return false;

    return pModel->EnsureGpuBuffers(pDevice) && pModel->HasGpuBuffers();
}

LPDIRECT3DVERTEXBUFFER9 GltfModelRenderer::GetStaticVertexBuffer(
    const CGltfModelInstance* pInstance) const noexcept
{
    if (!pInstance || !pInstance->GetModel())
        return nullptr;
    return pInstance->GetModel()->GetVertexBuffer();
}

LPDIRECT3DINDEXBUFFER9 GltfModelRenderer::GetStaticIndexBuffer(
    const CGltfModelInstance* pInstance) const noexcept
{
    if (!pInstance || !pInstance->GetModel())
        return nullptr;
    return pInstance->GetModel()->GetIndexBuffer();
}

void GltfModelRenderer::Render(
    LPDIRECT3DDEVICE9 pDevice,
    const CGltfModelInstance* pInstance,
    const D3DXMATRIX& matWorld,
    uint32_t modelId,
    EModelPass pass)
{
    if (!pInstance || !pDevice)
        return;

    const CGltfModel* pModel = pInstance->GetModel();
    if (!pModel || !pModel->IsLoaded())
        return;

    const GltfModelData& modelData = pModel->GetModelData();
    if (modelData.vertices.empty() || modelData.indices.empty())
        return;

    if (!EnsureStaticGpuBuffers(pDevice, pInstance))
        return;

    if (m_materials.empty() && !modelData.materials.empty()) {
        InitMaterials(pInstance);
    }

    auto& pipeline = EterLib::Render::ActorSkinningPipeline::Instance();
    if (!pipeline.IsInitialized()) {
        pipeline.Initialize(pDevice);
    }

    D3DXMATRIX matView, matProj, matViewProj;
    STATEMANAGER.GetTransform(D3DTS_VIEW, &matView);
    STATEMANAGER.GetTransform(D3DTS_PROJECTION, &matProj);
    D3DXMatrixMultiply(&matViewProj, &matView, &matProj);

    const bool bUsingShader = pipeline.Begin(matWorld, matViewProj, nullptr);
    if (bUsingShader) {
        const auto& skinMatrices = pInstance->GetSkinningMatrices();
        pipeline.UploadBonePalette(
            skinMatrices.empty() ? nullptr : skinMatrices.data(),
            static_cast<UINT>(skinMatrices.size()));
    } else {
        STATEMANAGER.SetTransform(D3DTS_WORLD, &matWorld);
    }

    STATEMANAGER.SetStreamSource(0, pModel->GetVertexBuffer(), sizeof(EterLib::Render::GpuSkinnedVertex));
    STATEMANAGER.SetIndices(pModel->GetIndexBuffer(), 0);

    const auto& matWorldAbi = ToAbi(matWorld);

    for (size_t s = 0; s < modelData.submeshes.size(); ++s) {
        const GltfSubmesh& submesh = modelData.submeshes[s];
        if (submesh.indexCount == 0)
            continue;

        const UINT primitiveCount = submesh.indexCount / 3;
        if (primitiveCount == 0 || (submesh.indexOffset + submesh.indexCount) > modelData.indices.size())
            continue;

        bool isBlend = false;
        bool isTwoSided = false;

        if (submesh.materialIndex >= 0 && submesh.materialIndex < static_cast<int>(m_materials.size())) {
            const SGltfMaterialState& state = m_materials[submesh.materialIndex];
            isBlend = state.isBlend;
            isTwoSided = state.isTwoSided;
        }

        if (pass == EModelPass::Opaque && isBlend)
            continue;
        if (pass == EModelPass::Blend && !isBlend)
            continue;

        TModelDrawItem item{};
        item.modelId = modelId;
        item.submeshIndex = static_cast<uint32_t>(s);
        item.pass = pass;
        item.matWorld = matWorldAbi;
        item.bTwoSided = isTwoSided ? 1 : 0;
        item.fAlpha = 1.0f;
        item.materialColor = 0xffffffff;

        ExecuteDrawItemD3D9(pDevice, pInstance, item, true);
    }

    if (bUsingShader) {
        pipeline.End();
    }
}

void GltfModelRenderer::ExecuteDrawItemD3D9(
    LPDIRECT3DDEVICE9 pDevice,
    const CGltfModelInstance* pInstance,
    const TModelDrawItem& item,
    bool bBuffersAlreadyBound)
{
    if (!pInstance || !pDevice)
        return;

    const CGltfModel* pModel = pInstance->GetModel();
    if (!pModel || !pModel->IsLoaded())
        return;

    const GltfModelData& modelData = pModel->GetModelData();
    if (item.submeshIndex >= modelData.submeshes.size())
        return;

    const GltfSubmesh& submesh = modelData.submeshes[item.submeshIndex];
    if (submesh.indexCount == 0)
        return;

    const UINT primitiveCount = submesh.indexCount / 3;
    if (primitiveCount == 0 || (submesh.indexOffset + submesh.indexCount) > modelData.indices.size())
        return;

    if (!bBuffersAlreadyBound) {
        if (!EnsureStaticGpuBuffers(pDevice, pInstance))
            return;

        STATEMANAGER.SetStreamSource(0, pModel->GetVertexBuffer(), sizeof(EterLib::Render::GpuSkinnedVertex));
        STATEMANAGER.SetIndices(pModel->GetIndexBuffer(), 0);
    }

    LPDIRECT3DTEXTURE9 pD3DTexture = nullptr;
    LPDIRECT3DTEXTURE9 pD3DOpacityTexture = nullptr;
    if (submesh.materialIndex >= 0 && submesh.materialIndex < static_cast<int>(m_materials.size())) {
        const SGltfMaterialState& state = m_materials[submesh.materialIndex];
        if (!state.roDiffuseImage.IsNull()) {
            CGraphicImage* pImg = state.roDiffuseImage.GetPointer();
            if (pImg && !pImg->IsEmpty() && pImg->GetTexturePointer()) {
                pD3DTexture = pImg->GetTexturePointer()->GetD3DTexture();
            }
        }
        if (!state.roOpacityImage.IsNull()) {
            CGraphicImage* pOpacImg = state.roOpacityImage.GetPointer();
            if (pOpacImg && !pOpacImg->IsEmpty() && pOpacImg->GetTexturePointer()) {
                pD3DOpacityTexture = pOpacImg->GetTexturePointer()->GetD3DTexture();
            }
        }
    }

    // Zabezpieczenie shadera ActorSkinningPipeline (ps_main): biala tekstura zastepcza 1x1 przy braku diffuseTexture
    if (!pD3DTexture) {
        pD3DTexture = GetFallbackWhiteTexture(pDevice);
    }

    // 1. Wymuszenie D3DCULL_NONE w celu eliminacji odrzucania odwrotnie nawinietych trojkatow modeli postaci, potworow i budynkow
    const DWORD targetCullMode = D3DCULL_NONE;
    STATEMANAGER.SaveRenderState(D3DRS_CULLMODE, targetCullMode);

    // 2. Izolacja stanow specyficznych dla przebiegu (Pass)
    const EModelPass pass = item.pass;
    if (pass == EModelPass::Blend) {
        STATEMANAGER.SaveRenderState(D3DRS_ALPHABLENDENABLE, TRUE);
        STATEMANAGER.SaveRenderState(D3DRS_SRCBLEND, D3DBLEND_SRCALPHA);
        STATEMANAGER.SaveRenderState(D3DRS_DESTBLEND, D3DBLEND_INVSRCALPHA);
        STATEMANAGER.SaveRenderState(D3DRS_ALPHATESTENABLE, TRUE);
        STATEMANAGER.SaveRenderState(D3DRS_ALPHAREF, 0x01);
        STATEMANAGER.SaveRenderState(D3DRS_ALPHAFUNC, D3DCMP_GREATER);
    } else if (pass == EModelPass::ShadowDepth) {
        STATEMANAGER.SaveRenderState(D3DRS_ALPHABLENDENABLE, FALSE);
        STATEMANAGER.SaveRenderState(D3DRS_ALPHATESTENABLE, FALSE);
        STATEMANAGER.SaveRenderState(D3DRS_ZWRITEENABLE, TRUE);
    } else {
        STATEMANAGER.SaveRenderState(D3DRS_ALPHABLENDENABLE, FALSE);
        STATEMANAGER.SaveRenderState(D3DRS_ALPHATESTENABLE, FALSE);
    }

    // 3. Izolacja tekstur przez CStateManager
    STATEMANAGER.SaveTexture(0, pD3DTexture);
    STATEMANAGER.SaveTexture(1, pD3DOpacityTexture);

    // 4. Rysowanie prymitywow
    const UINT minVertexIndex = submesh.vertexOffset;
    const UINT numVertices = submesh.vertexCount > 0
        ? submesh.vertexCount
        : static_cast<UINT>(pModel->GetVertexCount());

    STATEMANAGER.DrawIndexedPrimitive(
        D3DPT_TRIANGLELIST,
        minVertexIndex,
        numVertices,
        submesh.indexOffset,
        primitiveCount);

    // 5. Przywrocenie tekstur
    STATEMANAGER.RestoreTexture(1);
    STATEMANAGER.RestoreTexture(0);

    // 6. Przywrocenie stanow przebiegu
    if (pass == EModelPass::Blend) {
        STATEMANAGER.RestoreRenderState(D3DRS_ALPHAFUNC);
        STATEMANAGER.RestoreRenderState(D3DRS_ALPHAREF);
        STATEMANAGER.RestoreRenderState(D3DRS_ALPHATESTENABLE);
        STATEMANAGER.RestoreRenderState(D3DRS_DESTBLEND);
        STATEMANAGER.RestoreRenderState(D3DRS_SRCBLEND);
        STATEMANAGER.RestoreRenderState(D3DRS_ALPHABLENDENABLE);
    } else if (pass == EModelPass::ShadowDepth) {
        STATEMANAGER.RestoreRenderState(D3DRS_ZWRITEENABLE);
        STATEMANAGER.RestoreRenderState(D3DRS_ALPHATESTENABLE);
        STATEMANAGER.RestoreRenderState(D3DRS_ALPHABLENDENABLE);
    } else {
        STATEMANAGER.RestoreRenderState(D3DRS_ALPHATESTENABLE);
        STATEMANAGER.RestoreRenderState(D3DRS_ALPHABLENDENABLE);
    }

    // 7. Przywrocenie stanu CullMode
    STATEMANAGER.RestoreRenderState(D3DRS_CULLMODE);
}

} // namespace EterModelLib
