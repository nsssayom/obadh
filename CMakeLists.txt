cmake_minimum_required(VERSION 3.10)
project(obadh VERSION 1.0)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

include_directories(include)

add_executable(obadh src/main.cpp src/Transliterator.cpp)
