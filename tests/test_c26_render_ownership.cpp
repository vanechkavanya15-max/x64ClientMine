#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "EterLib/RenderOwnership.h"

TEST_CASE("RenderOwnership - Default Ownership")
{
	RenderOwnership::ResetDefaults();

	// Effect i UI -> D3D9Backend
	CHECK(RenderOwnership::Owner(ERenderCategory::Effect) == ERenderOwner::D3D9Backend);
	CHECK(RenderOwnership::Owner(ERenderCategory::UI) == ERenderOwner::D3D9Backend);
	CHECK_FALSE(RenderOwnership::LegacyDraws(ERenderCategory::Effect));
	CHECK_FALSE(RenderOwnership::LegacyDraws(ERenderCategory::UI));

	// Wszystkie pozostale kategorie -> Legacy
	const ERenderCategory legacyCategories[] = {
		ERenderCategory::SkyBox,
		ERenderCategory::Terrain,
		ERenderCategory::Water,
		ERenderCategory::Dungeon,
		ERenderCategory::SpeedTree,
		ERenderCategory::Model,
		ERenderCategory::WeaponTrace,
		ERenderCategory::EffectMesh,
		ERenderCategory::Text,
		ERenderCategory::TextTail,
		ERenderCategory::MiniMap,
		ERenderCategory::Shadow,
		ERenderCategory::Debug,
		ERenderCategory::ImGui,
		ERenderCategory::Movie,
		ERenderCategory::Cursor
	};

	for (auto cat : legacyCategories)
	{
		CHECK(RenderOwnership::Owner(cat) == ERenderOwner::Legacy);
		CHECK(RenderOwnership::LegacyDraws(cat) == true);
	}
}

TEST_CASE("RenderOwnership - Change Ownership via SetOwner")
{
	RenderOwnership::ResetDefaults();

	// Zmiana kategorii Terrain na D3D9Backend
	RenderOwnership::SetOwner(ERenderCategory::Terrain, ERenderOwner::D3D9Backend);
	CHECK(RenderOwnership::Owner(ERenderCategory::Terrain) == ERenderOwner::D3D9Backend);
	CHECK_FALSE(RenderOwnership::LegacyDraws(ERenderCategory::Terrain));

	// Zmiana kategorii Terrain na Wgpu
	RenderOwnership::SetOwner(ERenderCategory::Terrain, ERenderOwner::Wgpu);
	CHECK(RenderOwnership::Owner(ERenderCategory::Terrain) == ERenderOwner::Wgpu);
	CHECK_FALSE(RenderOwnership::LegacyDraws(ERenderCategory::Terrain));

	// Przywrocenie kategorii Terrain do Legacy
	RenderOwnership::SetOwner(ERenderCategory::Terrain, ERenderOwner::Legacy);
	CHECK(RenderOwnership::Owner(ERenderCategory::Terrain) == ERenderOwner::Legacy);
	CHECK(RenderOwnership::LegacyDraws(ERenderCategory::Terrain) == true);

	// Zmiana kategorii Effect na Legacy
	RenderOwnership::SetOwner(ERenderCategory::Effect, ERenderOwner::Legacy);
	CHECK(RenderOwnership::Owner(ERenderCategory::Effect) == ERenderOwner::Legacy);
	CHECK(RenderOwnership::LegacyDraws(ERenderCategory::Effect) == true);

	// Zmiana kategorii UI na Wgpu
	RenderOwnership::SetOwner(ERenderCategory::UI, ERenderOwner::Wgpu);
	CHECK(RenderOwnership::Owner(ERenderCategory::UI) == ERenderOwner::Wgpu);
	CHECK_FALSE(RenderOwnership::LegacyDraws(ERenderCategory::UI));

	// Reset na koniec
	RenderOwnership::ResetDefaults();
}

TEST_CASE("RenderOwnership - LegacyDraws contract")
{
	RenderOwnership::ResetDefaults();

	for (size_t i = 0; i < static_cast<size_t>(ERenderCategory::Count); ++i)
	{
		auto cat = static_cast<ERenderCategory>(i);

		RenderOwnership::SetOwner(cat, ERenderOwner::Legacy);
		CHECK(RenderOwnership::LegacyDraws(cat) == true);

		RenderOwnership::SetOwner(cat, ERenderOwner::D3D9Backend);
		CHECK(RenderOwnership::LegacyDraws(cat) == false);

		RenderOwnership::SetOwner(cat, ERenderOwner::Wgpu);
		CHECK(RenderOwnership::LegacyDraws(cat) == false);
	}

	RenderOwnership::ResetDefaults();
}

TEST_CASE("RenderOwnership - Out of bounds safety")
{
	CHECK(RenderOwnership::Owner(ERenderCategory::Count) == ERenderOwner::Legacy);
	CHECK(RenderOwnership::LegacyDraws(ERenderCategory::Count) == true);
}

TEST_CASE("RenderOwnership - SetAllWgpu Wave W5 Transition (T228)")
{
	RenderOwnership::ResetDefaults();

	// T228: Przelaczenie wszystkich 18 kategorii na Wgpu
	RenderOwnership::SetAllWgpu();

	for (size_t i = 0; i < static_cast<size_t>(ERenderCategory::Count); ++i)
	{
		auto cat = static_cast<ERenderCategory>(i);
		CHECK(RenderOwnership::Owner(cat) == ERenderOwner::Wgpu);
		CHECK_FALSE(RenderOwnership::LegacyDraws(cat));
	}

	// Przywrocenie domyslnego stanu
	RenderOwnership::ResetDefaults();
}
