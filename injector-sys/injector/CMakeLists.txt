cmake_minimum_required(VERSION 3.12)
project(injector C)

file(GLOB_RECURSE files ./*.h ./*.c)
add_library(STATIC ${files} include/injector.h)
