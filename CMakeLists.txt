if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD cargo build)
    set(TARGET_DIR "debug")
else()
    set(CARGO_CMD cargo build --release)
    set(TARGET_DIR "release")
endif()

set(DIR_SCAN_SO "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/libdir_scan.so")

add_custom_target(dir_scan ALL
    COMMENT "Compiling dir_scan module"
    COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} ${CARGO_CMD}
    COMMAND cp ${DIR_SCAN_SO} ${CMAKE_CURRENT_BINARY_DIR}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
set_target_properties(dir_scan PROPERTIES LOCATION ${CMAKE_CURRENT_BINARY_DIR})

add_test (NAME dir_scan_test
    COMMAND cargo test
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
