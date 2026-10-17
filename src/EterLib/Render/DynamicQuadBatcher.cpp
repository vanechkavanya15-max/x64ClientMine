#include "../StdAfx.h"
#include "DynamicQuadBatcher.h"
#include "EterLib/RenderQueueManager.h"
#include "EterLib/GrpImageInstance.h"
#include "EterLib/GrpImage.h"
#include <algorithm>

namespace EterLib::Render
{
    DynamicQuadBatcher& DynamicQuadBatcher::Instance()
    {
        static DynamicQuadBatcher s_instance;
        return s_instance;
    }

    DynamicQuadBatcher::DynamicQuadBatcher(size_t vertexCapacity, size_t batchCapacity)
    {
        m_frameVertexBuffer.resize(vertexCapacity);
        m_frameVertexOffset = 0;
        m_activeVertexCount = 0;
        m_batchPool.resize(batchCapacity);
        m_activeBatchCount = 0;
        m_overflowQuads = 0;
        m_overflowVertices = 0;
    }

    void DynamicQuadBatcher::Reserve(size_t vertexCapacity, size_t batchCapacity)
    {
        if (vertexCapacity > m_frameVertexBuffer.size())
            m_frameVertexBuffer.resize(vertexCapacity);

        if (batchCapacity > m_batchPool.size())
            m_batchPool.resize(batchCapacity);
    }

    void DynamicQuadBatcher::BeginFrame() noexcept
    {
        m_frameVertexOffset = 0;
        m_activeVertexCount = 0;
        m_activeBatchCount = 0;
        m_overflowQuads = 0;
        m_overflowVertices = 0;
    }

    void DynamicQuadBatcher::SetWhiteTexture(uintptr_t dwTextureID, float u, float v) noexcept
    {
        m_dwWhiteTextureID = dwTextureID;
        m_fWhitePixelU = u;
        m_fWhitePixelV = v;
    }

    void DynamicQuadBatcher::SetScissorRect(const RECT& rect, bool bEnable) noexcept
    {
        if (m_bScissorEnable == bEnable && AreRectsEqual(m_currentScissorRect, rect))
            return;

        m_bScissorEnable = bEnable;
        m_currentScissorRect = rect;

        // Wymus nowa partie przy zmianie stanu Scissor
        if (m_activeBatchCount > 0 && m_batchPool[m_activeBatchCount - 1].vertexCount > 0)
        {
            const auto& last = m_batchPool[m_activeBatchCount - 1];
            PrepareBatch(last.dwTextureID, last.dwBlendMode);
        }
    }

    void DynamicQuadBatcher::ResetScissorRect() noexcept
    {
        RECT emptyRect{ 0, 0, 0, 0 };
        SetScissorRect(emptyRect, false);
    }

    void DynamicQuadBatcher::GetScissorRect(RECT* pOutRect, bool* pbOutEnable) const noexcept
    {
        if (pOutRect)
            *pOutRect = m_currentScissorRect;
        if (pbOutEnable)
            *pbOutEnable = m_bScissorEnable;
    }

    bool DynamicQuadBatcher::AreRectsEqual(const RECT& a, const RECT& b) noexcept
    {
        return a.left == b.left && a.top == b.top && a.right == b.right && a.bottom == b.bottom;
    }

    void DynamicQuadBatcher::PrepareBatch(uintptr_t dwTextureID, uint32_t dwBlendMode)
    {
        if (m_activeBatchCount > 0)
        {
            auto& lastBatch = m_batchPool[m_activeBatchCount - 1];
            if (lastBatch.dwTextureID == dwTextureID &&
                lastBatch.dwBlendMode == dwBlendMode &&
                lastBatch.bUseScissor == (m_bScissorEnable ? 1u : 0u) &&
                (!m_bScissorEnable || AreRectsEqual(lastBatch.scissorRect, m_currentScissorRect)))
            {
                // Jesli partia nie przekracza limitu 16380 czworokatow (65520 wierzcholkow), dopisujemy do niej
                if (lastBatch.vertexCount + 4 <= MAX_VERTICES_PER_BATCH)
                {
                    return;
                }
                // W przeciwnym razie wymuszamy nowa podpartie (sub-batch split) dla zachowania limitu 16-bitowych indeksow
            }
        }

        if (m_activeBatchCount >= m_batchPool.size())
        {
            m_batchPool.resize(m_batchPool.size() * 2);
        }

        auto& newBatch = m_batchPool[m_activeBatchCount++];
        newBatch.dwTextureID = dwTextureID;
        newBatch.dwBlendMode = dwBlendMode;
        newBatch.bUseScissor = m_bScissorEnable ? 1u : 0u;
        newBatch.scissorRect = m_currentScissorRect;
        newBatch.startIndex = static_cast<uint32_t>(m_frameVertexOffset);
        newBatch.vertexCount = 0;
    }

    void DynamicQuadBatcher::AddQuad(float x, float y, float w, float h,
                                     uint32_t color, uintptr_t dwTextureID,
                                     float u0, float v0, float u1, float v1,
                                     uint32_t dwBlendMode)
    {
        TUIVertex vertices[4];

        // V0: Lewy-gorny
        vertices[0].position = D3DXVECTOR3(x, y, 0.0f);
        vertices[0].color = color;
        vertices[0].texCoord = D3DXVECTOR2(u0, v0);

        // V1: Prawy-gorny
        vertices[1].position = D3DXVECTOR3(x + w, y, 0.0f);
        vertices[1].color = color;
        vertices[1].texCoord = D3DXVECTOR2(u1, v0);

        // V2: Lewy-dolny
        vertices[2].position = D3DXVECTOR3(x, y + h, 0.0f);
        vertices[2].color = color;
        vertices[2].texCoord = D3DXVECTOR2(u0, v1);

        // V3: Prawy-dolny
        vertices[3].position = D3DXVECTOR3(x + w, y + h, 0.0f);
        vertices[3].color = color;
        vertices[3].texCoord = D3DXVECTOR2(u1, v1);

        AddQuadVertices(vertices, dwTextureID, dwBlendMode);
    }

    void DynamicQuadBatcher::AddQuadVertices(const TUIVertex vertices[4],
                                             uintptr_t dwTextureID,
                                             uint32_t dwBlendMode)
    {
        // Zakaz reallokacji / resize Vertex Buffera w trakcie rysowania klatki (eliminacja Use-After-Free)
        if (m_frameVertexOffset + 4 > m_frameVertexBuffer.size())
        {
            ++m_overflowQuads;
            m_overflowVertices += 4;
            return;
        }

        PrepareBatch(dwTextureID, dwBlendMode);

        TUIVertex* pDst = &m_frameVertexBuffer[m_frameVertexOffset];
        pDst[0] = vertices[0];
        pDst[1] = vertices[1];
        pDst[2] = vertices[2];
        pDst[3] = vertices[3];

        m_frameVertexOffset += 4;
        m_activeVertexCount += 4;
        m_batchPool[m_activeBatchCount - 1].vertexCount += 4;
    }

    void DynamicQuadBatcher::AddBar(float x, float y, float w, float h, uint32_t color)
    {
        uintptr_t texId = m_dwWhiteTextureID;
        float u = m_fWhitePixelU;
        float v = m_fWhitePixelV;
        AddQuad(x, y, w, h, color, texId, u, v, u, v, 0);
    }

    void DynamicQuadBatcher::AddBox(float x, float y, float w, float h, uint32_t color)
    {
        AddBox2d(x, y, x + w, y + h, color, 1.0f);
    }

    void DynamicQuadBatcher::AddBar2d(float sx, float sy, float ex, float ey, uint32_t color)
    {
        float x = std::min(sx, ex);
        float y = std::min(sy, ey);
        float w = std::abs(ex - sx);
        float h = std::abs(ey - sy);
        AddBar(x, y, w, h, color);
    }

    void DynamicQuadBatcher::AddBox2d(float sx, float sy, float ex, float ey, uint32_t color, float thickness)
    {
        float left = std::min(sx, ex);
        float right = std::max(sx, ex);
        float top = std::min(sy, ey);
        float bottom = std::max(sy, ey);
        float w = right - left;
        float h = bottom - top;

        // Gora, dol, lewo, prawo
        AddBar(left, top, w, thickness, color);
        AddBar(left, bottom - thickness, w, thickness, color);
        AddBar(left, top + thickness, thickness, std::max(0.0f, h - 2.0f * thickness), color);
        AddBar(right - thickness, top + thickness, thickness, std::max(0.0f, h - 2.0f * thickness), color);
    }

    void DynamicQuadBatcher::AddLine2d(float sx, float sy, float ex, float ey, uint32_t color, float thickness)
    {
        float dx = ex - sx;
        float dy = ey - sy;
        float len = sqrtf(dx * dx + dy * dy);
        if (len < 0.001f)
            return;

        float nx = -dy / len * (thickness * 0.5f);
        float ny = dx / len * (thickness * 0.5f);

        TUIVertex v[4];
        v[0].position = D3DXVECTOR3(sx + nx, sy + ny, 0.0f);
        v[0].color = color;
        v[0].texCoord = D3DXVECTOR2(m_fWhitePixelU, m_fWhitePixelV);

        v[1].position = D3DXVECTOR3(ex + nx, ey + ny, 0.0f);
        v[1].color = color;
        v[1].texCoord = D3DXVECTOR2(m_fWhitePixelU, m_fWhitePixelV);

        v[2].position = D3DXVECTOR3(sx - nx, sy - ny, 0.0f);
        v[2].color = color;
        v[2].texCoord = D3DXVECTOR2(m_fWhitePixelU, m_fWhitePixelV);

        v[3].position = D3DXVECTOR3(ex - nx, ey - ny, 0.0f);
        v[3].color = color;
        v[3].texCoord = D3DXVECTOR2(m_fWhitePixelU, m_fWhitePixelV);

        AddQuadVertices(v, m_dwWhiteTextureID, 0);
    }

    void DynamicQuadBatcher::AddImageInstance(CGraphicImageInstance* pInstance, float x, float y, uint32_t overrideColor)
    {
        if (!pInstance)
            return;

        CGraphicImage* pImage = pInstance->GetGraphicImagePointer();
        if (!pImage)
            return;

        AddImageInstance(pInstance, x, y, static_cast<float>(pImage->GetWidth()), static_cast<float>(pImage->GetHeight()),
                         overrideColor != 0 ? overrideColor : static_cast<uint32_t>(pInstance->GetDiffuseColor()));
    }

    void DynamicQuadBatcher::AddImageInstance(CGraphicImageInstance* pInstance, float x, float y,
                                              float width, float height, uint32_t diffuseColor)
    {
        if (!pInstance)
            return;

        CGraphicImage* pImage = pInstance->GetGraphicImagePointer();
        if (!pImage)
            return;

        CGraphicTexture* pTexture = pImage->GetTexturePointer();
        if (!pTexture)
            return;

        int iWidth = pTexture->GetWidth();
        int iHeight = pTexture->GetHeight();
        if (iWidth <= 0 || iHeight <= 0)
            return;

        LPDIRECT3DTEXTURE9 pD3DTexture = pTexture->GetD3DTexture();
        uintptr_t dwTextureID = reinterpret_cast<uintptr_t>(pD3DTexture);

        const RECT& rRect = pImage->GetRectReference();
        const float texReverseWidth = 1.0f / static_cast<float>(iWidth);
        const float texReverseHeight = 1.0f / static_cast<float>(iHeight);

        const float su = rRect.left * texReverseWidth;
        const float sv = rRect.top * texReverseHeight;
        const float eu = (rRect.left + (rRect.right - rRect.left)) * texReverseWidth;
        const float ev = (rRect.top + (rRect.bottom - rRect.top)) * texReverseHeight;

        AddQuad(x - 0.5f, y - 0.5f, width, height, diffuseColor, dwTextureID, su, sv, eu, ev, 0);
    }

    void DynamicQuadBatcher::Flush()
    {
        FlushToQueue(CRenderQueueManager::Instance());
    }

    void DynamicQuadBatcher::FlushToQueue(CRenderQueueManager& queue)
    {
        for (size_t i = 0; i < m_activeBatchCount; ++i)
        {
            const auto& batch = m_batchPool[i];
            if (batch.vertexCount == 0)
                continue;

            TUIBatchPacket packet{};
            packet.dwTextureID = batch.dwTextureID;
            packet.dwBlendMode = batch.dwBlendMode;
            packet.dwVertexCount = batch.vertexCount;
            packet.bUseScissor = batch.bUseScissor;
            packet.scissorRect = batch.scissorRect;
            // Wskaznik na bezpieczny bufor klatki m_frameVertexBuffer (nie jest czyszczony przez Clear!)
            packet.pVertices = &m_frameVertexBuffer[batch.startIndex];

            queue.SubmitUI(packet);
        }

        Clear();
    }

    void DynamicQuadBatcher::Clear() noexcept
    {
        // Resetujemy wylacznie licznik aktywnych partii i aktywnych wierzcholkow oczekujacych na Flush.
        // Wierzcholki w m_frameVertexBuffer pozostaja nienaruszone az do BeginFrame()!
        m_activeBatchCount = 0;
        m_activeVertexCount = 0;
    }
}
