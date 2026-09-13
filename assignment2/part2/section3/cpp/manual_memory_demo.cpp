// C++: Manual Memory Management demo
//
// C++ gives the programmer full control over the heap via new/delete
// (or malloc/free), but with NO safety net: the compiler will not stop
// you from forgetting to free memory (a LEAK) or from using memory after
// it has been freed (a DANGLING POINTER, undefined behavior).
//
// We observe real process memory using the Windows psapi
// GetProcessMemoryInfo API (self-instrumentation) as a lightweight
// in-process substitute for an external profiler such as Valgrind/massif
// (Linux-only) or Windows Performance Analyzer, neither of which was
// available in this environment.
#include <iostream>
#include <windows.h>
#include <psapi.h>
#include <memory>
using namespace std;

static size_t workingSetMB() {
    PROCESS_MEMORY_COUNTERS pmc;
    GetProcessMemoryInfo(GetCurrentProcess(), &pmc, sizeof(pmc));
    return pmc.WorkingSetSize / (1024 * 1024);
}

int main() {
    cout << "PID: " << GetCurrentProcessId() << endl;
    cout << "Baseline working set: " << workingSetMB() << " MB" << endl;

    // ---- 1) Correct manual management: paired new[] / delete[] ----
    {
        int* correct = new int[1000000]; // ~4 MB, heap-allocated
        for (int i = 0; i < 1000000; i++) correct[i] = i;
        cout << "After correct allocation: " << workingSetMB() << " MB" << endl;
        delete[] correct;               // explicitly freed by the programmer
        correct = nullptr;              // avoids leaving a dangling pointer around
        cout << "After delete[]: " << workingSetMB() << " MB" << endl;
    }

    // ---- 2) Memory leak: allocate repeatedly, never delete ----
    for (int i = 0; i < 50; i++) {
        int* leaked = new int[1000000]; // ~4 MB each, 50 times = ~200 MB
        // Touch every page (not just leaked[0]) so the OS actually commits
        // all of it into the working set instead of leaving it un-faulted.
        for (int j = 0; j < 1000000; j++) leaked[j] = i;
        // BUG: no delete[] leaked; -- the pointer goes out of scope and is
        // lost, but the memory it pointed to is NOT returned to the OS.
    }
    cout << "After 50 leaked 4MB allocations (~200MB leaked): " << workingSetMB() << " MB" << endl;

    // ---- 3) Dangling pointer: use memory after it has been freed ----
    int* dangling = new int[10];
    dangling[0] = 42;
    delete[] dangling;
    // BUG: `dangling` still holds the old address. Reading/writing through
    // it now is UNDEFINED BEHAVIOR -- it may print garbage, appear to
    // "work" by coincidence, or corrupt unrelated memory/crash.
    cout << "Dangling pointer read after delete[] (undefined behavior): "
         << dangling[0] << endl;
    dangling = nullptr; // the only real fix: don't use it again

    // ---- 4) The modern C++ fix: RAII via smart pointers (like Rust) ----
    {
        unique_ptr<int[]> safe(new int[1000000]);
        safe[0] = 7;
        cout << "unique_ptr value: " << safe[0] << endl;
        // No delete[] needed -- freed automatically when `safe` goes out
        // of scope, the same ownership idea Rust enforces at compile time.
    }
    cout << "After unique_ptr scope ends (auto-freed): " << workingSetMB() << " MB" << endl;

    return 0;
}
