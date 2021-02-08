package org.dedda.cellar.core

class GridCore {

    external fun contentAt(folderId: String, tableId: String, x: Int, y: Int): String

    external fun renderedAt(folderId: String, tableId: String, x: Int, y: Int): String

    external fun tableIdsForFolderId(folderId: String): Array<String>

    external fun putContentAt(folderId: String, tableId: String, x: Int, y: Int, content: String)
}