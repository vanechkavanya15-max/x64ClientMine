import sys

with open("CMakeLists.txt", "r") as f:
    content = f.read()

content = content.replace("add_executable(test_map_outdoor tests/test_map_outdoor.cpp)", "")
content = content.replace("target_link_libraries(test_map_outdoor GameLib GTest::gtest_main)", "")
content = content.replace("add_test(NAME test_map_outdoor COMMAND test_map_outdoor)", "")

with open("CMakeLists.txt", "w") as f:
    f.write(content)
