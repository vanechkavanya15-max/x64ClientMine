#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "EterLib/Camera.h"
#include <memory>

TEST_CASE("Camera Screen Audit - CCameraManager Refactor")
{
    CCameraManager manager;

    // By default, DEFAULT_PERSPECTIVE_CAMERA and DEFAULT_ORTHO_CAMERA are added and DEFAULT_PERSPECTIVE is set as current
    CHECK(manager.GetCurrentCameraNum() == CCameraManager::DEFAULT_PERSPECTIVE_CAMERA);
    CHECK(manager.isCurrentCamera(CCameraManager::DEFAULT_PERSPECTIVE_CAMERA));

    // Test Adding
    CHECK(manager.AddCamera(42) == true);
    CHECK(manager.AddCamera(42) == false); // Should fail adding again

    // Test Changing Current Camera
    manager.SetCurrentCamera(42);
    CHECK(manager.GetCurrentCameraNum() == 42);
    CHECK(manager.isCurrentCamera(42));
    CHECK(!manager.isCurrentCamera(CCameraManager::DEFAULT_PERSPECTIVE_CAMERA));

    // Test setting to invalid/non-existent camera should just return without action
    manager.SetCurrentCamera(100);
    CHECK(manager.GetCurrentCameraNum() == 42); // Should remain 42

    // Validate we can get the actual object
    CCamera* pCam = manager.GetCurrentCamera();
    CHECK(pCam != nullptr);
}
