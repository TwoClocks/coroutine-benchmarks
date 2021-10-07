package kotlin_servers

import net.openhft.affinity.AffinityLock
import net.openhft.chronicle.bytes.Bytes
//import java.nio.ByteBuffer


inline fun spinUntilChange(buf: Bytes<Void>, lastValue:Long ) : Long {
    var newValue = buf.readLong(0)
    while( newValue == lastValue ) {
        newValue = buf.readLong(0)
        java.lang.Thread.onSpinWait();
    }
    return newValue
}

fun doLoop(cliBuf:Bytes<Void>, srvBuf:Bytes<Void>) {
    var lastValue = 0L
    while(true) {
        lastValue = spinUntilChange(cliBuf,lastValue)
        srvBuf.writeLong(0,lastValue)
    }
}



fun main(args: Array<String>) {

    val cpu_num = args.get(0).toInt()


    val (cliBuf, srvBuf) = mapMemory(false)

    AffinityLock.acquireLock(cpu_num).use {
        doLoop(cliBuf,srvBuf)
    }

}