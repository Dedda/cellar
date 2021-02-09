package org.dedda.cellar.core

import org.apache.commons.lang3.SystemUtils
import java.io.File
import java.lang.RuntimeException

private var coreLoaded = false

fun getLibraryPath(): String {
    return System.getProperty("user.dir") + "/core/target/debug/".replace('/', File.separatorChar) +
    when (true) {
        SystemUtils.IS_OS_WINDOWS -> "cellar_core.dll"
        SystemUtils.IS_OS_LINUX -> "libcellar_core.so"
        else -> "UNKNOWN OS"
    }

}

fun loadCore() {
    synchronized(coreLoaded) {
        if (!coreLoaded) {
            coreLoaded = true
            print("Loading native functions... ")
            System.load(getLibraryPath())
            if (ping() != "Pong") {
                throw RuntimeException("Something went wrong in the core library :/")
            }
            println("OK")
        }
    }
}

external fun ping(): String
