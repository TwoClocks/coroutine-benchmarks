package kotlin_servers

import net.openhft.affinity.AffinityLock
import net.openhft.chronicle.bytes.Bytes


class ResumeEventLoop( val readBuf: Bytes<Void>, val writeBuf:Bytes<Void>) {

    private val suspendPoint = CoroutineSuspend<Long>()

    suspend fun asyncLoop() {
        var value = 0L
        while(true) {
            value = suspendPoint.suspendMe()
            writeBuf.writeLong(0,value)
        }

    }

    fun run() {
        var nextValue = 0L;
        var lastValue = 0L;
        while(true) {
            while(nextValue == lastValue) {
                java.lang.Thread.onSpinWait();
                nextValue = readBuf.readLong(0)
            }
            suspendPoint.resume( nextValue )
            lastValue = nextValue
        }
    }
}




fun main(args: Array<String>) {

    val cpu_num = args.get(0).toInt()

    val (cliBuf, srvBuf) = mapMemory()

    val loop = ResumeEventLoop(cliBuf,srvBuf)

    AffinityLock.acquireLock(cpu_num).use {


        // start it running.
        CoroutineTask(loop::asyncLoop).start()

        loop.run()

    }
}