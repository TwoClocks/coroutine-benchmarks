package kotlin_servers

import java.io.IOException
import kotlin.coroutines.Continuation
import kotlin.coroutines.CoroutineContext
import kotlin.coroutines.EmptyCoroutineContext
import kotlin.coroutines.resume
import kotlin.coroutines.resumeWithException
import kotlin.coroutines.startCoroutine
import kotlin.coroutines.suspendCoroutine

class CoroutineSuspend<T> {
    private var context: Continuation<T>? = null

    // swap out the context/stack for the current running code
    // and let the scheduler run a different one.
    suspend fun suspendMe(): T {
        if (context != null) {
            val exp = IOException("Co-routine trying to re-enter itself.")
            println({ exp.stackTrace.map { "\n\t" + it.classLoaderName.toString() }.reduce { acc, string -> acc + string } })
            this.resumeWithException(exp)
            throw exp
        }

        // Compiler does magic here....
        // stash the context/stack and go
        // do something else for a bit.
        return suspendCoroutine<T> { context = it }
    }

    // lets spin this "thread" back up
    // and continue where we left off!
    fun resume(value: T) {
        val tmp = context
        context = null
        tmp?.resume(value)
    }

    fun resumeWithException(exp: Throwable) {
        val tmp = context
        context = null
        tmp?.resumeWithException(exp)
    }

    fun isSuspended() = context != null
}

/**
 * container for launching a new coroutine/fiber
 * and getting info about it from the outside.
 */
class CoroutineTask(val block: suspend () -> Unit) {

    private var isRunning = false

    fun isRunning() = isRunning

    fun start(): CoroutineTask {
        isRunning = true
        block.startCoroutine(object : Continuation<Unit> {

            override val context: CoroutineContext = EmptyCoroutineContext

            // called when lambda / coroutine ends...
            // do nothing...
            override fun resumeWith(result: Result<Unit>) {
                isRunning = false
                result.onFailure { ex ->
                    // we shouldn't be exiting coroutines by
                    // letting a uncaught exception go all
                    // the way up the stack. print some info and exit.
                    println("Coroutine exiting from uncaught exception : \n\t$ex")
                    println(ex)
                }
            }
        })
        return this
    }
}
