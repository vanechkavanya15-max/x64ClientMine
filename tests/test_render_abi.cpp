#include <iostream>
#include <cstddef>
#include <cstdint>

#define RENDER_MATRIX_TYPE struct DummyMatrix { float m[16]; }
#define RENDER_VEC2_TYPE struct DummyVec2 { float v[2]; }
#define RENDER_VEC3_TYPE struct DummyVec3 { float v[3]; }
#define RENDER_VEC4_TYPE struct DummyVec4 { float v[4]; }
#define RENDER_COLOR4_TYPE struct DummyColor { float v[4]; }
#define RENDER_RECT_TYPE struct DummyRect { long left, top, right, bottom; }
#define RENDER_TREE_WRAPPER_TYPE void

#include "../src/EterLib/RenderAbi.h"

int main() {
    std::cout << sizeof(TUIBatchPacket) << std::endl;
}
