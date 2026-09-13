// C++: Type Systems demo
// C++ is STATICALLY typed: every variable's type is fixed at compile time,
// and the compiler rejects mismatched types before the program ever runs.
#include <iostream>
#include <string>
using namespace std;

int main() {
    int value = 5;
    cout << "value = " << value << " | sizeof(int) = " << sizeof(value) << " bytes" << endl;

    // Unlike Python/JS, `value` cannot be reassigned to a different type.
    // The next two lines are commented out because they FAIL TO COMPILE:
    // value = "five";      // error: cannot convert 'const char*' to 'int'
    // value = std::string("five"); // error: no viable conversion

    // To hold a different type we must declare a new, separately-typed variable.
    string valueStr = "five";
    cout << "valueStr = " << valueStr << endl;

    // Static typing catches the Python/JS "5" + 5 mistake at COMPILE time
    // instead of compile-time coercion or a runtime exception:
    // string bad = "5" + 5;  // error: invalid operands of types 'const char*' and 'int'

    // Explicit, type-safe conversion is required, same spirit as Python:
    string combined = string("5") + to_string(5);
    cout << "string(\"5\") + to_string(5) = " << combined << endl;

    int fromStr = stoi("5") + 5;
    cout << "stoi(\"5\") + 5 = " << fromStr << endl;

    return 0;
}
