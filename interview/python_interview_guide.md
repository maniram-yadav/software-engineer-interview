# The Complete Python Interview Guide
### Basics to Advanced — Core Language, Web Frameworks, and Databases

---

## Table of Contents

1. [Python Basics](#1-python-basics)
2. [Data Types & Data Structures](#2-data-types--data-structures)
3. [Functions, Scope & Functional Programming](#3-functions-scope--functional-programming)
4. [Object-Oriented Programming](#4-object-oriented-programming)
5. [Iterators, Generators & Decorators](#5-iterators-generators--decorators)
6. [Exception Handling](#6-exception-handling)
7. [File Handling & Context Managers](#7-file-handling--context-managers)
8. [Memory Management & Garbage Collection](#8-memory-management--garbage-collection)
9. [Concurrency: Threading, Multiprocessing, Asyncio](#9-concurrency-threading-multiprocessing-asyncio)
10. [Advanced Python (Metaclasses, Descriptors, `__slots__`)](#10-advanced-python)
11. [Web Frameworks: Django](#11-web-frameworks-django)
12. [Web Frameworks: Flask](#12-web-frameworks-flask)
13. [Web Frameworks: FastAPI](#13-web-frameworks-fastapi)
14. [Databases: SQL Fundamentals](#14-databases-sql-fundamentals)
15. [Databases: ORMs (SQLAlchemy & Django ORM)](#15-databases-orms)
16. [Databases: NoSQL](#16-databases-nosql)
17. [Testing in Python](#17-testing-in-python)
18. [Packaging, Tooling & Best Practices](#18-packaging-tooling--best-practices)
19. [Rapid-Fire Q&A Round](#19-rapid-fire-qa-round)

---

## 1. Python Basics

### Q1. What is Python and what are its key features?
Python is a high-level, interpreted, general-purpose, dynamically-typed programming language. Key features:
- **Interpreted**: Code runs line-by-line via the Python interpreter (CPython by default), no separate compile step for the developer.
- **Dynamically typed**: Variable types are determined at runtime.
- **Multi-paradigm**: Supports procedural, object-oriented, and functional styles.
- **Batteries included**: Rich standard library.
- **Garbage collected**: Automatic memory management via reference counting + cyclic GC.
- **Extensible**: Easily interfaces with C/C++ (via C-extensions) for performance-critical code.

### Q2. Is Python compiled or interpreted?
Both, technically. Python source (`.py`) is first compiled to platform-independent **bytecode** (`.pyc`), which is then executed by the **Python Virtual Machine (PVM)**. So it's "compiled" to bytecode, then "interpreted" by the PVM — this is why Python is often called a "byte-compiled interpreted language."

### Q3. What is PEP 8?
PEP 8 is Python's official style guide — covers naming conventions, indentation (4 spaces), line length (79/99 chars), import ordering, whitespace rules, etc. Tools like `flake8`, `black`, and `ruff` help enforce it.

### Q4. Difference between `is` and `==`?
```python
a = [1, 2, 3]
b = [1, 2, 3]
c = a

a == b   # True  -> compares VALUE (calls __eq__)
a is b   # False -> compares IDENTITY (same memory/object id)
a is c   # True  -> c refers to the same object as a
```
`is` checks whether two references point to the **same object in memory** (`id(a) == id(b)`); `==` checks **value equality**, and can be overridden via `__eq__`.

### Q5. What are Python's built-in data types?
- **Numeric**: `int`, `float`, `complex`
- **Sequence**: `str`, `list`, `tuple`, `range`
- **Mapping**: `dict`
- **Set types**: `set`, `frozenset`
- **Boolean**: `bool`
- **Binary**: `bytes`, `bytearray`, `memoryview`
- **None type**: `NoneType`

### Q6. Mutable vs Immutable types — give examples.
- **Immutable** (cannot change after creation): `int`, `float`, `str`, `tuple`, `frozenset`, `bytes`, `bool`
- **Mutable** (can change in place): `list`, `dict`, `set`, `bytearray`, custom class objects (by default)

```python
s = "hello"
s[0] = "H"     # TypeError: 'str' object does not support item assignment

lst = [1, 2, 3]
lst[0] = 99    # Works fine -> [99, 2, 3]
```
**Why it matters:** Immutable objects are hashable (usable as dict keys / set members) and safe to share across threads. Mutable default arguments are a classic gotcha (see Q-mutable-default below).

### Q7. Explain Python's variable scope — the LEGB rule.
Python resolves names using **LEGB**:
1. **L**ocal — inside the current function
2. **E**nclosing — any enclosing function (closures)
3. **G**lobal — top-level of the module
4. **B**uilt-in — Python's built-in namespace (`len`, `print`, etc.)

```python
x = "global"

def outer():
    x = "enclosing"
    def inner():
        x = "local"
        print(x)      # local
    inner()
    print(x)          # enclosing

outer()
print(x)               # global
```

Use `global` and `nonlocal` keywords to modify outer-scope variables from within a function:
```python
counter = 0
def increment():
    global counter
    counter += 1
```

### Q8. What is the mutable default argument gotcha?
Default argument values are evaluated **once**, at function definition time — not on every call. Using a mutable object as a default is a classic bug source.

```python
def append_item(item, bucket=[]):   # BAD
    bucket.append(item)
    return bucket

print(append_item(1))   # [1]
print(append_item(2))   # [1, 2]  <- unexpected! Same list reused across calls
```
**Fix:**
```python
def append_item(item, bucket=None):
    if bucket is None:
        bucket = []
    bucket.append(item)
    return bucket
```

### Q9. Explain `*args` and `**kwargs`.
- `*args` collects extra **positional** arguments into a tuple.
- `**kwargs` collects extra **keyword** arguments into a dict.

```python
def demo(a, *args, **kwargs):
    print(a)        # 1
    print(args)      # (2, 3)
    print(kwargs)    # {'x': 10, 'y': 20}

demo(1, 2, 3, x=10, y=20)
```
Order matters in the function signature: `def f(pos, *args, kw_only, **kwargs)`.

### Q10. What is the difference between `deepcopy` and `copy`?
```python
import copy

original = [[1, 2], [3, 4]]

shallow = copy.copy(original)      # or original[:] / list(original)
deep = copy.deepcopy(original)

shallow[0][0] = "CHANGED"
print(original)   # [['CHANGED', 2], [3, 4]]  -> inner list is shared!

deep[1][0] = "SAFE"
print(original)   # unaffected, nested objects are fully cloned
```
`copy.copy()` creates a new outer object but **references** the same nested objects. `copy.deepcopy()` recursively clones everything.

---

## 2. Data Types & Data Structures

### Q11. List vs Tuple vs Set vs Dict — when to use what?

| Feature | List | Tuple | Set | Dict |
|---|---|---|---|---|
| Ordered | Yes | Yes | No (insertion order preserved in CPython 3.7+, but no indexing) | Yes (3.7+) |
| Mutable | Yes | No | Yes | Yes |
| Duplicates | Allowed | Allowed | Not allowed | Keys unique, values can dup |
| Indexed access | Yes | Yes | No | By key |
| Use case | Ordered, changeable collection | Fixed record / hashable sequence | Fast membership tests, dedup | Key-value lookups |
| Underlying structure | Dynamic array | Dynamic array (immutable) | Hash table | Hash table |

```python
lst = [1, 2, 3]           # list
tup = (1, 2, 3)            # tuple
st  = {1, 2, 3}             # set
d   = {"a": 1, "b": 2}       # dict
```

### Q12. Why are tuples faster than lists, and when should you prefer them?
Tuples are immutable, so Python can allocate fixed-size memory and cache them internally (small tuples are sometimes reused). They're also **hashable** (if all elements are hashable), so they can be dict keys or set members — lists cannot.

```python
d = {(1, 2): "point A"}     # valid, tuple as key
# d = {[1, 2]: "point A"}   # TypeError: unhashable type: 'list'
```

### Q13. How does a Python `dict` work internally?
CPython dicts are implemented as **hash tables**. Each key is hashed via `hash(key)`, and the hash determines a slot in an internal array. Collisions are resolved via open addressing (probing). Since Python 3.7, dicts also maintain **insertion order** as a language guarantee (not just an implementation detail).

Time complexity: average **O(1)** for get/set/delete; worst case O(n) with many hash collisions.

```python
d = {}
d["b"] = 2
d["a"] = 1
print(d)   # {'b': 2, 'a': 1}  -> insertion order preserved
```

### Q14. List comprehension vs generator expression — what's the difference?
```python
squares_list = [x**2 for x in range(1000000)]     # builds full list in memory
squares_gen  = (x**2 for x in range(1000000))      # lazy, yields one at a time
```
List comprehensions build the entire result eagerly (more memory, but reusable/indexable). Generator expressions are lazy — memory-efficient for large or infinite sequences but can only be iterated once.

Other comprehension forms:
```python
{x: x**2 for x in range(5)}     # dict comprehension
{x**2 for x in range(5)}        # set comprehension
```

### Q15. How do you remove duplicates from a list while preserving order?
```python
def dedupe(seq):
    seen = set()
    result = []
    for item in seq:
        if item not in seen:
            seen.add(item)
            result.append(item)
    return result

dedupe([3, 1, 3, 2, 1])   # [3, 1, 2]

# Python 3.7+, concise one-liner using dict (order-preserving):
list(dict.fromkeys([3, 1, 3, 2, 1]))   # [3, 1, 2]
```

### Q16. Explain slicing in Python.
```python
s = "abcdefgh"
s[2:5]      # 'cde'   -> start:stop (stop excluded)
s[::2]      # 'aceg'  -> step
s[::-1]     # 'hgfedcba'  -> reverse
s[:3]       # 'abc'
s[3:]       # 'defgh'
s[-3:]      # 'fgh'   -> last 3 characters
```
Slicing works uniformly on lists, tuples, and strings since they're all sequences.

### Q17. What is the difference between `array` module, `list`, and NumPy `ndarray`?
- **`list`**: heterogeneous, dynamically typed, stores pointers to objects — flexible but memory-heavy.
- **`array` module**: homogeneous, typed, more memory-compact than list, but limited operations (no vectorized math).
- **NumPy `ndarray`**: homogeneous, contiguous memory block, supports vectorized operations, broadcasting, and is the backbone of the scientific Python stack (pandas, scikit-learn, etc.).

```python
import numpy as np
a = np.array([1, 2, 3])
b = np.array([10, 20, 30])
a + b   # array([11, 22, 33])  -> vectorized, no explicit loop
```

### Q18. What are `collections` module utilities you should know?
```python
from collections import Counter, defaultdict, namedtuple, OrderedDict, deque

Counter("mississippi")
# Counter({'i': 4, 's': 4, 'p': 2, 'm': 1})

dd = defaultdict(list)
dd["fruits"].append("apple")   # no KeyError, auto-creates empty list

Point = namedtuple("Point", ["x", "y"])
p = Point(1, 2)
p.x, p.y   # 1, 2

dq = deque([1, 2, 3])
dq.appendleft(0)   # O(1) append/pop at both ends, unlike list
```

### Q19. String formatting: `%`, `.format()`, and f-strings — which to use?
```python
name, age = "Alice", 30

"%s is %d years old" % (name, age)          # old style (C-like)
"{} is {} years old".format(name, age)       # .format() method
f"{name} is {age} years old"                 # f-string (Python 3.6+, PREFERRED)

# f-strings support expressions and formatting specs directly:
f"{3.14159:.2f}"      # '3.14'
f"{age=}"              # 'age=30'  (debug specifier, 3.8+)
```
F-strings are fastest and most readable — the modern default.

---

## 3. Functions, Scope & Functional Programming

### Q20. What are first-class functions?
In Python, functions are first-class objects: they can be assigned to variables, passed as arguments, returned from other functions, and stored in data structures.

```python
def greet(name):
    return f"Hello, {name}"

say_hi = greet          # assign to variable
print(say_hi("Bob"))    # 'Hello, Bob'

def apply(func, value):
    return func(value)

apply(greet, "Carol")   # 'Hello, Carol'
```

### Q21. What is a closure? Give a practical example.
A closure is a function that "remembers" variables from its enclosing scope even after that scope has finished executing.

```python
def make_multiplier(factor):
    def multiplier(x):
        return x * factor      # 'factor' captured from enclosing scope
    return multiplier

double = make_multiplier(2)
triple = make_multiplier(3)
print(double(5))   # 10
print(triple(5))   # 15
```
Closures are the basis for decorators and are useful for creating function factories without classes.

### Q22. `map`, `filter`, `reduce` — explain with examples.
```python
from functools import reduce

nums = [1, 2, 3, 4, 5]

list(map(lambda x: x**2, nums))            # [1, 4, 9, 16, 25]
list(filter(lambda x: x % 2 == 0, nums))    # [2, 4]
reduce(lambda acc, x: acc + x, nums, 0)      # 15  (cumulative reduction)
```
In modern Python, list comprehensions are often preferred over `map`/`filter` for readability, but `reduce` (from `functools`) has no direct comprehension equivalent.

### Q23. What is `lambda` and its limitations?
An anonymous, single-expression function.
```python
add = lambda a, b: a + b
add(2, 3)   # 5
```
Limitations: only a single expression (no statements, no multi-line logic, no assignments in older versions), harder to debug (shows as `<lambda>` in tracebacks), and typically less readable than a named `def` for anything non-trivial.

### Q24. Explain `functools.partial`.
Creates a new function with some arguments pre-filled.
```python
from functools import partial

def power(base, exponent):
    return base ** exponent

square = partial(power, exponent=2)
cube = partial(power, exponent=3)
square(4)   # 16
cube(2)     # 8
```

### Q25. What's the difference between positional-only, keyword-only, and normal parameters?
```python
def f(a, b, /, c, d, *, e, f):
    # a, b: positional-only  (before /)
    # c, d: positional-or-keyword
    # e, f: keyword-only     (after *)
    pass

f(1, 2, 3, d=4, e=5, f=6)   # valid
f(a=1, b=2, c=3, d=4, e=5, f=6)   # TypeError: a, b are positional-only
```
`/` and `*` were both formalized as syntax in PEP 570 (Python 3.8) for `/`, and PEP 3102 for `*`. Useful for API design: positional-only avoids locking in parameter names as public API; keyword-only forces clarity at call sites.

### Q26. What is recursion, and does Python optimize tail calls?
Recursion is a function calling itself. **Python does NOT perform tail-call optimization** — deep recursion can hit `RecursionError` (`sys.getrecursionlimit()`, default 1000). For deep/iterative-equivalent problems, an explicit loop or increasing the recursion limit (`sys.setrecursionlimit()`, risky) is often preferred, or refactor to an iterative approach / use `sys.setrecursionlimit` cautiously.

```python
import sys
def factorial(n):
    return 1 if n <= 1 else n * factorial(n - 1)

factorial(5)         # 120
# factorial(5000)    # RecursionError: maximum recursion depth exceeded
```

---

## 4. Object-Oriented Programming

### Q27. Explain the four pillars of OOP with Python examples.

**Encapsulation** — bundling data and methods; controlling access via naming conventions (Python has no true "private", only conventions):
```python
class BankAccount:
    def __init__(self, balance):
        self._balance = balance        # convention: "protected"
        self.__pin = "1234"             # name-mangled: "private-ish" -> _BankAccount__pin

    def deposit(self, amount):
        self._balance += amount

    @property
    def balance(self):
        return self._balance
```

**Abstraction** — hiding implementation details behind an interface:
```python
from abc import ABC, abstractmethod

class Shape(ABC):
    @abstractmethod
    def area(self):
        ...

class Circle(Shape):
    def __init__(self, r):
        self.r = r
    def area(self):
        return 3.14159 * self.r ** 2
```

**Inheritance** — reusing behavior from a base class:
```python
class Animal:
    def speak(self):
        return "..."

class Dog(Animal):
    def speak(self):
        return "Woof!"
```

**Polymorphism** — same interface, different implementations:
```python
for animal in [Dog(), Animal()]:
    print(animal.speak())   # 'Woof!' then '...'
```

### Q28. What is Method Resolution Order (MRO), and how does Python handle multiple inheritance?
Python uses the **C3 linearization algorithm** to determine method resolution order in multiple inheritance. You can inspect it via `ClassName.__mro__` or `ClassName.mro()`.

```python
class A:
    def hello(self):
        return "A"

class B(A):
    def hello(self):
        return "B"

class C(A):
    def hello(self):
        return "C"

class D(B, C):
    pass

D().hello()        # 'B'  -> follows MRO
print(D.__mro__)    # (D, B, C, A, object)
```
This solves the classic "diamond problem" deterministically.

### Q29. `@staticmethod` vs `@classmethod` vs instance method.
```python
class MyClass:
    class_var = "shared"

    def instance_method(self):
        return f"instance method, self={self}"

    @classmethod
    def class_method(cls):
        return f"class method, cls={cls}, class_var={cls.class_var}"

    @staticmethod
    def static_method():
        return "static method, no access to self or cls"
```
- **Instance method**: takes `self`, accesses/modifies instance state.
- **Class method**: takes `cls`, operates on the class itself (common for alternate constructors, e.g. `Point.from_tuple(...)`).
- **Static method**: no implicit first argument — just a regular function namespaced inside the class, for utility logic related to the class.

### Q30. What are dunder (magic) methods? Give common examples.
Double-underscore methods that let objects integrate with Python's built-in syntax/operators.
```python
class Vector:
    def __init__(self, x, y):
        self.x, self.y = x, y

    def __repr__(self):                      # dev-facing representation
        return f"Vector({self.x}, {self.y})"

    def __str__(self):                       # user-facing string
        return f"({self.x}, {self.y})"

    def __add__(self, other):                # supports v1 + v2
        return Vector(self.x + other.x, self.y + other.y)

    def __eq__(self, other):                 # supports v1 == v2
        return (self.x, self.y) == (other.x, other.y)

    def __len__(self):                       # supports len(v)
        return 2

    def __getitem__(self, idx):              # supports v[0]
        return (self.x, self.y)[idx]

v1, v2 = Vector(1, 2), Vector(3, 4)
print(v1 + v2)     # (4, 6)
print(v1 == Vector(1, 2))   # True
```

### Q31. Difference between `__new__` and `__init__`?
- `__new__(cls, ...)` — **creates** and returns a new instance (called before `__init__`); rarely overridden except for immutable types or metaclasses/singletons.
- `__init__(self, ...)` — **initializes** the already-created instance; returns `None`.

```python
class Singleton:
    _instance = None
    def __new__(cls, *args, **kwargs):
        if cls._instance is None:
            cls._instance = super().__new__(cls)
        return cls._instance

a = Singleton()
b = Singleton()
a is b   # True
```

### Q32. What is duck typing?
"If it walks like a duck and quacks like a duck, it's a duck." Python doesn't check types explicitly — it checks whether an object supports the required behavior (methods/attributes).
```python
class Duck:
    def quack(self): return "Quack!"

class Person:
    def quack(self): return "I'm imitating a duck!"

def make_it_quack(thing):
    print(thing.quack())    # works for ANY object with a .quack() method

make_it_quack(Duck())
make_it_quack(Person())
```

### Q33. Composition vs Inheritance — when to prefer which?
**Inheritance** ("is-a" relationship) couples subclass to superclass implementation tightly; **composition** ("has-a" relationship) is more flexible.
```python
# Composition example
class Engine:
    def start(self):
        return "Engine started"

class Car:
    def __init__(self):
        self.engine = Engine()      # Car HAS an Engine
    def start(self):
        return self.engine.start()
```
Guideline: "Favor composition over inheritance" — inheritance for genuine is-a hierarchies with shared behavior contracts; composition for flexible, swappable behavior (also avoids fragile base class problems).

---

## 5. Iterators, Generators & Decorators

### Q34. What is the iterator protocol?
An object is **iterable** if it implements `__iter__()`. An object is an **iterator** if it implements both `__iter__()` (returning itself) and `__next__()` (returning the next value, raising `StopIteration` when exhausted).

```python
class Countdown:
    def __init__(self, start):
        self.current = start
    def __iter__(self):
        return self
    def __next__(self):
        if self.current <= 0:
            raise StopIteration
        self.current -= 1
        return self.current + 1

for n in Countdown(3):
    print(n)     # 3, 2, 1
```

### Q35. What is a generator, and how does `yield` differ from `return`?
A generator is a function containing `yield`; calling it returns a **generator object** (an iterator) without running the body immediately. Each call to `next()` resumes execution right after the last `yield`, preserving local state between calls.

```python
def countdown(n):
    while n > 0:
        yield n     # pauses here, returns n, resumes on next() call
        n -= 1

gen = countdown(3)
next(gen)   # 3
next(gen)   # 2
next(gen)   # 1
next(gen)   # StopIteration
```
`return` inside a generator just stops iteration (raises `StopIteration` with the return value as its argument in some cases) — it doesn't behave like a normal function return.

**Why use generators:** memory efficiency (lazy evaluation) for large/infinite sequences, and pipeline-style data processing.
```python
def read_large_file(path):
    with open(path) as f:
        for line in f:
            yield line.strip()          # one line in memory at a time
```

### Q36. What is `yield from`?
Delegates iteration to a sub-generator/iterable, flattening nested generator logic.
```python
def inner():
    yield 1
    yield 2

def outer():
    yield from inner()
    yield 3

list(outer())   # [1, 2, 3]
```

### Q37. Explain decorators with a real-world example (timing/logging).
A decorator is a function that wraps another function to extend its behavior without modifying its source.

```python
import time
from functools import wraps

def timer(func):
    @wraps(func)                 # preserves original func's __name__/docstring
    def wrapper(*args, **kwargs):
        start = time.perf_counter()
        result = func(*args, **kwargs)
        elapsed = time.perf_counter() - start
        print(f"{func.__name__} took {elapsed:.4f}s")
        return result
    return wrapper

@timer
def slow_function():
    time.sleep(1)
    return "done"

slow_function()   # prints: slow_function took 1.0001s
```
**Without `@wraps`**, `slow_function.__name__` would become `'wrapper'`, breaking introspection/debugging tools.

### Q38. How do decorators with arguments work?
A decorator factory — a function that returns a decorator.
```python
def repeat(times):
    def decorator(func):
        @wraps(func)
        def wrapper(*args, **kwargs):
            results = []
            for _ in range(times):
                results.append(func(*args, **kwargs))
            return results
        return wrapper
    return decorator

@repeat(3)
def greet(name):
    return f"Hi {name}"

greet("Sam")   # ['Hi Sam', 'Hi Sam', 'Hi Sam']
```

### Q39. What are common built-in decorators?
- `@staticmethod`, `@classmethod`, `@property` (covered above / below)
- `@functools.lru_cache` — memoization:
```python
from functools import lru_cache

@lru_cache(maxsize=None)
def fib(n):
    return n if n < 2 else fib(n-1) + fib(n-2)

fib(35)    # fast, thanks to caching -> avoids exponential re-computation
```
- `@functools.total_ordering` — auto-generates comparison methods from `__eq__` + one of `__lt__`/`__le__`/`__gt__`/`__ge__`.

### Q40. What is `@property` and why use it?
Lets you define getter/setter/deleter logic while keeping attribute-style access syntax.
```python
class Temperature:
    def __init__(self, celsius):
        self._celsius = celsius

    @property
    def fahrenheit(self):
        return self._celsius * 9/5 + 32

    @fahrenheit.setter
    def fahrenheit(self, value):
        self._celsius = (value - 32) * 5/9

t = Temperature(25)
print(t.fahrenheit)   # 77.0
t.fahrenheit = 98.6
print(t._celsius)     # 37.0
```
Benefit: you can start with a plain attribute and later add validation/computed logic without breaking the calling code's API (`obj.attr` stays the same syntax).

---

## 6. Exception Handling

### Q41. Explain `try` / `except` / `else` / `finally`.
```python
def divide(a, b):
    try:
        result = a / b
    except ZeroDivisionError:
        print("Cannot divide by zero")
        return None
    except TypeError as e:
        print(f"Type error: {e}")
        return None
    else:
        print("Division succeeded")   # runs ONLY if no exception occurred
        return result
    finally:
        print("Cleanup runs always")   # runs no matter what (even on return)

divide(10, 2)
divide(10, 0)
```
- `else` runs only if the `try` block succeeds without exception.
- `finally` always runs — used for cleanup (closing files, releasing locks) regardless of success/failure.

### Q42. How do you create and raise custom exceptions?
```python
class InsufficientFundsError(Exception):
    """Raised when a withdrawal exceeds the available balance."""
    def __init__(self, balance, amount):
        self.balance = balance
        self.amount = amount
        super().__init__(f"Cannot withdraw {amount}, balance is {balance}")

def withdraw(balance, amount):
    if amount > balance:
        raise InsufficientFundsError(balance, amount)
    return balance - amount

try:
    withdraw(100, 150)
except InsufficientFundsError as e:
    print(e)   # Cannot withdraw 150, balance is 100
```
Custom exceptions should inherit from `Exception` (not `BaseException` directly — that's reserved for system-exiting exceptions like `SystemExit`, `KeyboardInterrupt`).

### Q43. What is exception chaining (`raise ... from ...`)?
```python
try:
    1 / 0
except ZeroDivisionError as e:
    raise ValueError("Invalid calculation") from e
```
This preserves the original traceback context (`__cause__`), so the traceback shows both the root cause and the higher-level error — important for debugging layered systems.

### Q44. What's the exception hierarchy — key built-in exceptions to know?
```
BaseException
 ├── SystemExit
 ├── KeyboardInterrupt
 └── Exception
      ├── ArithmeticError (ZeroDivisionError, OverflowError)
      ├── AttributeError
      ├── ImportError (ModuleNotFoundError)
      ├── LookupError (IndexError, KeyError)
      ├── NameError (UnboundLocalError)
      ├── OSError (FileNotFoundError, PermissionError, ConnectionError)
      ├── RuntimeError (RecursionError, NotImplementedError)
      ├── TypeError
      ├── ValueError
      └── StopIteration
```
Always catch the **most specific** exception possible — catching bare `Exception` (or worse, bare `except:`) can silently swallow bugs.

### Q45. What's the difference between catching `Exception` vs a bare `except:`?
```python
try:
    risky()
except:                 # BAD: also catches SystemExit, KeyboardInterrupt
    pass

try:
    risky()
except Exception:       # BETTER: still catches almost everything expected,
    pass                # but lets Ctrl+C / sys.exit() propagate correctly
```

---

## 7. File Handling & Context Managers

### Q46. How do you read/write files safely in Python?
```python
# Reading
with open("data.txt", "r", encoding="utf-8") as f:
    content = f.read()          # entire file
    # or: for line in f: ...    # line by line (memory-efficient)

# Writing
with open("output.txt", "w", encoding="utf-8") as f:
    f.write("Hello, World!\n")

# Appending
with open("log.txt", "a", encoding="utf-8") as f:
    f.write("New log entry\n")
```
Using `with` ensures the file is **automatically closed** even if an exception occurs — equivalent to `try/finally` but cleaner.

### Q47. What is the context manager protocol? Build a custom one.
Any object implementing `__enter__` and `__exit__` can be used with `with`.
```python
class ManagedFile:
    def __init__(self, filename, mode):
        self.filename = filename
        self.mode = mode

    def __enter__(self):
        self.file = open(self.filename, self.mode)
        return self.file

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.file.close()
        return False   # False = don't suppress exceptions; True would swallow them

with ManagedFile("test.txt", "w") as f:
    f.write("hello")
```
`__exit__`'s return value matters: returning `True` suppresses any exception raised inside the `with` block; returning `False`/`None` lets it propagate.

### Q48. How do you write a context manager using `contextlib`?
```python
from contextlib import contextmanager

@contextmanager
def managed_file(filename, mode):
    f = open(filename, mode)
    try:
        yield f              # code before yield = __enter__, after = __exit__
    finally:
        f.close()

with managed_file("test.txt", "w") as f:
    f.write("hello")
```
This generator-based approach is often more concise than writing a full class with `__enter__`/`__exit__`.

---

## 8. Memory Management & Garbage Collection

### Q49. How does Python manage memory?
- **Private heap**: All Python objects and data structures live in a private heap managed by the interpreter; programmers don't manually allocate/free memory.
- **Reference counting**: Every object has a reference count (`sys.getrefcount(obj)`); when it drops to zero, the memory is immediately deallocated.
- **Cyclic Garbage Collector**: Reference counting alone can't detect reference cycles (e.g., two objects referencing each other). The `gc` module periodically scans for and collects unreachable cycles using a generational algorithm.
- **Memory pools (pymalloc)**: CPython uses an internal allocator optimized for small objects to reduce fragmentation.

```python
import gc
gc.collect()          # force a collection cycle
gc.get_threshold()     # (700, 10, 10) default generational thresholds
```

### Q50. What causes a reference cycle, and how does the GC handle it?
```python
class Node:
    def __init__(self):
        self.parent = None
        self.child = None

a = Node()
b = Node()
a.child = b
b.parent = a          # cycle: a -> b -> a

del a, b               # refcount doesn't drop to 0 (they still reference each other)
                        # but the generational GC will detect and collect this cycle
```
Since Python 3.4 (PEP 442), objects with `__del__` methods involved in cycles can also be collected safely (this used to be a limitation in older Python).

### Q51. What is the Global Interpreter Lock (GIL)?
The GIL is a mutex in **CPython** that allows only **one thread to execute Python bytecode at a time**, even on multi-core machines. It exists to make CPython's memory management (reference counting) thread-safe without needing fine-grained locks everywhere.

**Implications:**
- CPU-bound multi-threaded Python code does **not** get true parallelism — threads take turns.
- I/O-bound code (network calls, file I/O, `sleep`) still benefits from threading, because the GIL is released during blocking I/O operations.
- For CPU-bound parallelism, use `multiprocessing` (separate processes, separate GILs) instead of `threading`.
- Note: Python 3.13 introduced an experimental **free-threaded build** (PEP 703) that can disable the GIL — worth mentioning as a forward-looking fact in interviews, though it's not yet the default.

### Q52. `sys.getsizeof()` vs actual memory usage — any gotchas?
```python
import sys
sys.getsizeof([1, 2, 3])   # size of the list object itself (pointers), NOT the ints it holds
```
`sys.getsizeof()` doesn't recursively account for referenced objects — a list of large objects will report a small size because it only stores pointers.

---

## 9. Concurrency: Threading, Multiprocessing, Asyncio

### Q53. Threading vs Multiprocessing vs Asyncio — when to use each?

| Approach | Best for | Parallelism | Overhead |
|---|---|---|---|
| `threading` | I/O-bound tasks (network, disk) | Concurrent, not parallel (GIL) | Low |
| `multiprocessing` | CPU-bound tasks (number crunching) | True parallel (separate processes) | High (process spawn, IPC) |
| `asyncio` | High-volume I/O-bound (many concurrent connections) | Single-threaded cooperative concurrency | Very low |

### Q54. Basic `threading` example.
```python
import threading, time

def worker(name):
    print(f"{name} starting")
    time.sleep(2)
    print(f"{name} done")

threads = [threading.Thread(target=worker, args=(f"Worker-{i}",)) for i in range(3)]
for t in threads:
    t.start()
for t in threads:
    t.join()       # wait for all threads to finish
```

### Q55. Basic `multiprocessing` example.
```python
from multiprocessing import Pool

def square(n):
    return n * n

if __name__ == "__main__":
    with Pool(processes=4) as pool:
        results = pool.map(square, [1, 2, 3, 4, 5])
    print(results)   # [1, 4, 9, 16, 25]  -> ran across 4 separate processes
```
Each process has its own memory space and GIL — genuine parallel execution on multiple cores, but data must be pickled to pass between processes (communication overhead).

### Q56. Basic `asyncio` example — `async`/`await`.
```python
import asyncio

async def fetch_data(id, delay):
    print(f"Task {id} started")
    await asyncio.sleep(delay)      # non-blocking sleep, yields control
    print(f"Task {id} finished")
    return f"result-{id}"

async def main():
    results = await asyncio.gather(
        fetch_data(1, 2),
        fetch_data(2, 1),
        fetch_data(3, 3),
    )
    print(results)

asyncio.run(main())
```
`asyncio` uses an **event loop** and cooperative multitasking: a coroutine voluntarily yields control at `await` points, letting other coroutines run. This scales to thousands of concurrent I/O-bound tasks with minimal overhead, unlike OS threads.

### Q57. What's the difference between concurrency and parallelism?
- **Concurrency**: dealing with multiple tasks by interleaving execution (may run on a single core) — e.g., asyncio, threading under the GIL.
- **Parallelism**: literally executing multiple tasks at the same instant on multiple cores — e.g., multiprocessing.

### Q58. What is a race condition and how do you prevent it in Python?
A race condition occurs when multiple threads access/modify shared state concurrently without synchronization, causing unpredictable results.
```python
import threading

counter = 0
lock = threading.Lock()

def increment():
    global counter
    for _ in range(100000):
        with lock:              # ensures atomic read-modify-write
            counter += 1

threads = [threading.Thread(target=increment) for _ in range(2)]
for t in threads: t.start()
for t in threads: t.join()
print(counter)   # 200000, guaranteed correct because of the lock
```
Without the lock, the final value would be unpredictable/less than 200000 due to interleaved read-modify-write operations.

### Q59. What is `concurrent.futures` and why use it?
A high-level API unifying thread and process pools.
```python
from concurrent.futures import ThreadPoolExecutor, ProcessPoolExecutor

def task(n):
    return n * n

with ThreadPoolExecutor(max_workers=4) as executor:
    results = list(executor.map(task, range(10)))

# Swap ThreadPoolExecutor -> ProcessPoolExecutor for CPU-bound work, same API
```

---

## 10. Advanced Python

### Q60. What is a metaclass? Give a real use case.
A metaclass is "the class of a class" — it controls how classes themselves are created. By default, all classes are instances of `type`.

```python
class Meta(type):
    def __new__(mcs, name, bases, namespace):
        # e.g., enforce that all methods are lowercase
        for key in namespace:
            if callable(namespace[key]) and not key.startswith("__"):
                if key.lower() != key:
                    raise TypeError(f"Method {key} must be lowercase")
        return super().__new__(mcs, name, bases, namespace)

class MyClass(metaclass=Meta):
    def valid_method(self): pass
    # def InvalidMethod(self): pass   # would raise TypeError at class definition time
```
**Real-world use cases**: Django's ORM uses a metaclass (`ModelBase`) to turn class attributes into database fields; ABCs use `ABCMeta`; frameworks use metaclasses for automatic registration of plugin/subclasses.

### Q61. What are descriptors, and how does `@property` relate to them?
A descriptor is any object implementing `__get__`, `__set__`, or `__delete__`, used to customize attribute access at the class level. `property` is itself implemented as a descriptor.

```python
class PositiveNumber:
    def __set_name__(self, owner, name):
        self.name = "_" + name

    def __get__(self, instance, owner):
        return getattr(instance, self.name)

    def __set__(self, instance, value):
        if value < 0:
            raise ValueError("Must be positive")
        setattr(instance, self.name, value)

class Product:
    price = PositiveNumber()      # reusable validation logic across attributes/classes

    def __init__(self, price):
        self.price = price

p = Product(10)
p.price = -5      # raises ValueError
```
Descriptors power `property`, `staticmethod`, `classmethod`, and ORM field definitions (Django model fields are descriptors under the hood).

### Q62. What is `__slots__` and why use it?
By default, instances store attributes in a per-instance `__dict__`, which has memory overhead. `__slots__` declares a fixed set of attributes, removing the instance `__dict__` and saving memory — useful when creating millions of small objects.

```python
class PointDict:
    def __init__(self, x, y):
        self.x, self.y = x, y

class PointSlots:
    __slots__ = ("x", "y")
    def __init__(self, x, y):
        self.x, self.y = x, y

# PointSlots instances use significantly less memory than PointDict instances
# PointSlots().z = 1   -> AttributeError, can't add new attributes dynamically
```

### Q63. Explain the difference between shallow equality (`__eq__`) and hashability (`__hash__`).
If you override `__eq__`, Python sets `__hash__` to `None` by default (making the object unhashable) unless you also define `__hash__`. Objects used as dict keys / set members must be hashable **and** their hash must remain constant over their lifetime (which is why mutable objects like lists are unhashable).

```python
class Point:
    def __init__(self, x, y):
        self.x, self.y = x, y
    def __eq__(self, other):
        return (self.x, self.y) == (other.x, other.y)
    def __hash__(self):
        return hash((self.x, self.y))     # must be consistent with __eq__

{Point(1, 2), Point(1, 2)}   # set of size 1, treated as duplicates
```

### Q64. What is monkey patching? Give an example and its risks.
Dynamically modifying/extending a class or module at runtime.
```python
class Greeter:
    def hello(self):
        return "Hi"

def new_hello(self):
    return "Hello, patched!"

Greeter.hello = new_hello    # monkey patch
Greeter().hello()             # 'Hello, patched!'
```
**Risks:** makes code harder to trace/debug, can silently break behavior other code depends on, and is fragile against library version updates. Commonly (and more safely) used in testing via `unittest.mock.patch`.

### Q65. What is the difference between `__str__` and `__repr__`?
- `__repr__`: unambiguous, developer-facing (ideally `eval(repr(obj)) == obj`); used by the REPL and inside containers.
- `__str__`: readable, user-facing string; used by `print()` and `str()`.

```python
class Point:
    def __init__(self, x, y):
        self.x, self.y = x, y
    def __repr__(self):
        return f"Point(x={self.x}, y={self.y})"
    def __str__(self):
        return f"({self.x}, {self.y})"

p = Point(1, 2)
print(p)          # ( 1, 2 )  -> uses __str__
print([p])         # [Point(x=1, y=2)]  -> list repr uses __repr__ for elements
```
If `__str__` is not defined, Python falls back to `__repr__`.

### Q66. What are type hints, and how does `mypy` help?
Type hints (PEP 484) annotate expected types without enforcing them at runtime — Python remains dynamically typed. Static type checkers like `mypy` or `pyright` catch type errors before runtime.

```python
from typing import List, Optional, Union

def greet(name: str) -> str:
    return f"Hello, {name}"

def find_user(id: int) -> Optional[dict]:
    ...

def process(items: List[int]) -> Union[int, float]:
    ...

# Modern syntax (3.10+):
def process_modern(items: list[int]) -> int | float:
    ...
```

### Q67. What is walrus operator (`:=`) and when is it useful?
Introduced in Python 3.8 (PEP 572), it assigns a value as part of an expression.
```python
# Without walrus
data = fetch()
if data:
    process(data)

# With walrus
if (data := fetch()):
    process(data)

# Useful in loops:
while (chunk := file.read(1024)):
    process(chunk)

# Useful in comprehensions to avoid re-computation:
results = [y for x in data if (y := expensive_transform(x)) is not None]
```

---

## 11. Web Frameworks: Django

### Q68. What is Django's architecture pattern — is it MVC?
Django follows **MTV (Model-Template-View)**, which is conceptually similar to MVC:
- **Model** — data layer (defines schema, talks to the database via the ORM).
- **Template** — presentation layer (HTML rendering).
- **View** — business logic layer (Django's "View" ≈ traditional MVC's "Controller"); it receives requests and returns responses.
- Django itself acts as the "Controller" routing requests via `urls.py`.

### Q69. Basic Django project structure & a simple model/view/URL example.
```python
# models.py
from django.db import models

class Article(models.Model):
    title = models.CharField(max_length=200)
    content = models.TextField()
    published_at = models.DateTimeField(auto_now_add=True)
    author = models.ForeignKey("auth.User", on_delete=models.CASCADE)

    def __str__(self):
        return self.title

# views.py
from django.shortcuts import render, get_object_or_404
from django.http import JsonResponse
from .models import Article

def article_list(request):
    articles = Article.objects.all().order_by("-published_at")
    return render(request, "articles/list.html", {"articles": articles})

def article_detail(request, pk):
    article = get_object_or_404(Article, pk=pk)
    return JsonResponse({"title": article.title, "content": article.content})

# urls.py
from django.urls import path
from . import views

urlpatterns = [
    path("articles/", views.article_list, name="article-list"),
    path("articles/<int:pk>/", views.article_detail, name="article-detail"),
]
```

### Q70. Function-Based Views (FBV) vs Class-Based Views (CBV)?
```python
# FBV
def article_list(request):
    articles = Article.objects.all()
    return render(request, "articles/list.html", {"articles": articles})

# CBV
from django.views.generic import ListView

class ArticleListView(ListView):
    model = Article
    template_name = "articles/list.html"
    context_object_name = "articles"
    ordering = ["-published_at"]
```
**CBVs** promote reuse via mixins/generic views (`ListView`, `DetailView`, `CreateView`, etc.) and are DRY for CRUD-heavy apps. **FBVs** are more explicit/readable for simple or highly custom logic. Many teams mix both based on complexity.

### Q71. What are Django migrations?
Migrations are version-controlled files that describe changes to your models' schema, allowing the database schema to be evolved incrementally and consistently across environments.
```bash
python manage.py makemigrations   # generates migration files from model changes
python manage.py migrate           # applies migrations to the database
python manage.py showmigrations    # lists migration status
```

### Q72. What is Django middleware?
A middleware is a hook into Django's request/response processing — a chain of components each request passes through before reaching the view, and each response passes through on the way out.
```python
class TimingMiddleware:
    def __init__(self, get_response):
        self.get_response = get_response

    def __call__(self, request):
        import time
        start = time.time()
        response = self.get_response(request)     # calls next middleware / view
        response["X-Response-Time"] = f"{time.time() - start:.4f}s"
        return response
```
Register in `settings.py`'s `MIDDLEWARE` list. Common uses: authentication, CORS, logging, GZip compression, security headers.

### Q73. Explain Django's ORM query optimization: `select_related` vs `prefetch_related`.
```python
# N+1 query problem:
for article in Article.objects.all():
    print(article.author.username)     # 1 query per article to fetch author!

# select_related: SQL JOIN, for ForeignKey / OneToOne (single query)
articles = Article.objects.select_related("author").all()
for article in articles:
    print(article.author.username)     # no extra queries

# prefetch_related: separate query + Python-side join, for ManyToMany / reverse FK
articles = Article.objects.prefetch_related("tags").all()
for article in articles:
    print(list(article.tags.all()))    # no extra queries per article
```
`select_related` does a SQL JOIN (works for forward FK/OneToOne). `prefetch_related` issues a separate query and joins in Python (works for M2M and reverse FK relationships where a JOIN would multiply rows).

### Q74. What is Django REST Framework (DRF), and how do you build a simple API?
```python
# serializers.py
from rest_framework import serializers
from .models import Article

class ArticleSerializer(serializers.ModelSerializer):
    class Meta:
        model = Article
        fields = ["id", "title", "content", "published_at", "author"]

# views.py
from rest_framework.viewsets import ModelViewSet
from .models import Article
from .serializers import ArticleSerializer

class ArticleViewSet(ModelViewSet):
    queryset = Article.objects.all()
    serializer_class = ArticleSerializer

# urls.py
from rest_framework.routers import DefaultRouter
router = DefaultRouter()
router.register("articles", ArticleViewSet)
urlpatterns = router.urls
```
`ModelViewSet` auto-generates list/create/retrieve/update/delete endpoints. DRF also provides authentication classes, permissions, throttling, pagination, and browsable API out of the box.

### Q75. How does Django handle authentication & permissions?
```python
from django.contrib.auth.decorators import login_required
from rest_framework.permissions import IsAuthenticated
from rest_framework.decorators import api_view, permission_classes

@login_required                    # traditional Django view protection
def dashboard(request):
    ...

@api_view(["GET"])
@permission_classes([IsAuthenticated])   # DRF-style protection
def profile(request):
    ...
```
Django ships with a built-in `User` model, session-based auth, and a permissions/groups system; DRF adds token/JWT-based auth commonly via `djangorestframework-simplejwt` for API use cases.

### Q76. What is Django's signal framework?
Signals allow decoupled apps to get notified when certain actions occur.
```python
from django.db.models.signals import post_save
from django.dispatch import receiver
from django.contrib.auth.models import User
from .models import Profile

@receiver(post_save, sender=User)
def create_profile(sender, instance, created, **kwargs):
    if created:
        Profile.objects.create(user=instance)
```
Common signals: `pre_save`, `post_save`, `pre_delete`, `post_delete`, `m2m_changed`. Use sparingly — overuse makes control flow hard to trace.

---

## 12. Web Frameworks: Flask

### Q77. What is Flask, and how does it differ philosophically from Django?
Flask is a **micro-framework** — minimal core (routing, request/response, templating via Jinja2), and everything else (ORM, auth, admin panel) is opt-in via extensions. Django is "batteries-included" (ORM, admin, auth, forms all built-in). Flask suits smaller services, microservices, and APIs where you want fine-grained control; Django suits larger, more conventional full-stack apps that benefit from built-in scaffolding.

### Q78. Basic Flask app with routing.
```python
from flask import Flask, jsonify, request

app = Flask(__name__)

articles = []

@app.route("/articles", methods=["GET"])
def list_articles():
    return jsonify(articles)

@app.route("/articles", methods=["POST"])
def create_article():
    data = request.get_json()
    article = {"id": len(articles) + 1, "title": data["title"]}
    articles.append(article)
    return jsonify(article), 201

@app.route("/articles/<int:article_id>", methods=["GET"])
def get_article(article_id):
    article = next((a for a in articles if a["id"] == article_id), None)
    if article is None:
        return jsonify({"error": "Not found"}), 404
    return jsonify(article)

if __name__ == "__main__":
    app.run(debug=True)
```

### Q79. What are Flask Blueprints and why use them?
Blueprints let you organize a Flask app into modular, reusable components (similar to Django "apps").
```python
# articles/routes.py
from flask import Blueprint, jsonify

articles_bp = Blueprint("articles", __name__, url_prefix="/articles")

@articles_bp.route("/")
def list_articles():
    return jsonify([])

# app.py
from flask import Flask
from articles.routes import articles_bp

app = Flask(__name__)
app.register_blueprint(articles_bp)
```

### Q80. How does Flask handle application context and request context?
Flask uses **context locals** (`current_app`, `g`, `request`, `session`) that are thread-local/greenlet-local proxies, valid only during an active request or app context — this lets you write code like `request.args` without explicitly passing the request object everywhere.
```python
from flask import g, request

@app.before_request
def load_user():
    g.user = get_user_from_token(request.headers.get("Authorization"))

@app.route("/me")
def me():
    return jsonify({"user": g.user})
```

### Q81. Flask extensions you should know: SQLAlchemy, Marshmallow, Flask-Migrate.
```python
from flask_sqlalchemy import SQLAlchemy
from flask_migrate import Migrate

app = Flask(__name__)
app.config["SQLALCHEMY_DATABASE_URI"] = "postgresql://user:pass@localhost/db"
db = SQLAlchemy(app)
migrate = Migrate(app, db)

class Article(db.Model):
    id = db.Column(db.Integer, primary_key=True)
    title = db.Column(db.String(200), nullable=False)
```

### Q82. How do you handle errors globally in Flask?
```python
from werkzeug.exceptions import HTTPException

@app.errorhandler(404)
def not_found(e):
    return jsonify({"error": "Resource not found"}), 404

@app.errorhandler(Exception)
def handle_exception(e):
    if isinstance(e, HTTPException):
        return e
    return jsonify({"error": "Internal server error"}), 500
```

---

## 13. Web Frameworks: FastAPI

### Q83. Why has FastAPI become popular? Key features.
- Built on **Starlette** (ASGI, async-native) + **Pydantic** (data validation via type hints).
- Automatic **OpenAPI/Swagger** docs generation from type hints.
- Native `async`/`await` support for high-throughput I/O-bound APIs.
- Automatic request validation, serialization, and clear error messages from Pydantic models.
- Performance comparable to Node.js/Go frameworks (via Starlette + `uvicorn`/ASGI).

### Q84. Basic FastAPI app with request/response models.
```python
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from typing import List, Optional

app = FastAPI()

class ArticleCreate(BaseModel):
    title: str
    content: str

class Article(ArticleCreate):
    id: int

fake_db: List[Article] = []

@app.post("/articles", response_model=Article, status_code=201)
async def create_article(article: ArticleCreate):
    new_article = Article(id=len(fake_db) + 1, **article.dict())
    fake_db.append(new_article)
    return new_article

@app.get("/articles/{article_id}", response_model=Article)
async def get_article(article_id: int):
    for article in fake_db:
        if article.id == article_id:
            return article
    raise HTTPException(status_code=404, detail="Article not found")

@app.get("/articles", response_model=List[Article])
async def list_articles(skip: int = 0, limit: int = 10):
    return fake_db[skip: skip + limit]
```
Run with: `uvicorn main:app --reload`. Swagger UI auto-available at `/docs`.

### Q85. What is dependency injection in FastAPI?
```python
from fastapi import Depends

def get_db():
    db = SessionLocal()
    try:
        yield db          # dependency with cleanup, like a context manager
    finally:
        db.close()

@app.get("/users/{user_id}")
async def read_user(user_id: int, db=Depends(get_db)):
    return db.query(User).filter(User.id == user_id).first()
```
`Depends()` lets FastAPI manage shared resources (DB sessions, auth checks, config) declaratively and supports easy overriding in tests.

### Q86. How does FastAPI validate request data, and what happens on invalid input?
Pydantic models validate incoming JSON automatically based on type hints; invalid data returns a `422 Unprocessable Entity` with a detailed JSON error — no manual validation code needed.
```python
class UserCreate(BaseModel):
    email: str
    age: int

    class Config:
        str_strip_whitespace = True

# POST {"email": "a@b.com", "age": "not-a-number"}
# -> 422 automatically, with a JSON body pinpointing the "age" field error
```

### Q87. FastAPI vs Flask vs Django — quick comparison for interviews.

| Aspect | Django | Flask | FastAPI |
|---|---|---|---|
| Type | Full-stack, batteries-included | Micro-framework | Modern, async-first API framework |
| Async support | Partial (since 3.1, improving) | Limited (via extensions) | Native, first-class |
| Validation | Django Forms/DRF serializers | Manual / extensions | Built-in via Pydantic |
| Auto API docs | Via DRF + drf-spectacular | Via extensions | Built-in (Swagger/ReDoc) |
| ORM | Built-in | Not built-in (use SQLAlchemy) | Not built-in (use SQLAlchemy/Tortoise) |
| Best for | Full web apps, admin-heavy systems | Small services, flexible custom stacks | High-performance async APIs, microservices |

---

## 14. Databases: SQL Fundamentals

### Q88. What are the different types of SQL JOINs?
```sql
-- INNER JOIN: only matching rows in both tables
SELECT a.title, u.username
FROM articles a
INNER JOIN users u ON a.author_id = u.id;

-- LEFT JOIN: all rows from left table, matched rows (or NULL) from right
SELECT a.title, u.username
FROM articles a
LEFT JOIN users u ON a.author_id = u.id;

-- RIGHT JOIN: all rows from right table, matched (or NULL) from left
-- FULL OUTER JOIN: all rows from both, NULLs where no match
-- CROSS JOIN: Cartesian product of both tables (every row x every row)
```

### Q89. What is database normalization? Explain 1NF, 2NF, 3NF briefly.
Normalization organizes data to reduce redundancy and improve integrity.
- **1NF**: Atomic columns — no repeating groups/arrays in a single field.
- **2NF**: 1NF + every non-key column depends on the **whole** primary key (relevant for composite keys).
- **3NF**: 2NF + no transitive dependencies (non-key columns depend only on the primary key, not on other non-key columns).

**Denormalization** (intentionally duplicating data) is sometimes used to trade write-complexity for read-performance in read-heavy systems.

### Q90. What is a database index, and what's the tradeoff?
An index is a data structure (commonly a B-tree) that speeds up row lookups on a column, at the cost of extra storage and slower writes (the index must be updated on every INSERT/UPDATE/DELETE).
```sql
CREATE INDEX idx_articles_author_id ON articles(author_id);
CREATE UNIQUE INDEX idx_users_email ON users(email);
```
Use indexes on columns frequently used in `WHERE`, `JOIN`, and `ORDER BY` clauses; avoid over-indexing tables with heavy write loads.

### Q91. Explain ACID properties.
- **Atomicity** — a transaction is all-or-nothing.
- **Consistency** — a transaction brings the DB from one valid state to another, respecting constraints.
- **Isolation** — concurrent transactions don't interfere with each other (governed by isolation levels: Read Uncommitted, Read Committed, Repeatable Read, Serializable).
- **Durability** — once committed, data survives crashes (written to durable storage).

### Q92. What is a transaction, and how do you use one in raw Python (`sqlite3`/`psycopg2`)?
```python
import sqlite3

conn = sqlite3.connect("app.db")
try:
    cur = conn.cursor()
    cur.execute("UPDATE accounts SET balance = balance - 100 WHERE id = 1")
    cur.execute("UPDATE accounts SET balance = balance + 100 WHERE id = 2")
    conn.commit()      # both succeed together
except Exception:
    conn.rollback()    # both roll back together on any failure
    raise
finally:
    conn.close()
```

### Q93. What is SQL injection, and how do you prevent it in Python?
SQL injection happens when untrusted input is concatenated directly into a SQL query string, letting attackers alter query logic.
```python
# VULNERABLE:
cur.execute(f"SELECT * FROM users WHERE username = '{username}'")
# If username = "' OR '1'='1", this returns ALL users!

# SAFE: use parameterized queries
cur.execute("SELECT * FROM users WHERE username = ?", (username,))
```
ORMs (SQLAlchemy, Django ORM) parameterize queries automatically under the hood, which is one of their key security benefits over raw string-built SQL.

### Q94. GROUP BY and aggregate functions example.
```sql
SELECT author_id, COUNT(*) AS article_count, AVG(word_count) AS avg_words
FROM articles
GROUP BY author_id
HAVING COUNT(*) > 5
ORDER BY article_count DESC;
```
`WHERE` filters rows before grouping; `HAVING` filters groups after aggregation.

---

## 15. Databases: ORMs

### Q95. What is an ORM, and what problems does it solve?
An Object-Relational Mapper maps database tables to Python classes and rows to instances, letting you interact with the database using Python objects instead of raw SQL. Benefits: less boilerplate, database-agnostic code (mostly), automatic parameterization (SQL-injection safety), and migrations tooling. Tradeoffs: potential performance overhead, "leaky abstraction" for complex queries, and the classic **N+1 query problem** if not used carefully.

### Q96. SQLAlchemy — Core vs ORM, and a basic model example.
- **SQLAlchemy Core**: expression language for building SQL queries programmatically (closer to raw SQL, more control).
- **SQLAlchemy ORM**: higher-level, maps Python classes to tables (built on top of Core).

```python
from sqlalchemy import create_engine, Column, Integer, String, ForeignKey
from sqlalchemy.orm import declarative_base, relationship, sessionmaker

Base = declarative_base()

class Author(Base):
    __tablename__ = "authors"
    id = Column(Integer, primary_key=True)
    name = Column(String(100), nullable=False)
    articles = relationship("Article", back_populates="author")

class Article(Base):
    __tablename__ = "articles"
    id = Column(Integer, primary_key=True)
    title = Column(String(200), nullable=False)
    author_id = Column(Integer, ForeignKey("authors.id"))
    author = relationship("Author", back_populates="articles")

engine = create_engine("postgresql://user:pass@localhost/mydb")
Base.metadata.create_all(engine)

Session = sessionmaker(bind=engine)
session = Session()

# Querying
new_author = Author(name="Jane Doe")
session.add(new_author)
session.commit()

authors = session.query(Author).filter(Author.name.like("%Jane%")).all()

# Eager loading to avoid N+1 (SQLAlchemy equivalent of select_related/prefetch_related)
from sqlalchemy.orm import joinedload
articles = session.query(Article).options(joinedload(Article.author)).all()
```

### Q97. Django ORM: common QuerySet operations.
```python
# CRUD
Article.objects.create(title="New", content="...", author=user)
Article.objects.filter(author=user).update(title="Updated")
Article.objects.filter(id=5).delete()

# Filtering & lookups
Article.objects.filter(title__icontains="python")
Article.objects.filter(published_at__year=2024)
Article.objects.exclude(author=user)

# Aggregation
from django.db.models import Count, Avg
Article.objects.values("author").annotate(total=Count("id")).order_by("-total")

# Q objects for complex OR/AND logic
from django.db.models import Q
Article.objects.filter(Q(title__icontains="python") | Q(content__icontains="python"))

# F objects for field-to-field comparisons (avoids race conditions)
from django.db.models import F
Article.objects.filter(views__gt=F("shares"))
Article.objects.update(views=F("views") + 1)   # atomic increment at the DB level
```

### Q98. What is lazy evaluation in QuerySets, and why does it matter?
Django QuerySets are **lazy** — they don't hit the database until evaluated (iterated, sliced with a concrete index, `list()`-ed, or used in a boolean context).
```python
qs = Article.objects.filter(author=user)   # NO query executed yet
qs = qs.filter(published_at__year=2024)     # still no query, just builds up
articles = list(qs)                          # NOW the query executes
```
This allows chaining filters efficiently, but be aware: iterating the same unevaluated QuerySet twice in certain contexts can trigger duplicate queries unless cached (Django caches results after the first full evaluation of a given QuerySet instance).

### Q99. Alembic migrations (SQLAlchemy) vs Django migrations — conceptually similar?
Yes — both provide version-controlled, incremental schema change scripts.
```bash
# Alembic (used with SQLAlchemy / Flask / FastAPI)
alembic revision --autogenerate -m "add articles table"
alembic upgrade head
alembic downgrade -1
```
Both tools track schema state and generate migration scripts by diffing models against the current database schema.

---

## 16. Databases: NoSQL

### Q100. SQL vs NoSQL — when to choose which?

| Aspect | SQL (Relational) | NoSQL |
|---|---|---|
| Schema | Fixed, defined upfront | Flexible / schema-less |
| Scaling | Vertical (mostly), harder to shard | Horizontal, built for distributed scale |
| Relationships | Strong (JOINs, FKs) | Weaker, often denormalized |
| Consistency | Strong (ACID) | Often eventual consistency (BASE) |
| Examples | PostgreSQL, MySQL | MongoDB, Redis, Cassandra, DynamoDB |
| Best for | Complex relationships, transactions, reporting | High write-throughput, flexible/evolving schemas, caching, huge scale |

### Q101. Basic PyMongo (MongoDB) example.
```python
from pymongo import MongoClient

client = MongoClient("mongodb://localhost:27017/")
db = client["blog"]
articles = db["articles"]

# Insert
articles.insert_one({"title": "Python Tips", "tags": ["python", "tips"], "views": 0})

# Query
result = articles.find_one({"title": "Python Tips"})
for doc in articles.find({"tags": "python"}):
    print(doc)

# Update
articles.update_one({"title": "Python Tips"}, {"$inc": {"views": 1}})

# Aggregation pipeline
pipeline = [
    {"$match": {"tags": "python"}},
    {"$group": {"_id": "$author", "total_views": {"$sum": "$views"}}},
    {"$sort": {"total_views": -1}}
]
list(articles.aggregate(pipeline))
```

### Q102. Redis basics with `redis-py` — common use cases.
```python
import redis

r = redis.Redis(host="localhost", port=6379, db=0)

# Caching
r.set("user:1:name", "Alice", ex=3600)      # expires in 1 hour
r.get("user:1:name")                          # b'Alice'

# Counters (atomic)
r.incr("page:home:views")

# Lists / Queues
r.lpush("task_queue", "job1")
r.rpop("task_queue")

# Pub/Sub
pubsub = r.pubsub()
pubsub.subscribe("notifications")
```
**Common Redis use cases**: caching (reduce DB load), session storage, rate limiting, distributed locks, task queues (with Celery as broker), leaderboard/ranking (`ZADD`/sorted sets), real-time pub/sub messaging.

### Q103. What does "eventual consistency" mean, and when is it acceptable?
In distributed NoSQL systems, after a write, not all replicas may reflect it immediately — but they will **converge** given enough time without new writes. Acceptable for use cases like social media likes/counters, product view counts, or caches, where a brief staleness window is tolerable; not acceptable for financial transactions or inventory counts where correctness must be immediate (favor strongly-consistent SQL there).

---

## 17. Testing in Python

### Q104. `unittest` vs `pytest` — key differences.
```python
# unittest (standard library, class-based, JUnit-style)
import unittest

class TestMath(unittest.TestCase):
    def test_addition(self):
        self.assertEqual(1 + 1, 2)

    def setUp(self):
        self.data = [1, 2, 3]

if __name__ == "__main__":
    unittest.main()

# pytest (third-party, function-based, more concise, powerful fixtures)
def test_addition():
    assert 1 + 1 == 2

import pytest
@pytest.fixture
def sample_data():
    return [1, 2, 3]

def test_sum(sample_data):
    assert sum(sample_data) == 6
```
`pytest` offers plain `assert` statements (better failure diffs), fixtures with dependency injection, parametrization, and a huge plugin ecosystem — the de facto standard in most modern Python codebases.

### Q105. What is `pytest.mark.parametrize`?
```python
import pytest

@pytest.mark.parametrize("input_val,expected", [
    (2, 4),
    (3, 9),
    (4, 16),
])
def test_square(input_val, expected):
    assert input_val ** 2 == expected
```
Runs the same test logic against multiple input/output pairs — avoids repetitive test functions.

### Q106. How do you mock dependencies in tests?
```python
from unittest.mock import patch, MagicMock

def get_weather(api_client):
    return api_client.fetch("weather")

def test_get_weather():
    mock_client = MagicMock()
    mock_client.fetch.return_value = {"temp": 72}
    result = get_weather(mock_client)
    assert result == {"temp": 72}
    mock_client.fetch.assert_called_once_with("weather")

# Patching a module-level dependency
@patch("myapp.services.requests.get")
def test_api_call(mock_get):
    mock_get.return_value.status_code = 200
    mock_get.return_value.json.return_value = {"ok": True}
    # ... call code that internally uses requests.get ...
```
Mocking isolates the unit under test from external dependencies (databases, APIs, filesystems), making tests fast and deterministic.

### Q107. What is test coverage, and how do you measure it?
```bash
pip install coverage
coverage run -m pytest
coverage report -m
coverage html   # generates an interactive HTML report
```
Coverage measures the percentage of code lines/branches executed during tests. High coverage doesn't guarantee correctness (a line can be "hit" without being properly asserted), but low coverage clearly signals untested code.

### Q108. What are fixtures with different scopes in pytest?
```python
import pytest

@pytest.fixture(scope="function")   # default: fresh instance per test
def db_session():
    session = create_session()
    yield session
    session.close()

@pytest.fixture(scope="module")     # shared across all tests in a file
def api_client():
    return APIClient(base_url="http://test")

@pytest.fixture(scope="session")    # shared across the entire test run
def db_engine():
    engine = create_engine("sqlite:///:memory:")
    yield engine
    engine.dispose()
```
Scope controls fixture lifetime/reuse — narrower scopes (`function`) ensure test isolation; wider scopes (`session`) improve performance for expensive setup (e.g., spinning up a test database engine once).

---

## 18. Packaging, Tooling & Best Practices

### Q109. Why use virtual environments?
Virtual environments isolate project dependencies so different projects can use different (potentially conflicting) package versions without polluting the global Python installation.
```bash
python -m venv venv
source venv/bin/activate      # Linux/macOS
venv\Scripts\activate          # Windows

pip install -r requirements.txt
pip freeze > requirements.txt
```
Modern alternatives: `poetry`, `pipenv`, or `uv` (fast Rust-based resolver/installer) manage both virtual environments and lockfiles together.

### Q110. What is the difference between `requirements.txt` and `pyproject.toml`?
- **`requirements.txt`**: a flat list of pinned dependencies for `pip install -r`. Simple but no metadata about the project itself.
- **`pyproject.toml`** (PEP 518/621): the modern standard — declares build system, project metadata (name, version, dependencies), and tool configuration (`black`, `pytest`, `mypy`) all in one file. Used by Poetry, Hatch, Flit, and modern `pip`/`setuptools`.

```toml
[project]
name = "my-package"
version = "0.1.0"
dependencies = ["requests>=2.28", "pydantic>=2.0"]

[build-system]
requires = ["setuptools>=61.0"]
build-backend = "setuptools.build_meta"
```

### Q111. What is `__name__ == "__main__"` for?
Every module has a `__name__` attribute; when a file is run directly it's `"__main__"`, but when imported it's the module's name. This guard lets a file be both an importable module and a runnable script.
```python
def main():
    print("Running as a script")

if __name__ == "__main__":
    main()      # only runs when executed directly, not when imported
```

### Q112. What are common linters/formatters in the Python ecosystem?
- **`black`** — opinionated auto-formatter (no config debates).
- **`ruff`** — extremely fast linter (increasingly replacing `flake8`/`isort`/`pyupgrade`), written in Rust.
- **`mypy` / `pyright`** — static type checkers.
- **`isort`** — import sorting.
- **`pre-commit`** — runs these checks automatically before each git commit.

### Q113. What is the difference between `pip install package` and `pip install -e .`?
`-e` (editable install) links the package to your local source directory instead of copying it into `site-packages` — changes to source code are reflected immediately without reinstalling. Commonly used during local development of a package.

### Q114. How do you structure a production-grade Python project?
```
myproject/
├── src/
│   └── myproject/
│       ├── __init__.py
│       ├── models.py
│       ├── services/
│       └── api/
├── tests/
│   ├── unit/
│   └── integration/
├── pyproject.toml
├── README.md
├── .env.example
└── Dockerfile
```
The `src/` layout (vs. a flat layout) prevents accidentally importing the package from the working directory instead of the installed version — catches packaging bugs early.

---

## 19. Rapid-Fire Q&A Round

Quick-hit questions useful for warm-up rounds or screening calls.

**Q. What's the difference between `append()` and `extend()` on a list?**
`append(x)` adds `x` as a single element; `extend(iterable)` adds each element of the iterable individually.
```python
[1, 2].append([3, 4])   # [1, 2, [3, 4]]
[1, 2].extend([3, 4])   # [1, 2, 3, 4]
```

**Q. How do you swap two variables in Python?**
```python
a, b = 1, 2
a, b = b, a     # tuple packing/unpacking, no temp variable needed
```

**Q. What does `*` do when unpacking a list?**
```python
first, *middle, last = [1, 2, 3, 4, 5]
# first = 1, middle = [2, 3, 4], last = 5
```

**Q. Difference between `range()` in Python 2 vs Python 3?**
Python 2's `range()` returns a list (eager); Python 3's `range()` returns a lazy, memory-efficient `range` object (similar to how `xrange()` worked in Python 2).

**Q. What is the difference between `.pyc` and `.py` files?**
`.py` is source code; `.pyc` is the compiled bytecode cache (in `__pycache__/`), used by the interpreter to skip re-compiling unchanged source on subsequent runs.

**Q. How do you check an object's type at runtime?**
```python
type(obj) == int          # exact type match, avoid — doesn't account for subclasses
isinstance(obj, int)       # PREFERRED — respects inheritance
```

**Q. What's the output of `0.1 + 0.2 == 0.3` and why?**
`False` — floating point numbers use binary representation (IEEE 754) that can't exactly represent most decimal fractions, causing tiny rounding errors. Use `math.isclose()` for float comparisons, or the `decimal` module for exact decimal arithmetic (e.g., financial calculations).

**Q. What is `__init__.py` used for?**
Marks a directory as a Python package (required in older Python; optional "namespace packages" exist since 3.3, but `__init__.py` is still common for explicit package init logic and controlling `from package import *` via `__all__`).

**Q. Difference between `json.dumps` / `loads` and `dump` / `load`?**
`dumps`/`loads` work with strings; `dump`/`load` work with file objects directly.
```python
import json
json.dumps({"a": 1})            # -> string
json.dump({"a": 1}, open("f.json", "w"))   # -> writes to file
```

**Q. What is the difference between `del`, `remove()`, and `pop()` on a list?**
```python
lst = [10, 20, 30]
del lst[0]        # removes by index, no return value
lst.remove(20)     # removes by VALUE (first match), no return value
lst.pop()           # removes by index (default last), RETURNS the removed item
```

**Q. What does `if __debug__:` do?**
`__debug__` is `True` unless Python is run with the `-O` (optimize) flag, which also strips `assert` statements — a way to write debug-only checks that vanish in optimized/production builds.

**Q. What is duck typing vs structural typing (`Protocol`)?**
Duck typing is runtime, implicit ("if it has the method, it works"). `typing.Protocol` (PEP 544) brings **static structural typing** — type checkers can verify duck-typed compatibility ahead of runtime.
```python
from typing import Protocol

class Quacker(Protocol):
    def quack(self) -> str: ...

def make_it_quack(thing: Quacker) -> str:
    return thing.quack()
```

---

## Final Tips for the Interview

1. **Always explain the "why", not just the "what"** — interviewers probe for reasoning (e.g., not just "use `select_related`" but "because it avoids the N+1 query problem via a SQL JOIN").
2. **Write runnable, minimal examples** on the whiteboard/editor — avoid over-engineering during live coding.
3. **Know the tradeoffs** — almost every technical answer in Python has a "it depends" nuance (list vs generator, threading vs multiprocessing, SQL vs NoSQL). Naming the tradeoff signals seniority.
4. **Be ready to discuss real production experience** — GIL implications, N+1 queries, caching strategy, migrations, testing strategy — these come from lived experience, not just textbook knowledge.
5. **Practice explaining complexity** (Big-O) for common data structure operations — interviewers frequently follow up "what's the time complexity of that?"

Good luck with your interview!

