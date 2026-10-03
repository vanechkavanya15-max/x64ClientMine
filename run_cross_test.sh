#!/bin/bash
export PYO3_CROSS_PYTHON_VERSION=3.10
export PYO3_CROSS_LIB_DIR=/usr/lib
cargo xwin test --package m2-client --test test_full-audio-rodio_02 --target x86_64-pc-windows-msvc > cross_test.log 2>&1
