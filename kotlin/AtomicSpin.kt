package kotlin_servers
import kotlin_servers.setupMemory

fun doLoop(mem: Memory) {
    var lastValue = 0L
    while(true) {
        lastValue = mem.spinUntilChange(lastValue)
        mem.write(lastValue)
    }
}



fun main() {

    val memory = setupMemory()

    doLoop(memory)

}
