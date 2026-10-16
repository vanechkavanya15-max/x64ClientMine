#include "MiniMapCoordinateProjector.h"

namespace MiniMap
{
    MiniMapCoordinateProjector::MiniMapCoordinateProjector()
    {
        RecalculateMetrics();
    }

    void MiniMapCoordinateProjector::SetDimensions(float fWidth, float fHeight)
    {
        m_fWidth = fWidth;
        m_fHeight = fHeight;
        RecalculateMetrics();
    }

    void MiniMapCoordinateProjector::SetScale(float fScale)
    {
        m_fScale = ClampScale(fScale);
        RecalculateMetrics();
    }

    void MiniMapCoordinateProjector::ScaleUp()
    {
        m_fScale = StepScaleUp(m_fScale);
        RecalculateMetrics();
    }

    void MiniMapCoordinateProjector::ScaleDown()
    {
        m_fScale = StepScaleDown(m_fScale);
        RecalculateMetrics();
    }

    void MiniMapCoordinateProjector::SetCenterPosition(float fCenterX, float fCenterY)
    {
        m_fCenterX = fCenterX;
        m_fCenterY = fCenterY;
        RecalculateMetrics();
    }

    void MiniMapCoordinateProjector::SetScreenPosition(float fScreenX, float fScreenY)
    {
        m_fScreenX = fScreenX;
        m_fScreenY = fScreenY;
    }

    void MiniMapCoordinateProjector::SetTerrainCoordinate(int16_t sCoordX, int16_t sCoordY)
    {
        m_sTerrainCoordX = sCoordX;
        m_sTerrainCoordY = sCoordY;
        RecalculateMetrics();
    }

    void MiniMapCoordinateProjector::SetCellScale(float fCellScale)
    {
        if (fCellScale > 0.0f)
        {
            m_fCellScale = fCellScale;
            RecalculateMetrics();
        }
    }

    void MiniMapCoordinateProjector::RecalculateMetrics()
    {
        Vector2 cellPos = ComputeCenterCell(m_fCenterX, m_fCenterY, m_sTerrainCoordX, m_sTerrainCoordY, DEFAULT_TERRAIN_XSIZE, DEFAULT_TERRAIN_YSIZE, m_fCellScale);
        m_fCenterCellX = cellPos.x;
        m_fCenterCellY = cellPos.y;
        m_fRadius = ComputeMiniMapRadius(m_fWidth, m_fHeight, m_fScale, m_fCellScale, DEFAULT_BORDER_MARGIN);
    }

    bool MiniMapCoordinateProjector::Project(float fWorldX, float fWorldY, float fMarkW, float fMarkH, float* pfOutX, float* pfOutY, float* pfOutDist) const
    {
        return ProjectWorldToMiniMap(
            fWorldX, fWorldY,
            m_fCenterX, m_fCenterY,
            m_fScale,
            m_fWidth, m_fHeight,
            m_fRadius,
            m_fScreenX, m_fScreenY,
            fMarkW, fMarkH,
            pfOutX, pfOutY,
            pfOutDist,
            m_fCellScale);
    }

    Vector2 MiniMapCoordinateProjector::ProjectTarget(float fTargetX, float fTargetY, float fMarkW, float fMarkH) const
    {
        return ProjectTargetToEdge(
            fTargetX, fTargetY,
            m_fCenterX, m_fCenterY,
            m_fScale,
            m_fWidth, m_fHeight,
            m_fScreenX, m_fScreenY,
            fMarkW, fMarkH,
            DEFAULT_BORDER_MARGIN,
            m_fCellScale);
    }

    bool MiniMapCoordinateProjector::ScreenToWorld(float fClickX, float fClickY, float* pfOutWorldX, float* pfOutWorldY) const
    {
        return ScreenToWorldPosition(
            fClickX, fClickY,
            m_fScreenX, m_fScreenY,
            m_fWidth, m_fHeight,
            m_fRadius,
            m_fCenterX, m_fCenterY,
            m_fScale,
            pfOutWorldX, pfOutWorldY,
            m_fCellScale);
    }

    TerrainMeshTransform MiniMapCoordinateProjector::GetMeshTransform() const
    {
        return ComputeTerrainMeshTransform(m_fWidth, m_fHeight, m_fScale, m_fCenterCellX, m_fCenterCellY, m_fScreenX, m_fScreenY);
    }

    // ====================================================================
    // Statyczne implementacje
    // ====================================================================

    Vector2 MiniMapCoordinateProjector::ComputeCenterCell(
        float fCenterX, float fCenterY,
        int16_t sTerrainCoordX, int16_t sTerrainCoordY,
        float fTerrainXSize,
        float fTerrainYSize,
        float fCellScale)
    {
        if (fCellScale <= 0.0f)
            return Vector2(0.0f, 0.0f);

        float cellX = (fCenterX - (static_cast<float>(sTerrainCoordX) * fTerrainXSize)) / fCellScale;
        float cellY = (fCenterY - (static_cast<float>(sTerrainCoordY) * fTerrainYSize)) / fCellScale;
        return Vector2(cellX, cellY);
    }

    float MiniMapCoordinateProjector::ComputeMiniMapRadius(
        float fWidth, float fHeight,
        float fScale,
        float fCellScale,
        float fBorderMargin)
    {
        float fWindowRadius = (std::min(fWidth, fHeight) * 0.5f) - fBorderMargin;
        if (fWindowRadius < 0.0f)
            fWindowRadius = 0.0f;

        float fMaxRadiusFromCells = (DEFAULT_RADAR_MAX_CELL_RANGE / fCellScale) * fScale;
        return std::min(fMaxRadiusFromCells, fWindowRadius);
    }

    float MiniMapCoordinateProjector::NormalizeRotation(float fActorRotationDeg)
    {
        float fRotation = 540.0f - fActorRotationDeg;
        while (fRotation > 360.0f)
            fRotation -= 360.0f;
        while (fRotation < 0.0f)
            fRotation += 360.0f;
        return fRotation;
    }

    float MiniMapCoordinateProjector::ClampScale(float fScale, float fMin, float fMax)
    {
        return std::clamp(fScale, fMin, fMax);
    }

    float MiniMapCoordinateProjector::StepScaleUp(float fCurrentScale, float fMax)
    {
        float fScale = fCurrentScale * 2.0f;
        if (fScale >= fMax)
            fScale = fMax;
        return fScale;
    }

    float MiniMapCoordinateProjector::StepScaleDown(float fCurrentScale, float fMin)
    {
        float fScale = fCurrentScale * 0.5f;
        if (fScale <= fMin)
            fScale = fMin;
        return fScale;
    }

    bool MiniMapCoordinateProjector::ProjectWorldToMiniMap(
        float fWorldX, float fWorldY,
        float fCenterX, float fCenterY,
        float fScale,
        float fWidth, float fHeight,
        float fMiniMapRadius,
        float fScreenX, float fScreenY,
        float fMarkWidth, float fMarkHeight,
        float* pfOutScreenX, float* pfOutScreenY,
        float* pfOutDistance,
        float fCellScale)
    {
        if (fCellScale <= 0.0f)
            return false;

        float fooCellScale = 1.0f / fCellScale;
        float fDistanceFromCenterX = (fWorldX - fCenterX) * fooCellScale * fScale;
        float fDistanceFromCenterY = (fWorldY - fCenterY) * fooCellScale * fScale;

        if (std::fabs(fDistanceFromCenterX) >= fMiniMapRadius || std::fabs(fDistanceFromCenterY) >= fMiniMapRadius)
            return false;

        float fDistanceFromCenter = std::sqrt(fDistanceFromCenterX * fDistanceFromCenterX + fDistanceFromCenterY * fDistanceFromCenterY);
        if (fDistanceFromCenter >= fMiniMapRadius)
            return false;

        if (pfOutScreenX)
            *pfOutScreenX = (fWidth - fMarkWidth) * 0.5f + fDistanceFromCenterX + fScreenX;
        if (pfOutScreenY)
            *pfOutScreenY = (fHeight - fMarkHeight) * 0.5f + fDistanceFromCenterY + fScreenY;
        if (pfOutDistance)
            *pfOutDistance = fDistanceFromCenter;

        return true;
    }

    ProjectedPoint MiniMapCoordinateProjector::ProjectWorld(
        float fWorldX, float fWorldY,
        float fCenterX, float fCenterY,
        float fScale,
        float fWidth, float fHeight,
        float fMiniMapRadius,
        float fScreenX, float fScreenY,
        float fMarkWidth, float fMarkHeight,
        float fCellScale)
    {
        ProjectedPoint pt;
        if (fCellScale <= 0.0f)
            return pt;

        float fooCellScale = 1.0f / fCellScale;
        pt.distanceFromCenterX = (fWorldX - fCenterX) * fooCellScale * fScale;
        pt.distanceFromCenterY = (fWorldY - fCenterY) * fooCellScale * fScale;
        pt.distance = std::sqrt(pt.distanceFromCenterX * pt.distanceFromCenterX + pt.distanceFromCenterY * pt.distanceFromCenterY);

        pt.localX = (fWidth - fMarkWidth) * 0.5f + pt.distanceFromCenterX;
        pt.localY = (fHeight - fMarkHeight) * 0.5f + pt.distanceFromCenterY;
        pt.screenX = pt.localX + fScreenX;
        pt.screenY = pt.localY + fScreenY;

        bool insideAxis = (std::fabs(pt.distanceFromCenterX) < fMiniMapRadius) && (std::fabs(pt.distanceFromCenterY) < fMiniMapRadius);
        pt.isInsideRadius = insideAxis && (pt.distance < fMiniMapRadius);

        return pt;
    }

    Vector2 MiniMapCoordinateProjector::ProjectTargetToEdge(
        float fTargetWorldX, float fTargetWorldY,
        float fCenterX, float fCenterY,
        float fScale,
        float fWidth, float fHeight,
        float fScreenX, float fScreenY,
        float fMarkWidth, float fMarkHeight,
        float fBorderMargin,
        float fCellScale)
    {
        const float c_fMiniMapWindowRadius = (fWidth < fHeight ? fWidth : fHeight) * 0.5f - fBorderMargin;
        float fooCellScale = 1.0f / fCellScale;
        float fDistanceFromCenterX = (fTargetWorldX - fCenterX) * fooCellScale * fScale;
        float fDistanceFromCenterY = (fTargetWorldY - fCenterY) * fooCellScale * fScale;
        float fDistanceFromCenter = std::sqrt(fDistanceFromCenterX * fDistanceFromCenterX + fDistanceFromCenterY * fDistanceFromCenterY);

        if (fDistanceFromCenter >= c_fMiniMapWindowRadius && fDistanceFromCenter > 0.0001f)
        {
            float fRadian = std::atan2(fDistanceFromCenterY, fDistanceFromCenterX);
            fDistanceFromCenterX = c_fMiniMapWindowRadius * std::cos(fRadian);
            fDistanceFromCenterY = c_fMiniMapWindowRadius * std::sin(fRadian);
        }

        float screenX = (fWidth - fMarkWidth) * 0.5f + fDistanceFromCenterX + fScreenX;
        float screenY = (fHeight - fMarkHeight) * 0.5f + fDistanceFromCenterY + fScreenY;
        return Vector2(screenX, screenY);
    }

    bool MiniMapCoordinateProjector::ScreenToWorldPosition(
        float fClickScreenX, float fClickScreenY,
        float fMiniMapScreenX, float fMiniMapScreenY,
        float fWidth, float fHeight,
        float fMiniMapRadius,
        float fCenterX, float fCenterY,
        float fScale,
        float* pfOutWorldX, float* pfOutWorldY,
        float fCellScale)
    {
        if (fScale <= 0.0f || fCellScale <= 0.0f)
            return false;

        float fDistanceFromMiniMapCenterX = fClickScreenX - fMiniMapScreenX - fWidth * 0.5f;
        float fDistanceFromMiniMapCenterY = fClickScreenY - fMiniMapScreenY - fHeight * 0.5f;

        float fDistSq = fDistanceFromMiniMapCenterX * fDistanceFromMiniMapCenterX + fDistanceFromMiniMapCenterY * fDistanceFromMiniMapCenterY;
        if (fDistSq > fMiniMapRadius * fMiniMapRadius)
            return false;

        if (pfOutWorldX)
            *pfOutWorldX = fCenterX + (fDistanceFromMiniMapCenterX / fScale) * fCellScale;
        if (pfOutWorldY)
            *pfOutWorldY = fCenterY + (fDistanceFromMiniMapCenterY / fScale) * fCellScale;

        return true;
    }

    Vector2 MiniMapCoordinateProjector::InterpolateObserver(
        float fSrcX, float fSrcY,
        float fDstX, float fDstY,
        uint32_t dwSrcTime, uint32_t dwDstTime,
        uint32_t dwCurTime)
    {
        if (dwDstTime <= dwSrcTime)
            return Vector2(fDstX, fDstY);

        float fPos = static_cast<float>(dwCurTime - dwSrcTime) / static_cast<float>(dwDstTime - dwSrcTime);
        fPos = std::clamp(fPos, 0.0f, 1.0f);

        float curX = (fDstX - fSrcX) * fPos + fSrcX;
        float curY = (fDstY - fSrcY) * fPos + fSrcY;
        return Vector2(curX, curY);
    }

    AtlasOffsets MiniMapCoordinateProjector::ComputeAtlasCenteringOffsets(
        float fAtlasImageSizeX, float fAtlasImageSizeY,
        float fAtlasMaxX, float fAtlasMaxY)
    {
        AtlasOffsets result;
        if (fAtlasMaxX <= 0.0f || fAtlasMaxY <= 0.0f)
            return result;

        float fScaleX = fAtlasImageSizeX / fAtlasMaxX;
        float fScaleY = fAtlasImageSizeY / fAtlasMaxY;
        result.uniformScale = std::min(fScaleX, fScaleY);

        float fScaledMapWidth = fAtlasMaxX * result.uniformScale;
        float fScaledMapHeight = fAtlasMaxY * result.uniformScale;

        result.offsetX = (fAtlasImageSizeX - fScaledMapWidth) * 0.5f;
        result.offsetY = (fAtlasImageSizeY - fScaledMapHeight) * 0.5f;
        return result;
    }

    Vector2 MiniMapCoordinateProjector::GlobalToAtlasPosition(
        float fWorldX, float fWorldY,
        float fUniformScale,
        float fAtlasOffsetX, float fAtlasOffsetY)
    {
        return Vector2(
            fWorldX * fUniformScale + fAtlasOffsetX,
            fWorldY * fUniformScale + fAtlasOffsetY);
    }

    Vector2 MiniMapCoordinateProjector::AtlasToGlobalPosition(
        float fAtlasX, float fAtlasY,
        float fUniformScale,
        float fAtlasOffsetX, float fAtlasOffsetY)
    {
        if (fUniformScale <= 0.0f)
            return Vector2(0.0f, 0.0f);

        float fReverseScale = 1.0f / fUniformScale;
        return Vector2(
            (fAtlasX - fAtlasOffsetX) * fReverseScale,
            (fAtlasY - fAtlasOffsetY) * fReverseScale);
    }

    TerrainMeshTransform MiniMapCoordinateProjector::ComputeTerrainMeshTransform(
        float fWidth, float fHeight,
        float fScale,
        float fCenterCellX, float fCenterCellY,
        float fScreenX, float fScreenY)
    {
        TerrainMeshTransform out;
        out.scaleX = fWidth * fScale;
        out.scaleY = fHeight * fScale;
        out.translationX = (1.0f + fScale) * fWidth * 0.5f - fCenterCellX * fScale + fScreenX;
        out.translationY = (1.0f + fScale) * fHeight * 0.5f - fCenterCellY * fScale + fScreenY;
        return out;
    }
}
