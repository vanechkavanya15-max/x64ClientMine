#pragma once
#define D3DXMATRIX struct DummyMatrix { float m[16]; }
#define D3DXVECTOR2 struct DummyVec2 { float v[2]; }
#define D3DXVECTOR3 struct DummyVec3 { float v[3]; }
#define D3DXVECTOR4 struct DummyVec4 { float v[4]; }
#define D3DXCOLOR struct DummyColor { float v[4]; }
struct RECT { long left, top, right, bottom; };
#define D3DTEXF_POINT 1
#define D3DTADDRESS_CLAMP 3
#define D3DTOP_SELECTARG1 4
#define D3DTOP_MODULATE 4
