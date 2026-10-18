cmake --build build --config Release --target test_c26_culling_manager -- /m:1
ctest --test-dir build -C Release -R CullingManagerTest --output-on-failure
