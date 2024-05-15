cmake_minimum_required(VERSION 3.21)
project( Lovebug 
         VERSION 0.0.1 
         DESCRIPTION "" 
         LANGUAGES CXX)
set(CMAKE_CXX_STANDARD 23)                      # TODO is 23 really required?
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_INTERPROCEDURAL_OPTIMIZATION ON)

set(GameVersion "Skyrim")

include(GNUInstallDirs)
include(ExternalProject)

configure_file(
        ${CMAKE_CURRENT_SOURCE_DIR}/cmake/version.rc.in
        ${CMAKE_CURRENT_BINARY_DIR}/version.rc
        @ONLY)

set(BUILD_TESTS OFF)
        
set(sources
        src/Plugin.cpp
        rust/src/Bridge.cpp
        ${CMAKE_CURRENT_BINARY_DIR}/version.rc)

source_group(
        TREE ${CMAKE_CURRENT_SOURCE_DIR}
        FILES
        ${sources})

########################################################################################################################
## Rust library
########################################################################################################################

set(RUST_LIBRARY ${CMAKE_SOURCE_DIR}/rust/target/release/${CMAKE_STATIC_LIBRARY_PREFIX}lovebug${CMAKE_STATIC_LIBRARY_SUFFIX})

set(RUST_CXX_BRIDGE_CPP ${CMAKE_SOURCE_DIR}/rust/target/cxxbridge/lbug/src/lib.rs.cc 
                        ${CMAKE_SOURCE_DIR}/rust/target/cxxbridge/lbug/src/logging.rs.cc)
add_custom_command(
    OUTPUT ${RUST_BRIDGE_CPP} ${RUST_LIBRARY}
    COMMAND cargo build --manifest-path ${CMAKE_SOURCE_DIR}/rust/Cargo.toml --release
    DEPENDS ${CMAKE_SOURCE_DIR}/rust/src/config.rs
            ${CMAKE_SOURCE_DIR}/rust/src/events.rs
            ${CMAKE_SOURCE_DIR}/rust/src/lib.rs
            ${CMAKE_SOURCE_DIR}/rust/src/logging.rs
            ${CMAKE_SOURCE_DIR}/rust/src/settings.rs
    USES_TERMINAL
    COMMENT "Running cargo..."
)


########################################################################################################################
## Configure target DLL
########################################################################################################################

# TODO Required? What does it do?
add_compile_definitions(SKYRIM)
add_subdirectory("extern/CommonLibSSE-NG" "CommonLibSSE" EXCLUDE_FROM_ALL)

include(extern/CommonLibSSE-NG/cmake/CommonLibSSE.cmake)
add_commonlibsse_plugin(
        ${PROJECT_NAME} 
        SOURCES 
        ${headers}
        ${sources}
        ${RUST_CXX_BRIDGE_CPP})

target_include_directories(${PROJECT_NAME}
        PRIVATE
        ${CMAKE_SOURCE_DIR}/rust/target/cxxbridge
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/rust/src>
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/src>
        $<INSTALL_INTERFACE:src>)     

########################################################################################################################
## Link rust library
########################################################################################################################

# libraries that are required for linking with the rust library 
# statically this list was created by trial- and error
target_link_libraries(
        ${PROJECT_NAME}
        PRIVATE
        ${RUST_LIBRARY}
        ws2_32.lib 
        bcrypt.lib 
        userenv.lib 
        Advapi32.lib 
        crypt32.lib 
        secur32.lib 
        Setupapi.lib 
        ntdll.lib 
        ncrypt.lib)

target_precompile_headers(
        ${PROJECT_NAME}
        PRIVATE
        src/PCH.h)

# TODO remove?
install(TARGETS ${PROJECT_NAME}
        DESTINATION "${CMAKE_INSTALL_LIBDIR}")

########################################################################################################################
## Copy build result to deploy/data directory
########################################################################################################################

add_custom_command(TARGET ${PROJECT_NAME} POST_BUILD
COMMAND ${CMAKE_COMMAND} -E make_directory "${CMAKE_CURRENT_SOURCE_DIR}/deploy/Data/SKSE/Plugins/")

add_custom_command(TARGET ${PROJECT_NAME} POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy $<TARGET_FILE:${PROJECT_NAME}> "${CMAKE_CURRENT_SOURCE_DIR}/deploy/Data/SKSE/Plugins/")
