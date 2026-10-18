#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "EterLib/RenderOwnership.h"
#include "EterLib/RenderBypassGuard.h"
#include "EterLib/StateManager.h"

TEST_CASE("RenderBypassGuard - Initial State and Reset")
{
	CRenderBypassGuard guard;
	guard.Reset();

	CHECK(guard.IsEnabled());
	CHECK(guard.GetTotalBypassCount() == 0);

	const ERenderCategory categories[] = {
		ERenderCategory::SkyBox,
		ERenderCategory::Terrain,
		ERenderCategory::Water,
		ERenderCategory::Dungeon,
		ERenderCategory::SpeedTree,
		ERenderCategory::Model,
		ERenderCategory::WeaponTrace,
		ERenderCategory::Effect,
		ERenderCategory::EffectMesh,
		ERenderCategory::UI,
		ERenderCategory::Text,
		ERenderCategory::TextTail,
		ERenderCategory::MiniMap,
		ERenderCategory::Shadow,
		ERenderCategory::Debug,
		ERenderCategory::ImGui,
		ERenderCategory::Movie,
		ERenderCategory::Cursor
	};

	for (auto cat : categories)
	{
		CHECK(guard.GetBypassCount(cat) == 0);
	}

	guard.ReportBypasses();
	CHECK(guard.GetLastReport().find("No bypasses detected") != std::string::npos);
}

TEST_CASE("RenderBypassGuard - Direct Bypass Counting")
{
	CRenderBypassGuard guard;
	guard.Reset();

	guard.RecordBypass(ERenderCategory::Effect, "Site_EffectParticle");
	CHECK(guard.GetBypassCount(ERenderCategory::Effect) == 1);
	CHECK(guard.GetTotalBypassCount() == 1);

	guard.RecordBypass(ERenderCategory::Effect, "Site_EffectParticle2");
	CHECK(guard.GetBypassCount(ERenderCategory::Effect) == 2);
	CHECK(guard.GetTotalBypassCount() == 2);

	guard.RecordBypass(ERenderCategory::UI, "Site_UIImageDirect");
	CHECK(guard.GetBypassCount(ERenderCategory::UI) == 1);
	CHECK(guard.GetTotalBypassCount() == 3);

	// Out of bounds safety
	guard.RecordBypass(ERenderCategory::Count, "InvalidSite");
	CHECK(guard.GetBypassCount(ERenderCategory::Count) == 0);
	CHECK(guard.GetTotalBypassCount() == 3);

	auto sites = guard.GetRecordedSites();
	CHECK(sites.size() == 3);
	CHECK(sites[0].category == ERenderCategory::Effect);
	CHECK(sites[0].siteName == "Site_EffectParticle");
	CHECK(sites[1].category == ERenderCategory::Effect);
	CHECK(sites[1].siteName == "Site_EffectParticle2");
	CHECK(sites[2].category == ERenderCategory::UI);
	CHECK(sites[2].siteName == "Site_UIImageDirect");

	guard.Reset();
	CHECK(guard.GetTotalBypassCount() == 0);
	CHECK(guard.GetRecordedSites().empty());
}

TEST_CASE("RenderBypassGuard - ValidateDraw with RenderOwnership")
{
	RenderOwnership::ResetDefaults();

	CRenderBypassGuard guard;
	guard.Reset();

	// Test trybu Legacy
	RenderOwnership::SetOwner(ERenderCategory::SkyBox, ERenderOwner::Legacy);
	RenderOwnership::SetOwner(ERenderCategory::Terrain, ERenderOwner::Legacy);

	CHECK(guard.IsDrawAllowed(ERenderCategory::SkyBox));
	CHECK(guard.ValidateDraw(ERenderCategory::SkyBox, "LegacySkyBoxDraw"));
	CHECK(guard.GetBypassCount(ERenderCategory::SkyBox) == 0);

	// Effect pod zarzadem Wgpu -> bezposredni direct draw jest nielegalnym bypassem
	CHECK_FALSE(guard.IsDrawAllowed(ERenderCategory::Effect));
	CHECK_FALSE(guard.ValidateDraw(ERenderCategory::Effect, "IllegalDirectEffectDraw"));
	CHECK(guard.GetBypassCount(ERenderCategory::Effect) == 1);

	// UI pod zarzadem Wgpu -> bezposredni draw jest nielegalny
	CHECK_FALSE(guard.IsDrawAllowed(ERenderCategory::UI));
	CHECK_FALSE(guard.ValidateDrawCall(ERenderCategory::UI, "IllegalDirectUIDraw"));
	CHECK(guard.GetBypassCount(ERenderCategory::UI) == 1);

	// Przelaczenie Terrain z Legacy na Wgpu -> draw staje sie nielegalny
	CHECK(guard.IsDrawAllowed(ERenderCategory::Terrain));
	RenderOwnership::SetOwner(ERenderCategory::Terrain, ERenderOwner::Wgpu);
	CHECK_FALSE(guard.IsDrawAllowed(ERenderCategory::Terrain));
	CHECK_FALSE(guard.CheckDrawAllowed(ERenderCategory::Terrain, "IllegalDirectTerrainDraw"));
	CHECK(guard.GetBypassCount(ERenderCategory::Terrain) == 1);

	// Przywrocenie Terrain do Legacy -> draw staje sie legalny
	RenderOwnership::SetOwner(ERenderCategory::Terrain, ERenderOwner::Legacy);
	CHECK(guard.IsDrawAllowed(ERenderCategory::Terrain));
	CHECK(guard.ValidateDraw(ERenderCategory::Terrain, "LegalDirectTerrainDraw"));
	CHECK(guard.GetBypassCount(ERenderCategory::Terrain) == 1); // licznik nie wzrosl

	RenderOwnership::ResetDefaults();
}

TEST_CASE("RenderBypassGuard - Enable Disable Flag")
{
	RenderOwnership::ResetDefaults();

	CRenderBypassGuard guard;
	guard.Reset();

	guard.SetEnabled(false);
	CHECK_FALSE(guard.IsEnabled());

	// Przy wylaczonym guardzie, nawet jesli kategoria nie jest Legacy, walidacja zwraca true i nie zlicza ominiec
	CHECK(guard.ValidateDraw(ERenderCategory::Effect, "DisabledGuardDraw"));
	CHECK(guard.GetBypassCount(ERenderCategory::Effect) == 0);

	guard.RecordBypass(ERenderCategory::Effect, "DisabledRecord");
	CHECK(guard.GetBypassCount(ERenderCategory::Effect) == 0);

	// Ponowne wlaczenie
	guard.SetEnabled(true);
	CHECK(guard.IsEnabled());

	CHECK_FALSE(guard.ValidateDraw(ERenderCategory::Effect, "EnabledGuardDraw"));
	CHECK(guard.GetBypassCount(ERenderCategory::Effect) == 1);
}

TEST_CASE("RenderBypassGuard - Report Formatting and Content")
{
	RenderOwnership::ResetDefaults();

	CRenderBypassGuard guard;
	guard.Reset();

	guard.ValidateDraw(ERenderCategory::Effect, "Site_EffectA");
	guard.ValidateDraw(ERenderCategory::Effect, "Site_EffectB");
	guard.ValidateDraw(ERenderCategory::UI, "Site_UI_Font");

	guard.ReportBypasses();
	const std::string& report = guard.GetLastReport();

	CHECK(report.find("=== RenderBypassGuard Report ===") != std::string::npos);
	CHECK(report.find("Category Effect") != std::string::npos);
	CHECK(report.find("Bypasses: 2") != std::string::npos);
	CHECK(report.find("Owner: Wgpu") != std::string::npos);
	CHECK(report.find("Category UI") != std::string::npos);
	CHECK(report.find("Bypasses: 1") != std::string::npos);
	CHECK(report.find("Total illegal bypasses detected: 3") != std::string::npos);

	guard.Reset();
	CHECK(guard.GetLastReport().empty());
}

TEST_CASE("RenderBypassGuard - CategoryToString Conversion")
{
	CHECK(std::string(CRenderBypassGuard::CategoryToString(ERenderCategory::SkyBox)) == "SkyBox");
	CHECK(std::string(CRenderBypassGuard::CategoryToString(ERenderCategory::Terrain)) == "Terrain");
	CHECK(std::string(CRenderBypassGuard::CategoryToString(ERenderCategory::Water)) == "Water");
	CHECK(std::string(CRenderBypassGuard::CategoryToString(ERenderCategory::Dungeon)) == "Dungeon");
	CHECK(std::string(CRenderBypassGuard::CategoryToString(ERenderCategory::SpeedTree)) == "SpeedTree");
	CHECK(std::string(CRenderBypassGuard::CategoryToString(ERenderCategory::Model)) == "Model");
	CHECK(std::string(CRenderBypassGuard::CategoryToString(ERenderCategory::WeaponTrace)) == "WeaponTrace");
	CHECK(std::string(CRenderBypassGuard::CategoryToString(ERenderCategory::Effect)) == "Effect");
	CHECK(std::string(CRenderBypassGuard::CategoryToString(ERenderCategory::EffectMesh)) == "EffectMesh");
	CHECK(std::string(CRenderBypassGuard::CategoryToString(ERenderCategory::UI)) == "UI");
	CHECK(std::string(CRenderBypassGuard::CategoryToString(ERenderCategory::Text)) == "Text");
	CHECK(std::string(CRenderBypassGuard::CategoryToString(ERenderCategory::TextTail)) == "TextTail");
	CHECK(std::string(CRenderBypassGuard::CategoryToString(ERenderCategory::MiniMap)) == "MiniMap");
	CHECK(std::string(CRenderBypassGuard::CategoryToString(ERenderCategory::Shadow)) == "Shadow");
	CHECK(std::string(CRenderBypassGuard::CategoryToString(ERenderCategory::Debug)) == "Debug");
	CHECK(std::string(CRenderBypassGuard::CategoryToString(ERenderCategory::ImGui)) == "ImGui");
	CHECK(std::string(CRenderBypassGuard::CategoryToString(ERenderCategory::Movie)) == "Movie");
	CHECK(std::string(CRenderBypassGuard::CategoryToString(ERenderCategory::Cursor)) == "Cursor");
	CHECK(std::string(CRenderBypassGuard::CategoryToString(ERenderCategory::Count)) == "Unknown");
}

TEST_CASE("RenderBypassGuard - StateManager Integration and RAII Scope")
{
	RenderOwnership::ResetDefaults();

	CRenderBypassGuard guard;
	guard.Reset();

	if (CStateManager::InstancePtr())
	{
		STATEMANAGER.SetCurrentRenderCategory(ERenderCategory::Count);
		CHECK(STATEMANAGER.GetCurrentRenderCategory() == ERenderCategory::Count);

		// RAII Scope test
		{
			CRenderCategoryScope scope(ERenderCategory::UI);
			CHECK(STATEMANAGER.GetCurrentRenderCategory() == ERenderCategory::UI);

			// Symulacja bezposredniego draw call w kategorii UI
			STATEMANAGER.RecordDrawCall("TestUIDrawSite");
			CHECK(guard.GetBypassCount(ERenderCategory::UI) == 1);
		}

		// Po opuszczeniu zakresu kategoria powinna wrocic do Count
		CHECK(STATEMANAGER.GetCurrentRenderCategory() == ERenderCategory::Count);

		// RecordDrawCall z kategoria Count nie zwieksza licznikow
		STATEMANAGER.RecordDrawCall("CountDrawSite");
		CHECK(guard.GetTotalBypassCount() == 1);
	}
	else
	{
		// Fallback jesli CStateManager nie byl zainicjalizowany z LPDIRECT3DDEVICE9EX
		CRenderBypassGuard::CheckBypass(ERenderCategory::UI, "StaticFallback");
		CHECK(guard.GetBypassCount(ERenderCategory::UI) == 1);
	}
}
