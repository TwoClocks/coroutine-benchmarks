package kotlin_servers

import net.openhft.affinity.AffinityLock
import net.openhft.chronicle.bytes.Bytes


class SuspendEventLoop(val readBuf: Bytes<Void>, val writeBuf:Bytes<Void>) {
    private val suspendPoint = CoroutineSuspend<Unit>()
    private var value = 0L

    suspend fun asyncLoop() {
        println("as starting")
        var lastValue = value
        while(true) {
            while( lastValue == value ) {
                java.lang.Thread.onSpinWait();
                value = readBuf.readLong(0)
            }
            suspendPoint.suspendMe();
            lastValue = value
        }
    }

    fun run() {
        while(true) {
            writeBuf.writeLong(0,value);
            suspendPoint.resume( Unit )
        }
    }
}




fun main(args: Array<String>) {

    val cpu_num = args.get(0).toInt()

    val (cliBuf, srvBuf) = mapMemory()

    val loop = SuspendEventLoop(cliBuf,srvBuf)

    AffinityLock.acquireLock(cpu_num).use {

        // start it running.
        CoroutineTask(loop::asyncLoop).start()

        loop.run()

    }
}