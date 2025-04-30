# CMakeLists.txt for Enigma JNI bindings
cmake_minimum_required(VERSION 3.10)

project(enigma_bindings)

# Ne compile rien ici, on utilise juste le .so précompilé de Rust
# On indique uniquement où le système doit le chercher

# Chemin vers les bibliothèques Rust compilées (libenigma.so)
set(LIB_PATH "${CMAKE_CURRENT_SOURCE_DIR}/libs")

add_library(
    enigma
    SHARED
    IMPORTED
)

set_target_properties(
    enigma
    PROPERTIES
    IMPORTED_LOCATION
    "${LIB_PATH}/${ANDROID_ABI}/libenigma.so"
)
