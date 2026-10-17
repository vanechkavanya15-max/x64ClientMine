#pragma once

#include "RenderOwnership.h"
#include "EterBase/Singleton.h"
#include <cstdint>
#include <atomic>
#include <string>
#include <vector>
#include <mutex>

class CRenderBypassGuard : public CSingleton<CRenderBypassGuard>
{
public:
	CRenderBypassGuard();
	virtual ~CRenderBypassGuard();

	// Glowna rejestracja ominiecia w danej kategorii
	void RecordBypass(RenderOwnership::ERenderCategory category, const char* siteName = nullptr);

	// Odczyt licznika ominiec dla danej kategorii
	uint32_t GetBypassCount(RenderOwnership::ERenderCategory category) const;

	// Sumaryczna liczba ominiec we wszystkich kategoriach
	uint32_t GetTotalBypassCount() const;

	// Reset licznikow i historii raportow
	void Reset();

	// Raportowanie wykrytych ominiec do logu silnika oraz bufora raportu
	void ReportBypasses();

	// Metoda pomocnicza sprawdzajaca czy bezposredni draw call jest dozwolony wzgledem wlasciciela.
	// Jesli kategoria ma wlasciciela innego niz Legacy (np. D3D9Backend lub Wgpu), bezposredni draw call
	// jest liczony jako nielegalny bypass i rejestrowany przez RecordBypass.
	bool IsDrawAllowed(RenderOwnership::ERenderCategory category) const;
	bool ValidateDraw(RenderOwnership::ERenderCategory category, const char* siteName = nullptr);
	bool ValidateDrawCall(RenderOwnership::ERenderCategory category, const char* siteName = nullptr);
	bool CheckDrawAllowed(RenderOwnership::ERenderCategory category, const char* siteName = nullptr);

	// Sterowanie aktywnoscia detektora
	void SetEnabled(bool bEnable);
	bool IsEnabled() const;

	// Odczyt ostatnio wygenerowanego raportu
	const std::string& GetLastReport() const;

	// Konwersja kategorii na czytelna nazwe tekstowa
	static const char* CategoryToString(RenderOwnership::ERenderCategory category);

	// Statyczna metoda pomocnicza bezpieczna dla braku aktywnego singletona
	static bool CheckBypass(RenderOwnership::ERenderCategory category, const char* siteName = nullptr);

	struct SBypassSiteRecord
	{
		RenderOwnership::ERenderCategory category;
		std::string siteName;
	};

	std::vector<SBypassSiteRecord> GetRecordedSites() const;

private:
	static constexpr size_t kCategoryCount = static_cast<size_t>(RenderOwnership::ERenderCategory::Count);

	std::atomic<uint32_t> m_bypassCounts[kCategoryCount];
	std::atomic<bool> m_bEnabled;

	mutable std::mutex m_mutex;
	std::vector<SBypassSiteRecord> m_recordedSites;
	std::string m_lastReport;
};

// Klasa pomocnicza RAII do tymczasowego ustawienia aktywnej kategorii renderowania
class CRenderCategoryScope
{
public:
	CRenderCategoryScope(RenderOwnership::ERenderCategory category);
	~CRenderCategoryScope();

private:
	RenderOwnership::ERenderCategory m_prevCategory;
};
