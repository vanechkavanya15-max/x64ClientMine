#include "doctest.h"
#include "Client/Actors/ActorEffectComponent.h"

using namespace Client::Actors;

TEST_SUITE("ActorEffectComponent Tests") {
    
    TEST_CASE("Poczatkowy stan to zero aktywnych efektow") {
        ActorEffectComponent comp;
        CHECK(comp.GetActiveEffectCount() == 0);
        CHECK(comp.GetActiveEffects().empty());
    }

    TEST_CASE("Dodawanie poprawnego efektu") {
        ActorEffectComponent comp;
        
        auto res = comp.AttachEffect(EffectId{123});
        REQUIRE(res.has_value());
        
        EffectHandle handle = res.value();
        CHECK(comp.GetActiveEffectCount() == 1);
        CHECK(comp.HasEffect(handle));
        
        auto effects = comp.GetActiveEffects();
        REQUIRE(effects.size() == 1);
        CHECK(effects[0] == handle);
    }

    TEST_CASE("Dodawanie zlego efektu (ID = 0) zwroci błąd InvalidEffect") {
        ActorEffectComponent comp;
        
        auto res = comp.AttachEffect(EffectId{0});
        REQUIRE_FALSE(res.has_value());
        CHECK(res.error() == EffectError::InvalidEffect);
        CHECK(comp.GetActiveEffectCount() == 0);
    }

    TEST_CASE("Usuwanie istniejacego efektu") {
        ActorEffectComponent comp;
        
        auto res = comp.AttachEffect(EffectId{111});
        REQUIRE(res.has_value());
        EffectHandle handle = res.value();
        
        auto detachRes = comp.DetachEffect(handle);
        REQUIRE(detachRes.has_value());
        
        CHECK(comp.GetActiveEffectCount() == 0);
        CHECK_FALSE(comp.HasEffect(handle));
    }

    TEST_CASE("Usuwanie nieistniejacego efektu zwraca blad") {
        ActorEffectComponent comp;
        
        auto detachRes = comp.DetachEffect(EffectHandle{999});
        REQUIRE_FALSE(detachRes.has_value());
        CHECK(detachRes.error() == EffectError::EffectNotFound);
    }

    TEST_CASE("Osiagniecie limitu efektow (LimitReached)") {
        ActorEffectComponent comp;
        
        // Wypełniamy do maksimum
        for (uint32_t i = 1; i <= ActorEffectComponent::MAX_ACTIVE_EFFECTS; ++i) {
            auto res = comp.AttachEffect(EffectId{i});
            REQUIRE(res.has_value());
        }
        
        CHECK(comp.GetActiveEffectCount() == ActorEffectComponent::MAX_ACTIVE_EFFECTS);
        
        // Próba dodania jeszcze jednego efektu ponad limit
        auto overRes = comp.AttachEffect(EffectId{9999});
        REQUIRE_FALSE(overRes.has_value());
        CHECK(overRes.error() == EffectError::LimitReached);
    }

    TEST_CASE("DetachAllEffects czysci wszystkie przypisane efekty") {
        ActorEffectComponent comp;
        
        comp.AttachEffect(EffectId{10});
        comp.AttachEffect(EffectId{20});
        comp.AttachEffect(EffectId{30});
        
        CHECK(comp.GetActiveEffectCount() == 3);
        
        auto detachAllRes = comp.DetachAllEffects();
        REQUIRE(detachAllRes.has_value());
        
        CHECK(comp.GetActiveEffectCount() == 0);
    }
}
