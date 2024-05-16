cmake_minimum_required(VERSION 3.21)

project(Lovebug 
        VERSION 0.0.1 
        DESCRIPTION "Enter some description"
        LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)   
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_INTERPROCEDURAL_OPTIMIZATION ON)

set(DLL_OUTPUT_DIR ${CMAKE_CURRENT_SOURCE_DIR}/deploy/Data/SKSE/Plugins/)

# Rust

set(RUST_PKG_NAME "lbug")
set(RUST_LIB_NAME "lovebug")

set(RUST_LIBRARY ${CMAKE_CURRENT_SOURCE_DIR}/rust/target/release/${CMAKE_STATIC_LIBRARY_PREFIX}${RUST_LIB_NAME}${CMAKE_STATIC_LIBRARY_SUFFIX})
set(RUST_CXX_BRIDGE ${CMAKE_CURRENT_SOURCE_DIR}/rust/target/cxxbridge/${RUST_PKG_NAME}/src/lib.rs.cc 
                    ${CMAKE_CURRENT_SOURCE_DIR}/rust/target/cxxbridge/${RUST_PKG_NAME}/src/logging.rs.cc)

add_custom_command(
    OUTPUT ${RUST_CXX_BRIDGE} ${RUST_LIBRARY}
    COMMAND cargo build --manifest-path ${CMAKE_CURRENT_SOURCE_DIR}/rust/Cargo.toml --release
    DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/rust/src/config.rs
            ${CMAKE_CURRENT_SOURCE_DIR}/rust/src/events.rs
            ${CMAKE_CURRENT_SOURCE_DIR}/rust/src/lib.rs
            ${CMAKE_CURRENT_SOURCE_DIR}/rust/src/logging.rs
            ${CMAKE_CURRENT_SOURCE_DIR}/rust/src/settings.rs
    USES_TERMINAL
    COMMENT "Running cargo..."
)

# CLib Plugin

set(BUILD_TESTS OFF)
include(extern/CommonLibSSE-NG/cmake/CommonLibSSE.cmake)
add_subdirectory(extern/CommonLibSSE-NG CommonLibSSE EXCLUDE_FROM_ALL)
configure_file(
        ${CMAKE_CURRENT_SOURCE_DIR}/version.rc.in 
        ${CMAKE_CURRENT_BINARY_DIR}/version.rc 
        @ONLY)

add_commonlibsse_plugin(  
        ${PROJECT_NAME} 
        SOURCES
        ${CMAKE_CURRENT_BINARY_DIR}/version.rc
        ${CMAKE_CURRENT_SOURCE_DIR}/src/Plugin.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/rust/src/Bridge.cpp
        ${RUST_CXX_BRIDGE})

target_precompile_headers(
        ${PROJECT_NAME} 
        PRIVATE 
        src/PCH.h)

target_include_directories(
        ${PROJECT_NAME}
        PRIVATE
        ${CMAKE_CURRENT_SOURCE_DIR}/src
        ${CMAKE_CURRENT_SOURCE_DIR}/rust/src
        ${CMAKE_CURRENT_SOURCE_DIR}/rust/target/cxxbridge)     

target_link_libraries(
        ${PROJECT_NAME}
        PRIVATE
        ${RUST_LIBRARY}
        wsock32 ws2_32 bcrypt ntdll userenv)

# Copy dll to output dir

add_custom_command(
        TARGET ${PROJECT_NAME} POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E make_directory "${DLL_OUTPUT_DIR}")

add_custom_command(
        TARGET ${PROJECT_NAME} POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy $<TARGET_FILE:${PROJECT_NAME}> "${DLL_OUTPUT_DIR}")
