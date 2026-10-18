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
		// 1. Sprawdz wiersz polecen (GetCommandLineA)
		const char* cmdLine = ::GetCommandLineA();
		if (cmdLine)
		{
			std::string cl(cmdLine);
			for (char& c : cl)
				c = static_cast<char>(::tolower(static_cast<unsigned char>(c)));

			if (cl.find("--d3d9") != std::string::npos || cl.find("-d3d9") != std::string::npos ||
				cl.find("--legacy") != std::string::npos || cl.find("-legacy") != std::string::npos)
			{
				return false;
			}
			if (cl.find("--wgpu") != std::string::npos || cl.find("-wgpu") != std::string::npos)
			{
				return true;
			}
		}

		// 2. Sprawdz zmienna srodowiskowa M2_GRAPHICS_ENGINE
		if (const char* envVal = std::getenv("M2_GRAPHICS_ENGINE"))
		{
			std::string envStr(envVal);
			for (char& c : envStr)
				c = static_cast<char>(::tolower(static_cast<unsigned char>(c)));

			if (envStr == "legacy" || envStr == "d3d9")
				return false;
			if (envStr == "wgpu")
				return true;
		}

		// 3. Sprawdz plik konfiguracyjny config.json
		std::ifstream file("config.json");
		if (file.is_open())
		{
			std::string content((std::istreambuf_iterator<char>(file)), std::istreambuf_iterator<char>());
			const auto keyPos = content.find("\"graphics_engine\"");
			if (keyPos != std::string::npos)
			{
				std::string sub = content.substr(keyPos, 64);
				for (char& c : sub)
					c = static_cast<char>(::tolower(static_cast<unsigned char>(c)));

				if (sub.find("\"legacy\"") != std::string::npos || sub.find("\"d3d9\"") != std::string::npos)
					return false;
				if (sub.find("\"wgpu\"") != std::string::npos)
					return true;
			}
		}

		// 4. DOMYSLNIE zwracamy true (WGPU backend jest domyslnym rendererem dla klienta)
		return true;
	}
}

bool RenderOwnership::LegacyDraws(ERenderCategory cat)
{
	if (!IsWgpuConfigured())
		return true;
	return Owner(cat) == ERenderOwner::Legacy;
}

ERenderOwner RenderOwnership::Owner(ERenderCategory cat)
{
	if (!IsWgpuConfigured())
		return ERenderOwner::Legacy;

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
	static std::atomic<int> s_wgpuExplicitState{ -1 }; // -1: niezainicjalizowany, 0: jawne wylaczenie (false), 1: jawne wlaczenie (true)
}

bool RenderOwnership::IsWgpuConfigured()
{
	const int explicitVal = s_wgpuExplicitState.load(std::memory_order_relaxed);
	if (explicitVal == 1)
		return true;
	if (explicitVal == 0)
		return false;

	static const bool s_bWgpuConfigured = ReadWgpuConfigOnce();
	return s_bWgpuConfigured;
}

void RenderOwnership::SetWgpuConfigured(bool configured)
{
	s_wgpuExplicitState.store(configured ? 1 : 0, std::memory_order_relaxed);
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
