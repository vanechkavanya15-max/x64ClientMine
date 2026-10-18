#include "../../src/EterBase/Debug.h"
#include <cstdio>

namespace EterBase::Diagnostics {
    void TraceErrorOnceImpl(const std::source_location& loc, const char* format, ...) {}
    void TraceErrorImpl(const std::source_location& loc, const char* format, ...) {}
}
