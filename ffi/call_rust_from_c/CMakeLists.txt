cmake_minimum_required(VERSION 3.16)
project(call_rust_from_c_example
    LANGUAGES C
)

add_executable(call_rust_from_c_example
    main.c
)
target_include_directories(call_rust_from_c_example PRIVATE 
    .
)
target_link_directories(call_rust_from_c_example PRIVATE 
    my_rust_lib/target/release
)
target_link_libraries(call_rust_from_c_example PRIVATE
    my_rust_lib
)

if (${UNIX})
message("Adding UNIX specific libraries for statically linking rust libraries to C executable")
target_link_libraries(call_rust_from_c_example PRIVATE
    pthread
    dl
)
endif()

if (${WIN32})
message("Adding WIN32 specific libraries for statically linking rust libraries to C executable")
target_link_libraries(call_rust_from_c_example PRIVATE
    ws2_32
    bcrypt
    userenv
)
endif()

add_dependencies(call_rust_from_c_example compile_rust_lib)

add_custom_target(compile_rust_lib
    COMMAND cbindgen --config cbindgen.toml --output my_header.h --lang c
    COMMAND cargo build --release
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/my_rust_lib
)
