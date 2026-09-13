# Python: Type Systems demo
# Python is DYNAMICALLY typed (types are checked at runtime, not compile time)
# and STRONGLY typed (it will not silently coerce unrelated types for you).

value = 5
print("value =", value, "| type:", type(value).__name__)

# Dynamic typing: the same name can be rebound to a completely different type.
value = "five"
print("value =", value, "| type:", type(value).__name__)

value = [1, 2, 3]
print("value =", value, "| type:", type(value).__name__)

# Strong typing: Python refuses to implicitly coerce int + str.
try:
    result = "5" + 5
except TypeError as e:
    print("TypeError raised for '5' + 5:", e)

# Explicit conversion is required -- the programmer must state the intent.
result = "5" + str(5)
print("'5' + str(5) =", result)

result = int("5") + 5
print("int('5') + 5 =", result)
