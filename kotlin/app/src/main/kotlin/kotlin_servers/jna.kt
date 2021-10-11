package kotlin_servers

import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.platform.linux.Fcntl
import com.sun.jna.platform.linux.Mman
import com.sun.jna.platform.unix.LibCUtil.ftruncate
import com.sun.jna.platform.unix.LibCUtil.mmap
import net.openhft.chronicle.bytes.Bytes
import net.openhft.chronicle.bytes.PointerBytesStore
import net.openhft.chronicle.core.OS
import java.io.IOException


fun mapMemory(): Pair<Bytes<Void>, Bytes<Void>> {

    val lrt = com.sun.jna.platform.linux.LibRT.INSTANCE


    val shm = lrt.shm_open(
        "/spinnmem",
        Fcntl.O_RDWR,
        Fcntl.S_IRUSR or Fcntl.S_IWUSR or Fcntl.S_IRGRP or Fcntl.S_IWGRP
    )

    if ( shm < 0) throw IOException("shm_open failed erno : ${Native.getLastError()}")

    if( ftruncate(shm, OS.pageSize().toLong()) < 0 ) throw IOException("ftruncate failed erno : ${Native.getLastError()}")


    val mem_ptr = mmap(
        null,
        OS.pageSize().toLong(),
        Mman.PROT_READ or Mman.PROT_WRITE,
        Mman.MAP_SHARED,
        shm,
        0
    )
    if( mem_ptr == Mman.MAP_FAILED ) throw IOException("mmap failed erno : ${Native.getLastError()}")


    val raw_ptr = Pointer.nativeValue( mem_ptr )

    println("pointer at ${raw_ptr.toULong().toString(16)}")

    val buf_one = PointerBytesStore()
    buf_one.set( raw_ptr, 8 )


    val buf_two = PointerBytesStore()
    buf_two.set( raw_ptr+2048, 8 )

    val first = buf_one.bytesForRead()
    first.readPositionRemaining(0L,8L )

    val second = buf_two.bytesForRead()
    second.readPositionRemaining( 0L, 8L )


    return Pair(first,second)

}