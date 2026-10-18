#include <iostream>
#include <cstddef>
#include <cstdint>

// Fake D3DX structs
struct DummyMatrix { float m[16]; };
struct DummyVec2 { float v[2]; };
struct DummyVec3 { float v[3]; };
struct DummyVec4 { float v[4]; };
struct DummyColor { float v[4]; };
struct DummyRect { int32_t left, top, right, bottom; };

#define RENDER_MATRIX_TYPE DummyMatrix
#define RENDER_VEC2_TYPE DummyVec2
#define RENDER_VEC3_TYPE DummyVec3
#define RENDER_VEC4_TYPE DummyVec4
#define RENDER_COLOR4_TYPE DummyColor
#define RENDER_RECT_TYPE DummyRect

#define RENDER_ABI_C_ONLY 1
#include "src/EterLib/RenderAbi.h"

int main() {
    return 0;
}
