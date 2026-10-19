#pragma once
#include <cmath>

struct D3DXVECTOR3 {
    float x, y, z;
};

struct D3DXVECTOR4 {
    float x, y, z, w;
};

struct D3DXMATRIX {
    union {
        struct {
            float        _11, _12, _13, _14;
            float        _21, _22, _23, _24;
            float        _31, _32, _33, _34;
            float        _41, _42, _43, _44;
        };
        float m[4][4];
    };
};

struct D3DXQUATERNION {
    float x, y, z, w;
};
struct D3DCOLORVALUE {
    float r, g, b, a;
};
struct D3DXCOLOR {
    float r, g, b, a;
};

typedef unsigned long DWORD;
typedef unsigned short WORD;
typedef unsigned char BYTE;
typedef int BOOL;
typedef unsigned int UINT;
#define TRUE 1
#define FALSE 0
