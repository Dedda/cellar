package org.dedda.cellar.core

import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.extension.ExtendWith

@ExtendWith(CoreLoaderExtension::class)
internal class GridCoreTest {

    @Test
    fun settingValueCreatesRequiredStructs() {
        val gridCore = GridCore()
        val structsPostfix = "${this.javaClass.canonicalName}::settingValueCreatesRequiredStructs"
        val folderId = "Folder $structsPostfix"
        val tableId = "Table $structsPostfix"

        gridCore.putContentAt(folderId, tableId, 3, 4, "Hello there!")

        assertThat(gridCore.tableIdsForFolderId(folderId)).contains(tableId)
        assertThat(gridCore.contentAt(folderId, tableId, 3, 4)).isEqualTo("Hello there!")
    }
}