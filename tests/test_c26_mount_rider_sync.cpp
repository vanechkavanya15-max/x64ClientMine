#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include <cmath>
#include <vector>
#include <string>

// Model symulacji hierarchii kosci siodla wierzchowca i jezdzca (Fala W10: T333)
struct Vector3D {
    float x{0.0f}, y{0.0f}, z{0.0f};

    Vector3D operator+(const Vector3D& o) const noexcept {
        return {x + o.x, y + o.y, z + o.z};
    }
    Vector3D operator-(const Vector3D& o) const noexcept {
        return {x - o.x, y - o.y, z - o.z};
    }
    float Length() const noexcept {
        return std::sqrt(x * x + y * y + z * z);
    }
};

struct MountComponentSimulation {
    Vector3D position{0.0f, 0.0f, 0.0f};
    Vector3D velocity{0.0f, 0.0f, 0.0f};
    Vector3D saddleBoneOffset{0.0f, 20.0f, 90.0f}; // Wzgledem srodka wierzchowca
    Vector3D currentSaddleWorldPos{0.0f, 20.0f, 90.0f};
    uint32_t deformCounter{0};

    void Update(float dt) noexcept {
        position.x += velocity.x * dt;
        position.y += velocity.y * dt;
        position.z += velocity.z * dt;
    }

    void Deform() noexcept {
        ++deformCounter;
        // Wierzchowiec deformuje swoj szkielet: oblicza aktualna pozycje kosci siodla w swiecie
        currentSaddleWorldPos = position + saddleBoneOffset;
    }
};

struct RiderComponentSimulation {
    Vector3D position{0.0f, 0.0f, 0.0f};
    uint32_t deformCounter{0};
    uint32_t saddleReadFrame{0};

    void Deform(const MountComponentSimulation& mount) noexcept {
        ++deformCounter;
        // Jezdziec pobiera aktualna kosc siodla z wierzchowca
        position = mount.currentSaddleWorldPos;
        saddleReadFrame = mount.deformCounter;
    }
};

struct InstanceSimulation {
    MountComponentSimulation mount;
    RiderComponentSimulation rider;
    bool isRiding{true};

    // Stara, wadliwa implementacja: Deform jezdzca PRZED wierzchowcem (1-Frame Lag)
    void DeformLegacyBuggy() noexcept {
        if (!isRiding) return;
        rider.Deform(mount); // Odczytuje stary stan sprzed aktualizacji kosci mount!
        mount.Deform();
    }

    // Nowa, naprawiona implementacja W10: Deform wierzchowca PRZED jezdzcem (0-Frame Lag)
    void DeformSynchronizedW10() noexcept {
        if (!isRiding) return;
        mount.Deform(); // Najpierw aktualizacja szkieletu i kosci siodla konia
        rider.Deform(mount); // Nastepnie jezdziec pobiera swieza macierz siodla w tej samej klatce
    }
};

TEST_CASE("Mount & Rider Synchronization - 1-Frame Lag Elimination (T333)")
{
    SUBCASE("Legacy order suffers from 1-frame displacement lag during movement")
    {
        InstanceSimulation inst;
        inst.mount.velocity = {500.0f, 0.0f, 0.0f}; // Ruch z predkoscia 500 j/s
        float dt = 0.016667f; // ~60 FPS

        // Inicjalizacja poczatkowa
        inst.mount.Deform();
        inst.rider.Deform(inst.mount);

        // Klatka 1: wierzchowiec sie przemieszcza
        inst.mount.Update(dt);
        inst.DeformLegacyBuggy();

        // Przy wadliwej kolejnosci, jezdziec otrzymal pozycje sprzed kroku Deform
        Vector3D expectedSaddle = inst.mount.position + inst.mount.saddleBoneOffset;
        float lagDisplacement = (inst.rider.position - expectedSaddle).Length();

        // Displacement jest rowny dokladnie velocity * dt
        CHECK(lagDisplacement == doctest::Approx(500.0f * dt));
        CHECK(inst.rider.saddleReadFrame != inst.mount.deformCounter);
    }

    SUBCASE("Synchronized order guarantees 0-frame displacement lag (T333 W10)")
    {
        InstanceSimulation inst;
        inst.mount.velocity = {750.0f, 250.0f, 0.0f}; // Szybki bieg po skosie
        float dt = 0.016667f;

        // Inicjalizacja
        inst.DeformSynchronizedW10();

        // 60 kolejnych klatek animacji
        for (int frame = 0; frame < 60; ++frame) {
            inst.mount.Update(dt);
            inst.DeformSynchronizedW10();

            Vector3D expectedSaddle = inst.mount.position + inst.mount.saddleBoneOffset;
            float lagDisplacement = (inst.rider.position - expectedSaddle).Length();

            // Bezwzgledna synchronizacja: 0 pikseli odchylenia
            CHECK(lagDisplacement == doctest::Approx(0.0f));
            CHECK(inst.rider.saddleReadFrame == inst.mount.deformCounter);
        }
    }
}
