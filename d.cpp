#include <iostream>
#include <cstdint>
#include <cstddef>

#define RENDER_ABI_ALIGN(n) alignas(n)
typedef struct DummyMatrix { float m[16]; } DummyMatrix;
typedef struct DummyColor { float r,g,b,a; } DummyColor;
typedef struct DummyVec3 { float x,y,z; } DummyVec3;
typedef struct DummyVec4 { float x,y,z,w; } DummyVec4;

struct TPNT2Vertex { float f[10]; };

typedef struct RENDER_ABI_ALIGN(16) TWaterPacket
{
    float fWorldPosX; // 4
    float fWorldPosY; // 4 (8)
    uint32_t dwSectorX; // 4 (12)
    uint32_t dwSectorY; // 4 (16)
    uint16_t sectorX; // 2
    uint16_t sectorY; // 2 (20)
    float fWaterHeight; // 4 (24)
    float fWaveTime; // 4 (28)
    float fWaveFrequency; // 4 (32)
    float fWaveAmplitude; // 4 (36)
    DummyColor vWaterColor; // 16 (52)
    float fAlpha; // 4 (56)
    uint32_t dwWaterTextureID; // 4 (60)
    uint32_t dwPadTexture; // 4 (64)
    union
    {
        void* pWaterTexture;
        uint32_t dwWaterTextureHandle;
    }; // 8 (72)
    uint32_t dwCellCount; // 4 (76)
    uint32_t dwVertexCount; // 4 (80)
    void* pVertexBuffer; // 8 (88)
    uint32_t dwFaceCount; // 4 (92)
    uint32_t patchNum; // 4 (96)
    uint8_t bFarPass; // 1
    uint8_t bAlphaBlend; // 1
    uint8_t bPadWater[2]; // 2 (100)
    float fFogDistance; // 4 (104)
    float fPad; // 4 (108)
    uint32_t dwPadAlign16; // 4 (112)
    union
    {
        DummyMatrix matTexTransformWater;
        DummyMatrix matUV;
    }; // 64 (176)
} TWaterPacket;

int main() {
    std::cout << "TWaterPacket: " << sizeof(TWaterPacket) << std::endl;
}
