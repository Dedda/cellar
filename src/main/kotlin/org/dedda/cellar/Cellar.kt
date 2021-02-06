package org.dedda.cellar

import org.dedda.cellar.core.loadCore

fun main(args: Array<String>) {
    Cellar.initialize()
}

object Cellar {
    fun initialize() {
        loadCore()
    }
}
