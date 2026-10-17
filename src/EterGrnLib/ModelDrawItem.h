#pragma once

#include <d3d9.h>
#include <d3dx9.h>
#include <cstdint>
#include "EterLib/RenderPacketTypes.h"

#ifndef _EMODELPASS_DEFINED_
#define _EMODELPASS_DEFINED_
enum class EModelPass : uint32_t
{
	Opaque = 0,
	Blend = 1,
	ShadowDepth = 2
};
#endif

class CGrannyLODController;

/**
 * @brief Centralna funkcja wykonania pojedynczego wywolania rysowania submesha modelu Direct3D 9.
 * Zapewnia pelna izolacje stanow D3D9 (SaveRenderState / RestoreRenderState) i deterministyczny pass filtering.
 */
void ExecuteModelDrawItemD3D9(const TModelDrawItem& item);
