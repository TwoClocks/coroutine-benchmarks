package kotlin_servers

import net.openhft.affinity.AffinityLock
import net.openhft.chronicle.bytes.Bytes


class SpinEventLoop( val readBuf: Bytes<Void>) {
    private val suspendPoint = CoroutineSuspend<Long>()
    private var lastValue = 0L

    suspend fun getNextValue( currentValue:Long ) : Long {

        lastValue = currentValue

        return suspendPoint.suspendMe()

    }

    fun spinLoop() {
        var value = readBuf.readLong(0)
        while(true) {
            while(value == lastValue) {
                java.lang.Thread.onSpinWait();
                value = readBuf.readLong(0)
            }
            suspendPoint.resume( value )
        }
    }


}




fun main(args: Array<String>) {

    val cpu_num = args.get(0).toInt()

    val (cliBuf, srvBuf) = mapMemory(false)

    val loop = SpinEventLoop(cliBuf)

    AffinityLock.acquireLock(cpu_num).use {
//        it.bind(false)


        val mainLoop = suspend {
            var lastValue = 0L
            while(true) {
                lastValue = loop.getNextValue(lastValue)
                srvBuf.writeLong(0,lastValue)
            }
        }

        // start it running.
        CoroutineTask(mainLoop).start()

        loop.spinLoop()

    }
}