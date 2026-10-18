#pragma once
#include <d3d9.h>
struct D3DXMATRIX {
    float m[4][4];
};
void D3DXMatrixScaling(D3DXMATRIX* pOut, float sx, float sy, float sz);
void D3DXMatrixMultiply(D3DXMATRIX* pOut, const D3DXMATRIX* pM1, const D3DXMATRIX* pM2);
