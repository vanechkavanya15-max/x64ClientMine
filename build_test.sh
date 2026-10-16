#!/bin/bash
g++ -std=c++23 tests/test_c26_harness_world_query.cpp src/Client/IPC/HarnessWorldQueryRouter.cpp -I./src -I./extern/include -DTEST_MODE_DISABLE_STDAFX=1 -o test_bin
./test_bin
