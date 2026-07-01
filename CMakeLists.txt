cmake_minimum_required(VERSION 3.15)
project(GuitarPedalboard VERSION 1.0.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# add source files to compile here
add_executable(guitar_pedalboard apps/main.cpp src/PedalBoard.cpp)

target_include_directories(guitar_pedalboard PRIVATE include)

if(APPLE)
    find_library(CORE_AUDIO_FRAMEWORK CoreAudio REQUIRED)
    find_library(AUDIO_TOOLBOX_FRAMEWORK AudioToolbox REQUIRED)

    target_link_libraries(guitar_pedalboard PRIVATE 
        ${CORE_AUDIO_FRAMEWORK}
        ${AUDIO_TOOLBOX_FRAMEWORK}
    )

    target_compile_options(guitar_pedalboard PRIVATE -O3)
endif()