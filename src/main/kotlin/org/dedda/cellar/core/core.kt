package org.dedda.cellar.core

private var coreLoaded = false

fun getLibraryPath(): String {
    return System.getProperty("user.dir") + "/core/target/debug/libcellar_core.so"
}

fun loadCore() {
    System.load(getLibraryPath())
}

external fun ping(): String
