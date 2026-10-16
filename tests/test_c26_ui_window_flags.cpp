#include <doctest.h>
#include "Client/UI/UIWindowStateFlags.h"

using namespace Client::UI;

TEST_CASE("UIWindowStateFlags: Core Flag Operations") {
    UIWindowStateFlags flags;
    
    // Default state: Show & Enabled
    CHECK(flags.IsVisible());
    CHECK(flags.IsEnabled());
    CHECK(flags.HasFlag(WindowFlag::Show));
    CHECK(flags.HasFlag(WindowFlag::Enabled));
    CHECK_FALSE(flags.HasFlag(WindowFlag::Modal));

    SUBCASE("Set and Remove Flags") {
        auto res = flags.SetFlag(WindowFlag::Modal);
        CHECK(res.has_value());
        CHECK(flags.HasFlag(WindowFlag::Modal));

        res = flags.RemoveFlag(WindowFlag::Show);
        CHECK(res.has_value());
        CHECK_FALSE(flags.IsVisible());
    }

    SUBCASE("Toggle Flags") {
        auto res = flags.ToggleFlag(WindowFlag::Dragable);
        CHECK(res.has_value());
        CHECK(flags.HasFlag(WindowFlag::Dragable));

        res = flags.ToggleFlag(WindowFlag::Dragable);
        CHECK(res.has_value());
        CHECK_FALSE(flags.HasFlag(WindowFlag::Dragable));
    }
}

TEST_CASE("UIWindowStateFlags: Transition Constraints") {
    UIWindowStateFlags flags;

    SUBCASE("Cannot be Hovered and NotPick") {
        flags.SetFlag(WindowFlag::NotPick);
        auto res = flags.SetFlag(WindowFlag::Hovered);
        CHECK_FALSE(res.has_value());
        CHECK(res.error() == WindowStateError::TransitionConstraintViolated);
        CHECK_FALSE(flags.HasFlag(WindowFlag::Hovered));
    }

    SUBCASE("Cannot be Dragable if Restricted") {
        flags.SetFlag(WindowFlag::Restricted);
        auto res = flags.SetFlag(WindowFlag::Dragable);
        CHECK_FALSE(res.has_value());
        CHECK(res.error() == WindowStateError::TransitionConstraintViolated);
        CHECK_FALSE(flags.HasFlag(WindowFlag::Dragable));
    }

    SUBCASE("Cannot be Pressed if NotShow or NotEnabled") {
        flags.RemoveFlag(WindowFlag::Show);
        auto res = flags.SetFlag(WindowFlag::Pressed);
        CHECK_FALSE(res.has_value());
        CHECK(res.error() == WindowStateError::TransitionConstraintViolated);
        
        flags.SetFlag(WindowFlag::Show);
        flags.RemoveFlag(WindowFlag::Enabled);
        res = flags.SetFlag(WindowFlag::Pressed);
        CHECK_FALSE(res.has_value());
        CHECK(res.error() == WindowStateError::TransitionConstraintViolated);
    }
}

TEST_CASE("UIWindowStateFlags: Alpha transparency and auto-hide") {
    UIWindowStateFlags flags;
    CHECK(flags.IsVisible());

    SUBCASE("Set valid alpha") {
        auto res = flags.SetAlpha(0.5f);
        CHECK(res.has_value());
        CHECK(flags.GetAlpha() == doctest::Approx(0.5f));
        CHECK(flags.IsVisible());
    }

    SUBCASE("Clamp alpha values") {
        flags.SetAlpha(1.5f);
        CHECK(flags.GetAlpha() == doctest::Approx(1.0f));

        flags.SetAlpha(-0.5f);
        CHECK(flags.GetAlpha() == doctest::Approx(0.0f));
    }

    SUBCASE("Alpha reaching 0 removes Show flag") {
        flags.SetAlpha(0.005f); // Below 0.01f
        CHECK_FALSE(flags.IsVisible());
    }
}

TEST_CASE("UIWindowStateFlags: Lock Mechanism") {
    UIWindowStateFlags flags;
    
    auto res = flags.SetFlag(WindowFlag::Locked);
    CHECK(res.has_value());

    SUBCASE("Cannot change flags when locked") {
        res = flags.SetFlag(WindowFlag::Modal);
        CHECK_FALSE(res.has_value());
        CHECK(res.error() == WindowStateError::Locked);
        CHECK_FALSE(flags.HasFlag(WindowFlag::Modal));
    }

    SUBCASE("Cannot change alpha when locked") {
        res = flags.SetAlpha(0.5f);
        CHECK_FALSE(res.has_value());
        CHECK(res.error() == WindowStateError::Locked);
        CHECK(flags.GetAlpha() == doctest::Approx(1.0f));
    }

    SUBCASE("Can unlock") {
        res = flags.RemoveFlag(WindowFlag::Locked);
        CHECK(res.has_value());
        CHECK_FALSE(flags.HasFlag(WindowFlag::Locked));
        
        res = flags.SetFlag(WindowFlag::Modal);
        CHECK(res.has_value());
        CHECK(flags.HasFlag(WindowFlag::Modal));
    }
}

TEST_CASE("UIWindowStateFlags: Observers") {
    UIWindowStateFlags flags;
    bool observerCalled = false;
    WindowFlag lastOldFlags = WindowFlag::None;
    WindowFlag lastNewFlags = WindowFlag::None;

    flags.AddObserver(1, [&](WindowFlag oldF, WindowFlag newF) {
        observerCalled = true;
        lastOldFlags = oldF;
        lastNewFlags = newF;
    });

    flags.SetFlag(WindowFlag::Dragable);
    
    CHECK(observerCalled);
    CHECK((lastOldFlags & WindowFlag::Dragable) != WindowFlag::Dragable);
    CHECK((lastNewFlags & WindowFlag::Dragable) == WindowFlag::Dragable);

    observerCalled = false;
    flags.RemoveObserver(1);
    flags.SetFlag(WindowFlag::Modal);
    CHECK_FALSE(observerCalled);
}

TEST_CASE("UIWindowStateFlags: Snapshot Mechanism") {
    UIWindowStateFlags flags;
    flags.SetFlag(WindowFlag::Modal);
    flags.SetAlpha(0.7f);

    flags.SaveSnapshot(100, 123456789);

    flags.RemoveFlag(WindowFlag::Modal);
    flags.SetAlpha(1.0f);

    SUBCASE("Restore snapshot") {
        auto res = flags.RestoreSnapshot(100);
        CHECK(res.has_value());
        CHECK(flags.HasFlag(WindowFlag::Modal));
        CHECK(flags.GetAlpha() == doctest::Approx(0.7f));
    }

    SUBCASE("Restore non-existent snapshot") {
        auto res = flags.RestoreSnapshot(999);
        CHECK_FALSE(res.has_value());
        CHECK(res.error() == WindowStateError::SnapshotNotFound);
    }
    
    SUBCASE("Restore while locked fails") {
        flags.SetFlag(WindowFlag::Locked);
        auto res = flags.RestoreSnapshot(100);
        CHECK_FALSE(res.has_value());
        CHECK(res.error() == WindowStateError::Locked);
    }
}

TEST_CASE("UIWindowStateManager: Management and Globals") {
    UIWindowStateManager manager;
    manager.RegisterWindow(1);
    manager.RegisterWindow(2);
    manager.RegisterWindow(3);

    auto* w1 = manager.GetWindowFlags(1);
    auto* w2 = manager.GetWindowFlags(2);
    auto* w3 = manager.GetWindowFlags(3);

    REQUIRE(w1 != nullptr);
    REQUIRE(w2 != nullptr);
    REQUIRE(w3 != nullptr);

    SUBCASE("Bulk Hide and Show") {
        manager.BulkHideAll();
        CHECK_FALSE(w1->IsVisible());
        CHECK_FALSE(w2->IsVisible());
        CHECK_FALSE(w3->IsVisible());

        manager.BulkShowAll();
        CHECK(w1->IsVisible());
        CHECK(w2->IsVisible());
        CHECK(w3->IsVisible());
    }

    SUBCASE("Global Modal Stack") {
        auto res = manager.PushGlobalModal(2);
        CHECK(res.has_value());
        
        CHECK(w2->HasFlag(WindowFlag::Modal));
        CHECK(w2->HasFlag(WindowFlag::TopMost));
        CHECK(w2->IsEnabled());
        
        // Others should be disabled
        CHECK_FALSE(w1->IsEnabled());
        CHECK_FALSE(w3->IsEnabled());

        CHECK(manager.GetActiveGlobalModal() == 2);

        res = manager.PopGlobalModal();
        CHECK(res.has_value());
        
        CHECK_FALSE(w2->HasFlag(WindowFlag::Modal));
        CHECK_FALSE(w2->HasFlag(WindowFlag::TopMost));
        
        // Others re-enabled
        CHECK(w1->IsEnabled());
        CHECK(w3->IsEnabled());
    }
}