#include <iostream>
#include <cstddef>

#define RENDER_MATRIX_TYPE struct { float m[16]; }
#define RENDER_VEC2_TYPE struct { float v[2]; }
#define RENDER_VEC3_TYPE struct { float v[3]; }
#define RENDER_VEC4_TYPE struct { float v[4]; }
#define RENDER_COLOR4_TYPE struct { float v[4]; }
#define RENDER_RECT_TYPE struct { long left, top, right, bottom; }
#define RENDER_TREE_WRAPPER_TYPE void

#include "src/EterLib/RenderAbi.h"

int main() {
    std::cout << "TPNT2Vertex: " << sizeof(TPNT2Vertex) << std::endl;
    std::cout << "TRenderFrameUniforms: " << sizeof(TRenderFrameUniforms) << std::endl;
    std::cout << "TActorRenderPacket: " << sizeof(TActorRenderPacket) << std::endl;
    std::cout << "TUIBatchPacket: " << sizeof(TUIBatchPacket) << std::endl;
    std::cout << "TRenderFramePacket: " << sizeof(TRenderFramePacket) << std::endl;
    return 0;
}
