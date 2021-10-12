package kotlin_servers

import net.openhft.affinity.AffinityLock
import net.openhft.chronicle.bytes.Bytes
//import java.nio.ByteBuffer



class Worker(val writeBuf:Bytes<Void>)  {
    private var someState = 0L

    fun doWork(newValue : Long ) {
        writeBuf.writeLong(0,newValue)
        someState = newValue
    }
}

class EventLoop(val readBuf:Bytes<Void>) {

    private var callback : ((Long)->Unit)? = null

    fun run() {
        var lastValue = 0L
        while (true) {
            lastValue = spinUntilChange(readBuf, lastValue)
            callback?.invoke(lastValue)
        }
    }

    fun setCallback(cb: (Long)->Unit) {
        callback = cb
    }

}



fun main(args: Array<String>) {

    val cpu_num = args.get(0).toInt()

    val (cliBuf, srvBuf) = mapMemory()

    val ev = EventLoop( cliBuf )

    val wk = Worker( srvBuf )

    ev.setCallback( wk::doWork )

    AffinityLock.acquireLock(cpu_num).use {
        ev.run()
    }

}