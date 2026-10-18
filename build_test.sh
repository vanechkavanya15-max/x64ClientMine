#!/bin/bash
g++ -std=c++23 -I./src -I./extern/include -I./tests tests/test_c37_buffer_pool_audit.cpp src/EterLib/BufferPool.cpp -o buffer_pool_test && ./buffer_pool_test
