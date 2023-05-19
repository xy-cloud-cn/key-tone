cmake_minimum_required(VERSION 3.25)

project(key_tone)

set(CMAKE_CXX_STANDARD 14)

add_executable(key_tone
        main.cpp)

target_link_libraries(key_tone winmm)