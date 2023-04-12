package kotlin_servers
import kotlin_servers.setupMemory
import kotlin_servers.CoroutineSuspend
import kotlin_servers.CoroutineTask



class Worker(val mem:Memory) {
    private var someState = 0L

    fun doWork(newValue : Long ) {
        mem.write(newValue)
        someState = newValue
    }
}

class EventLoop(val mem:Memory) {

    private var callback : ((Long)->Unit)? = null

    fun run() {
        var lastValue = 0L
        while (true) {
            lastValue = mem.spinUntilChange(lastValue)
            callback?.invoke(lastValue)
        }
    }

    fun setCallback(cb: (Long)->Unit) {
        callback = cb
    }

}



fun main() {

    val mem=setupMemory()

    val ev = EventLoop( mem )

    val wk = Worker( mem )

    ev.setCallback( wk::doWork )

    ev.run()

}