#include <windows.h>
#include <d3d9.h>

void f() {
    IDirect3DDevice9Ex* device;
    D3DPRESENT_PARAMETERS pp;
    D3DDISPLAYMODEEX mode;
    device->ResetEx(&pp, &mode);
}
