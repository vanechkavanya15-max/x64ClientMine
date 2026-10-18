sed -i 's/#include <windows.h>/#ifdef _WIN32\n#include <windows.h>\n#endif/g' src/EterBase/Debug.h
