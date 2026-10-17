#pragma once

#include <cstdint>

enum class ERenderCategory
{
	SkyBox,
	Terrain,
	Water,
	Dungeon,
	SpeedTree,
	Model,
	WeaponTrace,
	Effect,
	EffectMesh,
	UI,
	Text,
	TextTail,
	MiniMap,
	Shadow,
	Debug,
	ImGui,
	Movie,
	Cursor,
	Count
};

enum class ERenderOwner
{
	Legacy,
	D3D9Backend,
	Wgpu
};

class RenderOwnership
{
public:
	using ERenderCategory = ::ERenderCategory;
	using ERenderOwner = ::ERenderOwner;

	static bool LegacyDraws(ERenderCategory cat);
	static ERenderOwner Owner(ERenderCategory cat);
	static void SetOwner(ERenderCategory cat, ERenderOwner owner);
	static bool IsWgpuConfigured();

	static void ResetDefaults();
};
