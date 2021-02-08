package org.dedda.cellar.core

import org.dedda.cellar.func.CompileException

class FunctionCore {

    @Throws(CompileException::class)
    fun compile(source: String): String {
        val split = _compile(source).split(";")
        if (split[1].isNotEmpty()) {
            throw CompileException(split[1])
        }
        return split[0]
    }

    private external fun _compile(source: String): String

    external fun delete(functionId: String)

    external fun listIds(): Array<String>
}