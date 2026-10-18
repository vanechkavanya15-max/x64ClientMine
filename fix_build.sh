sed -i 's/stdafx.h/StdAfx.h/g' src/SphereLib/StdAfx.cpp
sed -i 's/Stdafx.h/StdAfx.h/g' src/SphereLib/frustum.cpp
sed -i 's/builds\/unix\/ftsystem.c/src\/base\/ftsystem.c/g' vendor/freetype-2.13.3/CMakeLists.txt
sed -i 's/#include <objbase.h>/\/\/#include <objbase.h>/g' extern/include/d3d9.h
sed -i 's/#include <windows.h>/\/\/#include <windows.h>/g' extern/include/d3d9.h
