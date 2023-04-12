package kotlin_servers
import kotlin_servers.setupMemory
import kotlin_servers.CoroutineSuspend
import kotlin_servers.CoroutineTask

class SuspendEventLoop(val mem:Memory) {
    private val suspendPoint = CoroutineSuspend<Unit>()
    private var value = 0L

    suspend fun asyncLoop() {
        println("as starting")
        while(true) {
            value = mem.spinUntilChange( value )
            suspendPoint.suspendMe();
        }
    }

    fun run() {
        while(true) {
            mem.write(value);
            suspendPoint.resume( Unit )
        }
    }
}




fun main() {

    val loop = SuspendEventLoop(setupMemory())

    // start it running.
    CoroutineTask(loop::asyncLoop).start()

    loop.run()

}