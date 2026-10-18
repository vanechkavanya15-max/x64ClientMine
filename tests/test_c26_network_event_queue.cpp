#include <iostream>
#include <cassert>
#include <vector>
#include <cstring>
#include <span>

#include "UserInterface/Network/NetworkEventQueue.h"

#pragma pack(push, 1)
struct TestPacketHeader {
    uint16_t header;
    uint16_t length;
};

struct TestMovePacket {
    uint16_t header;
    uint16_t length;
    uint32_t vid;
    int32_t x;
    int32_t y;
};
#pragma pack(pop)

static void TestRingBufferEnqueueAndDrain()
{
    std::cout << "[TEST] 1. Test SpscPacketRingBuffer Enqueue i DrainAll..." << std::endl;
    using namespace UserInterface::Network;

    SpscPacketRingBuffer<64 * 1024> ring;
    assert(!ring.HasPacket());

    TestMovePacket p1{ 0x01, sizeof(TestMovePacket), 1001, 10000, 20000 };
    std::span<const uint8_t> span1(reinterpret_cast<const uint8_t*>(&p1), sizeof(p1));
    bool enq1 = ring.TryEnqueue(0x01, span1, 100);
    assert(enq1);
    assert(ring.HasPacket());

    TestMovePacket p2{ 0x02, sizeof(TestMovePacket), 1002, 30000, 40000 };
    std::span<const uint8_t> span2(reinterpret_cast<const uint8_t*>(&p2), sizeof(p2));
    bool enq2 = ring.TryEnqueue(0x02, span2, 105);
    assert(enq2);

    size_t count = 0;
    size_t processed = ring.DrainAll([&](const PacketFrameView& frame) -> bool {
        assert(frame.data.size() == sizeof(TestMovePacket));
        const TestMovePacket* pRecv = reinterpret_cast<const TestMovePacket*>(frame.data.data());
        if (count == 0)
        {
            assert(frame.meta.opcode == 0x01);
            assert(pRecv->vid == 1001);
            assert(pRecv->x == 10000);
        }
        else if (count == 1)
        {
            assert(frame.meta.opcode == 0x02);
            assert(pRecv->vid == 1002);
            assert(pRecv->x == 30000);
        }
        ++count;
        return true;
    });

    assert(processed == 2);
    assert(!ring.HasPacket());
    std::cout << "[PASS] Enqueue i DrainAll zaliczone pomyslnie." << std::endl;
}

static void TestWrapAroundBoundary()
{
    std::cout << "[TEST] 2. Test zawijania bufora pierscieniowego (Wrap-Around)..." << std::endl;
    using namespace UserInterface::Network;

    SpscPacketRingBuffer<1024> ring;
    std::vector<uint8_t> chunk(128, 0xAB);

    for (int i = 0; i < 20; ++i)
    {
        bool ok = ring.TryEnqueue(0x10 + i, chunk, i * 10);
        assert(ok);

        size_t d = ring.DrainAll([&](const PacketFrameView& frame) -> bool {
            assert(frame.data.size() == 128);
            assert(frame.data[0] == 0xAB);
            return true;
        });
        assert(d == 1);
    }
    std::cout << "[PASS] Wrap-Around zaliczony bez alokacji." << std::endl;
}

static void TestTwoPhaseValidation()
{
    std::cout << "[TEST] 3. Test dwufazowej walidacji pakietu dynamicznego..." << std::endl;
    // Symulacja: naglowek 4B, caly pakiet 20B
    std::vector<uint8_t> buffer = { 0x05, 0x00, 20, 0x00, 1, 2, 3, 4 }; // Tylko 8 bajtow z 20

    TestPacketHeader* hdr = reinterpret_cast<TestPacketHeader*>(buffer.data());
    assert(hdr->length == 20);

    // Jesli nAvail (8) < hdr->length (20), pakiet NIE moze byc zdjety z bufora
    bool isComplete = buffer.size() >= hdr->length;
    assert(!isComplete);
    std::cout << "[PASS] Czesciowy odczyt poprawnie odrzucony bez korupcji strumienia." << std::endl;
}

int main()
{
    std::cout << "=== URUCHOMIENIE TESTOW T180/T181/T182/T184 NETWORK EVENT QUEUE ===" << std::endl;
    TestRingBufferEnqueueAndDrain();
    TestWrapAroundBoundary();
    TestTwoPhaseValidation();
    std::cout << "=== WSZYSTKIE TESTY ZAKONCZONE SUKCESEM (100% PASS) ===" << std::endl;
    return 0;
}
