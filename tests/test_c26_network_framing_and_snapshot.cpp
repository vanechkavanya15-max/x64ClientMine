#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "EterLib/CFrameSnapshot.h"
#include "UserInterface/Network/NetworkEventQueue.h"
#include <vector>
#include <span>

TEST_CASE("T185: TImmutableFrameSnapshot POD Compliance and Layout")
{
    static_assert(sizeof(TImmutableFrameSnapshot) == 640, "Size must be 640 bytes");
    static_assert(alignof(TImmutableFrameSnapshot) == 16, "Alignment must be 16 bytes");
    static_assert(std::is_trivial_v<TImmutableFrameSnapshot>, "Must be strictly trivial");
    static_assert(std::is_standard_layout_v<TImmutableFrameSnapshot>, "Must have standard layout");

    TImmutableFrameSnapshot snap{};
    CHECK(snap.dwFrameIndex == 0);
    CHECK(snap.uSequenceNumber == 0);

    // Matrix conversion
    snap.matView._11 = 2.0f;
    snap.matView._44 = 1.0f;
    const D3DXMATRIX& d3dMat = snap.GetViewMatrix();
    CHECK(d3dMat._11 == 2.0f);
    CHECK(d3dMat._44 == 1.0f);

    // Camera pos conversion
    snap.v3CameraPos.x = 100.0f;
    snap.v3CameraPos.y = 200.0f;
    snap.v3CameraPos.z = 300.0f;
    D3DXVECTOR3 d3dVec = snap.v3CameraPos;
    CHECK(d3dVec.x == 100.0f);
    CHECK(d3dVec.y == 200.0f);
    CHECK(d3dVec.z == 300.0f);
}

TEST_CASE("T181/T182: SpscPacketRingBuffer Enqueue, Wrap, and Drain FIFO")
{
    using namespace UserInterface::Network;
    SpscPacketRingBuffer<64 * 1024> ringBuf; // 64 KB test ring

    CHECK(!ringBuf.HasPacket());

    std::vector<uint8_t> payload1 = { 0xAA, 0xBB, 0xCC, 0xDD, 0xEE };
    std::vector<uint8_t> payload2 = { 0x11, 0x22, 0x33, 0x44 };

    bool enq1 = ringBuf.TryEnqueue(0x0100, payload1, 1000);
    CHECK(enq1);
    CHECK(ringBuf.HasPacket());

    bool enq2 = ringBuf.TryEnqueue(0x0200, payload2, 1016);
    CHECK(enq2);

    std::vector<uint16_t> drainedOpcodes;
    std::vector<std::vector<uint8_t>> drainedPayloads;

    size_t count = ringBuf.DrainAll([&](const PacketFrameView& frame) -> bool {
        drainedOpcodes.push_back(frame.meta.opcode);
        drainedPayloads.emplace_back(frame.data.begin(), frame.data.end());
        return true;
    });

    CHECK(count == 2);
    CHECK(drainedOpcodes.size() == 2);
    CHECK(drainedOpcodes[0] == 0x0100);
    CHECK(drainedOpcodes[1] == 0x0200);
    CHECK(drainedPayloads[0] == payload1);
    CHECK(drainedPayloads[1] == payload2);
    CHECK(!ringBuf.HasPacket());
}
