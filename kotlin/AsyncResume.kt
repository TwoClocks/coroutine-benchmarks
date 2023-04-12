package kotlin_servers
import kotlin_servers.setupMemory
import kotlin_servers.CoroutineSuspend
import kotlin_servers.CoroutineTask


class ResumeEventLoop(val mem:Memory) {

    private val suspendPoint = CoroutineSuspend<Long>()

    suspend fun asyncLoop() {
        var value:Long
        while(true) {
            value = suspendPoint.suspendMe()
            mem.write(value)
        }

    }

    fun run() {
        var value = 0L;
        while(true) {
            value = mem.spinUntilChange(value)
            suspendPoint.resume( value )
        }
    }
}

fun main() {

    val loop = ResumeEventLoop(setupMemory())

    // start it running.
    CoroutineTask(loop::asyncLoop).start()

    loop.run()

}