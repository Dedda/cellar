package org.dedda.cellar.core

import org.assertj.core.api.Assertions
import org.junit.jupiter.api.Test

import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.extension.ExtendWith

@ExtendWith(CoreLoaderExtension::class)
internal class CoreKtTest {

    @Test
    fun rustCoreLoaded() {
        Assertions.assertThat(ping()).isEqualTo("Pong")
    }
}