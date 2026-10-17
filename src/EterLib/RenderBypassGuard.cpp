#include "StdAfx.h"
#include "RenderBypassGuard.h"
#include "StateManager.h"
#include <cstdio>

CRenderBypassGuard::CRenderBypassGuard()
	: m_bEnabled(true)
{
	Reset();
}

CRenderBypassGuard::~CRenderBypassGuard()
{
}

void CRenderBypassGuard::RecordBypass(RenderOwnership::ERenderCategory category, const char* siteName)
{
	if (!m_bEnabled.load(std::memory_order_relaxed))
		return;

	const auto idx = static_cast<size_t>(category);
	if (idx >= kCategoryCount)
		return;

	m_bypassCounts[idx].fetch_add(1, std::memory_order_relaxed);

	{
		std::lock_guard<std::mutex> lock(m_mutex);
		if (siteName && *siteName)
		{
			m_recordedSites.push_back({ category, std::string(siteName) });
		}
	}

#ifdef _DEBUG
	TraceError("[RenderBypassGuard] Direct draw bypass detected for category %s (%s)",
		CategoryToString(category), siteName ? siteName : "unknown");
#endif
}

uint32_t CRenderBypassGuard::GetBypassCount(RenderOwnership::ERenderCategory category) const
{
	const auto idx = static_cast<size_t>(category);
	if (idx >= kCategoryCount)
		return 0;

	return m_bypassCounts[idx].load(std::memory_order_relaxed);
}

uint32_t CRenderBypassGuard::GetTotalBypassCount() const
{
	uint32_t total = 0;
	for (size_t i = 0; i < kCategoryCount; ++i)
	{
		total += m_bypassCounts[i].load(std::memory_order_relaxed);
	}
	return total;
}

void CRenderBypassGuard::Reset()
{
	for (size_t i = 0; i < kCategoryCount; ++i)
	{
		m_bypassCounts[i].store(0, std::memory_order_relaxed);
	}

	std::lock_guard<std::mutex> lock(m_mutex);
	m_recordedSites.clear();
	m_lastReport.clear();
}

bool CRenderBypassGuard::IsDrawAllowed(RenderOwnership::ERenderCategory category) const
{
	const auto idx = static_cast<size_t>(category);
	if (idx >= kCategoryCount)
		return true;

	return RenderOwnership::LegacyDraws(category);
}

bool CRenderBypassGuard::ValidateDraw(RenderOwnership::ERenderCategory category, const char* siteName)
{
	if (!m_bEnabled.load(std::memory_order_relaxed))
		return true;

	if (IsDrawAllowed(category))
		return true;

	RecordBypass(category, siteName);
	return false;
}

bool CRenderBypassGuard::ValidateDrawCall(RenderOwnership::ERenderCategory category, const char* siteName)
{
	return ValidateDraw(category, siteName);
}

bool CRenderBypassGuard::CheckDrawAllowed(RenderOwnership::ERenderCategory category, const char* siteName)
{
	return ValidateDraw(category, siteName);
}

void CRenderBypassGuard::SetEnabled(bool bEnable)
{
	m_bEnabled.store(bEnable, std::memory_order_relaxed);
}

bool CRenderBypassGuard::IsEnabled() const
{
	return m_bEnabled.load(std::memory_order_relaxed);
}

void CRenderBypassGuard::ReportBypasses()
{
	std::lock_guard<std::mutex> lock(m_mutex);

	uint32_t total = 0;
	std::string report = "=== RenderBypassGuard Report ===\n";

	for (size_t i = 0; i < kCategoryCount; ++i)
	{
		uint32_t count = m_bypassCounts[i].load(std::memory_order_relaxed);
		if (count > 0)
		{
			total += count;
			auto cat = static_cast<RenderOwnership::ERenderCategory>(i);
			const char* ownerStr = "Unknown";
			switch (RenderOwnership::Owner(cat))
			{
			case ERenderOwner::Legacy: ownerStr = "Legacy"; break;
			case ERenderOwner::D3D9Backend: ownerStr = "D3D9Backend"; break;
			case ERenderOwner::Wgpu: ownerStr = "Wgpu"; break;
			}

			char line[256];
			snprintf(line, sizeof(line), "  [Category %-12s] Bypasses: %u (Owner: %s)\n",
				CategoryToString(cat), count, ownerStr);
			report += line;
		}
	}

	if (total == 0)
	{
		report += "  No bypasses detected. All draws respected render ownership.\n";
	}
	else
	{
		char summary[128];
		snprintf(summary, sizeof(summary), "Total illegal bypasses detected: %u\n", total);
		report += summary;
	}

	m_lastReport = report;

#ifdef _DEBUG
	TraceError("%s", m_lastReport.c_str());
#endif
}

const std::string& CRenderBypassGuard::GetLastReport() const
{
	std::lock_guard<std::mutex> lock(m_mutex);
	return m_lastReport;
}

std::vector<CRenderBypassGuard::SBypassSiteRecord> CRenderBypassGuard::GetRecordedSites() const
{
	std::lock_guard<std::mutex> lock(m_mutex);
	return m_recordedSites;
}

const char* CRenderBypassGuard::CategoryToString(RenderOwnership::ERenderCategory category)
{
	switch (category)
	{
	case RenderOwnership::ERenderCategory::SkyBox:      return "SkyBox";
	case RenderOwnership::ERenderCategory::Terrain:     return "Terrain";
	case RenderOwnership::ERenderCategory::Water:       return "Water";
	case RenderOwnership::ERenderCategory::Dungeon:     return "Dungeon";
	case RenderOwnership::ERenderCategory::SpeedTree:   return "SpeedTree";
	case RenderOwnership::ERenderCategory::Model:       return "Model";
	case RenderOwnership::ERenderCategory::WeaponTrace: return "WeaponTrace";
	case RenderOwnership::ERenderCategory::Effect:      return "Effect";
	case RenderOwnership::ERenderCategory::EffectMesh:  return "EffectMesh";
	case RenderOwnership::ERenderCategory::UI:          return "UI";
	case RenderOwnership::ERenderCategory::Text:        return "Text";
	case RenderOwnership::ERenderCategory::TextTail:    return "TextTail";
	case RenderOwnership::ERenderCategory::MiniMap:     return "MiniMap";
	case RenderOwnership::ERenderCategory::Shadow:      return "Shadow";
	case RenderOwnership::ERenderCategory::Debug:       return "Debug";
	case RenderOwnership::ERenderCategory::ImGui:       return "ImGui";
	case RenderOwnership::ERenderCategory::Movie:       return "Movie";
	case RenderOwnership::ERenderCategory::Cursor:      return "Cursor";
	default:                                            return "Unknown";
	}
}

bool CRenderBypassGuard::CheckBypass(RenderOwnership::ERenderCategory category, const char* siteName)
{
	CRenderBypassGuard* pGuard = InstancePtr();
	if (pGuard && pGuard->IsEnabled())
	{
		return pGuard->ValidateDraw(category, siteName);
	}
	return true;
}

// CRenderCategoryScope RAII
CRenderCategoryScope::CRenderCategoryScope(RenderOwnership::ERenderCategory category)
{
	if (CStateManager::InstancePtr())
	{
		m_prevCategory = CStateManager::Instance().GetCurrentRenderCategory();
		CStateManager::Instance().SetCurrentRenderCategory(category);
	}
	else
	{
		m_prevCategory = RenderOwnership::ERenderCategory::Count;
	}
}

CRenderCategoryScope::~CRenderCategoryScope()
{
	if (CStateManager::InstancePtr())
	{
		CStateManager::Instance().SetCurrentRenderCategory(m_prevCategory);
	}
}
