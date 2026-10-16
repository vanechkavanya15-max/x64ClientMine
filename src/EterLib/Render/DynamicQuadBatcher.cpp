#include "DynamicQuadBatcher.h"
#include "../RenderQueueManager.h"
#include "../GrpImageInstance.h"
#include "../GrpImage.h"
#include "../GrpTexture.h"
#include <cmath>
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
        m_vertexBuffer.reserve(vertexCapacity);
        m_batchPool.resize(batchCapacity);
        m_activeBatchCount = 0;
        memset(&m_currentScissorRect, 0, sizeof(RECT));
    }

    void DynamicQuadBatcher::Begin()
    {
        Clear();
    }

    void DynamicQuadBatcher::SetScissorRect(const RECT& rect, bool bEnable) noexcept
    {
        m_currentScissorRect = rect;
        m_bScissorEnable = bEnable;
    }

    void DynamicQuadBatcher::ResetScissorRect() noexcept
    {
        m_bScissorEnable = false;
        memset(&m_currentScissorRect, 0, sizeof(RECT));
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
        return a.left == b.left && a.top == b.top &&
               a.right == b.right && a.bottom == b.bottom;
    }

    void DynamicQuadBatcher::PrepareBatch(uint32_t dwTextureID, uint32_t dwBlendMode)
    {
        const uint32_t scissorFlag = m_bScissorEnable ? 1u : 0u;

        if (m_activeBatchCount > 0)
        {
            auto& currentBatch = m_batchPool[m_activeBatchCount - 1];
            if (currentBatch.dwTextureID == dwTextureID &&
                currentBatch.dwBlendMode == dwBlendMode &&
                currentBatch.bUseScissor == scissorFlag &&
                (!m_bScissorEnable || AreRectsEqual(currentBatch.scissorRect, m_currentScissorRect)))
            {
                return;
            }
        }

        if (m_activeBatchCount >= m_batchPool.size())
        {
            m_batchPool.emplace_back();
        }

        auto& newBatch = m_batchPool[m_activeBatchCount++];
        newBatch.dwTextureID = dwTextureID;
        newBatch.dwBlendMode = dwBlendMode;
        newBatch.bUseScissor = scissorFlag;
        newBatch.scissorRect = m_currentScissorRect;
        newBatch.startIndex = static_cast<uint32_t>(m_vertexBuffer.size());
        newBatch.vertexCount = 0;
    }

    void DynamicQuadBatcher::AddQuad(float x, float y, float w, float h,
                                     uint32_t color, uint32_t dwTextureID,
                                     float u0, float v0, float u1, float v1,
                                     uint32_t dwBlendMode)
    {
        if (w <= 0.0f || h <= 0.0f)
            return;

        PrepareBatch(dwTextureID, dwBlendMode);

        const D3DXCOLOR dxColor(color);

        TUIVertex vert0{ D3DXVECTOR3(x, y, 0.0f), dxColor, D3DXVECTOR2(u0, v0) };
        TUIVertex vert1{ D3DXVECTOR3(x + w, y, 0.0f), dxColor, D3DXVECTOR2(u1, v0) };
        TUIVertex vert2{ D3DXVECTOR3(x, y + h, 0.0f), dxColor, D3DXVECTOR2(u0, v1) };
        TUIVertex vert3{ D3DXVECTOR3(x + w, y + h, 0.0f), dxColor, D3DXVECTOR2(u1, v1) };

        // Trojkat 1: vert0, vert1, vert2
        m_vertexBuffer.push_back(vert0);
        m_vertexBuffer.push_back(vert1);
        m_vertexBuffer.push_back(vert2);

        // Trojkat 2: vert1, vert3, vert2
        m_vertexBuffer.push_back(vert1);
        m_vertexBuffer.push_back(vert3);
        m_vertexBuffer.push_back(vert2);

        m_batchPool[m_activeBatchCount - 1].vertexCount += 6;
    }

    void DynamicQuadBatcher::AddQuadVertices(const TUIVertex vertices[4],
                                             uint32_t dwTextureID,
                                             uint32_t dwBlendMode)
    {
        PrepareBatch(dwTextureID, dwBlendMode);

        m_vertexBuffer.push_back(vertices[0]);
        m_vertexBuffer.push_back(vertices[1]);
        m_vertexBuffer.push_back(vertices[2]);

        m_vertexBuffer.push_back(vertices[1]);
        m_vertexBuffer.push_back(vertices[3]);
        m_vertexBuffer.push_back(vertices[2]);

        m_batchPool[m_activeBatchCount - 1].vertexCount += 6;
    }

    void DynamicQuadBatcher::AddBar(float x, float y, float w, float h, uint32_t color)
    {
        AddQuad(x, y, w, h, color, 0, 0.0f, 0.0f, 0.0f, 0.0f, 0);
    }

    void DynamicQuadBatcher::AddBox(float x, float y, float w, float h, uint32_t color)
    {
        if (w <= 0.0f || h <= 0.0f)
            return;

        AddBar(x, y, w, 1.0f, color);
        AddBar(x, y + h - 1.0f, w, 1.0f, color);
        if (h > 2.0f)
        {
            AddBar(x, y + 1.0f, 1.0f, h - 2.0f, color);
            AddBar(x + w - 1.0f, y + 1.0f, 1.0f, h - 2.0f, color);
        }
    }

    void DynamicQuadBatcher::AddBar2d(float sx, float sy, float ex, float ey, uint32_t color)
    {
        const float left = (sx < ex) ? sx : ex;
        const float right = (sx < ex) ? ex : sx;
        const float top = (sy < ey) ? sy : ey;
        const float bottom = (sy < ey) ? ey : sy;
        AddQuad(left, top, right - left, bottom - top, color, 0, 0.0f, 0.0f, 0.0f, 0.0f, 0);
    }

    void DynamicQuadBatcher::AddBox2d(float sx, float sy, float ex, float ey, uint32_t color, float thickness)
    {
        const float left = (sx < ex) ? sx : ex;
        const float right = (sx < ex) ? ex : sx;
        const float top = (sy < ey) ? sy : ey;
        const float bottom = (sy < ey) ? ey : sy;
        const float width = right - left;
        const float height = bottom - top;

        AddQuad(left, top, width, thickness, color, 0);
        AddQuad(left, bottom - thickness, width, thickness, color, 0);
        AddQuad(left, top + thickness, thickness, height - (thickness * 2.0f), color, 0);
        AddQuad(right - thickness, top + thickness, thickness, height - (thickness * 2.0f), color, 0);
    }

    void DynamicQuadBatcher::AddLine2d(float sx, float sy, float ex, float ey, uint32_t color, float thickness)
    {
        if (sy == ey)
        {
            const float left = (sx < ex) ? sx : ex;
            const float right = (sx < ex) ? ex : sx;
            AddQuad(left, sy - thickness * 0.5f, right - left, thickness, color, 0);
        }
        else if (sx == ex)
        {
            const float top = (sy < ey) ? sy : ey;
            const float bottom = (sy < ey) ? ey : sy;
            AddQuad(sx - thickness * 0.5f, top, thickness, bottom - top, color, 0);
        }
        else
        {
            const float dx = ex - sx;
            const float dy = ey - sy;
            const float len = sqrtf(dx * dx + dy * dy);
            if (len > 0.0001f)
            {
                const float nx = -dy / len * (thickness * 0.5f);
                const float ny = dx / len * (thickness * 0.5f);
                const D3DXCOLOR dxColor(color);

                TUIVertex v[4];
                v[0] = { D3DXVECTOR3(sx + nx, sy + ny, 0.0f), dxColor, D3DXVECTOR2(0, 0) };
                v[1] = { D3DXVECTOR3(ex + nx, ey + ny, 0.0f), dxColor, D3DXVECTOR2(0, 0) };
                v[2] = { D3DXVECTOR3(sx - nx, sy - ny, 0.0f), dxColor, D3DXVECTOR2(0, 0) };
                v[3] = { D3DXVECTOR3(ex - nx, ey - ny, 0.0f), dxColor, D3DXVECTOR2(0, 0) };
                AddQuadVertices(v, 0, 0);
            }
        }
    }

    void DynamicQuadBatcher::AddImageInstance(CGraphicImageInstance* pInstance, float x, float y, uint32_t overrideColor)
    {
        if (!pInstance || pInstance->IsEmpty())
            return;

        CGraphicImage* pImage = pInstance->GetGraphicImagePointer();
        if (!pImage)
            return;

        const float fimgWidth = static_cast<float>(pImage->GetWidth());
        const float fimgHeight = static_cast<float>(pImage->GetHeight());
        const uint32_t color = (overrideColor != 0) ? overrideColor : pInstance->GetDiffuseColorDWORD();

        AddImageInstance(pInstance, x, y, fimgWidth, fimgHeight, color);
    }

    void DynamicQuadBatcher::AddImageInstance(CGraphicImageInstance* pInstance, float x, float y,
                                              float width, float height, uint32_t diffuseColor)
    {
        if (!pInstance || pInstance->IsEmpty())
            return;

        CGraphicImage* pImage = pInstance->GetGraphicImagePointer();
        if (!pImage)
            return;

        CGraphicTexture* pTexture = pImage->GetTexturePointer();
        if (!pTexture)
            return;

        LPDIRECT3DTEXTURE9 pD3DTexture = pTexture->GetD3DTexture();
        const uint32_t dwTextureID = static_cast<uint32_t>(reinterpret_cast<uintptr_t>(pD3DTexture));

        const RECT& rRect = pImage->GetRectReference();
        const float texReverseWidth = 1.0f / float(pTexture->GetWidth());
        const float texReverseHeight = 1.0f / float(pTexture->GetHeight());

        const float su = rRect.left * texReverseWidth;
        const float sv = rRect.top * texReverseHeight;
        const float eu = (rRect.left + (rRect.right - rRect.left)) * texReverseWidth;
        const float ev = (rRect.top + (rRect.bottom - rRect.top)) * texReverseHeight;

        // Alignment -0.5f dla D3D9 texeli
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

            TUIBatchPacket packet;
            packet.dwTextureID = batch.dwTextureID;
            packet.dwBlendMode = batch.dwBlendMode;
            packet.dwVertexCount = batch.vertexCount;
            packet.bUseScissor = batch.bUseScissor;
            packet.scissorRect = batch.scissorRect;
            packet.pVertices = &m_vertexBuffer[batch.startIndex];
            packet.fPad[0] = 0.0f;
            packet.fPad[1] = 0.0f;

            queue.SubmitUI(packet);
        }

        Clear();
    }

    void DynamicQuadBatcher::Clear() noexcept
    {
        m_vertexBuffer.clear();
        m_activeBatchCount = 0;
    }
}
