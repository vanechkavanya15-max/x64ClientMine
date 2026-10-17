#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include <algorithm>
#include <optional>
#include <vector>
#include <deque>

/**
 * @brief Model maszyny stanow cooltime slotow UI i logiki gry.
 * 
 * Odzwierciedla zachowanie CSlotWindow oraz SkillCooldownTracker:
 * - Update() odpowiada za mutacje stanu (przejscia stanow, wygaszanie, tworzenie efektu).
 * - Render() jest czysto odczytowy (oblicza postep renderowania bez efektow ubocznych).
 */
namespace CooltimeStateModel {

enum class ECooltimePhase {
	Idle,
	Active,
	FinishEffectActive,
	ExpiredWithoutEffect
};

struct SlotCooltimeState {
	float fCoolTime{0.0f};
	float fStartCoolTime{0.0f};
	bool bHasFinishEffect{false};
	bool bFinishEffectDestroyReserved{false};

	[[nodiscard]] bool IsActive() const noexcept {
		return fCoolTime > 0.0f;
	}

	[[nodiscard]] ECooltimePhase GetPhase() const noexcept {
		if (bHasFinishEffect)
			return ECooltimePhase::FinishEffectActive;
		if (fCoolTime > 0.0f)
			return ECooltimePhase::Active;
		return ECooltimePhase::Idle;
	}

	void StartCooltime(float curTime, float duration, float elapsed = 0.0f) noexcept {
		if (duration <= 0.0f) {
			fCoolTime = 0.0f;
			fStartCoolTime = 0.0f;
			return;
		}
		fCoolTime = duration;
		fStartCoolTime = curTime - elapsed;
		bHasFinishEffect = false;
		bFinishEffectDestroyReserved = false;
	}

	void Reset() noexcept {
		fCoolTime = 0.0f;
		fStartCoolTime = 0.0f;
		bHasFinishEffect = false;
		bFinishEffectDestroyReserved = false;
	}

	/**
	 * @brief Czysta funkcja renderujaca - BRAK efektow ubocznych.
	 * Nie modyfikuje fCoolTime, fStartCoolTime ani bHasFinishEffect.
	 */
	[[nodiscard]] std::optional<float> ComputeRenderProgress(float curTime) const noexcept {
		if (fCoolTime <= 0.0f)
			return std::nullopt;

		float rawPercentage = (curTime - fStartCoolTime) / fCoolTime;
		return std::clamp(rawPercentage, 0.0f, 1.0f);
	}

	/**
	 * @brief Metoda aktualizacji logiki klatki (Update).
	 * Wykonuje wylacznie tutaj mutacje stanu i przejscia.
	 */
	void Update(float curTime) noexcept {
		// 1. Czyszczenie zarezerwowanych efektow
		if (bFinishEffectDestroyReserved) {
			bHasFinishEffect = false;
			bFinishEffectDestroyReserved = false;
		}

		// 2. Aktualizacja cooltime
		if (fCoolTime > 0.0f) {
			float elapsedSinceStart = curTime - fStartCoolTime;
			if (elapsedSinceStart >= fCoolTime) {
				float overdue = elapsedSinceStart - fCoolTime;
				if (overdue < 1.0f) {
					bHasFinishEffect = true;
				}
				fCoolTime = 0.0f;
				fStartCoolTime = 0.0f;
			}
		}
	}

	void ReserveDestroyFinishEffect() noexcept {
		if (bHasFinishEffect) {
			bFinishEffectDestroyReserved = true;
		}
	}
};

} // namespace CooltimeStateModel

using namespace CooltimeStateModel;

TEST_SUITE("CooltimeState - Side Effect Free Render & Update Semantics") {

	TEST_CASE("StateTransition_InitialStateIsIdle") {
		SlotCooltimeState slot;
		CHECK(slot.GetPhase() == ECooltimePhase::Idle);
		CHECK_FALSE(slot.IsActive());
		CHECK_FALSE(slot.ComputeRenderProgress(10.0f).has_value());
	}

	TEST_CASE("StateTransition_StartCooltime") {
		SlotCooltimeState slot;
		const float t0 = 100.0f;
		slot.StartCooltime(t0, 10.0f);

		CHECK(slot.IsActive());
		CHECK(slot.GetPhase() == ECooltimePhase::Active);
		CHECK(slot.fCoolTime == 10.0f);
		CHECK(slot.fStartCoolTime == 100.0f);

		auto progress = slot.ComputeRenderProgress(t0);
		REQUIRE(progress.has_value());
		CHECK(doctest::Approx(*progress).epsilon(0.001f) == 0.0f);
	}

	TEST_CASE("SideEffectFreeRender_RenderDoesNotMutateState") {
		SlotCooltimeState slot;
		const float t0 = 100.0f;
		slot.StartCooltime(t0, 5.0f);

		// Render w polowie trwania cooltime
		for (int frame = 0; frame < 50; ++frame) {
			auto progress = slot.ComputeRenderProgress(102.5f);
			REQUIRE(progress.has_value());
			CHECK(doctest::Approx(*progress).epsilon(0.001f) == 0.5f);
			CHECK(slot.IsActive());
			CHECK(slot.fCoolTime == 5.0f);
			CHECK(slot.fStartCoolTime == 100.0f);
			CHECK_FALSE(slot.bHasFinishEffect);
		}

		// Render po przekroczeniu czasu cooltime - NIE MOZE modyfikowac stanu ani tworzyc finish effect!
		for (int frame = 0; frame < 50; ++frame) {
			auto progress = slot.ComputeRenderProgress(106.0f);
			REQUIRE(progress.has_value());
			CHECK(*progress == 1.0f); // clamped to 1.0
			CHECK(slot.IsActive());   // wciaz aktywny dopoki nie wykona sie Update()!
			CHECK(slot.fCoolTime == 5.0f);
			CHECK_FALSE(slot.bHasFinishEffect);
		}
	}

	TEST_CASE("StateTransition_UpdateTriggersFinishEffectWithinOneSecondWindow") {
		SlotCooltimeState slot;
		const float t0 = 100.0f;
		slot.StartCooltime(t0, 5.0f);

		// Klatka przed zakonczeniem (t = 104.9s)
		slot.Update(104.9f);
		CHECK(slot.IsActive());
		CHECK_FALSE(slot.bHasFinishEffect);

		// Klatka w momencie zakonczenia (t = 105.2s, 0.2s po terminie < 1.0s)
		slot.Update(105.2f);
		CHECK_FALSE(slot.IsActive());
		CHECK(slot.fCoolTime == 0.0f);
		CHECK(slot.fStartCoolTime == 0.0f);
		CHECK(slot.bHasFinishEffect == true);
		CHECK(slot.GetPhase() == ECooltimePhase::FinishEffectActive);

		// Render po zakonczeniu nie rysuje juz cooltime boxa
		CHECK_FALSE(slot.ComputeRenderProgress(105.3f).has_value());

		// Rezerwacja usuniecia efektu finiszu
		slot.ReserveDestroyFinishEffect();
		CHECK(slot.bHasFinishEffect == true);

		// Kolejny Update czysci efekt
		slot.Update(105.4f);
		CHECK_FALSE(slot.bHasFinishEffect);
		CHECK(slot.GetPhase() == ECooltimePhase::Idle);
	}

	TEST_CASE("StateTransition_UpdateExpiresWithoutEffectWhenLagExceedsOneSecond") {
		SlotCooltimeState slot;
		const float t0 = 100.0f;
		slot.StartCooltime(t0, 5.0f);

		// Update wywolany 2 sekundy po uplywie cooltime (np. lag lub minimalizacja)
		slot.Update(107.5f);
		CHECK_FALSE(slot.IsActive());
		CHECK(slot.fCoolTime == 0.0f);
		CHECK(slot.fStartCoolTime == 0.0f);
		CHECK_FALSE(slot.bHasFinishEffect);
		CHECK(slot.GetPhase() == ECooltimePhase::Idle);
	}

	TEST_CASE("StateTransition_TransferCooltimeBetweenSlots") {
		SlotCooltimeState slotA;
		SlotCooltimeState slotB;

		slotA.StartCooltime(50.0f, 10.0f, 3.0f); // 3s uplynelo
		CHECK(slotA.IsActive());

		// Transfer ze slota A do slota B
		slotB.fCoolTime = slotA.fCoolTime;
		slotB.fStartCoolTime = slotA.fStartCoolTime;
		slotA.Reset();

		CHECK_FALSE(slotA.IsActive());
		CHECK(slotB.IsActive());
		auto progressB = slotB.ComputeRenderProgress(50.0f);
		REQUIRE(progressB.has_value());
		CHECK(doctest::Approx(*progressB).epsilon(0.001f) == 0.3f);
	}

	TEST_CASE("StateTransition_ResetOrOverrideActiveCooltime") {
		SlotCooltimeState slot;
		slot.StartCooltime(10.0f, 20.0f);
		CHECK(slot.IsActive());

		// Nadpisanie nowym czasem
		slot.StartCooltime(15.0f, 5.0f);
		CHECK(slot.IsActive());
		CHECK(slot.fCoolTime == 5.0f);
		CHECK(slot.fStartCoolTime == 15.0f);

		// Reczny reset
		slot.Reset();
		CHECK_FALSE(slot.IsActive());
		CHECK(slot.GetPhase() == ECooltimePhase::Idle);
	}
}
