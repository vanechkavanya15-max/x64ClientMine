cat << 'DIFF_EOF' > renderer2.diff
<<<<<<< SEARCH
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

    // 1. Poprawne respektowanie flagi dwustronnosci materialu (bTwoSided)
    const DWORD targetCullMode = (item.bTwoSided != 0) ? D3DCULL_NONE : D3DCULL_CW;
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
=======
    if (!bBuffersAlreadyBound) {
        if (!EnsureStaticGpuBuffers(pDevice, pInstance))
            return;

        STATEMANAGER.SetStreamSource(0, pModel->GetVertexBuffer(), sizeof(EterLib::Render::GpuSkinnedVertex));
        STATEMANAGER.SetIndices(pModel->GetIndexBuffer(), 0);

        auto& pipeline = EterLib::Render::ActorSkinningPipeline::Instance();
        if (pipeline.IsInitialized()) {
            const auto& skinMatrices = pInstance->GetSkinningMatrices();
            if (!skinMatrices.empty()) {
                pipeline.UploadBonePalette(skinMatrices.data(), static_cast<UINT>(skinMatrices.size()));
            }
        }
        
        const D3DXMATRIX& matWorld = ToD3DX(item.matWorld);
        STATEMANAGER.SetTransform(D3DTS_WORLD, &matWorld);
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

    // 1. Poprawne respektowanie flagi dwustronnosci materialu (bTwoSided) bez alokacji pamieci
    DWORD oldCullMode = D3DCULL_CCW;
    STATEMANAGER.GetRenderState(D3DRS_CULLMODE, &oldCullMode);
    
    const DWORD targetCullMode = (item.bTwoSided != 0) ? D3DCULL_NONE : D3DCULL_CW;
    STATEMANAGER.SetRenderState(D3DRS_CULLMODE, targetCullMode);

    // 2. Izolacja stanow specyficznych dla przebiegu (Pass)
    const EModelPass pass = item.pass;
    
    DWORD oldAlphaBlend = FALSE, oldAlphaTest = FALSE;
    DWORD oldSrcBlend = D3DBLEND_ONE, oldDestBlend = D3DBLEND_ZERO;
    DWORD oldAlphaRef = 0, oldAlphaFunc = D3DCMP_ALWAYS, oldZWrite = TRUE;

    STATEMANAGER.GetRenderState(D3DRS_ALPHABLENDENABLE, &oldAlphaBlend);
    STATEMANAGER.GetRenderState(D3DRS_ALPHATESTENABLE, &oldAlphaTest);

    if (pass == EModelPass::Blend) {
        STATEMANAGER.GetRenderState(D3DRS_SRCBLEND, &oldSrcBlend);
        STATEMANAGER.GetRenderState(D3DRS_DESTBLEND, &oldDestBlend);
        STATEMANAGER.GetRenderState(D3DRS_ALPHAREF, &oldAlphaRef);
        STATEMANAGER.GetRenderState(D3DRS_ALPHAFUNC, &oldAlphaFunc);

        STATEMANAGER.SetRenderState(D3DRS_ALPHABLENDENABLE, TRUE);
        STATEMANAGER.SetRenderState(D3DRS_SRCBLEND, D3DBLEND_SRCALPHA);
        STATEMANAGER.SetRenderState(D3DRS_DESTBLEND, D3DBLEND_INVSRCALPHA);
        STATEMANAGER.SetRenderState(D3DRS_ALPHATESTENABLE, TRUE);
        STATEMANAGER.SetRenderState(D3DRS_ALPHAREF, 0x01);
        STATEMANAGER.SetRenderState(D3DRS_ALPHAFUNC, D3DCMP_GREATER);
    } else if (pass == EModelPass::ShadowDepth) {
        STATEMANAGER.GetRenderState(D3DRS_ZWRITEENABLE, &oldZWrite);

        STATEMANAGER.SetRenderState(D3DRS_ALPHABLENDENABLE, FALSE);
        STATEMANAGER.SetRenderState(D3DRS_ALPHATESTENABLE, FALSE);
        STATEMANAGER.SetRenderState(D3DRS_ZWRITEENABLE, TRUE);
    } else {
        STATEMANAGER.SetRenderState(D3DRS_ALPHABLENDENABLE, FALSE);
        STATEMANAGER.SetRenderState(D3DRS_ALPHATESTENABLE, FALSE);
    }

    // 3. Izolacja tekstur przez CStateManager bez alokacji na stosie CStateManager
    LPDIRECT3DBASETEXTURE9 oldTex0 = nullptr;
    LPDIRECT3DBASETEXTURE9 oldTex1 = nullptr;
    STATEMANAGER.GetTexture(0, &oldTex0);
    STATEMANAGER.GetTexture(1, &oldTex1);

    STATEMANAGER.SetTexture(0, pD3DTexture);
    STATEMANAGER.SetTexture(1, pD3DOpacityTexture);

    // 4. Rysowanie prymitywow (naprawiony UB obliczania numVertices dla submesh.vertexCount == 0)
    const UINT minVertexIndex = submesh.vertexOffset;
    const UINT totalVertices = static_cast<UINT>(pModel->GetVertexCount());
    const UINT numVertices = submesh.vertexCount > 0
        ? submesh.vertexCount
        : (totalVertices > minVertexIndex ? totalVertices - minVertexIndex : 0);

    STATEMANAGER.DrawIndexedPrimitive(
        D3DPT_TRIANGLELIST,
        minVertexIndex,
        numVertices,
        submesh.indexOffset,
        primitiveCount);

    // 5. Przywrocenie tekstur
    STATEMANAGER.SetTexture(1, oldTex1);
    STATEMANAGER.SetTexture(0, oldTex0);

    // 6. Przywrocenie stanow przebiegu
    if (pass == EModelPass::Blend) {
        STATEMANAGER.SetRenderState(D3DRS_ALPHAFUNC, oldAlphaFunc);
        STATEMANAGER.SetRenderState(D3DRS_ALPHAREF, oldAlphaRef);
        STATEMANAGER.SetRenderState(D3DRS_DESTBLEND, oldDestBlend);
        STATEMANAGER.SetRenderState(D3DRS_SRCBLEND, oldSrcBlend);
    } else if (pass == EModelPass::ShadowDepth) {
        STATEMANAGER.SetRenderState(D3DRS_ZWRITEENABLE, oldZWrite);
    }

    STATEMANAGER.SetRenderState(D3DRS_ALPHATESTENABLE, oldAlphaTest);
    STATEMANAGER.SetRenderState(D3DRS_ALPHABLENDENABLE, oldAlphaBlend);

    // 7. Przywrocenie stanu CullMode
    STATEMANAGER.SetRenderState(D3DRS_CULLMODE, oldCullMode);
>>>>>>> REPLACE
DIFF_EOF
patch src/EterModelLib/GltfModelRenderer.cpp < renderer2.diff
