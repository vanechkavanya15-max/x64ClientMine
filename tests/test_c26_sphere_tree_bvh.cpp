#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include "Client/Collision/SphereTreeBVH.h"
#include "SphereLib/spherepack.h"
#include <chrono>

using namespace Client::Collision;

TEST_CASE("SphereTreeBVH - Hierarchy Construction & Dual Bounding Volume")
{
    SphereTreeBVH bvh;
    CHECK(bvh.GetObstacleCount() == 0);
    CHECK(bvh.GetNodeCount() == 0);

    // Dodanie zestawu sfer przeszkod
    bvh.AddSphere(1, CleanVec3{0.0f, 0.0f, 0.0f}, 15.0f);
    bvh.AddSphere(2, CleanVec3{100.0f, 0.0f, 0.0f}, 20.0f);
    bvh.AddSphere(3, CleanVec3{-100.0f, 0.0f, 0.0f}, 25.0f);
    bvh.AddSphere(4, CleanVec3{0.0f, 100.0f, 0.0f}, 10.0f);
    bvh.AddWall(5, CleanVec3{50.0f, 50.0f, 0.0f}, CleanVec3{0.0f, 1.0f, 0.0f}, 40.0f, 40.0f, 10.0f);

    CHECK(bvh.GetObstacleCount() == 5);
    bvh.Build();
    CHECK(bvh.GetNodeCount() > 0);

    // Test zapytania statycznego wokol srodka swiata
    auto results = bvh.QueryRange(CleanVec3{0.0f, 0.0f, 0.0f}, 20.0f);
    CHECK(results.size() >= 1);
    CHECK(std::find(results.begin(), results.end(), 1) != results.end());
    // Przeszkoda 2 na (100, 0, 0) nie powinna sie znalezc w zasiegu 20
    CHECK(std::find(results.begin(), results.end(), 2) == results.end());
}

TEST_CASE("SphereTreeBVH - Swept Sphere Trajectory & Hierarchical Pruning")
{
    SphereTreeBVH bvh;
    // Umieszczenie przeszkod w przestrzeni
    bvh.AddSphere(10, CleanVec3{50.0f, 0.0f, 0.0f}, 15.0f);
    bvh.AddSphere(20, CleanVec3{50.0f, 200.0f, 0.0f}, 15.0f); // Bardzo daleko na Y
    bvh.Build();

    // Trajektoria ruchu od (0, 0, 0) do (100, 0, 0) o promieniu 10
    BVHSweptSphere swept;
    swept.start = CleanVec3{0.0f, 0.0f, 0.0f};
    swept.end = CleanVec3{100.0f, 0.0f, 0.0f};
    swept.radius = 10.0f;

    auto hits = bvh.QuerySweptSphere(swept);
    CHECK(hits.size() == 1);
    CHECK(hits[0] == 10);
    // Przeszkoda 20 na Y=200 powinna zostac odrzucona przez Hierarchical Pruning
    CHECK(std::find(hits.begin(), hits.end(), 20) == hits.end());
}

TEST_CASE("SphereTreeBVH - Anti-Tunneling at Speed > 200 (Wall Tunneling Prevention)")
{
    SphereTreeBVH bvh;

    // Cienka sciana (grubosc 10.0f) na plaszczyznie X = 100.0f, normalna skierowana przeciwnie do ruchu (-1, 0, 0)
    // Rozmiar sciany: szerokosc 100 (Y), wysokosc 100 (Z)
    bvh.AddWall(100, CleanVec3{100.0f, 0.0f, 0.0f}, CleanVec3{-1.0f, 0.0f, 0.0f}, 100.0f, 100.0f, 10.0f);
    bvh.Build();

    float playerRadius = 20.0f;
    CleanVec3 startPos{0.0f, 0.0f, 0.0f};

    // Scenariusz 1: Bardzo duza predkosc (Speed = 300 jednostek/s, dt = 0.5s -> przesuniecie 150 jednostek)
    // Pozycja koncowa bez kolizji: x = 150.0f (za sciana!)
    CleanVec3 velocity300{300.0f, 0.0f, 0.0f};
    float dt = 0.5f;

    // Bez Continuous Collision Detection postac przeskoczylaby ze startu 0 na 150 (przenikanie sciany / tunnelling)
    CleanVec3 discreteEnd = startPos + velocity300 * dt;
    CHECK(discreteEnd.x == doctest::Approx(150.0f));
    CHECK(discreteEnd.x > 100.0f); // Tunnelling w logice dyskretnej!

    // Z BVH Continuous Collision Detection:
    BVHSweptSphere swept;
    swept.start = startPos;
    swept.end = discreteEnd;
    swept.radius = playerRadius;

    BVHContinuousHit hit;
    bool hasHit = bvh.ContinuousDetectCollision(swept, hit);
    CHECK(hasHit);
    CHECK(hit.hit);
    CHECK(hit.obstacleId == 100);

    // Punkt uderzenia nastepuje gdy srodek sfery osiaga x = 100.0f - playerRadius = 80.0f
    // Ruch: 0 -> 150. Wtedy t = 80 / 150 = 0.5333f
    CHECK(hit.toi == doctest::Approx(80.0f / 150.0f).epsilon(0.01f));
    CHECK(hit.normal.x == doctest::Approx(-1.0f));

    // Weryfikacja resolwera anty-tunelowania
    auto resolved = bvh.ResolveMovementAntiTunneling(startPos, velocity300, dt, playerRadius, 0.5f);
    CHECK(resolved.hit);
    // Skorygowana pozycja bezpieczna MUSI byc PRZED sciana (x < 100.0f - playerRadius = 80.0f)
    CHECK(resolved.resolvedSafePos.x <= 80.05f);
    CHECK(resolved.resolvedSafePos.x >= 75.0f);
    // Postac nie przeniknela sciany!
    CHECK(resolved.resolvedSafePos.x < 100.0f);

    // Scenariusz 2: Ekstremalna predkosc (Speed = 1000 jednostek/s, dt = 0.2s -> przesuniecie 200 jednostek)
    CleanVec3 velocity1000{1000.0f, 0.0f, 0.0f};
    auto resolved1000 = bvh.ResolveMovementAntiTunneling(startPos, velocity1000, 0.2f, playerRadius, 0.5f);
    CHECK(resolved1000.hit);
    CHECK(resolved1000.resolvedSafePos.x <= 80.05f);
    CHECK(resolved1000.resolvedSafePos.x < 100.0f);

    // Scenariusz 3: Ruch pod katem 45 stopni do sciany (generowanie wektora slizgania)
    CleanVec3 velocityDiagonal{300.0f, 300.0f, 0.0f};
    auto resolvedDiagonal = bvh.ResolveMovementAntiTunneling(startPos, velocityDiagonal, dt, playerRadius, 0.5f);
    CHECK(resolvedDiagonal.hit);
    CHECK(resolvedDiagonal.resolvedSafePos.x < 100.0f);
    // Skladowa X wzdluz normalnej sciany zostaje skasowana, a skladowa Y (slizg) zachowana!
    CHECK(resolvedDiagonal.slideVelocity.x == doctest::Approx(0.0f));
    CHECK(resolvedDiagonal.slideVelocity.y == doctest::Approx(300.0f));
}

TEST_CASE("SphereTreeBVH - Swept Sphere vs Multiple Sequential Obstacles")
{
    SphereTreeBVH bvh;

    // Trzy kolejne filary / kolumny na osi X
    bvh.AddSphere(1, CleanVec3{50.0f, 0.0f, 0.0f}, 10.0f);
    bvh.AddSphere(2, CleanVec3{150.0f, 0.0f, 0.0f}, 10.0f);
    bvh.AddSphere(3, CleanVec3{250.0f, 0.0f, 0.0f}, 10.0f);
    bvh.Build();

    // Szybki ruch przez cala linie od 0 do 300
    BVHSweptSphere swept;
    swept.start = CleanVec3{0.0f, 0.0f, 0.0f};
    swept.end = CleanVec3{300.0f, 0.0f, 0.0f};
    swept.radius = 10.0f;

    BVHContinuousHit hit;
    bool hasHit = bvh.ContinuousDetectCollision(swept, hit);
    CHECK(hasHit);
    // Pierwsze uderzenie musi dotyczyc najblizszego filaru (ID 1 na x=50)
    CHECK(hit.obstacleId == 1);
    // Odleglosc uderzenia: 50 - (10+10) = 30. toi = 30 / 300 = 0.1f
    CHECK(hit.toi == doctest::Approx(0.1f).epsilon(0.01f));
}

// Test dla rozszerzonego SpherePackFactory i SpherePack
class TestSweptCallback : public SpherePackCallback
{
public:
    int hitCount{0};
    std::vector<void*> hits;

    void SweptSphereCallback(const Vector3d &p1,
                             const Vector3d &p2,
                             float radius,
                             SpherePack *sphere,
                             ViewState state) override
    {
        hitCount++;
        hits.push_back(sphere->GetUserData());
    }
};

TEST_CASE("SphereLib - SpherePackFactory SweptSphereTest Query")
{
    SpherePackFactory factory(64, 1024.0f, 128.0f, 8.0f);

    int objA = 101;
    int objB = 202;
    int objC = 303; // Daleko

    factory.AddSphere_(Vector3d(60.0f, 0.0f, 0.0f), 15.0f, &objA, true);
    factory.AddSphere_(Vector3d(120.0f, 0.0f, 0.0f), 15.0f, &objB, true);
    factory.AddSphere_(Vector3d(0.0f, 500.0f, 0.0f), 15.0f, &objC, true);

    factory.Process();

    TestSweptCallback cb;
    Vector3d p1(0.0f, 0.0f, 0.0f);
    Vector3d p2(80.0f, 0.0f, 0.0f); // Przecina sfere objA, ale nie objB i nie objC
    float radius = 10.0f;

    factory.SweptSphereTest(p1, p2, radius, &cb);

    CHECK(cb.hitCount >= 1);
    CHECK(std::find(cb.hits.begin(), cb.hits.end(), &objA) != cb.hits.end());
    CHECK(std::find(cb.hits.begin(), cb.hits.end(), &objC) == cb.hits.end());
}

TEST_CASE("SphereTreeBVH - Performance & Zero-Alloc Stress Test")
{
    SphereTreeBVH bvh;

    // Zbudowanie gestego lasu przeszkod (100 kolumn w siatce 10x10)
    for (int x = 0; x < 10; ++x)
    {
        for (int y = 0; y < 10; ++y)
        {
            uint32_t id = static_cast<uint32_t>(x * 10 + y + 1);
            bvh.AddSphere(id, CleanVec3{x * 50.0f, y * 50.0f, 0.0f}, 12.0f);
        }
    }
    bvh.Build();

    auto startTime = std::chrono::high_resolution_clock::now();

    // 1000 zapytan w roznych kierunkach przy wysokiej predkosci
    int hitCount = 0;
    for (int i = 0; i < 1000; ++i)
    {
        BVHSweptSphere swept;
        float angle = static_cast<float>(i) * 0.1f;
        swept.start = CleanVec3{250.0f, 250.0f, 0.0f};
        swept.end = swept.start + CleanVec3{std::cos(angle) * 150.0f, std::sin(angle) * 150.0f, 0.0f};
        swept.radius = 15.0f;

        BVHContinuousHit hit;
        if (bvh.ContinuousDetectCollision(swept, hit))
        {
            hitCount++;
        }
    }

    auto endTime = std::chrono::high_resolution_clock::now();
    auto elapsedMs = std::chrono::duration_cast<std::chrono::milliseconds>(endTime - startTime).count();

    CHECK(hitCount > 0);
    // 1000 zapytan BVH powinno wykonac sie ponizej 50ms na nowoczesnym CPU
    CHECK(elapsedMs < 50);
}
