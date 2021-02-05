package org.dedda.cellar.core

private var coreLoaded = false

fun loadCore() {
    System.loadLibrary("cellar-core")
}
