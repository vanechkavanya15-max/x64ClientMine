#!/bin/bash
cat src/PRTerrainLib/Terrain.h | grep -E "TERRAIN_SIZE|HEIGHTMAP_RAW_XSIZE|CELLSCALE"
