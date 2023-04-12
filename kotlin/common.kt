package kotlin_servers
import java.lang.foreign.*
import java.lang.foreign.ValueLayout.OfLong
import java.nio.ByteOrder


class Memory(val srvAddr:MemoryAddress, val cliAddr:MemoryAddress) {
    
    val NATIVE_LONG = ValueLayout.JAVA_LONG.withOrder(ByteOrder.LITTLE_ENDIAN)

    @Suppress("NOTHING_TO_INLINE")
    inline fun spinUntilChange(lastValue:Long):Long {
        var newValue = lastValue
        while( newValue == lastValue ) {
            java.lang.Thread.onSpinWait();
            newValue = cliAddr.get(NATIVE_LONG, 0L)
        }
        return newValue
    }
    
    @Suppress("NOTHING_TO_INLINE")
    inline fun write(value:Long) {
        srvAddr.set(NATIVE_LONG, 0L, value)
    }
}

fun setupMemory(): Memory
{
    System.loadLibrary("zigmap")
    var linker = Linker.nativeLinker()
    var sym = SymbolLookup.loaderLookup()
    var setup = linker.downcallHandle(
        sym.lookup("mapSetup").get(),
        FunctionDescriptor.of(ValueLayout.ADDRESS)
    )
    var memPtr = setup.invoke() as MemoryAddress;
    var other = memPtr.addOffset(2048)
    return Memory(other, memPtr)
}