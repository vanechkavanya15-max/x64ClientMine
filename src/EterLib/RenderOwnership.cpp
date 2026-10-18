#include "StdAfx.h"
#include "RenderOwnership.h"
#include <atomic>
#include <fstream>
#include <string>
#include <cstdlib>

namespace
{
	static std::atomic<ERenderOwner> s_owners[static_cast<size_t>(ERenderCategory::Count)] = {
		ERenderOwner::Wgpu, // SkyBox
		ERenderOwner::Wgpu, // Terrain
		ERenderOwner::Wgpu, // Water
		ERenderOwner::Wgpu, // Dungeon
		ERenderOwner::Wgpu, // SpeedTree
		ERenderOwner::Wgpu, // Model
		ERenderOwner::Wgpu, // WeaponTrace
		ERenderOwner::Wgpu, // Effect
		ERenderOwner::Wgpu, // EffectMesh
		ERenderOwner::Wgpu, // UI
		ERenderOwner::Wgpu, // Text
		ERenderOwner::Wgpu, // TextTail
		ERenderOwner::Wgpu, // MiniMap
		ERenderOwner::Wgpu, // Shadow
		ERenderOwner::Wgpu, // Debug
		ERenderOwner::Wgpu, // ImGui
		ERenderOwner::Wgpu, // Movie
		ERenderOwner::Wgpu  // Cursor
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

namespace
{
	static std::atomic<bool> s_bWgpuConfiguredExplicit{ false };
}

bool RenderOwnership::IsWgpuConfigured()
{
	if (s_bWgpuConfiguredExplicit.load(std::memory_order_relaxed))
		return true;
	static const bool s_bWgpuConfigured = ReadWgpuConfigOnce();
	return s_bWgpuConfigured;
}

void RenderOwnership::SetWgpuConfigured(bool configured)
{
	s_bWgpuConfiguredExplicit.store(configured, std::memory_order_relaxed);
}

void RenderOwnership::SetAllWgpu()
{
	for (size_t i = 0; i < static_cast<size_t>(ERenderCategory::Count); ++i)
	{
		s_owners[i].store(ERenderOwner::Wgpu, std::memory_order_relaxed);
	}
}

void RenderOwnership::ResetDefaults()
{
	for (size_t i = 0; i < static_cast<size_t>(ERenderCategory::Count); ++i)
	{
		s_owners[i].store(ERenderOwner::Wgpu, std::memory_order_relaxed);
	}
}
