#include "doctest.h"
#include "../src/Client/World/AreaPortalManager.h"

using namespace Client::World;

TEST_SUITE("AreaPortalManager") {

    TEST_CASE("Poczatkowy stan to wylaczone portale i pusta lista") {
        AreaPortalManager manager;
        CHECK_FALSE(manager.IsPortalEnabled());
        CHECK(manager.GetShowingPortals().empty());
    }

    TEST_CASE("Wlaczanie i wylaczanie portali") {
        AreaPortalManager manager;
        manager.EnablePortal(true);
        CHECK(manager.IsPortalEnabled());
        
        manager.EnablePortal(false);
        CHECK_FALSE(manager.IsPortalEnabled());
    }

    TEST_CASE("Dodawanie id portali do wyswietlania") {
        AreaPortalManager manager;
        
        PortalId id1(10);
        auto result = manager.AddShowingPortalId(id1);
        CHECK(result.has_value());
        CHECK(manager.IsPortalShowing(id1));
        CHECK(manager.GetShowingPortals().size() == 1);
        
        // Dodawanie tego samego id powinno zwrocic blad
        auto result2 = manager.AddShowingPortalId(id1);
        CHECK_FALSE(result2.has_value());
        CHECK(result2.error() == PortalError::AlreadyVisible);
        CHECK(manager.GetShowingPortals().size() == 1);
    }

    TEST_CASE("Usuwanie id portali") {
        AreaPortalManager manager;
        
        PortalId id1(20);
        manager.AddShowingPortalId(id1);
        CHECK(manager.IsPortalShowing(id1));
        
        auto result = manager.RemoveShowingPortalId(id1);
        CHECK(result.has_value());
        CHECK_FALSE(manager.IsPortalShowing(id1));
        CHECK(manager.GetShowingPortals().empty());
        
        // Usuwanie nieistniejacego id powinno zwrocic blad
        auto result2 = manager.RemoveShowingPortalId(id1);
        CHECK_FALSE(result2.has_value());
        CHECK(result2.error() == PortalError::NotVisible);
    }

    TEST_CASE("Wyczyszczenie wszystkich portali") {
        AreaPortalManager manager;
        
        manager.AddShowingPortalId(PortalId(1));
        manager.AddShowingPortalId(PortalId(2));
        manager.AddShowingPortalId(PortalId(3));
        
        CHECK(manager.GetShowingPortals().size() == 3);
        
        manager.ClearPortals();
        CHECK(manager.GetShowingPortals().empty());
        CHECK_FALSE(manager.IsPortalShowing(PortalId(1)));
    }
    
    TEST_CASE("Dodawanie niepoprawnego id powinno zwrocic blad") {
        AreaPortalManager manager;
        
        PortalId invalidId(-5);
        auto result = manager.AddShowingPortalId(invalidId);
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == PortalError::InvalidId);
        
        auto result2 = manager.RemoveShowingPortalId(invalidId);
        CHECK_FALSE(result2.has_value());
        CHECK(result2.error() == PortalError::InvalidId);
    }
}
