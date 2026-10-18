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

TEST_CASE("Camera Screen Audit - CCamera Kinematics and Collision Configuration")
{
    CCamera camera;

    // Check initial distance bounds
    CHECK(CCamera::GetCameraMinDistance() > 0.0f);
    CHECK(CCamera::GetCameraMaxDistance() >= CCamera::GetCameraMinDistance());

    // Test terrain collision toggles
    camera.SetTerrainCollision(true);
    CHECK(camera.isTerrainCollisionEnable() == true);
    camera.SetTerrainCollision(false);
    CHECK(camera.isTerrainCollisionEnable() == false);

    // Test target height
    camera.SetTargetHeight(120.0f);
    CHECK(camera.GetTargetHeight() == doctest::Approx(120.0f));

    // Test setting distance and angles
    camera.SetDistance(1500.0f);
    CHECK(camera.GetDistance() == doctest::Approx(1500.0f));

    camera.Pitch(15.0f);
    camera.Roll(45.0f);

    // Test ViewRay and Collision rays
    CRay& viewRay = camera.GetViewRay();
    CRay& bottomRay = camera.GetBottomObjectCollisionRay();
    CRay& topRay = camera.GetTopObjectCollisionRay();
    CRay& leftRay = camera.GetLeftObjectCollisionRay();
    CRay& rightRay = camera.GetRightObjectCollisionRay();

    // Verify ray objects are accessible and valid references
    CHECK(&viewRay != nullptr);
    CHECK(&bottomRay != nullptr);
    CHECK(&topRay != nullptr);
    CHECK(&leftRay != nullptr);
    CHECK(&rightRay != nullptr);
}

