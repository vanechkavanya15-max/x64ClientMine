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
     * wedlug identyfikatora tekstury oraz stanu Scissor Rect.
     * Wykorzystuje Monotonic Frame Ring-Buffer eliminujacy blad Use-After-Clear.
     */
    class DynamicQuadBatcher
    {
    public:
        static constexpr size_t DEFAULT_VERTEX_CAPACITY = 262144; // 262144 wierzcholkow = 65536 quadow
        static constexpr size_t DEFAULT_BATCH_CAPACITY = 1024;
        static constexpr size_t MAX_QUADS_PER_BATCH = 16380;
        static constexpr size_t MAX_VERTICES_PER_BATCH = MAX_QUADS_PER_BATCH * 4; // 65520 wierzcholkow

        static constexpr uint32_t D3D_LOCK_DISCARD = 0x2000;      // D3DLOCK_DISCARD
        static constexpr uint32_t D3D_LOCK_NOOVERWRITE = 0x1000;   // D3DLOCK_NOOVERWRITE

        static DynamicQuadBatcher& Instance();

        DynamicQuadBatcher(size_t vertexCapacity = DEFAULT_VERTEX_CAPACITY,
                           size_t batchCapacity = DEFAULT_BATCH_CAPACITY);
        ~DynamicQuadBatcher() = default;

        DynamicQuadBatcher(const DynamicQuadBatcher&) = delete;
        DynamicQuadBatcher& operator=(const DynamicQuadBatcher&) = delete;
        DynamicQuadBatcher(DynamicQuadBatcher&&) = delete;
        DynamicQuadBatcher& operator=(DynamicQuadBatcher&&) = delete;

        // Cykl zycia klatki
        void BeginFrame() noexcept;
        void Begin() noexcept { BeginFrame(); }
        void Flush();
        void FlushToQueue();
        void FlushToQueue(CRenderQueueManager& queue);
        void FlushPendingUI();
        void FlushPendingUI(CRenderQueueManager& queue);
        void SubmitToQueue() { Flush(); }
        void Clear() noexcept;
        bool IsBatching() const noexcept { return true; }

        // Rezerwacja bufora poza cyklem rysowania klatki
        void Reserve(size_t vertexCapacity, size_t batchCapacity = 0);

        // Konfiguracja stanu Scissor Rect
        void SetScissorRect(const RECT& rect, bool bEnable) noexcept;
        void ResetScissorRect() noexcept;
        void GetScissorRect(RECT* pOutRect, bool* pbOutEnable) const noexcept;

        // Biala tekstura (White Pixel Atlas)
        void SetWhiteTexture(uintptr_t dwTextureID, float u = 0.0f, float v = 0.0f) noexcept;

        // Dodawanie geometrii quadow
        void AddQuad(float x, float y, float w, float h,
                     uint32_t color, uintptr_t dwTextureID = 0,
                     float u0 = 0.0f, float v0 = 0.0f,
                     float u1 = 1.0f, float v1 = 1.0f,
                     uint32_t dwBlendMode = 0);

        void AddQuadVertices(const TUIVertex vertices[4],
                             uintptr_t dwTextureID = 0,
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
        [[nodiscard]] size_t GetVertexCount() const noexcept { return m_activeVertexCount; }
        [[nodiscard]] size_t GetTotalVertexCount() const noexcept { return m_frameVertexOffset; }
        [[nodiscard]] size_t GetBatchCount() const noexcept { return m_activeBatchCount; }
        [[nodiscard]] size_t GetVertexCapacity() const noexcept { return m_frameVertexBuffer.size(); }
        [[nodiscard]] size_t GetBatchCapacity() const noexcept { return m_batchPool.size(); }
        [[nodiscard]] size_t GetOverflowQuads() const noexcept { return m_overflowQuads; }
        [[nodiscard]] size_t GetOverflowVertices() const noexcept { return m_overflowVertices; }
        [[nodiscard]] size_t GetOverflowCount() const noexcept { return m_overflowQuads; }
        [[nodiscard]] const TUIVertex* GetVertexBufferData() const noexcept { return m_frameVertexBuffer.data(); }

        // Ring-Buffer D3D lock helpers
        [[nodiscard]] static uint32_t GetLockFlagsForStartIndex(uint32_t startIndex) noexcept
        {
            return (startIndex == 0) ? D3D_LOCK_DISCARD : D3D_LOCK_NOOVERWRITE;
        }

        [[nodiscard]] uint32_t GetCurrentLockFlags() const noexcept
        {
            return (m_frameVertexOffset == 0) ? D3D_LOCK_DISCARD : D3D_LOCK_NOOVERWRITE;
        }

        [[nodiscard]] uint32_t GetBatchLockFlags(size_t index) const noexcept;

    private:
        struct BatchEntry
        {
            uintptr_t dwTextureID = 0;
            uint32_t dwBlendMode = 0;
            uint32_t bUseScissor = 0;
            RECT scissorRect{ 0, 0, 0, 0 };
            uint32_t startIndex = 0;
            uint32_t vertexCount = 0;
            uint32_t lockFlags = 0;
        };

        void PrepareBatch(uintptr_t dwTextureID, uint32_t dwBlendMode);
        static bool AreRectsEqual(const RECT& a, const RECT& b) noexcept;

        // Monotoniczny bufor klatki (Frame Ring-Buffer) - zero alokacji i zwalniania w trakcie klatki
        std::vector<TUIVertex> m_frameVertexBuffer;
        size_t m_frameVertexOffset{ 0 };
        size_t m_activeVertexCount{ 0 };

        std::vector<BatchEntry> m_batchPool;
        size_t m_activeBatchCount{ 0 };

        size_t m_overflowQuads{ 0 };
        size_t m_overflowVertices{ 0 };

        RECT m_currentScissorRect{ 0, 0, 0, 0 };
        bool m_bScissorEnable{ false };

        uintptr_t m_dwWhiteTextureID{ 0 };
        float m_fWhitePixelU{ 0.0f };
        float m_fWhitePixelV{ 0.0f };
    };
}

using CDynamicQuadBatcher = EterLib::Render::DynamicQuadBatcher;
using DynamicQuadBatcher = EterLib::Render::DynamicQuadBatcher;
