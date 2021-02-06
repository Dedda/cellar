package org.dedda.cellar.core

import org.junit.jupiter.api.extension.BeforeAllCallback
import org.junit.jupiter.api.extension.ExtensionContext

class CoreLoaderExtension: BeforeAllCallback {

    override fun beforeAll(context: ExtensionContext?) {
        if (!loaded) {
            loaded = true
            loadCore()
        }
    }

    companion object {
        private var loaded = false
    }
}