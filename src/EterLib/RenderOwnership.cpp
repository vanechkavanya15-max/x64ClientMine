#include "StdAfx.h"
#include "RenderOwnership.h"
#include <atomic>
#include <fstream>
#include <string>
#include <cstdlib>

namespace
{
	static std::atomic<ERenderOwner> s_owners[static_cast<size_t>(ERenderCategory::Count)] = {
		ERenderOwner::Legacy,      // SkyBox
		ERenderOwner::Legacy,      // Terrain
		ERenderOwner::Legacy,      // Water
		ERenderOwner::Legacy,      // Dungeon
		ERenderOwner::Legacy,      // SpeedTree
		ERenderOwner::Legacy,      // Model
		ERenderOwner::Legacy,      // WeaponTrace
		ERenderOwner::D3D9Backend, // Effect
		ERenderOwner::Legacy,      // EffectMesh
		ERenderOwner::D3D9Backend, // UI
		ERenderOwner::Legacy,      // Text
		ERenderOwner::Legacy,      // TextTail
		ERenderOwner::Legacy,      // MiniMap
		ERenderOwner::Legacy,      // Shadow
		ERenderOwner::Legacy,      // Debug
		ERenderOwner::Legacy,      // ImGui
		ERenderOwner::Legacy,      // Movie
		ERenderOwner::Legacy       // Cursor
	};

	bool ReadWgpuConfigOnce()
	{
		if (const char* envVal = std::getenv("M2_GRAPHICS_ENGINE"))
		{
			if (std::string(envVal) == "wgpu")
				return true;
		}

		std::ifstream file("config.json");
		if (!file.is_open())
			return false;

		std::string content((std::istreambuf_iterator<char>(file)), std::istreambuf_iterator<char>());
		const auto keyPos = content.find("\"graphics_engine\"");
		if (keyPos == std::string::npos)
			return false;

		const auto wgpuPos = content.find("\"wgpu\"", keyPos);
		return (wgpuPos != std::string::npos) && ((wgpuPos - keyPos) < 64);
	}
}

bool RenderOwnership::LegacyDraws(ERenderCategory cat)
{
	return Owner(cat) == ERenderOwner::Legacy;
}

ERenderOwner RenderOwnership::Owner(ERenderCategory cat)
{
	const auto idx = static_cast<size_t>(cat);
	if (idx < static_cast<size_t>(ERenderCategory::Count))
	{
		return s_owners[idx].load(std::memory_order_relaxed);
	}
	return ERenderOwner::Legacy;
}

void RenderOwnership::SetOwner(ERenderCategory cat, ERenderOwner owner)
{
	const auto idx = static_cast<size_t>(cat);
	if (idx < static_cast<size_t>(ERenderCategory::Count))
	{
		s_owners[idx].store(owner, std::memory_order_relaxed);
	}
}

bool RenderOwnership::IsWgpuConfigured()
{
	static const bool s_bWgpuConfigured = ReadWgpuConfigOnce();
	return s_bWgpuConfigured;
}

void RenderOwnership::ResetDefaults()
{
	for (size_t i = 0; i < static_cast<size_t>(ERenderCategory::Count); ++i)
	{
		s_owners[i].store(ERenderOwner::Legacy, std::memory_order_relaxed);
	}
	s_owners[static_cast<size_t>(ERenderCategory::Effect)].store(ERenderOwner::D3D9Backend, std::memory_order_relaxed);
	s_owners[static_cast<size_t>(ERenderCategory::UI)].store(ERenderOwner::D3D9Backend, std::memory_order_relaxed);
}
