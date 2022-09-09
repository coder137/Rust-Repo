cmake_minimum_required(VERSION 3.16.0)

project(c_lib)

add_library(c_lib STATIC
    c_lib.c
)
target_include_directories(c_lib PRIVATE .)
