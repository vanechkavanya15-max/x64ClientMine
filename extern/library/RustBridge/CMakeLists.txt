add_library(RustBridge INTERFACE)
target_include_directories(RustBridge INTERFACE "${CMAKE_SOURCE_DIR}/src")
target_link_libraries(RustBridge INTERFACE "${CMAKE_CURRENT_LIST_DIR}/m2_client_bridge.lib")
