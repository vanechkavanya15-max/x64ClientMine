#pragma once

#include "../RenderPacketTypes.h"
#include <vector>
#include <cstdint>
#include <windows.h>

class CRenderQueueManager;
class CGraphicImage;
class CGraphicImageInstance;

namespace EterLib::Render
{
    /**
     * @brief Dynamiczny batcher quadow dla systemu 2D UI w Metin2.
     * Zapewnia zero-alokacji na klatke, automatyczne grupowanie i rozbijanie partii
     * wedlug identyfikatora tekstury (tekstury .sub / atlasy) oraz stanu Scissor Rect.
     */
    class DynamicQuadBatcher
    {
    public:
        static constexpr size_t DEFAULT_VERTEX_CAPACITY = 65536;
        static constexpr size_t DEFAULT_BATCH_CAPACITY = 512;

        static DynamicQuadBatcher& Instance();

        DynamicQuadBatcher(size_t vertexCapacity = DEFAULT_VERTEX_CAPACITY,
                           size_t batchCapacity = DEFAULT_BATCH_CAPACITY);
        ~DynamicQuadBatcher() = default;

        DynamicQuadBatcher(const DynamicQuadBatcher&) = delete;
        DynamicQuadBatcher& operator=(const DynamicQuadBatcher&) = delete;
        DynamicQuadBatcher(DynamicQuadBatcher&&) = delete;
        DynamicQuadBatcher& operator=(DynamicQuadBatcher&&) = delete;

        // Cykl zycia klatki
        void Begin();
        void Flush();
        void FlushToQueue(CRenderQueueManager& queue);
        void Clear() noexcept;
        bool IsBatching() const noexcept { return true; }

        // Konfiguracja stanu Scissor Rect
        void SetScissorRect(const RECT& rect, bool bEnable) noexcept;
        void ResetScissorRect() noexcept;
        void GetScissorRect(RECT* pOutRect, bool* pbOutEnable) const noexcept;

        // Dodawanie geometrii quadow
        void AddQuad(float x, float y, float w, float h,
                     uint32_t color, uint32_t dwTextureID = 0,
                     float u0 = 0.0f, float v0 = 0.0f,
                     float u1 = 1.0f, float v1 = 1.0f,
                     uint32_t dwBlendMode = 0);

        void AddQuadVertices(const TUIVertex vertices[4],
                             uint32_t dwTextureID = 0,
                             uint32_t dwBlendMode = 0);

        // Pomocnicze metody UI (parametry x, y, w, h)
        void AddBar(float x, float y, float w, float h, uint32_t color);
        void AddBox(float x, float y, float w, float h, uint32_t color);

        // Pomocnicze metody UI (parametry sx, sy, ex, ey)
        void AddBar2d(float sx, float sy, float ex, float ey, uint32_t color);
        void AddBox2d(float sx, float sy, float ex, float ey, uint32_t color, float thickness = 1.0f);
        void AddLine2d(float sx, float sy, float ex, float ey, uint32_t color, float thickness = 1.0f);

        // Wsparcie dla atlasow i tekstur .sub
        void AddImageInstance(CGraphicImageInstance* pInstance, float x, float y, uint32_t overrideColor = 0);
        void AddImageInstance(CGraphicImageInstance* pInstance, float x, float y,
                              float width, float height, uint32_t diffuseColor = 0xFFFFFFFF);

        // Metryki i diagnostyka
        [[nodiscard]] size_t GetVertexCount() const noexcept { return m_vertexBuffer.size(); }
        [[nodiscard]] size_t GetBatchCount() const noexcept { return m_activeBatchCount; }
        [[nodiscard]] size_t GetVertexCapacity() const noexcept { return m_vertexBuffer.capacity(); }

    private:
        struct BatchEntry
        {
            uint32_t dwTextureID = 0;
            uint32_t dwBlendMode = 0;
            uint32_t bUseScissor = 0;
            RECT scissorRect{ 0, 0, 0, 0 };
            uint32_t startIndex = 0;
            uint32_t vertexCount = 0;
        };

        void PrepareBatch(uint32_t dwTextureID, uint32_t dwBlendMode);
        static bool AreRectsEqual(const RECT& a, const RECT& b) noexcept;

        std::vector<TUIVertex> m_vertexBuffer;
        std::vector<BatchEntry> m_batchPool;
        size_t m_activeBatchCount = 0;

        RECT m_currentScissorRect{ 0, 0, 0, 0 };
        bool m_bScissorEnable = false;
    };
}

using CDynamicQuadBatcher = EterLib::Render::DynamicQuadBatcher;
using DynamicQuadBatcher = EterLib::Render::DynamicQuadBatcher;
