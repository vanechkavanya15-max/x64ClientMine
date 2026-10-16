#pragma once

#include <cmath>
#include <cstdint>
#include <algorithm>
#include <optional>

namespace MiniMap
{
    /**
     * @brief Stale konfiguracyjne dla siatki terenu i radaru minimapy.
     */
    inline constexpr float DEFAULT_CELL_SCALE = 200.0f;
    inline constexpr float DEFAULT_TERRAIN_XSIZE = 25600.0f;
    inline constexpr float DEFAULT_TERRAIN_YSIZE = 25600.0f;
    inline constexpr float DEFAULT_BORDER_MARGIN = 9.0f;
    inline constexpr float DEFAULT_RADAR_MAX_CELL_RANGE = 6400.0f;
    inline constexpr float MIN_SCALE = 0.5f;
    inline constexpr float MAX_SCALE = 4.0f;
    inline constexpr float DEFAULT_SCALE = 2.0f;

    /**
     * @brief Czysta 2D struktura wektora bez zaleznosci od Direct3D.
     */
    struct Vector2
    {
        float x{0.0f};
        float y{0.0f};

        constexpr Vector2() = default;
        constexpr Vector2(float inX, float inY) : x(inX), y(inY) {}

        constexpr bool operator==(const Vector2& other) const
        {
            return (x == other.x) && (y == other.y);
        }
    };

    /**
     * @brief Pelny wynik rzutowania pozycji swiata na ekran minimapy.
     */
    struct ProjectedPoint
    {
        float screenX{0.0f};
        float screenY{0.0f};
        float localX{0.0f};
        float localY{0.0f};
        float distanceFromCenterX{0.0f};
        float distanceFromCenterY{0.0f};
        float distance{0.0f};
        bool isInsideRadius{false};
    };

    /**
     * @brief Parametry transformacji siatki terenu (zastepuje macierz D3DXMATRIX).
     */
    struct TerrainMeshTransform
    {
        float scaleX{0.0f};
        float scaleY{0.0f};
        float translationX{0.0f};
        float translationY{0.0f};
    };

    /**
     * @brief Przesuniecia i wspolczynnik skali dla mapy Atlasu.
     */
    struct AtlasOffsets
    {
        float offsetX{0.0f};
        float offsetY{0.0f};
        float uniformScale{1.0f};
    };

    /**
     * @brief Kalkulator matematyki rzutowania wspolrzednych minimapy i radaru.
     * Czysta klasa obliczeniowa calkowicie pozbawiona zaleznosci od Direct3D9.
     */
    class MiniMapCoordinateProjector
    {
    public:
        MiniMapCoordinateProjector();
        ~MiniMapCoordinateProjector() = default;

        // ====================================================================
        // Statyczne metody czystej matematyki rzutowania
        // ====================================================================

        /**
         * @brief Oblicza m_fCenterCellX i m_fCenterCellY na podstawie wspolrzednych swiata i sektora terenu.
         */
        static Vector2 ComputeCenterCell(
            float fCenterX, float fCenterY,
            int16_t sTerrainCoordX, int16_t sTerrainCoordY,
            float fTerrainXSize = DEFAULT_TERRAIN_XSIZE,
            float fTerrainYSize = DEFAULT_TERRAIN_YSIZE,
            float fCellScale = DEFAULT_CELL_SCALE);

        /**
         * @brief Oblicza promien radaru w pikselach (z uwzglednieniem klamrowania do okna).
         */
        static float ComputeMiniMapRadius(
            float fWidth, float fHeight,
            float fScale,
            float fCellScale = DEFAULT_CELL_SCALE,
            float fBorderMargin = DEFAULT_BORDER_MARGIN);

        /**
         * @brief Normalizuje rotacje aktora do zakresu [0.0f, 360.0f) wedlug formuly 540.0f - actorRot.
         */
        static float NormalizeRotation(float fActorRotationDeg);

        /**
         * @brief Ogranicza skale do dozwolonego zakresu [minScale, maxScale].
         */
        static float ClampScale(float fScale, float fMin = MIN_SCALE, float fMax = MAX_SCALE);

        /**
         * @brief Skaluje minimape o stopien w gore (* 2.0f).
         */
        static float StepScaleUp(float fCurrentScale, float fMax = MAX_SCALE);

        /**
         * @brief Skaluje minimape o stopien w dol (* 0.5f).
         */
        static float StepScaleDown(float fCurrentScale, float fMin = MIN_SCALE);

        /**
         * @brief Rzutuje wspolrzedne swiata (worldX, worldY) na wspolrzedne ekranowe znacznika minimapy.
         * Zwraca true, jesli punkt miesci sie w zadanym promieniu radaru.
         */
        static bool ProjectWorldToMiniMap(
            float fWorldX, float fWorldY,
            float fCenterX, float fCenterY,
            float fScale,
            float fWidth, float fHeight,
            float fMiniMapRadius,
            float fScreenX, float fScreenY,
            float fMarkWidth, float fMarkHeight,
            float* pfOutScreenX, float* pfOutScreenY,
            float* pfOutDistance = nullptr,
            float fCellScale = DEFAULT_CELL_SCALE);

        /**
         * @brief Zwraca pelna strukture ProjectedPoint dla podanej pozycji swiata.
         */
        static ProjectedPoint ProjectWorld(
            float fWorldX, float fWorldY,
            float fCenterX, float fCenterY,
            float fScale,
            float fWidth, float fHeight,
            float fMiniMapRadius,
            float fScreenX, float fScreenY,
            float fMarkWidth, float fMarkHeight,
            float fCellScale = DEFAULT_CELL_SCALE);

        /**
         * @brief Rzutuje cel/znacznik na krawedz radaru (clamping do okregu), gdy cel znajduje sie poza radarem.
         */
        static Vector2 ProjectTargetToEdge(
            float fTargetWorldX, float fTargetWorldY,
            float fCenterX, float fCenterY,
            float fScale,
            float fWidth, float fHeight,
            float fScreenX, float fScreenY,
            float fMarkWidth, float fMarkHeight,
            float fBorderMargin = DEFAULT_BORDER_MARGIN,
            float fCellScale = DEFAULT_CELL_SCALE);

        /**
         * @brief Rzutowanie odwrotne: przelicza wspolrzedne klikniecia ekranu na pozycje w swiecie 3D.
         */
        static bool ScreenToWorldPosition(
            float fClickScreenX, float fClickScreenY,
            float fMiniMapScreenX, float fMiniMapScreenY,
            float fWidth, float fHeight,
            float fMiniMapRadius,
            float fCenterX, float fCenterY,
            float fScale,
            float* pfOutWorldX, float* pfOutWorldY,
            float fCellScale = DEFAULT_CELL_SCALE);

        /**
         * @brief Interpolacja liniowa pozycji obserwatora pomiedzy znacznikami czasowymi.
         */
        static Vector2 InterpolateObserver(
            float fSrcX, float fSrcY,
            float fDstX, float fDstY,
            uint32_t dwSrcTime, uint32_t dwDstTime,
            uint32_t dwCurTime);

        /**
         * @brief Oblicza jednolite skalowanie i przesuniecia centrowania dla mapy Atlasu.
         */
        static AtlasOffsets ComputeAtlasCenteringOffsets(
            float fAtlasImageSizeX, float fAtlasImageSizeY,
            float fAtlasMaxX, float fAtlasMaxY);

        /**
         * @brief Rzutuje wspolrzedne globalne na lokalne wspolrzedne obrazu Atlasu.
         */
        static Vector2 GlobalToAtlasPosition(
            float fWorldX, float fWorldY,
            float fUniformScale,
            float fAtlasOffsetX, float fAtlasOffsetY);

        /**
         * @brief Rzutowanie odwrotne: przelicza lokalne wspolrzedne Atlasu na wspolrzedne globalne.
         */
        static Vector2 AtlasToGlobalPosition(
            float fAtlasX, float fAtlasY,
            float fUniformScale,
            float fAtlasOffsetX, float fAtlasOffsetY);

        /**
         * @brief Wylicza parametry macierzy transformacji siatki minimapy bez Direct3D.
         */
        static TerrainMeshTransform ComputeTerrainMeshTransform(
            float fWidth, float fHeight,
            float fScale,
            float fCenterCellX, float fCenterCellY,
            float fScreenX, float fScreenY);

        // ====================================================================
        // Stanowy interfejs instancji (encapsulated projector)
        // ====================================================================

        void SetDimensions(float fWidth, float fHeight);
        void SetScale(float fScale);
        void ScaleUp();
        void ScaleDown();
        void SetCenterPosition(float fCenterX, float fCenterY);
        void SetScreenPosition(float fScreenX, float fScreenY);
        void SetTerrainCoordinate(int16_t sCoordX, int16_t sCoordY);
        void SetCellScale(float fCellScale);

        float GetWidth() const { return m_fWidth; }
        float GetHeight() const { return m_fHeight; }
        float GetScale() const { return m_fScale; }
        float GetCenterX() const { return m_fCenterX; }
        float GetCenterY() const { return m_fCenterY; }
        float GetCenterCellX() const { return m_fCenterCellX; }
        float GetCenterCellY() const { return m_fCenterCellY; }
        float GetScreenX() const { return m_fScreenX; }
        float GetScreenY() const { return m_fScreenY; }
        float GetRadius() const { return m_fRadius; }

        void RecalculateMetrics();

        bool Project(float fWorldX, float fWorldY, float fMarkW, float fMarkH, float* pfOutX, float* pfOutY, float* pfOutDist = nullptr) const;
        Vector2 ProjectTarget(float fTargetX, float fTargetY, float fMarkW, float fMarkH) const;
        bool ScreenToWorld(float fClickX, float fClickY, float* pfOutWorldX, float* pfOutWorldY) const;
        TerrainMeshTransform GetMeshTransform() const;

    private:
        float m_fWidth{128.0f};
        float m_fHeight{128.0f};
        float m_fScale{DEFAULT_SCALE};
        float m_fCenterX{64.0f};
        float m_fCenterY{64.0f};
        float m_fCenterCellX{0.0f};
        float m_fCenterCellY{0.0f};
        float m_fScreenX{0.0f};
        float m_fScreenY{0.0f};
        float m_fRadius{32.0f};
        float m_fCellScale{DEFAULT_CELL_SCALE};
        int16_t m_sTerrainCoordX{0};
        int16_t m_sTerrainCoordY{0};
    };
}
