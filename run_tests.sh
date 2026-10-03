export CXX="clang++"
export CC="clang"
export AR="llvm-ar"
export NM="llvm-nm"
export RANLIB="llvm-ranlib"
export LD="lld-link"

export CFLAGS_x86_64_pc_windows_msvc="-Wno-macro-redefined"
export CXXFLAGS_x86_64_pc_windows_msvc="-Wno-macro-redefined"

cargo xwin test --package m2-client --test test_full-audio-rodio_02 --target x86_64-pc-windows-msvc
