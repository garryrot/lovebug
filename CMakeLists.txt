cmake_minimum_required(VERSION 3.21)
message("Using toolchain file ${CMAKE_TOOLCHAIN_FILE}.")

########################################################################################################################
## Define project
########################################################################################################################
project(
        Lovebug
        VERSION 0.0.1
        DESCRIPTION ""
        LANGUAGES CXX)
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_INTERPROCEDURAL_OPTIMIZATION ON)

include(GNUInstallDirs)
include(ExternalProject)

configure_file(
        ${CMAKE_CURRENT_SOURCE_DIR}/cmake/version.rc.in
        ${CMAKE_CURRENT_BINARY_DIR}/version.rc
        @ONLY)
         
set(sources
        src/Plugin.cpp
        rust/src/Tasks.cpp
        ${CMAKE_CURRENT_BINARY_DIR}/version.rc)

source_group(
        TREE ${CMAKE_CURRENT_SOURCE_DIR}
        FILES
        ${sources})

########################################################################################################################
## Rust library
########################################################################################################################

set(CARGO_MANIFEST ${CMAKE_SOURCE_DIR}/rust/Cargo.toml)
set(CARGO_TARGET_DIR ${CMAKE_SOURCE_DIR}/rust/target)
set(RUST_SOURCE_DIR ${CMAKE_SOURCE_DIR}/rust/src)
set(RUST_SOURCE_FILES ${RUST_SOURCE_DIR}/lib.rs ${RUST_SOURCE_DIR}/settings.rs ${RUST_SOURCE_DIR}/logging.rs ${RUST_SOURCE_DIR}/events.rs)
set(RUST_BRIDGE_CPP ${CARGO_TARGET_DIR}/cxxbridge/lbug/src/lib.rs.cc ${CARGO_TARGET_DIR}/cxxbridge/lbug/src/logging.rs.cc)
set(RUST_LIBRARY ${CARGO_TARGET_DIR}/release/${CMAKE_STATIC_LIBRARY_PREFIX}lovebug${CMAKE_STATIC_LIBRARY_SUFFIX})

add_custom_command(
    OUTPUT ${RUST_BRIDGE_CPP} ${RUST_LIBRARY}
    COMMAND cargo build --manifest-path ${CARGO_MANIFEST} --release
    DEPENDS ${RUST_SOURCE_FILES}
    USES_TERMINAL
    COMMENT "Running cargo..."
)

########################################################################################################################
## Configure target DLL
########################################################################################################################

find_package(CommonLibSSE CONFIG REQUIRED)

add_commonlibsse_plugin(
        ${PROJECT_NAME} 
        SOURCES 
        ${headers}
        ${sources}
        ${RUST_BRIDGE_CPP})
add_library("${PROJECT_NAME}::${PROJECT_NAME}" ALIAS "${PROJECT_NAME}")

target_include_directories(${PROJECT_NAME}
        PRIVATE
        ${CARGO_TARGET_DIR}/cxxbridge
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/rust/src>
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/src>
        $<INSTALL_INTERFACE:src>)

########################################################################################################################
## Link rust library
########################################################################################################################

# libraries that are required for linking with the rust library statically
# this list was created by trial- and error and might need to be extended
# or adapted in the future if any of the rust dependencies changes the system calls it uses. 
# If the linker complains about weird symbols missing and those symbols are windows system libraries,
# you might want to add that library here

set(STATIC_LINK_LIBS ws2_32.lib bcrypt.lib userenv.lib Advapi32.lib crypt32.lib secur32.lib Setupapi.lib ntdll.lib ncrypt.lib)
target_link_libraries(
        ${PROJECT_NAME}
        PRIVATE
        ${RUST_LIBRARY}
        ${STATIC_LINK_LIBS})

target_precompile_headers(
        ${PROJECT_NAME}
        PRIVATE
        src/PCH.h)

install(TARGETS ${PROJECT_NAME}
        DESTINATION "${CMAKE_INSTALL_LIBDIR}")

########################################################################################################################
## Copy build result to deploy/data directory
########################################################################################################################

add_custom_command(TARGET ${PROJECT_NAME} POST_BUILD
COMMAND ${CMAKE_COMMAND} -E make_directory "${CMAKE_CURRENT_SOURCE_DIR}/deploy/Data/SKSE/Plugins/")

add_custom_command(TARGET ${PROJECT_NAME} POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy $<TARGET_FILE:${PROJECT_NAME}> "${CMAKE_CURRENT_SOURCE_DIR}/deploy/Data/SKSE/Plugins/")
