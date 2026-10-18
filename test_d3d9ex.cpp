#include <d3d9.h>
void test() {
    IDirect3DDevice9Ex* dev = nullptr;
    dev->CheckDeviceState(nullptr);
    D3DPRESENT_PARAMETERS pp = {0};
    dev->ResetEx(&pp, nullptr);
}
