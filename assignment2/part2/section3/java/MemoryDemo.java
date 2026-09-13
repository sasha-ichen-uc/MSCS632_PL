// Java: Garbage Collection demo
//
// Java allocates objects dynamically with `new`, on the heap, but the
// programmer never calls `free`/`delete`. Instead, the JVM's Garbage
// Collector (GC) periodically scans the heap for objects with no
// remaining reachable references and reclaims their memory automatically.
//
// We observe this using the JVM's own Runtime introspection API
// (Runtime.getRuntime().totalMemory()/freeMemory()) as a lightweight,
// always-available in-process memory profiler -- a dedicated GUI tool
// such as VisualVM or JConsole would show the same heap trend graphically.

import java.util.ArrayList;
import java.util.List;

public class MemoryDemo {

    private static long usedMemoryMB() {
        Runtime rt = Runtime.getRuntime();
        return (rt.totalMemory() - rt.freeMemory()) / (1024 * 1024);
    }

    public static void main(String[] args) throws InterruptedException {
        System.out.println("PID: " + ProcessHandle.current().pid());
        System.out.println("Baseline used heap: " + usedMemoryMB() + " MB");

        // Dynamically allocate a large number of objects on the heap.
        List<byte[]> garbage = new ArrayList<>();
        for (int i = 0; i < 500; i++) {
            garbage.add(new byte[1024 * 1024]); // 1 MB each -> ~500 MB total
        }
        System.out.println("After allocating " + garbage.size() + " x 1MB objects: "
                + usedMemoryMB() + " MB used");

        // Drop the only references to all of that memory. The objects are
        // now unreachable, but nothing has been "freed" by us explicitly.
        garbage.clear();
        garbage = null;
        System.out.println("After clearing references (pre-GC): " + usedMemoryMB() + " MB used");

        // Hint the JVM to run garbage collection now. (System.gc() is only
        // a *request* -- the JVM decides if/when to actually collect --
        // which is itself a key semantic difference from manual free().)
        System.gc();
        Thread.sleep(500); // give the collector a moment to run
        System.out.println("After System.gc(): " + usedMemoryMB() + " MB used");

        System.out.println("main() ending -- any remaining garbage is reclaimed automatically,");
        System.out.println("with no explicit delete/free call anywhere in this program.");
    }
}
