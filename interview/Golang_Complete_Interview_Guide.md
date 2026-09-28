# The Complete Go (Golang) Interview Guide & Tutorial

> A single reference covering interview questions with detailed answers, the full language tutorial, Go's runtime internals, MVC web frameworks, and database implementation.
>
> Written against modern Go (1.22+). Version-specific features are marked where relevant.

---

## Table of Contents

**PART I — INTERVIEW QUESTIONS & ANSWERS**
1. [Basics & Language Fundamentals](#1-basics--language-fundamentals)
2. [Types, Variables & Constants](#2-types-variables--constants)
3. [Arrays, Slices & Maps](#3-arrays-slices--maps)
4. [Strings, Bytes & Runes](#4-strings-bytes--runes)
5. [Functions, Closures & Defer](#5-functions-closures--defer)
6. [Structs, Methods & Embedding](#6-structs-methods--embedding)
7. [Interfaces](#7-interfaces)
8. [Error Handling, Panic & Recover](#8-error-handling-panic--recover)
9. [Goroutines & Concurrency](#9-goroutines--concurrency)
10. [Channels & Select](#10-channels--select)
11. [sync Package & Memory Model](#11-sync-package--memory-model)
12. [Context](#12-context)
13. [Generics](#13-generics)
14. [Memory Management & Garbage Collection](#14-memory-management--garbage-collection)
15. [Packages, Modules & Tooling](#15-packages-modules--tooling)
16. [Testing, Benchmarking & Profiling](#16-testing-benchmarking--profiling)
17. [Tricky Output / Gotcha Questions](#17-tricky-output--gotcha-questions)
18. [Coding Problems with Solutions](#18-coding-problems-with-solutions)
19. [System Design Questions in Go](#19-system-design-questions-in-go)

**PART II — COMPLETE GO TUTORIAL (THEORY)**
20. [Complete Go Tutorial](#part-ii--complete-go-tutorial)

**PART III — INNER ARCHITECTURE OF GO**
21. [Compiler, Runtime, Scheduler, Memory, GC, Data Structure Internals](#part-iii--inner-architecture-of-go)

**PART IV — WEB DEVELOPMENT: MVC & FRAMEWORKS**
22. [net/http, MVC, Gin, Echo, Fiber, Chi, Middleware](#part-iv--web-development-mvc--frameworks)

**PART V — DATABASE IMPLEMENTATION**
23. [database/sql, pgx, sqlx, GORM, sqlc, Transactions, Migrations, Repository Pattern](#part-v--database-implementation)

**APPENDIX**
- [Cheat Sheet & Interview Tips](#appendix--cheat-sheet--interview-tips)

---

# PART I — INTERVIEW QUESTIONS & ANSWERS

## 1. Basics & Language Fundamentals

### Q1. What is Go and why was it created?
**Answer:** Go is a statically typed, compiled, garbage-collected language designed at Google (2007, public 2009) by Robert Griesemer, Rob Pike and Ken Thompson. It was created to address problems in large-scale software development at Google:
- **Slow builds** in C++ — Go compiles very fast thanks to a simple dependency model (no header files, unused imports are errors, each package compiled once).
- **Complexity** — Go has a small spec (~25 keywords), no inheritance, no exceptions, minimal implicit behaviour.
- **Concurrency** — multicore machines and networked servers needed first-class concurrency: goroutines and channels (based on Hoare's CSP).
- **Dependency management and readability** — `gofmt` enforces one style; the language favours explicitness.

### Q2. What are the key features of Go?
- Statically typed with type inference (`:=`)
- Compiles to a single native, statically linked binary (easy deploy, cross compilation via `GOOS`/`GOARCH`)
- Garbage collected (concurrent, tri-color mark-and-sweep, low latency)
- Goroutines (lightweight threads, ~2 KB initial stack) and channels
- Interfaces satisfied implicitly (structural typing)
- Composition over inheritance (struct embedding)
- Multiple return values, explicit errors
- Generics (since 1.18)
- Rich standard library (`net/http`, `encoding/json`, `crypto`, `testing`…)
- Built-in tooling: `go build`, `go test`, `go vet`, `go fmt`, `go mod`, `pprof`, race detector

### Q3. Is Go object-oriented?
**Answer:** "Yes and no." Go has types and methods and allows an object-oriented style, but there is **no type hierarchy / class inheritance**. Instead:
- **Encapsulation** → package-level visibility (Uppercase = exported, lowercase = unexported).
- **Polymorphism** → interfaces (implicit implementation).
- **Code reuse** → composition through struct embedding.
Methods can be defined on any named type (not only structs), e.g. `type Celsius float64`.

### Q4. How many keywords does Go have?
25:
```
break     default      func    interface  select
case      defer        go      map        struct
chan      else         goto    package    switch
const     fallthrough  if      range      type
continue  for          import  return     var
```
Note `any`, `comparable`, `true`, `false`, `nil`, `iota`, `len`, `make`, etc. are **predeclared identifiers**, not keywords — they can technically be shadowed.

### Q5. What is the difference between `var x = 5`, `var x int`, and `x := 5`?
- `var x int` — declaration with zero value (`0`).
- `var x = 5` — declaration with initialization; type inferred.
- `x := 5` — short variable declaration; only inside functions; infers type. In a multi-assignment `a, err := f()`, at least one variable on the left must be new; existing ones in the **same scope** are reassigned (a common source of shadowing bugs in nested scopes).

### Q6. What are zero values?
Every variable is initialized. Zero values:
| Type | Zero value |
|---|---|
| numeric | `0` |
| bool | `false` |
| string | `""` |
| pointer, slice, map, chan, func, interface | `nil` |
| struct | each field zero |
| array | each element zero |

Good Go design makes zero values useful: `sync.Mutex{}` is an unlocked mutex, `bytes.Buffer{}` is an empty ready-to-use buffer, a nil slice can be appended to.

### Q7. What is the `init()` function? What is the order of initialization?
`init()` is a special function with no args and no return that runs automatically. A package can have multiple `init()`s (even per file).
Order:
1. Imported packages are initialized first (depth-first, each package once).
2. Package-level variables are initialized in dependency order.
3. `init()` functions run in the order they appear (files presented to the compiler, typically sorted by name).
4. Finally `main.main()` runs.

`init` cannot be called or referenced manually. Overuse is discouraged (hidden side effects, harder testing).

### Q8. What is the blank identifier `_`?
A write-only placeholder used to discard values:
```go
_, err := strconv.Atoi("42")             // ignore value
import _ "github.com/lib/pq"              // import for side effects (init registers driver)
var _ io.Reader = (*MyReader)(nil)        // compile-time interface check
for _, v := range items {}                // ignore index
```

### Q9. Does Go have a `while` loop?
No — `for` is the only loop:
```go
for i := 0; i < 10; i++ {}     // classic
for cond {}                    // while
for {}                         // infinite
for i, v := range slice {}     // range
for i := range 10 {}           // range over int (Go 1.22+)
for k, v := range seq {}       // range over iterator funcs (Go 1.23+)
```

### Q10. How does `switch` differ from C?
- No automatic fall-through; each case breaks implicitly. Use `fallthrough` explicitly.
- Cases can be expressions, not only constants; multiple values per case: `case 1, 2, 3:`.
- Tagless switch acts like if/else chain: `switch { case x > 10: ... }`.
- Type switch: `switch v := i.(type) { case int: ... }`.

### Q11. What is `goto` and labels used for?
Labels allow `break`/`continue` out of nested loops:
```go
outer:
for i := 0; i < 3; i++ {
    for j := 0; j < 3; j++ {
        if j == 2 { continue outer }
        if i == 2 { break outer }
    }
}
```
`goto` exists but rarely used; it cannot jump over variable declarations or into blocks.

### Q12. What is the difference between `new` and `make`?
| `new(T)` | `make(T, args)` |
|---|---|
| Works for any type | Only slice, map, channel |
| Allocates zeroed storage, returns `*T` | Initializes internal data structure, returns `T` (not pointer) |
| `p := new(int)` → `*int` pointing to 0 | `s := make([]int, 0, 10)` |

`new(map[string]int)` returns a pointer to a **nil** map — writing to it panics. That's why maps need `make`. (Since Go 1.26, `new` also accepts an expression, e.g. `new(42)` returning `*int`.)

### Q13. Is Go pass-by-value or pass-by-reference?
**Everything is pass-by-value.** A copy of the argument is made. However some types are *descriptors* containing pointers:
- Slice → copy of header `{ptr, len, cap}`; modifying elements affects caller, but `append` that reallocates doesn't.
- Map, channel → internally pointers to runtime structures; modifications visible.
- Pointer → copy of address.
- Struct/array → full copy (large arrays are expensive to pass).

### Q14. What are exported and unexported identifiers?
Identifiers starting with an uppercase letter are exported (public outside the package). Lowercase are package-private. This applies to types, functions, methods, struct fields, constants. Unexported struct fields are ignored by `encoding/json`.

### Q15. What is `gofmt` and why does it matter?
The standard formatter. It removes style debates; all Go code looks the same. `goimports` additionally manages imports. Editors run it on save.

---

## 2. Types, Variables & Constants

### Q16. What basic types does Go have?
- `bool`
- `string` (immutable byte sequence, usually UTF-8)
- Signed ints: `int8 int16 int32 int64 int` (int is 32 or 64 bits by platform)
- Unsigned: `uint8 uint16 uint32 uint64 uint uintptr`
- `byte` = alias `uint8`, `rune` = alias `int32`
- Floats: `float32 float64`
- Complex: `complex64 complex128`

### Q17. Does Go allow implicit type conversion?
No. Even `int` and `int64` require explicit conversion: `var a int = 5; var b int64 = int64(a)`. Untyped constants are the exception—they adapt to context.

### Q18. What are typed vs untyped constants?
```go
const a = 10          // untyped integer constant — arbitrary precision
const b int32 = 10    // typed
var f float64 = a     // OK: untyped adapts
var g float64 = b     // compile error
const huge = 1 << 100 // fine as untyped constant (as long as not assigned to too small type)
```
Untyped constants have a *default type* (int, float64, rune, complex128, string, bool) used when a type is needed.

### Q19. What is `iota`?
A constant generator that resets to 0 in each `const` block and increments per line:
```go
type Weekday int
const (
    Sunday Weekday = iota // 0
    Monday                // 1
    Tuesday               // 2
)

const (
    _  = iota
    KB = 1 << (10 * iota) // 1024
    MB                    // 1048576
    GB
)

type Perm uint8
const (
    Read Perm = 1 << iota // 1
    Write                 // 2
    Exec                  // 4
)
```

### Q20. What is the difference between a type definition and a type alias?
```go
type MyInt int     // definition: NEW distinct type; no methods of int's inherited (int has none anyway); needs conversion
type Alias = int   // alias: SAME type, just another name
```
Aliases exist for gradual refactoring (moving types between packages). `byte`/`rune` are aliases. Generic type aliases are allowed since Go 1.24.

### Q21. What happens on integer overflow?
It wraps around silently (two's complement) at runtime — no panic. Constant overflow is a compile error: `var x int8 = 128` fails.
```go
var x int8 = 127
x++ // -128
```

### Q22. What is `uintptr` vs `unsafe.Pointer`?
- `unsafe.Pointer` — a generic pointer the GC tracks; can convert to/from any pointer type.
- `uintptr` — an integer large enough to hold a pointer; the GC does **not** treat it as a reference, so the object may be moved/collected. Only use it in a single expression for pointer arithmetic: `unsafe.Pointer(uintptr(p) + off)`.

### Q23. How do you check a variable's type at runtime?
`fmt.Printf("%T", v)`, `reflect.TypeOf(v)`, type assertion `v.(T)`, or type switch.

### Q24. What is shadowing?
Declaring a variable in an inner scope with the same name as outer:
```go
err := doA()
if true {
    x, err := doB() // new err shadows outer!
    _ = x
}
// outer err unchanged
```
`go vet -vettool=shadow` (or linters like `govet shadow`) detect it.

---

## 3. Arrays, Slices & Maps

### Q25. Difference between an array and a slice?
| Array | Slice |
|---|---|
| Fixed length, part of the type: `[5]int` ≠ `[6]int` | Dynamic view: `[]int` |
| Value type — assignment copies all elements | Header `{ptr, len, cap}`; copy shares backing array |
| Comparable with `==` (if element comparable) | Not comparable (only to `nil`) |
| Can be map key | Cannot be map key |

### Q26. Explain the internal structure of a slice.
```go
// runtime/slice.go
type slice struct {
    array unsafe.Pointer // pointer to backing array
    len   int
    cap   int
}
```
Slicing `s[low:high]` creates a new header pointing into the same array: `len = high-low`, `cap = cap(s)-low`. Full slice expression `s[low:high:max]` limits capacity to `max-low` — useful to force `append` to copy and avoid clobbering.

### Q27. How does `append` work and how does the slice grow?
If `len < cap`, append writes in place and returns a header with `len+1`. Otherwise it allocates a new, larger array, copies, and appends. Growth policy (Go 1.18+ `runtime.growslice`):
- If needed capacity > double old cap → use needed cap.
- If old cap < 256 → double.
- Else grow by `(newcap + 3*256) / 4` repeatedly (≈1.25x transitioning smoothly from 2x).
- Result rounded up to the nearest **size class** of the memory allocator, so actual cap may be larger.

**Always assign the result**: `s = append(s, x)`.

### Q28. What is the classic "shared backing array" bug?
```go
a := []int{1, 2, 3, 4}
b := a[:2]          // [1 2], cap 4
b = append(b, 99)   // writes into a[2]!
fmt.Println(a)      // [1 2 99 4]
```
Fix: `b := a[:2:2]` or copy: `b := slices.Clone(a[:2])`.

### Q29. nil slice vs empty slice?
```go
var s []int          // nil: ptr=nil, len=0, cap=0; s == nil → true
s2 := []int{}        // empty: ptr non-nil, len=0; s2 == nil → false
```
Both work with `len`, `append`, `range`. Difference shows in JSON: nil → `null`, empty → `[]`. Prefer `var s []int` in general; return empty when API contract needs `[]`.

### Q30. How do you copy a slice?
```go
dst := make([]int, len(src))
n := copy(dst, src)           // copies min(len(dst), len(src))
dst2 := append([]int(nil), src...)
dst3 := slices.Clone(src)     // Go 1.21+
```

### Q31. How do you remove an element from a slice?
```go
s = append(s[:i], s[i+1:]...)       // preserves order, O(n)
s[i] = s[len(s)-1]; s = s[:len(s)-1] // doesn't preserve order, O(1)
s = slices.Delete(s, i, i+1)         // Go 1.21+, zeroes tail (1.22+)
```
For slices of pointers, zero out the tail to avoid memory leaks.

### Q32. What's the memory leak risk with subslices?
A small subslice of a huge array keeps the entire array alive:
```go
func firstBytes(big []byte) []byte { return big[:10] } // pins whole big array
func fixed(big []byte) []byte     { return bytes.Clone(big[:10]) }
```

### Q33. How are maps implemented?
Before Go 1.24: hash table of **buckets**, each holding 8 key/value pairs plus `tophash` bytes (top 8 bits of hash), overflow buckets chained; grows when load factor > 6.5 with **incremental evacuation** (copying buckets gradually during writes).

Since Go 1.24: **Swiss Tables** — open addressing with groups of 8 slots and a control word of metadata bytes (7 bits of hash + state) allowing SIMD-like parallel probing; the map is split into multiple tables (directory) that grow independently, limiting growth latency. Faster lookups and lower memory. (See Part III.)

### Q34. Is map iteration order guaranteed?
No — it's intentionally randomized. To iterate in order, collect and sort keys:
```go
keys := slices.Sorted(maps.Keys(m)) // Go 1.23+
for _, k := range keys { fmt.Println(k, m[k]) }
```

### Q35. Are maps safe for concurrent use?
No. Concurrent write + read/write causes a **fatal error** "concurrent map writes" (not recoverable). Options: `sync.RWMutex` around the map, `sync.Map` (optimized for keys written once/read many or disjoint keys per goroutine), or sharded maps.

### Q36. What types can be map keys?
Any **comparable** type: bool, numbers, strings, pointers, channels, interfaces (panics at runtime if dynamic type not comparable), arrays of comparable, structs of comparable fields. Not: slices, maps, functions.

### Q37. How do you check if a key exists?
```go
v, ok := m["key"]
if !ok { /* missing */ }
```
Reading a missing key returns the zero value. Reading/deleting from a nil map is fine; writing to a nil map panics.

### Q38. Can you take the address of a map element?
No: `&m["k"]` is a compile error because map elements may move during growth. Same reason you can't do `m["k"].field = 1` for struct values; store pointers or reassign the whole struct.

### Q39. Does deleting from a map during range work?
Yes, it's safe. Entries deleted before being reached won't be produced. Added entries may or may not be produced.

### Q40. Does a map shrink after deleting elements?
No. The backing memory is not released; recreate the map to shrink. `clear(m)` (Go 1.21) deletes all entries but keeps capacity.

---

## 4. Strings, Bytes & Runes

### Q41. How are strings represented?
```go
type stringStruct struct { str unsafe.Pointer; len int }
```
An immutable, read-only byte sequence. `len(s)` is **bytes**, not characters. Indexing `s[i]` returns a `byte`.

### Q42. What's a rune?
An alias for `int32` representing a Unicode code point. `range` over a string decodes UTF-8 and yields `(byteIndex, rune)`:
```go
s := "héllo"
fmt.Println(len(s))                    // 6 (é is 2 bytes)
fmt.Println(utf8.RuneCountInString(s)) // 5
for i, r := range s { fmt.Println(i, string(r)) } // indexes 0,1,3,4,5
```
Invalid UTF-8 bytes yield `U+FFFD`.

### Q43. How do you efficiently concatenate strings?
`+` in a loop is O(n²) due to copies. Use `strings.Builder`:
```go
var b strings.Builder
b.Grow(64)
for i := 0; i < 10; i++ { fmt.Fprintf(&b, "%d,", i) }
s := b.String() // no copy
```
Alternatives: `strings.Join`, `bytes.Buffer`.

### Q44. Does converting between `string` and `[]byte` copy?
Generally yes (strings are immutable). The compiler optimizes some cases: `m[string(b)]` map lookup, `string(b) == "x"` comparisons, `for range []byte(s)`. `unsafe.String`/`unsafe.StringData` allow zero-copy but you must never modify the bytes afterward.

### Q45. How do you reverse a string correctly?
Reverse runes, not bytes:
```go
func Reverse(s string) string {
    r := []rune(s)
    for i, j := 0, len(r)-1; i < j; i, j = i+1, j-1 { r[i], r[j] = r[j], r[i] }
    return string(r)
}
```
(For grapheme clusters like emoji with modifiers, you'd need a segmentation library.)

### Q46. What does `string(65)` produce?
`"A"` — converting an integer to string interprets it as a code point, not a decimal. `go vet` warns. Use `strconv.Itoa(65)` for `"65"`.

---

## 5. Functions, Closures & Defer

### Q47. What are first-class functions in Go?
Functions are values: assign to variables, pass as arguments, return from functions, store in maps. Function types: `type Handler func(int) error`. Functions are not comparable except to `nil`.

### Q48. What is a closure? Explain the loop variable capture issue.
A closure is a function literal that references variables from its enclosing scope; the variables are captured **by reference** (they escape to the heap if needed).
```go
func counter() func() int {
    c := 0
    return func() int { c++; return c }
}
```
**Before Go 1.22**, the loop variable was shared across iterations:
```go
for i := 0; i < 3; i++ { go func() { fmt.Println(i) }() } // could print 3 3 3
```
**Since Go 1.22** (module `go` directive ≥ 1.22), each iteration has a fresh variable, so it prints 0,1,2 in some order. In older code the fix was `i := i` or passing `i` as argument.

### Q49. What are variadic functions?
```go
func sum(nums ...int) int { t := 0; for _, n := range nums { t += n }; return t }
sum(1, 2, 3)
sum(s...) // spread slice (no copy — nums shares s's array!)
```
The variadic parameter must be last; inside it's a slice.

### Q50. What are named return values?
```go
func divide(a, b float64) (result float64, err error) {
    if b == 0 { err = errors.New("div by zero"); return }
    result = a / b
    return // naked return
}
```
Useful for documentation and for **modifying return values in `defer`**. Avoid naked returns in long functions.

### Q51. What is `defer` and how does it work?
`defer` schedules a call to run when the surrounding function returns (normally or via panic). Rules:
1. **Arguments are evaluated immediately** at the defer statement.
2. Deferred calls execute in **LIFO** order.
3. Deferred functions can read/modify **named** return values.
```go
func f() (x int) {
    defer func() { x *= 2 }()
    return 5 // x=5 then defer → 10
}

for i := 0; i < 3; i++ { defer fmt.Print(i) } // prints 210

func g() {
    x := 1
    defer fmt.Println(x) // prints 1 — arg evaluated now
    x = 2
}
```
Implementation: since Go 1.14, **open-coded defers** inline defers at function exit when there are ≤8 and no loops, making defer nearly zero cost; otherwise a defer record is allocated (on stack/heap) and linked into the goroutine's defer chain.

### Q52. Common defer pitfalls?
- `defer` in a loop accumulates until function end (file handle leak): wrap in a function.
- `defer f.Close()` ignores the error; for writes check it:
```go
defer func() {
    if cerr := f.Close(); cerr != nil && err == nil { err = cerr }
}()
```
- `defer resp.Body.Close()` before checking `err` → nil pointer panic.
- `os.Exit` does not run defers; `log.Fatal` calls `os.Exit`.

### Q53. What is a method value vs method expression?
```go
type T struct{ n int }
func (t T) Get() int { return t.n }
t := T{5}
f := t.Get       // method value: receiver bound (copied now)
g := T.Get       // method expression: func(T) int
f(); g(t)
```

### Q54. Does Go support function overloading or default parameters?
No. Use different names, variadic args, option structs, or the **functional options** pattern:
```go
type Server struct { port int; timeout time.Duration }
type Option func(*Server)
func WithPort(p int) Option { return func(s *Server) { s.port = p } }
func NewServer(opts ...Option) *Server {
    s := &Server{port: 8080, timeout: 30 * time.Second}
    for _, o := range opts { o(s) }
    return s
}
srv := NewServer(WithPort(9090))
```

### Q55. Does Go have tail-call optimization?
No guaranteed TCO. Deep recursion grows the goroutine stack (up to 1 GB default max on 64-bit, then fatal "stack overflow").

---

## 6. Structs, Methods & Embedding

### Q56. Value receiver vs pointer receiver — when to use which?
Use a **pointer receiver** when:
- The method modifies the receiver.
- The struct is large (avoid copying).
- The struct contains a `sync.Mutex` or other non-copyable fields.
- Consistency: if some methods need pointers, make all pointer receivers.

Use a **value receiver** for small immutable types (e.g., `time.Time`, points), maps/funcs/chans types.

### Q57. What is a method set? Why does `T` sometimes not satisfy an interface?
- Method set of `T` = methods with value receivers.
- Method set of `*T` = methods with value **and** pointer receivers.
```go
type Speaker interface{ Speak() }
type Dog struct{}
func (d *Dog) Speak() {}
var _ Speaker = &Dog{} // OK
var _ Speaker = Dog{}  // compile error: Speak has pointer receiver
```
Reason: a value stored in an interface is not addressable, so the compiler can't take its address to call a pointer method. However, calling `d.Speak()` on an addressable variable `d` works because Go auto-inserts `(&d).Speak()`.

### Q58. What is struct embedding? Is it inheritance?
```go
type Animal struct{ Name string }
func (a Animal) Describe() string { return "I am " + a.Name }
type Dog struct {
    Animal        // embedded
    Breed string
}
d := Dog{Animal{"Rex"}, "Lab"}
d.Describe()      // promoted method
d.Name            // promoted field
```
It's **composition with automatic delegation**, not inheritance: `Dog` is not an `Animal` (can't pass Dog where Animal expected), and there is no virtual dispatch — if `Animal.Describe` calls another method that `Dog` "overrides", Animal's version is called. Embedding an interface in a struct is also possible (useful for partial mocks/decorators).

### Q59. How are name conflicts resolved with embedding?
The shallowest depth wins. If two embedded types at the same depth provide the same name, accessing it is ambiguous → compile error only if used.

### Q60. Are structs comparable?
Yes if all fields are comparable. Comparison is field-by-field. Structs with slices/maps/funcs can't be compared with `==` (use `reflect.DeepEqual` or custom `Equal`, or `cmp.Equal` in tests).

### Q61. What are struct tags?
String metadata read via reflection:
```go
type User struct {
    ID    int    `json:"id" db:"id"`
    Email string `json:"email,omitempty" validate:"required,email"`
    Pass  string `json:"-"`
}
```

### Q62. What is the empty struct `struct{}` used for?
It occupies **0 bytes** (all zero-size values may share the address `runtime.zerobase`). Uses:
- Sets: `map[string]struct{}`
- Signal channels: `done := make(chan struct{})`
- Method-only types.

### Q63. What is struct field alignment / padding?
Fields are aligned to their size; reordering reduces padding:
```go
type Bad struct { a bool; b int64; c bool }   // 24 bytes
type Good struct { b int64; a bool; c bool }  // 16 bytes
```
Check with `unsafe.Sizeof`, `fieldalignment` linter. 64-bit atomic ops need 8-byte alignment (on 32-bit platforms use `atomic.Int64` type, which guarantees it).

### Q64. How do you implement a constructor?
Go has no constructors; convention is `NewT(...) *T` functions that validate and initialize. Return concrete types, accept interfaces.

### Q65. Can you define methods on types from other packages?
No — methods must be declared in the same package as the type. Wrap it: `type MyTime time.Time` or embed.

---
## 7. Interfaces

### Q66. What is an interface and how is it satisfied?
An interface is a set of method signatures (and, for constraints, type sets). A type satisfies it **implicitly** by having the methods — no `implements` keyword. This decouples packages: consumers define the small interface they need.
```go
type Shape interface { Area() float64; Perimeter() float64 }
type Rect struct{ W, H float64 }
func (r Rect) Area() float64      { return r.W * r.H }
func (r Rect) Perimeter() float64 { return 2 * (r.W + r.H) }
var s Shape = Rect{2, 3}
```

### Q67. How are interfaces represented internally?
Two words:
```go
// non-empty interface
type iface struct {
    tab  *itab          // type info + method table
    data unsafe.Pointer // pointer to the value (or the value itself for pointer-shaped types)
}
type itab struct {
    inter *interfacetype
    _type *_type
    hash  uint32
    fun   [1]uintptr // variable-size array of method pointers (dynamic dispatch table)
}
// empty interface (any)
type eface struct {
    _type *_type
    data  unsafe.Pointer
}
```
`itab`s are computed on first use and cached in a global hash table. Method calls through interfaces are indirect calls via `itab.fun` — they generally prevent inlining, and storing non-pointer values usually causes heap allocation (escape). Small values like 0–255 integers and zero-size values use static data to avoid allocation.

### Q68. Explain the "nil interface is not nil" gotcha.
An interface is nil only if **both** type and value are nil.
```go
type MyErr struct{}
func (*MyErr) Error() string { return "boom" }

func do() error {
    var e *MyErr = nil
    return e // interface{type:*MyErr, value:nil}
}
func main() {
    err := do()
    fmt.Println(err == nil) // false!
}
```
Fix: return a literal `nil` for the interface type when there's no error.

### Q69. What is a type assertion? Type switch?
```go
var i any = "hello"
s := i.(string)        // panics if wrong type
s, ok := i.(string)    // safe form
n, ok := i.(int)       // ok=false, n=0

switch v := i.(type) {
case int:           fmt.Println("int", v)
case string, []byte: fmt.Println("stringish", v) // v is `any` here
case nil:           fmt.Println("nil")
default:            fmt.Printf("%T\n", v)
}
```
You can also assert to another interface: `if w, ok := r.(io.Writer); ok {...}` — this is how optional capabilities are discovered (e.g., `http.Flusher`, `io.WriterTo`).

### Q70. What is `interface{}` / `any`?
The empty interface, satisfied by every type. `any` is an alias since Go 1.18. Use sparingly; prefer generics or specific interfaces for type safety.

### Q71. What are good interface design principles?
- **Keep interfaces small** ("The bigger the interface, the weaker the abstraction"). `io.Reader` has one method.
- **Define interfaces at the consumer**, not the producer.
- **Accept interfaces, return structs.**
- Don't create interfaces before you need them (only for multiple implementations or testing seams).
- Name single-method interfaces with `-er`: `Reader`, `Stringer`, `Closer`.

### Q72. How to ensure at compile time that a type implements an interface?
```go
var _ http.Handler = (*MyHandler)(nil)
```

### Q73. Can interfaces embed other interfaces?
Yes:
```go
type ReadWriter interface { io.Reader; io.Writer }
```
Overlapping identical methods are allowed (since 1.14).

### Q74. Are interface values comparable?
Yes: equal if same dynamic type and equal values. If the dynamic type is not comparable (slice/map), `==` **panics at runtime**. Using such an interface as a map key also panics.

### Q75. What is the `Stringer` interface?
```go
type Stringer interface { String() string }
```
`fmt` uses it for `%v`/`%s`. Beware infinite recursion: calling `fmt.Sprintf("%v", t)` inside `t.String()`.

### Q76. Important standard interfaces?
`io.Reader`, `io.Writer`, `io.Closer`, `io.ReaderFrom`, `io.WriterTo`, `fmt.Stringer`, `error`, `sort.Interface` (`Len/Less/Swap`), `http.Handler` (`ServeHTTP`), `json.Marshaler/Unmarshaler`, `encoding.TextMarshaler`, `context.Context`, `heap.Interface`, `driver.Valuer`/`sql.Scanner`.

---

## 8. Error Handling, Panic & Recover

### Q77. How does Go handle errors? Why no exceptions?
Errors are ordinary values implementing:
```go
type error interface { Error() string }
```
Functions return errors as the last result; callers check explicitly. Rationale: control flow is visible, errors are handled where they occur, no hidden jumps. Exceptions-like `panic` exist only for truly unrecoverable programmer errors.

### Q78. How do you create errors?
```go
errors.New("not found")
fmt.Errorf("user %d: %w", id, err)  // %w wraps
var ErrNotFound = errors.New("not found") // sentinel error

type ValidationError struct { Field, Msg string }
func (e *ValidationError) Error() string { return e.Field + ": " + e.Msg } // custom type
```

### Q79. What are error wrapping, `errors.Is`, `errors.As`, and `errors.Join`?
- `%w` in `fmt.Errorf` wraps an error, preserving it in a chain (`Unwrap() error`).
- `errors.Is(err, target)` — walks the chain comparing with `==` (or custom `Is` method). Use for **sentinel** errors.
- `errors.As(err, &target)` — walks the chain finding the first error assignable to target's type. Use for **typed** errors.
- `errors.Join(e1, e2)` (1.20) — combines multiple errors; `Is/As` search all. Multiple `%w` in one `Errorf` is also allowed.
```go
if errors.Is(err, sql.ErrNoRows) { ... }
var ve *ValidationError
if errors.As(err, &ve) { fmt.Println(ve.Field) }
```

### Q80. Sentinel errors vs error types vs opaque errors?
- **Sentinel** (`io.EOF`): simple, but creates API coupling.
- **Error types**: carry structured context.
- **Opaque** errors + behaviour checks: `interface{ Temporary() bool }` — callers assert on behaviour, not type.
Wrap with context at each layer ("reading config: open file: permission denied"), and handle an error only once (either log or return, not both).

### Q81. What is panic?
`panic(v)` stops normal execution of the current goroutine, runs deferred functions while unwinding the stack, and if not recovered, crashes the program printing the stack trace. Runtime panics: nil pointer dereference, index out of range, nil map write, invalid type assertion, closed channel send, divide by zero (integers).

### Q82. What is recover?
`recover()` stops a panic and returns the panic value. **Only works when called directly inside a deferred function** in the panicking goroutine.
```go
func safe(fn func()) (err error) {
    defer func() {
        if r := recover(); r != nil { err = fmt.Errorf("recovered: %v", r) }
    }()
    fn()
    return nil
}
```
A panic in a goroutine can't be recovered by another goroutine — each goroutine needs its own recover (e.g., HTTP servers recover per request).

### Q83. When should you use panic?
- Programmer errors / impossible states (invariant violated).
- Initialization failures where the program can't continue (`regexp.MustCompile`, `template.Must`).
- Never for expected errors (file not found, bad input) in library code.

### Q84. What is not recoverable?
Fatal runtime errors: concurrent map writes, out of memory, stack overflow exceeding max, deadlock ("all goroutines are asleep"), `runtime.Goexit` isn't a panic. These call `fatal` and exit immediately.

### Q85. What does `runtime.Goexit` do?
Terminates the calling goroutine after running its deferred calls. Used by `t.FailNow()` in tests. Not recoverable via recover.

---

## 9. Goroutines & Concurrency

### Q86. What is a goroutine?
A function executing concurrently, managed by the Go runtime rather than the OS. Started with `go f(x)`. Characteristics:
- Starts with a small stack (**2 KB**) that grows/shrinks dynamically (copying stacks).
- Creation costs ~ hundreds of ns; millions can exist.
- Multiplexed onto OS threads by the **GMP scheduler**.
- No identity/ID exposed intentionally; no way to kill externally (use context/channels for cancellation).

### Q87. Goroutine vs OS thread?
| Goroutine | OS thread |
|---|---|
| ~2 KB growable stack | ~1–8 MB fixed stack |
| Scheduled by Go runtime (user space) | Scheduled by kernel |
| Context switch ~ tens-hundreds ns (only a few registers saved) | ~µs, kernel transition |
| Cheap to create (millions OK) | Expensive (thousands) |
| No ID, no priority | TID, priorities |

### Q88. Concurrency vs parallelism?
**Concurrency** is structuring a program as independently executing tasks (dealing with many things at once). **Parallelism** is simultaneous execution on multiple cores (doing many things at once). Go provides concurrency; parallelism happens when `GOMAXPROCS > 1` and multiple cores are available.

### Q89. Explain the GMP scheduler model.
- **G** — goroutine (stack, instruction pointer, status).
- **M** — machine = OS thread.
- **P** — processor = logical scheduling context with a **local run queue** (up to 256 Gs) and mcache. Number of Ps = `GOMAXPROCS` (default = CPU count; since Go 1.25 also respects container CPU limits).

An M must hold a P to run Go code. Scheduling:
1. New G goes to current P's `runnext`/local queue; overflow moves half to the **global run queue**.
2. M finds work: local queue → global queue (checked every 61 schedule ticks for fairness) → netpoller → **work stealing** (steal half another P's queue).
3. On a **blocking syscall**, the M releases its P (via `sysmon` handoff) so another M can run Gs.
4. Network I/O uses the **netpoller** (epoll/kqueue/IOCP): goroutine parks, no thread blocked.
5. **Preemption**: cooperative at function prologues, plus **asynchronous preemption** (Go 1.14+) via signals (SIGURG) for goroutines running > 10 ms in tight loops.

See Part III for depth.

### Q90. What is GOMAXPROCS?
Maximum number of Ps — OS threads executing Go code simultaneously. Threads blocked in syscalls don't count. Set via env var or `runtime.GOMAXPROCS(n)`.

### Q91. How do you wait for goroutines to finish?
```go
var wg sync.WaitGroup
for _, u := range urls {
    wg.Add(1)
    go func() {
        defer wg.Done()
        fetch(u)
    }()
}
wg.Wait()
// Go 1.25+: wg.Go(func(){ fetch(u) })
```
Call `Add` **before** starting the goroutine (not inside), else Wait may return early. Alternatives: channels, `errgroup.Group`.

### Q92. What is `errgroup`?
`golang.org/x/sync/errgroup` — WaitGroup + first error + context cancellation + concurrency limit:
```go
g, ctx := errgroup.WithContext(ctx)
g.SetLimit(10)
for _, id := range ids {
    g.Go(func() error { return process(ctx, id) })
}
if err := g.Wait(); err != nil { return err }
```

### Q93. What is a goroutine leak? How to prevent it?
A goroutine blocked forever (on channel send/recv, lock, or I/O without timeout) never exits, holding memory. Example:
```go
func leak() int {
    ch := make(chan int)
    go func() { ch <- compute() }() // blocked forever if caller returns early
    select {
    case v := <-ch: return v
    case <-time.After(time.Second): return -1
    }
}
```
Fix: buffered channel `make(chan int, 1)`, or pass `ctx` and select on `ctx.Done()`. Detect with `runtime.NumGoroutine()`, pprof goroutine profile, `go.uber.org/goleak` in tests.

### Q94. What is a race condition / data race? How do you detect it?
A **data race** occurs when two goroutines access the same memory concurrently, at least one writes, without synchronization. Result is undefined behaviour (torn reads, corrupted maps). Detect with the **race detector**: `go test -race`, `go run -race` (uses ThreadSanitizer, 5-10x slowdown, finds races that actually occur during execution). A race condition (logical) can exist even without data races (e.g., check-then-act).

### Q95. How do you limit concurrency (worker pool / semaphore)?
```go
sem := make(chan struct{}, 5) // max 5 concurrent
for _, job := range jobs {
    sem <- struct{}{}
    go func() {
        defer func() { <-sem }()
        work(job)
    }()
}
```
Or `golang.org/x/sync/semaphore`, `errgroup.SetLimit`, or a fixed worker pool:
```go
func workerPool(jobs <-chan int, results chan<- int, n int) {
    var wg sync.WaitGroup
    for w := 0; w < n; w++ {
        wg.Add(1)
        go func() {
            defer wg.Done()
            for j := range jobs { results <- j * j }
        }()
    }
    wg.Wait()
    close(results)
}
```

### Q96. What concurrency patterns do you know?
- **Generator**, **Pipeline** (stages connected by channels), **Fan-out/Fan-in**, **Worker pool**, **Semaphore**, **Pub/Sub**, **Rate limiting** (`time.Ticker`, `golang.org/x/time/rate`), **Or-done**, **Tee**, **Bridge**, **Future/Promise** via channel, **Circuit breaker**, **singleflight** (dedupe concurrent calls).

Fan-in example:
```go
func merge(ctx context.Context, cs ...<-chan int) <-chan int {
    out := make(chan int)
    var wg sync.WaitGroup
    wg.Add(len(cs))
    for _, c := range cs {
        go func(c <-chan int) {
            defer wg.Done()
            for v := range c {
                select {
                case out <- v:
                case <-ctx.Done(): return
                }
            }
        }(c)
    }
    go func() { wg.Wait(); close(out) }()
    return out
}
```

### Q97. "Don't communicate by sharing memory; share memory by communicating." Explain.
Rather than guarding shared data with locks, pass ownership of data through channels so only one goroutine accesses it at a time. But pragmatically: use **mutexes for protecting state** (caches, counters) and **channels for transferring ownership, coordinating, signaling**.

---

## 10. Channels & Select

### Q98. What is a channel?
A typed, goroutine-safe conduit for communication and synchronization.
```go
ch := make(chan int)      // unbuffered
bch := make(chan int, 10) // buffered
var ro <-chan int = ch    // receive-only
var so chan<- int = ch    // send-only
```

### Q99. Unbuffered vs buffered channels?
- **Unbuffered**: send blocks until a receiver takes it (and vice versa) → synchronous handoff; guarantees happens-before between send and receive completion.
- **Buffered**: send blocks only when buffer full; receive blocks only when empty. Decouples producer/consumer; capacity acts like a semaphore.

### Q100. Channel operation behaviour table (VERY common question)
| Operation | nil channel | closed channel | open channel |
|---|---|---|---|
| Send `ch <- v` | blocks forever | **panic** | blocks/succeeds |
| Receive `<-ch` | blocks forever | returns zero value, `ok=false` (after draining buffer) | blocks/succeeds |
| Close `close(ch)` | **panic** | **panic** | succeeds |
| `len`/`cap` | 0 | buffered count | buffered count |

### Q101. Who should close a channel?
The **sender** (never the receiver), and only when receivers need to know no more values will come (e.g., to end a `range`). With multiple senders, use a WaitGroup and a separate goroutine to close after all senders finish, or signal via a separate done channel. Closing is not required for GC.

### Q102. How does `range` over a channel work?
Receives until the channel is closed and drained:
```go
for v := range ch { fmt.Println(v) }
```
If never closed → blocks forever (leak/deadlock).

### Q103. What is `select`?
Waits on multiple channel operations; runs the one that's ready. If several are ready, one is chosen **pseudo-randomly** (fairness). `default` makes it non-blocking.
```go
select {
case msg := <-msgs:
    handle(msg)
case out <- v:
case <-time.After(2 * time.Second):
    fmt.Println("timeout")
case <-ctx.Done():
    return ctx.Err()
default:
    // nothing ready
}
```
`select {}` blocks forever. A nil channel case is never selected — useful to disable cases dynamically.

### Q104. How is a channel implemented internally?
```go
type hchan struct {
    qcount   uint           // items in buffer
    dataqsiz uint           // buffer capacity
    buf      unsafe.Pointer // circular ring buffer
    elemsize uint16
    closed   uint32
    elemtype *_type
    sendx    uint           // send index
    recvx    uint           // receive index
    recvq    waitq          // list of blocked receivers (sudog)
    sendq    waitq          // list of blocked senders
    lock     mutex
}
```
Send: lock → if a receiver is waiting in `recvq`, copy directly to its stack (**direct send**) and wake it; else if buffer has space, copy into `buf[sendx]`; else create a `sudog`, enqueue on `sendq`, and `gopark`. Receive is symmetric. `make(chan)` allocates `hchan` on the heap.

### Q105. How do you implement a timeout?
```go
select {
case res := <-ch:
case <-time.After(time.Second):
}
```
In loops prefer a reusable `time.NewTimer` (pre-1.23 `time.After` timers were not collected until fired; Go 1.23 fixed timer GC and made timer channels unbuffered/synchronous so Reset/Stop are safer).

### Q106. How do you broadcast a signal to many goroutines?
Close a channel: all receivers unblock. That's how `ctx.Done()` works. For repeated broadcasts use `sync.Cond` or per-subscriber channels.

### Q107. What causes "fatal error: all goroutines are asleep - deadlock!"?
The runtime detects that no goroutine can make progress (all blocked on channels/locks with no possible wakers, not counting ones blocked on I/O/timers). Example: `ch := make(chan int); ch <- 1` in main.

### Q108. Implement a ping-pong / print odd-even with two goroutines.
```go
func main() {
    odd, even := make(chan struct{}), make(chan struct{})
    done := make(chan struct{})
    go func() {
        for i := 1; i <= 9; i += 2 { <-odd; fmt.Println(i); even <- struct{}{} }
    }()
    go func() {
        for i := 2; i <= 10; i += 2 { <-even; fmt.Println(i)
            if i == 10 { close(done); return }
            odd <- struct{}{} }
    }()
    odd <- struct{}{}
    <-done
}
```

---

## 11. sync Package & Memory Model

### Q109. `sync.Mutex` vs `sync.RWMutex`?
- `Mutex`: exclusive lock. Two modes: **normal** (waiters queue FIFO but newly arriving goroutines can barge — better throughput) and **starvation mode** (entered if a waiter waits > 1 ms; ownership handed off directly to the waiter at queue front).
- `RWMutex`: many readers **or** one writer. Good for read-heavy workloads; a pending writer blocks new readers (prevents writer starvation). Higher overhead than Mutex; benchmark before choosing.

Rules: don't copy a mutex after use (vet `copylocks` catches), unlock with `defer`, keep critical sections small, Go mutexes are **not reentrant** (locking twice in the same goroutine deadlocks). `TryLock` exists (1.18) but is rarely appropriate.

### Q110. What is `sync.Once`?
Executes a function exactly once, even across goroutines; other callers block until it completes. Used for lazy init/singletons.
```go
var (once sync.Once; db *sql.DB)
func DB() *sql.DB { once.Do(func() { db = connect() }); return db }
```
Go 1.21 added `sync.OnceFunc`, `OnceValue`, `OnceValues`. If the function panics, `Do` considers it done.

### Q111. What is `sync.Pool`?
A cache of temporary, reusable objects to reduce allocation/GC pressure (e.g., buffers in `fmt`, `encoding/json`). Objects may be removed at any GC (victim cache survives one cycle). Not for connection pools or objects with state that must persist. Reset objects before `Put`.
```go
var bufPool = sync.Pool{New: func() any { return new(bytes.Buffer) }}
b := bufPool.Get().(*bytes.Buffer); b.Reset()
defer bufPool.Put(b)
```

### Q112. What is `sync.Cond`?
A condition variable: goroutines `Wait()` (atomically unlocks L and suspends) until `Signal()` (one) or `Broadcast()` (all). Always wait in a loop checking the condition. Channels usually preferred.

### Q113. What is `sync.Map` and when is it better than map+Mutex?
A concurrent map optimized for: (1) keys written once and read many times (caches), (2) goroutines reading/writing disjoint key sets. It uses a read-only map accessed atomically without locks plus a dirty map under a mutex (reimplemented on a concurrent hash-trie in Go 1.24). For general use a map with RWMutex or sharding is more type-safe and often faster.

### Q114. What are atomic operations?
`sync/atomic` provides lock-free operations on integers/pointers: `Add`, `Load`, `Store`, `Swap`, `CompareAndSwap`. Typed wrappers (1.19): `atomic.Int64`, `atomic.Bool`, `atomic.Pointer[T]`, `atomic.Value`.
```go
var hits atomic.Int64
hits.Add(1)
fmt.Println(hits.Load())
```
Use for counters/flags; for compound invariants use mutexes.

### Q115. What is the Go memory model?
It specifies when a read of a variable in one goroutine is guaranteed to observe a write in another: only when there is a **happens-before** relationship established by synchronization. Key rules:
- A send on a channel happens-before the corresponding receive completes.
- Closing a channel happens-before a receive that returns because the channel is closed.
- For unbuffered channels, the receive happens-before the send completes.
- The k-th receive on a channel with capacity C happens-before the (k+C)-th send completes (semaphore pattern).
- `Unlock` n happens-before `Lock` n+1 returns.
- `once.Do(f)` — f's completion happens-before any `Do` returns.
- `go` statement happens-before the goroutine starts; goroutine exit is **not** synchronized with anything.
- Atomic operations are **sequentially consistent** (since 2022 memory model revision).
Programs with data races have undefined results ("don't be clever").

### Q116. Why is this code broken?
```go
var ready bool; var data int
go func() { data = 42; ready = true }()
for !ready {}  // may spin forever (compiler can hoist), or see ready=true but data=0
fmt.Println(data)
```
No happens-before. Fix with channel, mutex, or `atomic.Bool`.

---

## 12. Context

### Q117. What is `context.Context` and why is it used?
It carries **deadlines, cancellation signals, and request-scoped values** across API boundaries and goroutines.
```go
type Context interface {
    Deadline() (deadline time.Time, ok bool)
    Done() <-chan struct{}
    Err() error          // Canceled or DeadlineExceeded
    Value(key any) any
}
```
Constructors: `context.Background()`, `context.TODO()`, `WithCancel`, `WithTimeout`, `WithDeadline`, `WithValue`, `WithCancelCause` (1.20), `WithoutCancel`, `AfterFunc` (1.21), `WithTimeoutCause`.

### Q118. Context best practices?
- Pass `ctx` as the **first parameter** named `ctx`; don't store it in structs (except where lifecycles demand).
- Always `defer cancel()` to release resources/timers.
- Never pass a `nil` context; use `context.TODO()`.
- Use `WithValue` only for request-scoped data (request ID, auth user), not optional parameters; use unexported key types to avoid collisions:
```go
type ctxKey struct{}
ctx = context.WithValue(ctx, ctxKey{}, userID)
id, _ := ctx.Value(ctxKey{}).(string)
```
- Contexts form a tree: cancelling a parent cancels all children.

### Q119. How to make a long-running function respect cancellation?
```go
func worker(ctx context.Context, jobs <-chan Job) error {
    for {
        select {
        case <-ctx.Done():
            return ctx.Err()
        case j, ok := <-jobs:
            if !ok { return nil }
            if err := process(ctx, j); err != nil { return err }
        }
    }
}
```
For CPU-bound loops, check `ctx.Err()` periodically.

### Q120. How does HTTP use context?
`r.Context()` is cancelled when the client disconnects or the request finishes. Outgoing requests: `http.NewRequestWithContext(ctx, ...)`. DB: `db.QueryContext(ctx, ...)`. This propagates timeouts end-to-end.

---

## 13. Generics

### Q121. What are generics in Go? (1.18+)
Type parameters allow functions and types to work over sets of types with compile-time type safety.
```go
func Map[T, U any](s []T, f func(T) U) []U {
    r := make([]U, 0, len(s))
    for _, v := range s { r = append(r, f(v)) }
    return r
}
type Stack[T any] struct{ items []T }
func (s *Stack[T]) Push(v T) { s.items = append(s.items, v) }
func (s *Stack[T]) Pop() (T, bool) {
    var zero T
    if len(s.items) == 0 { return zero, false }
    v := s.items[len(s.items)-1]
    s.items = s.items[:len(s.items)-1]
    return v, true
}
```

### Q122. What are constraints and type sets?
Constraints are interfaces that may include type unions:
```go
type Number interface {
    ~int | ~int8 | ~int16 | ~int32 | ~int64 | ~float32 | ~float64
}
func Sum[T Number](xs []T) T { var s T; for _, x := range xs { s += x }; return s }
```
- `~int` means "any type whose **underlying type** is int" (includes `type MyInt int`).
- `comparable` — types supporting `==`/`!=` (usable as map keys).
- `cmp.Ordered` — types supporting `< <= > >=`.
- Interfaces with type unions can only be used as constraints, not regular variable types.

### Q123. How are generics implemented?
**GC Shape Stenciling with dictionaries**: the compiler generates one instantiation per *GC shape* (types with the same underlying memory layout / pointer-ness share code — all pointer types share one shape), and passes a hidden **dictionary** containing type info and method pointers. It's a hybrid between full monomorphization (C++) and boxing (Java). Consequence: generic code with method calls on pointer-shaped type params can be slower than hand-written code, as calls go through the dictionary.

### Q124. Limitations of Go generics?
- No type parameters on **methods** (only on types and functions).
- No specialization, no variadic type params, no operator overloading.
- Can't use type switch on type param directly (convert to `any` first).
- Type inference is limited in some cases (return-only types need explicit instantiation).

### Q125. When to use generics vs interfaces?
- Generics: data structures (trees, sets, caches), functions over slices/maps/channels of any type, avoiding `any` + type assertions.
- Interfaces: when behaviour (methods) varies — polymorphism.
Rule: "Write code, don't design types." If you're writing the same code for multiple types, consider generics.

### Q126. Useful generic standard libraries?
`slices` (Sort, Contains, Index, BinarySearch, Max, Compact, Reverse, Collect), `maps` (Keys, Values, Clone), `cmp` (Compare, Or), `iter` (Seq, Seq2 — Go 1.23), `sync/atomic.Pointer[T]`, `unique` (1.23 interning).

### Q127. What are iterators (range-over-func)? (Go 1.23)
```go
func Count(n int) iter.Seq[int] {
    return func(yield func(int) bool) {
        for i := 0; i < n; i++ {
            if !yield(i) { return } // consumer did break
        }
    }
}
for i := range Count(5) { fmt.Println(i) }
```
Push iterators; `iter.Pull` converts to pull style. Stdlib: `maps.Keys`, `slices.All`, `slices.Values`, `strings.SplitSeq` (1.24), `bytes.Lines`.

---

## 14. Memory Management & Garbage Collection

### Q128. Stack vs heap allocation in Go?
Go decides via **escape analysis** at compile time. If the compiler can prove a value doesn't outlive the function (doesn't escape), it's put on the goroutine stack (cheap, freed automatically). Otherwise heap (GC managed). You don't choose explicitly; `new`/`&T{}` can still be on the stack.

### Q129. What causes a variable to escape to the heap?
- Returning a pointer to a local variable.
- Storing a pointer in a global, heap object, or channel.
- Assigning to an interface (often), calling methods via interfaces with pointer args.
- Closures capturing variables that outlive the function.
- Size unknown at compile time (`make([]int, n)` with variable n) or too large (> 64 KB for implicit, 10 MB explicit vars).
- Passing to `fmt.Println` (takes `...any`).
Check with `go build -gcflags="-m -m"`.

### Q130. Explain Go's garbage collector.
A **concurrent, non-generational, non-moving, tri-color mark-and-sweep** collector with a **hybrid write barrier**:
1. **Sweep termination** (short STW): finish previous sweep.
2. **Mark phase** (concurrent): enable write barrier, scan roots (stacks, globals), mark reachable objects. Tri-color: white (unvisited/garbage candidates), grey (reachable, children not scanned), black (reachable, scanned). Invariant: no black object points to white. **GC assist**: goroutines allocating heavily are drafted to help mark.
3. **Mark termination** (short STW): finish, disable write barrier.
4. **Sweep** (concurrent/lazy): reclaim white objects' spans.

STW pauses are typically < 1 ms (often tens of µs). Target: ~25% CPU for background GC workers. Go 1.25 introduced the experimental **Green Tea** GC (span/page-oriented marking for better locality), which became the default in Go 1.26.

### Q131. What are GOGC and GOMEMLIMIT?
- `GOGC` (default 100): next GC triggers when heap grows by 100% of live heap after last GC. `GOGC=200` → less frequent GC, more memory. `GOGC=off` disables.
- `GOMEMLIMIT` (1.19): a soft memory limit; GC runs more often as heap approaches the limit. Useful in containers: `GOGC=off GOMEMLIMIT=900MiB` for maximum throughput within a budget (beware GC death spirals—there's a 50% CPU cap on GC).
Set programmatically via `debug.SetGCPercent`, `debug.SetMemoryLimit`.

### Q132. Why isn't Go's GC generational or compacting?
- **Non-moving**: simplifies interop with C (cgo), unsafe pointers, and because Go's allocator (size-class segregated, like TCMalloc) already limits fragmentation.
- **Non-generational**: escape analysis already keeps many short-lived objects on the stack, reducing the generational hypothesis's benefit; generational requires write barriers always-on (costly).

### Q133. How do you reduce GC pressure?
- Reduce allocations: preallocate slices/maps (`make(.., 0, n)`), reuse buffers, `sync.Pool`, avoid `fmt` in hot paths, avoid unnecessary pointer-heavy structures (GC scans pointers; `[]int` with no pointers isn't scanned).
- Use `strings.Builder`, `strconv` over `fmt.Sprintf`.
- Profile with `pprof -alloc_space`, `GODEBUG=gctrace=1`.

### Q134. Can there be memory leaks in a GC language?
Yes: goroutine leaks, forgotten timers/tickers (`time.Ticker` without `Stop` pre-1.23), unbounded caches/maps, subslices pinning large arrays, global references, `finalizers` cycles, unclosed response bodies.

### Q135. What are finalizers and `runtime.AddCleanup`?
`runtime.SetFinalizer(obj, f)` runs f when obj becomes unreachable (not guaranteed to run, can resurrect objects). Go 1.24 added `runtime.AddCleanup` — more flexible, doesn't resurrect, multiple per object. Also `weak.Pointer` (1.24) for weak references (caches).

---

## 15. Packages, Modules & Tooling

### Q136. What is a Go module?
A collection of packages versioned together, defined by `go.mod` at the root:
```
module github.com/acme/shop

go 1.24

require (
    github.com/gin-gonic/gin v1.10.0
    github.com/jackc/pgx/v5 v5.7.1
)
replace github.com/acme/lib => ../lib
```
`go.sum` contains cryptographic checksums for integrity (verified against `sum.golang.org`).

### Q137. Key `go` commands?
| Command | Purpose |
|---|---|
| `go mod init <path>` | create module |
| `go mod tidy` | add missing/remove unused deps |
| `go get pkg@v1.2.3` | add/upgrade dependency |
| `go build`, `go run`, `go install` | compile / run / install binary |
| `go test ./... -race -cover` | tests |
| `go vet` | static analysis |
| `go fmt` / `gofmt -s` | format |
| `go generate` | run `//go:generate` directives |
| `go work init` | multi-module workspaces |
| `go mod vendor` | vendoring |
| `go tool pprof` | profiling |
| `go env` | env config (GOPATH, GOPROXY, GOFLAGS) |
| `go get -tool` / `go tool` | tool dependencies in go.mod (1.24) |

### Q138. What is Minimal Version Selection (MVS)?
Go picks the **minimum** version satisfying all `require`s (the max of the minimums), not the latest. Builds are reproducible without a lock file. Major versions ≥2 must have `/v2` suffix in module path (semantic import versioning).

### Q139. What is the `internal` directory?
Packages under `internal/` can only be imported by code rooted at the parent of `internal`. Enforced by the compiler; used to hide implementation details.

### Q140. Standard project layout?
```
myapp/
├── cmd/api/main.go          # entry points
├── internal/                # private app code
│   ├── handler/  service/  repository/  model/
├── pkg/                     # public reusable libs (optional, debated)
├── migrations/
├── configs/
├── go.mod  go.sum
└── Makefile  Dockerfile
```

### Q141. What are build tags / constraints?
```go
//go:build linux && amd64
```
Also filename suffixes: `file_linux.go`, `file_test.go`. Custom tags: `go build -tags integration`.

### Q142. How do you cross-compile?
`GOOS=linux GOARCH=arm64 go build -o app` — pure Go code cross-compiles without extra toolchains. `CGO_ENABLED=0` for fully static binaries.

### Q143. What is cgo and its costs?
cgo lets Go call C. Costs: slower builds, loss of easy cross-compilation, call overhead (~tens of ns, thread state switch), C memory not managed by GC, rules on passing Go pointers to C. "cgo is not Go."

### Q144. What are `//go:` compiler directives?
`//go:build`, `//go:generate`, `//go:embed` (embed files into binary), `//go:noinline`, `//go:nosplit`, `//go:linkname`, `//go:noescape`.
```go
import "embed"
//go:embed templates/*
var tmplFS embed.FS
```

---

## 16. Testing, Benchmarking & Profiling

### Q145. How do you write tests in Go?
Files ending `_test.go`, functions `TestXxx(t *testing.T)`:
```go
func TestAdd(t *testing.T) {
    tests := []struct {
        name string
        a, b, want int
    }{
        {"positive", 2, 3, 5},
        {"negative", -1, -2, -3},
        {"zero", 0, 0, 0},
    }
    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            t.Parallel()
            if got := Add(tt.a, tt.b); got != tt.want {
                t.Errorf("Add(%d,%d) = %d; want %d", tt.a, tt.b, got, tt.want)
            }
        })
    }
}
```
This is **table-driven testing** — idiomatic Go. `t.Error` continues, `t.Fatal` stops the test. `t.Helper()`, `t.Cleanup()`, `t.TempDir()`, `t.Setenv()`, `t.Context()` (1.24).

### Q146. How do you write benchmarks?
```go
func BenchmarkConcat(b *testing.B) {
    b.ReportAllocs()
    for b.Loop() { // Go 1.24; older: for i := 0; i < b.N; i++
        _ = strings.Repeat("a", 100)
    }
}
```
Run: `go test -bench=. -benchmem -count=10` and compare with `benchstat`.

### Q147. What is fuzzing? (1.18)
```go
func FuzzReverse(f *testing.F) {
    f.Add("hello")
    f.Fuzz(func(t *testing.T, s string) {
        if Reverse(Reverse(s)) != s { t.Errorf("mismatch %q", s) }
    })
}
```
`go test -fuzz=FuzzReverse` generates inputs guided by coverage; failures saved in `testdata/fuzz`.

### Q148. How do you mock dependencies?
Define small interfaces and inject fakes:
```go
type UserStore interface { Get(ctx context.Context, id int) (*User, error) }
type fakeStore struct{ users map[int]*User }
func (f *fakeStore) Get(_ context.Context, id int) (*User, error) { ... }
```
Tools: `gomock`/`mockery`, `testify` (assert/require/mock), `httptest` (`NewRecorder`, `NewServer`), `sqlmock`, testcontainers for real DB integration tests.

### Q149. Other test features?
`TestMain(m *testing.M)` for global setup; `Example` functions (documentation + tested output via `// Output:` comment); `-short`, `-run regex`, `-cover -coverprofile`; `testing/synctest` (Go 1.25) for deterministic concurrent tests with fake time.

### Q150. How do you profile a Go program?
- `net/http/pprof` endpoint: `import _ "net/http/pprof"` → `/debug/pprof/`.
- `go test -cpuprofile cpu.out -memprofile mem.out`.
- `go tool pprof -http=:8080 cpu.out` (flame graphs).
- Profiles: CPU, heap (inuse/alloc), goroutine, block, mutex, threadcreate.
- `go tool trace` for scheduler/GC latency.
- `GODEBUG=gctrace=1,schedtrace=1000`.
- PGO (Profile Guided Optimization, 1.21): put `default.pgo` in main package → 2-14% speedup.

---
## 17. Tricky Output / Gotcha Questions

### G1. Slice append aliasing
```go
func main() {
    s := make([]int, 3, 10)
    a := append(s, 1)
    b := append(s, 2)
    fmt.Println(a[3], b[3])
}
```
**Output:** `2 2` — both share the backing array (cap 10), so the second append overwrote index 3.

### G2. Modifying slice in function
```go
func modify(s []int) { s[0] = 100; s = append(s, 4); s[1] = 200 }
func main() { s := []int{1, 2, 3}; modify(s); fmt.Println(s) }
```
**Output:** `[100 2 3]` — `s[0]` modified shared array; `append` exceeded cap 3 so reallocated; later write went to the new array.

### G3. Defer evaluation
```go
func main() {
    for i := 0; i < 3; i++ {
        defer func() { fmt.Print(i) }()
    }
}
```
**Go ≥1.22:** `210`. **Before 1.22:** `333`.

### G4. Defer and return value
```go
func f() int { x := 5; defer func() { x++ }(); return x }        // 5
func g() (x int) { x = 5; defer func() { x++ }(); return x }     // 6
func h() (x int) { defer func() { x++ }(); return 5 }            // 6
```

### G5. Range copies value
```go
type P struct{ n int }
ps := []P{{1}, {2}}
for _, p := range ps { p.n *= 10 }
fmt.Println(ps) // [{1} {2}] — p is a copy
for i := range ps { ps[i].n *= 10 } // correct
```

### G6. Range expression evaluated once
```go
s := []int{1, 2, 3}
for i := range s { s = append(s, i) }
fmt.Println(len(s)) // 6 — range length fixed at start
```

### G7. Map of structs
```go
m := map[string]struct{ n int }{"a": {1}}
m["a"].n = 2 // compile error: cannot assign to struct field in map
```

### G8. nil interface
```go
var p *int = nil
var i any = p
fmt.Println(i == nil) // false
```

### G9. String iteration
```go
for i, c := range "aé" { fmt.Print(i, " ") } // 0 1
```

### G10. Closure over variable
```go
x := 1
f := func() { fmt.Println(x) }
x = 2
f() // 2 — captured by reference
```

### G11. Unbuffered channel deadlock
```go
func main() { ch := make(chan int); ch <- 1; fmt.Println(<-ch) } // fatal: deadlock
```

### G12. Select with multiple ready
```go
ch := make(chan int, 2); ch <- 1; ch <- 2
select { case v := <-ch: fmt.Println(v); case v := <-ch: fmt.Println(v) } // 1 (FIFO buffer, either case)
```

### G13. Method value with value receiver captured early
```go
type T struct{ n int }
func (t T) Print() { fmt.Println(t.n) }
t := T{1}
defer t.Print() // receiver copied now → prints 1
t.n = 2
```

### G14. Integer division and shift
```go
fmt.Println(7 / 2, -7 / 2, 7 % -3, -7 % 3) // 3 -3 1 -1 (truncates toward zero)
```

### G15. Constant arithmetic
```go
fmt.Println(1 << 62 * 4 / 8) // untyped const math at arbitrary precision: 2305843009213693952 (1<<61)
```

### G16. Struct comparison with interface fields
```go
type S struct{ v any }
fmt.Println(S{[]int{1}} == S{[]int{1}}) // runtime panic: comparing uncomparable type []int
```

### G17. Goroutine in main without waiting
```go
func main() { go fmt.Println("hi") } // likely prints nothing; main exits
```

### G18. Copying a struct with a mutex
```go
type C struct { mu sync.Mutex; n int }
func (c C) Inc() { c.mu.Lock(); c.n++; c.mu.Unlock() } // value receiver: modifies copy; vet warns
```

### G19. Switch without fallthrough
```go
switch 1 { case 1: fmt.Print("a"); fallthrough; case 2: fmt.Print("b"); case 3: fmt.Print("c") } // ab
```

### G20. Shadowed err
```go
var err error
if true { _, err := strconv.Atoi("x"); _ = err }
fmt.Println(err) // <nil>
```

### G21. append to nil map vs nil slice
```go
var s []int; s = append(s, 1)        // fine
var m map[string]int; m["a"] = 1     // panic: assignment to entry in nil map
```

### G22. `len` of a channel after close
```go
ch := make(chan int, 3); ch <- 1; ch <- 2; close(ch)
fmt.Println(len(ch)) // 2
v, ok := <-ch; fmt.Println(v, ok) // 1 true
<-ch; v, ok = <-ch; fmt.Println(v, ok) // 0 false
```

### G23. JSON with unexported fields
```go
type U struct{ name string; Age int }
b, _ := json.Marshal(U{"x", 3}); fmt.Println(string(b)) // {"Age":3}
```

### G24. Time comparison
Use `t1.Equal(t2)`, not `==` — `time.Time` contains monotonic clock readings and location pointers.

### G25. Float comparison
```go
fmt.Println(0.1+0.2 == 0.3) // true! constants computed exactly
a, b := 0.1, 0.2; fmt.Println(a+b == 0.3) // false (float64 runtime arithmetic)
```

---

## 18. Coding Problems with Solutions

### C1. Reverse a linked list
```go
type Node struct { Val int; Next *Node }
func Reverse(head *Node) *Node {
    var prev *Node
    for head != nil {
        head.Next, prev, head = prev, head, head.Next
    }
    return prev
}
```

### C2. Thread-safe LRU cache (generic)
```go
import "container/list"

type entry[K comparable, V any] struct { key K; val V }

type LRU[K comparable, V any] struct {
    mu    sync.Mutex
    cap   int
    ll    *list.List
    items map[K]*list.Element
}

func NewLRU[K comparable, V any](cap int) *LRU[K, V] {
    return &LRU[K, V]{cap: cap, ll: list.New(), items: make(map[K]*list.Element)}
}

func (c *LRU[K, V]) Get(k K) (V, bool) {
    c.mu.Lock(); defer c.mu.Unlock()
    if e, ok := c.items[k]; ok {
        c.ll.MoveToFront(e)
        return e.Value.(*entry[K, V]).val, true
    }
    var zero V
    return zero, false
}

func (c *LRU[K, V]) Put(k K, v V) {
    c.mu.Lock(); defer c.mu.Unlock()
    if e, ok := c.items[k]; ok {
        e.Value.(*entry[K, V]).val = v
        c.ll.MoveToFront(e)
        return
    }
    c.items[k] = c.ll.PushFront(&entry[K, V]{k, v})
    if c.ll.Len() > c.cap {
        last := c.ll.Back()
        c.ll.Remove(last)
        delete(c.items, last.Value.(*entry[K, V]).key)
    }
}
```

### C3. Rate limiter (token bucket)
```go
type TokenBucket struct {
    mu       sync.Mutex
    tokens   float64
    max      float64
    rate     float64 // tokens per second
    last     time.Time
}
func NewTokenBucket(rate, burst float64) *TokenBucket {
    return &TokenBucket{tokens: burst, max: burst, rate: rate, last: time.Now()}
}
func (b *TokenBucket) Allow() bool {
    b.mu.Lock(); defer b.mu.Unlock()
    now := time.Now()
    b.tokens = min(b.max, b.tokens+now.Sub(b.last).Seconds()*b.rate)
    b.last = now
    if b.tokens >= 1 { b.tokens--; return true }
    return false
}
```
Production: `golang.org/x/time/rate.NewLimiter(rate.Limit(10), 20)`.

### C4. Concurrent URL fetcher with bounded parallelism and timeout
```go
func FetchAll(ctx context.Context, urls []string, limit int) (map[string]int, error) {
    ctx, cancel := context.WithTimeout(ctx, 10*time.Second)
    defer cancel()
    var (
        mu  sync.Mutex
        res = make(map[string]int, len(urls))
    )
    g, ctx := errgroup.WithContext(ctx)
    g.SetLimit(limit)
    for _, u := range urls {
        g.Go(func() error {
            req, err := http.NewRequestWithContext(ctx, http.MethodGet, u, nil)
            if err != nil { return err }
            resp, err := http.DefaultClient.Do(req)
            if err != nil { return fmt.Errorf("fetch %s: %w", u, err) }
            defer resp.Body.Close()
            n, _ := io.Copy(io.Discard, resp.Body)
            mu.Lock(); res[u] = int(n); mu.Unlock()
            return nil
        })
    }
    return res, g.Wait()
}
```

### C5. Pipeline: generate → square → sum
```go
func gen(ctx context.Context, nums ...int) <-chan int {
    out := make(chan int)
    go func() {
        defer close(out)
        for _, n := range nums {
            select { case out <- n: case <-ctx.Done(): return }
        }
    }()
    return out
}
func sq(ctx context.Context, in <-chan int) <-chan int {
    out := make(chan int)
    go func() {
        defer close(out)
        for n := range in {
            select { case out <- n * n: case <-ctx.Done(): return }
        }
    }()
    return out
}
func main() {
    ctx, cancel := context.WithCancel(context.Background()); defer cancel()
    sum := 0
    for v := range sq(ctx, gen(ctx, 1, 2, 3, 4)) { sum += v }
    fmt.Println(sum) // 30
}
```

### C6. Two Sum / Anagram / Frequency
```go
func TwoSum(nums []int, target int) (int, int) {
    seen := map[int]int{}
    for i, n := range nums {
        if j, ok := seen[target-n]; ok { return j, i }
        seen[n] = i
    }
    return -1, -1
}

func IsAnagram(a, b string) bool {
    if len(a) != len(b) { return false }
    cnt := map[rune]int{}
    for _, r := range a { cnt[r]++ }
    for _, r := range b { cnt[r]--; if cnt[r] < 0 { return false } }
    return true
}

func TopK(words []string, k int) []string {
    freq := map[string]int{}
    for _, w := range words { freq[w]++ }
    keys := slices.Collect(maps.Keys(freq))
    slices.SortFunc(keys, func(a, b string) int {
        if c := cmp.Compare(freq[b], freq[a]); c != 0 { return c }
        return strings.Compare(a, b)
    })
    return keys[:min(k, len(keys))]
}
```

### C7. Implement a generic concurrent-safe Set
```go
type Set[T comparable] struct { mu sync.RWMutex; m map[T]struct{} }
func NewSet[T comparable]() *Set[T] { return &Set[T]{m: map[T]struct{}{}} }
func (s *Set[T]) Add(v T)          { s.mu.Lock(); s.m[v] = struct{}{}; s.mu.Unlock() }
func (s *Set[T]) Has(v T) bool     { s.mu.RLock(); defer s.mu.RUnlock(); _, ok := s.m[v]; return ok }
func (s *Set[T]) Len() int         { s.mu.RLock(); defer s.mu.RUnlock(); return len(s.m) }
```

### C8. Graceful HTTP shutdown
```go
func main() {
    srv := &http.Server{Addr: ":8080", Handler: routes(),
        ReadTimeout: 5 * time.Second, WriteTimeout: 10 * time.Second, IdleTimeout: 60 * time.Second}
    go func() {
        if err := srv.ListenAndServe(); err != nil && !errors.Is(err, http.ErrServerClosed) {
            log.Fatal(err)
        }
    }()
    ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
    defer stop()
    <-ctx.Done()
    shutdownCtx, cancel := context.WithTimeout(context.Background(), 15*time.Second)
    defer cancel()
    if err := srv.Shutdown(shutdownCtx); err != nil { log.Printf("shutdown: %v", err) }
}
```

### C9. Singleflight (prevent cache stampede)
```go
var g singleflight.Group // golang.org/x/sync/singleflight
func GetUser(id string) (*User, error) {
    v, err, _ := g.Do(id, func() (any, error) { return db.LoadUser(id) })
    if err != nil { return nil, err }
    return v.(*User), nil
}
```

### C10. Pub/Sub broker
```go
type Broker[T any] struct {
    mu   sync.RWMutex
    subs map[string][]chan T
}
func NewBroker[T any]() *Broker[T] { return &Broker[T]{subs: map[string][]chan T{}} }
func (b *Broker[T]) Subscribe(topic string, buf int) <-chan T {
    ch := make(chan T, buf)
    b.mu.Lock(); b.subs[topic] = append(b.subs[topic], ch); b.mu.Unlock()
    return ch
}
func (b *Broker[T]) Publish(topic string, msg T) {
    b.mu.RLock(); defer b.mu.RUnlock()
    for _, ch := range b.subs[topic] {
        select { case ch <- msg: default: /* drop if slow subscriber */ }
    }
}
```

### C11. Merge intervals, binary search, matrix — (algorithmic in Go)
```go
func Merge(iv [][2]int) [][2]int {
    slices.SortFunc(iv, func(a, b [2]int) int { return a[0] - b[0] })
    var out [][2]int
    for _, x := range iv {
        if n := len(out); n > 0 && x[0] <= out[n-1][1] {
            out[n-1][1] = max(out[n-1][1], x[1])
        } else { out = append(out, x) }
    }
    return out
}
func BinarySearch(a []int, t int) int {
    lo, hi := 0, len(a)-1
    for lo <= hi {
        mid := lo + (hi-lo)/2
        switch { case a[mid] == t: return mid; case a[mid] < t: lo = mid + 1; default: hi = mid - 1 }
    }
    return -1
}
```

---

## 19. System Design Questions in Go

### S1. Design a URL shortener service in Go.
- **API**: `POST /shorten` → `{code}`; `GET /{code}` → 302 redirect.
- **ID generation**: base62 encode of an auto-increment/Snowflake ID or random 7 chars with collision check.
- **Storage**: PostgreSQL (`urls(code PK, long_url, created_at, expires_at)`) + Redis cache for hot codes (read-through).
- **Go specifics**: `net/http` or chi/Gin, `pgxpool`, `go-redis`, context timeouts per request, `singleflight` for stampede, graceful shutdown, Prometheus metrics middleware, rate limiting per IP.
- **Scale**: stateless Go instances behind LB, read replicas, async click analytics via a buffered channel + batch writer goroutine, or Kafka.

### S2. How would you build a high-throughput job queue worker?
Consumer goroutine reads from queue (Kafka/SQS/Redis Streams) → bounded channel → N workers → ack after success, retries with exponential backoff + jitter, dead-letter queue, idempotency keys, `context` cancellation for graceful drain on SIGTERM, metrics for queue depth/latency.

### S3. How do you structure a large Go microservice?
Clean/Hexagonal architecture: `domain` (entities, interfaces) ← `service/usecase` ← `adapters` (http handlers, grpc, repositories). Dependencies point inward; wire via constructor injection in `main` (or `wire`/`fx`). Observability (slog structured logging, OpenTelemetry tracing), config via env, health/readiness endpoints.

### S4. REST vs gRPC in Go?
gRPC: Protobuf binary, HTTP/2, streaming, generated clients (`protoc-gen-go-grpc`), deadlines propagate — good for internal service-to-service. REST/JSON: universal, browser-friendly. `grpc-gateway` or ConnectRPC provide both.

---
# PART II — COMPLETE GO TUTORIAL

This part is a structured, from-zero-to-advanced tutorial of the entire language and its standard toolkit.

## 20.1 Installation & First Program

1. Download from https://go.dev/dl and install; verify with `go version`.
2. Create a module:
```bash
mkdir hello && cd hello
go mod init example.com/hello
```
3. `main.go`:
```go
package main

import "fmt"

func main() {
    fmt.Println("Hello, Go!")
}
```
4. Run: `go run .` Build: `go build -o hello .`

**Anatomy:**
- `package main` — executable packages must be named `main` and contain `func main()`.
- `import` — imports packages by path; unused imports are compile errors.
- Semicolons are inserted automatically by the lexer at line ends (so `{` must be on the same line).

## 20.2 Program Structure: Packages, Imports, Visibility

- Every `.go` file starts with `package name`. All files in one directory = one package (except `_test` external test packages).
- Import forms:
```go
import (
    "fmt"
    str "strings"      // alias
    . "math"           // dot import (discouraged)
    _ "image/png"      // side-effect import
)
```
- Visibility: Uppercase identifiers exported.
- Package names: short, lowercase, no underscores; avoid `util`, `common`.

## 20.3 Variables, Constants, and Basic Types

```go
var a int              // 0
var b, c = 1, "two"
d := 3.14              // float64
var (
    name string = "Go"
    age  = 15
)
const Pi = 3.14159
const (
    StatusOK = 200
    StatusNotFound = 404
)
```
Numeric literal forms: `1_000_000`, `0b1010`, `0o755`, `0xFF`, `1e9`, `0x1p-2`.

Type conversion is always explicit: `float64(i)`, `int(f)` (truncates), `string(rune)`, `[]byte(s)`, `strconv.Itoa`, `strconv.ParseFloat`.

**Operators:**
- Arithmetic: `+ - * / % ++ --` (`++` is a statement, not expression — `x = i++` is invalid).
- Comparison: `== != < <= > >=`
- Logical: `&& || !` (short-circuit)
- Bitwise: `& | ^ &^ (AND NOT) << >>`; unary `^x` is bitwise NOT.
- Address: `&x`, dereference `*p`. No pointer arithmetic.
- Channel: `<-`.

## 20.4 Control Flow

```go
// if with init statement
if v, err := strconv.Atoi(s); err != nil {
    return err
} else if v > 10 {
    fmt.Println("big")
}

// switch
switch day := time.Now().Weekday(); day {
case time.Saturday, time.Sunday:
    fmt.Println("weekend")
default:
    fmt.Println("weekday")
}

// for variants
for i := 0; i < 5; i++ {}
for n < 100 { n *= 2 }
for { break }
for i, v := range []string{"a", "b"} {}
for k, v := range map[string]int{} {}
for i, r := range "héllo" {}
for v := range ch {}
for i := range 10 {}        // 0..9 (1.22)
```
`break`, `continue`, labels, `goto`, `defer`.

## 20.5 Functions

```go
func add(a, b int) int { return a + b }
func divmod(a, b int) (q, r int) { q = a / b; r = a % b; return }
func sum(nums ...int) int {...}

// function types & higher-order functions
type Op func(int, int) int
func apply(op Op, a, b int) int { return op(a, b) }
apply(func(x, y int) int { return x * y }, 3, 4)

// closures
func adder() func(int) int { total := 0; return func(x int) int { total += x; return total } }

// recursion
func fact(n int) int { if n <= 1 { return 1 }; return n * fact(n-1) }
```
Functions can't be nested by name, but function literals can be assigned. Recursive literal requires declaring the var first:
```go
var fib func(int) int
fib = func(n int) int { if n < 2 { return n }; return fib(n-1) + fib(n-2) }
```

## 20.6 Pointers

```go
x := 10
p := &x     // *int
*p = 20     // x == 20
var np *int // nil
// *np → panic: nil pointer dereference
type Point struct{ X, Y int }
pp := &Point{1, 2}
pp.X = 5    // auto-deref, same as (*pp).X
```
No pointer arithmetic (except via `unsafe`). Returning pointers to locals is safe (escape analysis moves them to the heap).

## 20.7 Arrays

```go
var a [5]int
b := [3]string{"a", "b", "c"}
c := [...]int{1, 2, 3}      // length inferred
d := [5]int{1: 10, 3: 30}   // indexed literal
grid := [3][3]int{}
len(a) // 5
```
Arrays are values — assignment copies.

## 20.8 Slices

```go
s := []int{1, 2, 3}
s = append(s, 4, 5)
t := make([]int, 3, 10) // len 3, cap 10
u := s[1:3]             // [2 3]
v := s[:0]              // empty, same backing
w := s[1:3:4]           // cap limited
copy(t, s)
matrix := make([][]int, rows)
for i := range matrix { matrix[i] = make([]int, cols) }

// slices package (1.21+)
slices.Sort(s)
slices.Contains(s, 3)
slices.Index(s, 3)
idx, found := slices.BinarySearch(s, 3)
slices.Reverse(s)
slices.Max(s); slices.Min(s)
s = slices.Insert(s, 1, 99)
s = slices.Compact(s)   // remove consecutive dups
slices.Equal(a, b)
```

## 20.9 Maps

```go
m := map[string]int{"a": 1, "b": 2}
m2 := make(map[string][]string, 100) // size hint
m["c"] = 3
v, ok := m["z"]
delete(m, "a")
len(m)
for k, v := range m {}
clear(m)

// maps package
maps.Clone(m)
maps.Copy(dst, src)
maps.DeleteFunc(m, func(k string, v int) bool { return v < 0 })
keys := slices.Sorted(maps.Keys(m))

// nested map init
graph := map[string]map[string]bool{}
if graph["a"] == nil { graph["a"] = map[string]bool{} }
graph["a"]["b"] = true
```

## 20.10 Strings & Text Processing

```go
s := "Hello, 世界"
len(s); s[0]; s[7:]
strings.Contains(s, "llo")
strings.HasPrefix / HasSuffix / Index / LastIndex
strings.Split("a,b,c", ",")
strings.Fields(" a  b ")
strings.Join([]string{"a","b"}, "-")
strings.ToUpper / ToLower / Title (deprecated → cases pkg)
strings.TrimSpace / Trim / TrimPrefix / TrimSuffix
strings.Replace(s, "l", "L", -1) / ReplaceAll
strings.Repeat("ab", 3)
strings.EqualFold("Go", "GO") // case-insensitive
strings.Cut("k=v", "=")       // "k", "v", true

// conversions
strconv.Itoa(42); strconv.Atoi("42")
strconv.ParseInt("ff", 16, 64); strconv.FormatInt(255, 2)
strconv.ParseBool("true"); strconv.Quote(`a"b`)

// raw strings
path := `C:\dir\file` // no escapes, can span lines

// formatting verbs
fmt.Printf("%v %+v %#v %T %d %5.2f %s %q %x %p %t %b %e %08b\n", ...)
fmt.Sprintf / Fprintf / Errorf
```
`%v` default, `%+v` struct with field names, `%#v` Go syntax, `%T` type, `%q` quoted, `%p` pointer, `%w` wrap (Errorf only).

Regex: `regexp.MustCompile(`^\d+$`).MatchString("123")` — RE2 syntax, linear time, no backreferences.

## 20.11 Structs

```go
type Address struct { City, Zip string }
type Person struct {
    Name    string
    Age     int
    Address         // embedded
    Tags    []string `json:"tags,omitempty"`
}
p := Person{Name: "Ann", Age: 30, Address: Address{City: "Paris"}}
p.City              // promoted
anon := struct{ X, Y int }{1, 2}  // anonymous struct
pp := &p; pp.Age++
```

## 20.12 Methods

```go
type Rect struct{ W, H float64 }
func (r Rect) Area() float64 { return r.W * r.H }       // value receiver
func (r *Rect) Scale(f float64) { r.W *= f; r.H *= f }  // pointer receiver
r := Rect{2, 3}
r.Scale(2)  // (&r).Scale auto

type Celsius float64
func (c Celsius) ToF() float64 { return float64(c)*9/5 + 32 }
```

## 20.13 Interfaces & Polymorphism

```go
type Notifier interface { Notify(msg string) error }
type Email struct{ Addr string }
type SMS struct{ Phone string }
func (e Email) Notify(m string) error { fmt.Println("email", e.Addr, m); return nil }
func (s SMS) Notify(m string) error   { fmt.Println("sms", s.Phone, m); return nil }

func Broadcast(ns []Notifier, msg string) {
    for _, n := range ns { _ = n.Notify(msg) }
}
Broadcast([]Notifier{Email{"a@x"}, SMS{"123"}}, "hi")
```
Interface embedding, type assertions, type switches, empty interface — covered in Section 7.

**Decorator via interfaces:**
```go
type LoggingNotifier struct{ Next Notifier }
func (l LoggingNotifier) Notify(m string) error {
    log.Println("sending", m); return l.Next.Notify(m)
}
```

## 20.14 Error Handling

```go
type NotFoundError struct{ Resource string; ID int }
func (e *NotFoundError) Error() string { return fmt.Sprintf("%s %d not found", e.Resource, e.ID) }

var ErrUnauthorized = errors.New("unauthorized")

func FindUser(id int) (*User, error) {
    if id <= 0 { return nil, &NotFoundError{"user", id} }
    ...
}
func Handler() error {
    u, err := FindUser(0)
    if err != nil { return fmt.Errorf("handler: %w", err) }
    _ = u
    return nil
}
err := Handler()
var nf *NotFoundError
switch {
case errors.As(err, &nf): fmt.Println("404:", nf.ID)
case errors.Is(err, ErrUnauthorized): fmt.Println("401")
}
```
Panic/recover — Section 8.

## 20.15 Generics

Covered in Section 13. Additional example — generic `Filter`, `Reduce`, `Keys`, constrained types:
```go
func Filter[T any](s []T, keep func(T) bool) []T {
    var out []T
    for _, v := range s { if keep(v) { out = append(out, v) } }
    return out
}
func Reduce[T, A any](s []T, init A, f func(A, T) A) A {
    acc := init
    for _, v := range s { acc = f(acc, v) }
    return acc
}
type Pair[K comparable, V any] struct { Key K; Val V }
type Tree[T cmp.Ordered] struct { Left, Right *Tree[T]; Val T }
func (t *Tree[T]) Insert(v T) *Tree[T] {
    if t == nil { return &Tree[T]{Val: v} }
    if v < t.Val { t.Left = t.Left.Insert(v) } else { t.Right = t.Right.Insert(v) }
    return t
}
```

## 20.16 Concurrency (full tutorial)

### Goroutines
```go
go worker(1)
go func(msg string) { fmt.Println(msg) }("hi")
```

### Channels
```go
ch := make(chan string)
go func() { ch <- "ping" }()
msg := <-ch
```

### Buffered channels & direction
```go
jobs := make(chan int, 100)
func producer(out chan<- int) { for i := 0; i < 5; i++ { out <- i }; close(out) }
func consumer(in <-chan int)  { for v := range in { fmt.Println(v) } }
```

### Select, timeouts, tickers
```go
ticker := time.NewTicker(500 * time.Millisecond)
defer ticker.Stop()
timeout := time.After(3 * time.Second)
for {
    select {
    case t := <-ticker.C: fmt.Println("tick", t)
    case <-timeout: return
    }
}
```

### Mutex-protected counter
```go
type Counter struct { mu sync.Mutex; m map[string]int }
func (c *Counter) Inc(k string) { c.mu.Lock(); defer c.mu.Unlock(); c.m[k]++ }
```

### Worker pool with results
```go
func main() {
    jobs, results := make(chan int, 100), make(chan int, 100)
    var wg sync.WaitGroup
    for w := 1; w <= 3; w++ {
        wg.Add(1)
        go func(id int) {
            defer wg.Done()
            for j := range jobs { time.Sleep(10 * time.Millisecond); results <- j * 2 }
        }(w)
    }
    for j := 1; j <= 9; j++ { jobs <- j }
    close(jobs)
    go func() { wg.Wait(); close(results) }()
    for r := range results { fmt.Println(r) }
}
```

### Context cancellation
```go
ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
defer cancel()
select {
case <-slowOperation(ctx):
case <-ctx.Done(): fmt.Println(ctx.Err()) // context deadline exceeded
}
```

### Concurrency checklist
1. Every goroutine must have a clear exit condition.
2. Know who owns and closes each channel.
3. Protect shared state (mutex/atomic) or confine it to one goroutine.
4. Propagate `context`.
5. Run tests with `-race`.

## 20.17 Defer, Panic, Recover

```go
func CopyFile(dst, src string) (err error) {
    in, err := os.Open(src)
    if err != nil { return err }
    defer in.Close()
    out, err := os.Create(dst)
    if err != nil { return err }
    defer func() { if cerr := out.Close(); err == nil { err = cerr } }()
    _, err = io.Copy(out, in)
    return err
}
```

## 20.18 Standard Library Tour

### I/O — `io`, `bufio`, `os`
```go
f, err := os.Open("data.txt"); if err != nil { log.Fatal(err) }; defer f.Close()
sc := bufio.NewScanner(f)
for sc.Scan() { fmt.Println(sc.Text()) }
if err := sc.Err(); err != nil { log.Fatal(err) }

data, _ := os.ReadFile("a.txt")
_ = os.WriteFile("b.txt", data, 0o644)
w := bufio.NewWriter(os.Stdout); defer w.Flush()
io.Copy(dst, src); io.ReadAll(r); io.MultiWriter(a, b); io.TeeReader(r, w); io.LimitReader(r, n)
os.MkdirAll("a/b", 0o755); os.Getenv("HOME"); os.Args
filepath.Join("a", "b"); filepath.WalkDir(".", fn)
```

### JSON — `encoding/json`
```go
type Item struct {
    ID    int       `json:"id"`
    Name  string    `json:"name"`
    Price float64   `json:"price,omitempty"`
    At    time.Time `json:"created_at"`
}
b, _ := json.Marshal(item)
json.MarshalIndent(item, "", "  ")
var it Item
if err := json.Unmarshal(b, &it); err != nil {}
// streaming
json.NewEncoder(w).Encode(item)
dec := json.NewDecoder(r.Body); dec.DisallowUnknownFields(); dec.Decode(&it)
// dynamic
var anyJSON map[string]any
json.RawMessage // delay decoding
// custom
func (s Status) MarshalJSON() ([]byte, error) { return json.Marshal(s.String()) }
```
Numbers decode to `float64` in `any`; use `dec.UseNumber()`. `omitzero` tag option added in 1.24. An experimental `encoding/json/v2` exists (GOEXPERIMENT=jsonv2, 1.25).

### Time
```go
now := time.Now()
t := now.Add(2 * time.Hour); d := t.Sub(now)
now.Format(time.RFC3339); now.Format("2006-01-02 15:04:05") // reference time Mon Jan 2 15:04:05 MST 2006
time.Parse("2006-01-02", "2024-05-10")
time.Since(start); time.Until(t)
time.Sleep(100 * time.Millisecond)
```

### Others worth knowing
`sort`, `container/heap`, `container/list`, `math`, `math/rand/v2`, `crypto/rand`, `crypto/sha256`, `crypto/tls`, `encoding/base64`, `encoding/csv`, `encoding/xml`, `encoding/gob`, `html/template` (auto-escaping), `text/template`, `net`, `net/http`, `net/url`, `log`, `log/slog` (structured logging, 1.21), `flag`, `os/exec`, `os/signal`, `path/filepath`, `reflect`, `runtime`, `runtime/debug`, `unicode/utf8`, `compress/gzip`, `archive/zip`, `embed`, `testing`, `expvar`.

**Structured logging with slog:**
```go
logger := slog.New(slog.NewJSONHandler(os.Stdout, &slog.HandlerOptions{Level: slog.LevelInfo}))
slog.SetDefault(logger)
slog.Info("user login", "user_id", 42, "ip", "1.2.3.4")
logger.With("request_id", rid).Error("db failed", "err", err)
```

**Heap example (priority queue):**
```go
type IntHeap []int
func (h IntHeap) Len() int           { return len(h) }
func (h IntHeap) Less(i, j int) bool { return h[i] < h[j] }
func (h IntHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *IntHeap) Push(x any)        { *h = append(*h, x.(int)) }
func (h *IntHeap) Pop() any          { old := *h; n := len(old); x := old[n-1]; *h = old[:n-1]; return x }
h := &IntHeap{5, 2, 8}; heap.Init(h); heap.Push(h, 1); heap.Pop(h) // 1
```

**CLI with flag:**
```go
port := flag.Int("port", 8080, "listen port")
verbose := flag.Bool("v", false, "verbose")
flag.Parse()
```

## 20.19 Reflection

```go
v := reflect.ValueOf(&user).Elem()
t := v.Type()
for i := 0; i < t.NumField(); i++ {
    f := t.Field(i)
    fmt.Println(f.Name, f.Type, f.Tag.Get("json"), v.Field(i).Interface())
}
if v.FieldByName("Name").CanSet() { v.FieldByName("Name").SetString("Bob") }
```
Laws of reflection: (1) interface value → reflection object, (2) reflection object → interface value, (3) to modify, value must be settable (addressable, obtained via pointer). Reflection is slow and loses type safety — used by `encoding/json`, ORMs, validators.

## 20.20 unsafe (advanced)
```go
unsafe.Sizeof(x); unsafe.Alignof(x); unsafe.Offsetof(s.f)
b := unsafe.Slice(unsafe.StringData(s), len(s)) // zero-copy string→bytes (read only!)
```
Breaks portability and memory safety guarantees; avoid unless necessary.

## 20.21 Testing (tutorial)
See Section 16. Directory: `calc.go` + `calc_test.go`; `go test ./... -v -race -cover`. HTTP handler test:
```go
func TestHealth(t *testing.T) {
    req := httptest.NewRequest(http.MethodGet, "/health", nil)
    rec := httptest.NewRecorder()
    HealthHandler(rec, req)
    if rec.Code != http.StatusOK { t.Fatalf("got %d", rec.Code) }
}
```

## 20.22 Modules & Workspace (tutorial)
```bash
go mod init github.com/me/app
go get github.com/google/uuid@latest
go mod tidy
go list -m all
go get -u ./...                 # upgrade
go work init ./app ./lib        # multi-module local dev
GOPROXY=https://proxy.golang.org,direct
GOPRIVATE=github.com/mycorp/*
```

## 20.23 Idiomatic Go — Effective Go Summary
- Handle errors immediately; keep the happy path unindented ("line of sight").
- Return early.
- Make the zero value useful.
- Accept interfaces, return concrete types.
- Small interfaces; define them where used.
- Don't panic in libraries.
- Use `gofmt`, `go vet`, `staticcheck`, `golangci-lint`.
- Receiver names short & consistent (`func (s *Server)`), not `this`/`self`.
- Getters without `Get` prefix: `Owner()`, setter `SetOwner()`.
- Package names singular and lowercase; avoid stutter (`http.Server`, not `http.HTTPServer`).
- Document exported identifiers with comments starting with the name.
- Clear is better than clever; a little copying is better than a little dependency.
- Concurrency: start goroutines with a known end; the caller decides concurrency (synchronous APIs preferred).

## 20.24 Common Design Patterns in Go
| Pattern | Go idiom |
|---|---|
| Singleton | `sync.Once` |
| Factory | `NewX()` constructors returning interfaces/structs |
| Builder / Options | Functional options |
| Strategy | Interface or func value field |
| Decorator / Middleware | Wrap `http.Handler` or interface |
| Adapter | `http.HandlerFunc` (func type implementing interface) |
| Observer | Channels / callback slices |
| Repository | Interface over storage |
| Dependency Injection | Constructor injection, `wire`, `fx` |
| Iterator | `iter.Seq`, channels, `Next()` methods (`sql.Rows`, `bufio.Scanner`) |

The `http.HandlerFunc` adapter is the canonical Go trick:
```go
type HandlerFunc func(ResponseWriter, *Request)
func (f HandlerFunc) ServeHTTP(w ResponseWriter, r *Request) { f(w, r) }
```

---
# PART III — INNER ARCHITECTURE OF GO

## 21.1 The Big Picture

```
 your .go files
      │
      ▼
┌──────────────────── cmd/compile ────────────────────┐
│ scan/parse → AST → type check (types2) → IR (noder) │
│ → inlining, devirtualization, escape analysis       │
│ → walk (lowering) → SSA → ~50 optimization passes   │
│ → arch lowering → register alloc → machine code     │
└──────────────────────────────────────────────────────┘
      │  object files (.a per package, cached in GOCACHE)
      ▼
   cmd/link  (+ runtime package linked into every binary)
      │
      ▼
 single static executable: your code + Go runtime
 (scheduler, allocator, GC, netpoller, stack mgmt, maps, channels)
```
There is no VM: Go compiles to native machine code, but every binary embeds a **runtime** (~1–2 MB) written mostly in Go (with some assembly), which provides the services below.

## 21.2 The Compiler (gc toolchain)

1. **Parsing** (`cmd/compile/internal/syntax`): source → tokens → syntax tree. Automatic semicolon insertion.
2. **Type checking** (`types2`, same algorithm as `go/types`): resolves identifiers, checks types, infers generic type arguments, computes constants exactly.
3. **IR construction ("noding")**: unified IR format also used for export data (so importing packages don't re-parse sources).
4. **Middle-end optimizations**:
   - **Dead code elimination**
   - **Inlining**: small functions (budget ~80 AST nodes; "mid-stack" inlining since 1.12; PGO raises budget for hot functions).
   - **Devirtualization**: interface calls converted to direct calls when the concrete type is known (and PGO-guided for hot call sites).
   - **Escape analysis**: builds a data-flow graph of pointers to decide stack vs heap. `-gcflags=-m` shows decisions.
   - **Closure conversion**, **generic instantiation** (GC shape stenciling).
5. **Walk**: lowers complex constructs into simpler ones — `range` loops into index loops, `map[k]` into runtime calls `mapaccess1`, `append` into `growslice` calls, `switch` into binary searches/jump tables, string concatenation into `concatstrings`, `defer` into open-coded sequences, channel ops into `chansend1`/`chanrecv1`, `select` into `selectgo`.
6. **SSA backend** (`cmd/compile/internal/ssa`): converts to Static Single Assignment form; passes include common subexpression elimination, nil check elimination, **bounds check elimination (BCE)**, constant propagation, dead store elimination, loop rotation, phi elimination, register allocation (linear scan), instruction scheduling; architecture rewrite rules (`.rules` files) lower generic ops to machine ops.
7. **Object output** → **linker** (`cmd/link`): dead code elimination across packages, lays out sections, writes symbol tables, **pclntab** (PC→line tables used for stack traces, GC stack maps, and stack growth), DWARF debugging info.

Useful flags:
```bash
go build -gcflags="-m=2"          # escape & inlining decisions
go build -gcflags="-d=ssa/check_bce/debug=1"  # bounds checks left
GOSSAFUNC=MyFunc go build         # ssa.html visualizing each pass
go tool objdump -s MyFunc ./bin   # disassembly
go build -ldflags="-s -w"         # strip symbols/DWARF
go build -ldflags="-X main.version=1.2.3" # set string var at link time
```

**Calling convention:** Since Go 1.17 (amd64) the **register-based ABI (ABIInternal)** passes args/results in registers (~5% faster); ABI0 (stack-based) remains for assembly functions, with wrappers bridging.

**Function prologue:** each non-leaf (non-`nosplit`) function starts by comparing SP with `g.stackguard0`; if below, it calls `runtime.morestack` to grow the stack. The same check is used for **cooperative preemption** (the runtime sets `stackguard0 = stackPreempt` to force a goroutine into the scheduler).

## 21.3 Runtime Bootstrap

On start (`rt0_<os>_<arch>.s` → `runtime·rt0_go`):
1. Set up `g0` (the scheduler goroutine for M0) and `m0` (main thread).
2. `runtime.args`, `osinit` (CPU count, page size), `schedinit` (memory allocator `mallocinit`, GC init `gcinit`, create Ps per GOMAXPROCS, parse GODEBUG).
3. Create the main goroutine running `runtime.main`.
4. `mstart` → enter the scheduler loop.
5. `runtime.main` starts `sysmon` (on its own M without P), runs runtime `init`s, enables GC, runs all package `init`s, then calls `main.main`. When `main.main` returns, `exit(0)` — other goroutines are killed.

## 21.4 The Scheduler (GMP) in Depth

### Structures
```go
type g struct {           // goroutine
    stack       stack     // [lo, hi)
    stackguard0 uintptr
    m           *m        // current M (nil if not running)
    sched       gobuf     // saved SP, PC, BP, ctxt — for context switch
    atomicstatus atomic.Uint32 // _Gidle, _Grunnable, _Grunning, _Gsyscall, _Gwaiting, _Gdead, _Gcopystack, _Gpreempted
    waitreason  waitReason
    preempt     bool
    _defer      *_defer
    _panic      *_panic
    ...
}
type m struct {           // OS thread
    g0      *g            // scheduling stack (large, system stack)
    curg    *g            // currently running user G
    p       puintptr      // attached P
    nextp, oldp puintptr
    spinning bool         // looking for work
    gsignal *g            // signal handling goroutine
    ...
}
type p struct {           // logical processor
    status  uint32        // _Pidle, _Prunning, _Psyscall, _Pgcstop, _Pdead
    runqhead, runqtail uint32
    runq    [256]guintptr // local lock-free run queue
    runnext guintptr      // next G to run (for producer→consumer locality)
    mcache  *mcache       // per-P allocation cache (no locks needed)
    gFree   ...           // free G cache
    timers  ...           // per-P timer heap
    gcw     gcWork        // GC work buffer
    ...
}
type schedt struct {       // global scheduler state
    lock  mutex
    runq  gQueue           // global run queue
    midle, pidle ...       // idle Ms and Ps
    nmspinning atomic.Int32
    ...
}
```

### Goroutine lifecycle
```
 go f()  → newproc → G from P.gFree or allocate (2KB stack) → _Grunnable → put in P.runnext
                         │
                schedule() picks it → execute() → _Grunning (gogo: restore regs, jump)
                         │
   ┌──────────────┬──────┴────────┬─────────────────────┬──────────────────┐
 blocks on chan/  syscall         preempted (time      function returns
 mutex/netpoll    (entersyscall)  slice > 10ms)        → goexit → _Gdead
 → gopark          → _Gsyscall    → _Grunnable → queue    → back to gFree
 → _Gwaiting      exitsyscall
 ready() → _Grunnable → runnext/runq
```

### schedule() → findRunnable() order
1. If GC needs workers, run GC mark worker.
2. Every 61st tick: check **global runq** (fairness so global Gs don't starve).
3. `runnext`, then local **runq**.
4. Global runq (take a batch).
5. **Netpoll** (non-blocking) → ready Gs whose I/O completed.
6. **Steal** from other Ps (random order, up to 4 attempts, take half their runq; also steal timers and runnext).
7. Nothing → release P, M becomes idle (`stopm`), possibly blocking in netpoll.

"Spinning" Ms (looking for work) are limited to about GOMAXPROCS/2 to avoid burning CPU.

### Blocking system calls (handoff)
`entersyscall` marks P as `_Psyscall` but keeps it attached. If the syscall is fast, `exitsyscall` just reacquires it. If it takes > 20µs, **sysmon retakes the P** and hands it to another M (spawning one if necessary), so blocking syscalls don't reduce parallelism. This is why a Go program can have more threads than GOMAXPROCS. (Max threads: 10,000 by default, `debug.SetMaxThreads`.)

### Network I/O — the netpoller
Sockets are set non-blocking. On `EAGAIN`, the goroutine parks on a `pollDesc` and the fd is registered with **epoll** (Linux), **kqueue** (BSD/macOS), **IOCP** (Windows), event ports (Solaris). The scheduler polls for readiness in `findRunnable` and in sysmon and readies the goroutine. Result: blocking-style code (`conn.Read`) with event-driven efficiency — no callback hell, and one thread can serve thousands of connections. File I/O on regular files is still blocking syscalls (thread handoff).

### Preemption
- **Cooperative**: at function prologues via stackguard check; at channel/lock/park points.
- **Asynchronous (Go 1.14+)**: sysmon detects a G running > 10ms, sends **SIGURG** to its thread; the signal handler checks it's at an async-safe point and injects a call to `asyncPreempt`, which saves all registers and yields. Solves tight loops like `for {}` blocking GC STW.

### sysmon
A background thread (no P) waking every 20µs–10ms that: retakes Ps from long syscalls, preempts long-running Gs, polls the network if not polled for 10ms, triggers forced GC every 2 minutes if none happened, and returns unused memory to the OS (scavenger coordination).

### Timers
Each P holds a 4-ary min-heap of timers; checked in `schedule`/`findRunnable`. `time.Sleep` parks the G with a timer. (Go 1.23 redesigned timers: unreferenced timers are GC-able, channels are synchronous.)

### Locking the thread
`runtime.LockOSThread()` wires the G to its M (needed for thread-local C libraries, OpenGL, namespaces).

## 21.5 Goroutine Stacks

- Start at **2 KB** (min, since 1.4); average starting size may be adjusted based on historical usage (1.19).
- **Contiguous (copying) stacks**: on overflow, `morestack` → `newstack` allocates a stack **2× bigger**, copies the old contents, and **adjusts all pointers into the stack** (using stack maps from pclntab). Replaced the earlier "segmented stacks" which caused "hot split" thrashing.
- Stacks **shrink** during GC if less than 1/4 used.
- Consequence: pointers to stack objects can change — that's why Go cannot let C hold pointers to Go stack memory and why escape analysis matters.
- Max stack size: 1 GB (64-bit), 250 MB (32-bit).
- `g0` (per M) uses a large OS-allocated system stack for runtime code; signal handling uses `gsignal`.

## 21.6 Memory Allocator

Based on **TCMalloc** ideas: thread(P)-local caches, size classes, and page-level management.

### Hierarchy
```
 mcache  (per P, lock-free)          ─ one mspan per size class (×2: scan / noscan)
   │ refill
 mcentral (per size class, global, locked-lightly) ─ lists of partial/full spans
   │ grow
 mheap   (global, lock)              ─ manages pages (8 KB) via radix-tree page allocator
   │ sysAlloc
 OS (mmap / VirtualAlloc)           ─ arenas of 64 MB (64-bit Linux)
```

### Object categories
| Class | Size | Path |
|---|---|---|
| **Tiny** | < 16 B and pointer-free | combined into a 16 B block in mcache (`tiny` allocator) — e.g. small strings |
| **Small** | ≤ 32 KB | rounded to one of **~68 size classes** (8, 16, 24, 32, 48, 64, 80 …, 32768), allocated from the P's mcache span — no locks |
| **Large** | > 32 KB | allocated directly from mheap as dedicated span of N pages |

### mspan
A run of contiguous pages dedicated to a single size class. It has an `allocBits` bitmap, `freeindex`, and `allocCache` (64-bit cache for fast find-first-free using `ctz` instruction). **Span class** = size class × noscan bit: pointer-free objects live in `noscan` spans that the GC doesn't need to scan.

### Heap metadata
- Heap bitmap / pointer masks per span (redesigned in 1.22 with per-object headers for larger objects) tell GC which words are pointers.
- `mheap.arenas` → `heapArena` maps addresses to spans (`spans` array).

### Scavenger
Background goroutine returning unused physical pages to the OS (`MADV_DONTNEED`/`MADV_FREE`), paced to keep RSS near the heap goal / memory limit.

### Allocation fast path (`mallocgc`)
1. Size 0 → return `&zerobase`.
2. Pick size class; check `mcache.alloc[spanClass]`; use `nextFreeFast` via `allocCache`.
3. If span full → `mcache.refill` from mcentral → possibly mheap.
4. If GC is marking → allocate black (new objects are marked) and perform **GC assist** proportional to allocation.
5. Zero memory if needed (`needzero`).

## 21.7 Garbage Collector in Depth

### Algorithm
**Concurrent tri-color mark & sweep, non-moving, non-generational**, with a **hybrid write barrier** (Yuasa deletion barrier + Dijkstra insertion barrier, Go 1.8) that removed the need to re-scan stacks at mark termination.

### Phases
```
 ┌──────────┐ STW ┌──────────────────────────┐ STW ┌────────────┐
 │ Sweep off│ ──► │ Mark (concurrent)         │ ──► │ Mark term. │ ──► Sweep (concurrent, lazy) ──► Off
 └──────────┘     │ write barrier ON          │     │ WB off     │
                  │ 25% CPU dedicated workers │     └────────────┘
                  │ + mutator assists         │
                  └──────────────────────────┘
```
1. **Sweep termination (STW)**: finish sweeping any unswept spans; enable write barrier; set `gcphase = _GCmark`.
2. **Concurrent mark**: roots = goroutine stacks (each scanned at a safe point when the G is stopped briefly), globals (data/bss), finalizer specials. Grey objects are drained from per-P `gcWork` buffers to a global work list. Background mark workers get 25% of GOMAXPROCS (dedicated + fractional). Allocating goroutines pay **assist credit** — the more they allocate, the more they mark — preventing the heap from outrunning the GC.
3. **Mark termination (STW)**: flush work, verify done, disable write barrier, compute next heap goal.
4. **Sweep**: spans are swept lazily by allocating goroutines (before reusing a span) and by a background sweeper. Unmarked objects become free slots (allocBits ← gcmarkBits).

### Tri-color invariant
- White: not yet reached (garbage at end).
- Grey: reached, children not scanned.
- Black: reached, fully scanned.
Strong invariant: black never points to white. The write barrier shades objects when pointers are written during marking so the mutator cannot hide a white object.

**Hybrid write barrier (pseudo):**
```
writePointer(slot, ptr):
    shade(*slot)          // deletion barrier: grey the old target
    if current stack is grey:
        shade(ptr)        // insertion barrier
    *slot = ptr
```
Stacks are not barriered (too expensive) — handled by scanning them once and allocating new objects black.

### Pacer
Decides when to start the next cycle so marking finishes as the heap reaches the goal:
```
heap_goal = live_heap_after_last_mark × (1 + GOGC/100)   (plus stack & globals roots since 1.18)
```
Bounded by `GOMEMLIMIT`. A proportional-integral controller (redesigned in 1.18) estimates assist ratio and trigger point.

### Green Tea GC (1.25 experiment, default in 1.26)
Instead of scanning object-by-object (random memory access, cache misses), it queues and scans **whole spans/pages** of small objects, improving locality and enabling vectorized scanning — typically 10–40% less GC CPU for allocation-heavy programs.

### Observing
```bash
GODEBUG=gctrace=1 ./app
# gc 12 @3.2s 1%: 0.02+1.3+0.01 ms clock, 0.1+0.4/1.2/2.1+0.08 ms cpu, 4->5->2 MB, 5 MB goal, 8 P
#            │   STW sweep term + concurrent mark + STW mark term      heap start->end->live
```
`runtime.ReadMemStats`, `runtime/metrics`, `go tool trace` show GC timelines.

## 21.8 Escape Analysis Internals
The compiler builds a directed weighted graph of "locations" with edges representing assignments (with dereference counts). A location escapes if it flows to the heap (global, returned pointer, parameter tagged as leaking, interface conversion to unknown callee, closure captured and outlives frame). Function parameters get **escape tags** in export data (e.g., "leaking param: p to result ~r0 level=0") so cross-package calls are analyzed without source. Tips: avoid returning pointers for small structs, pass buffers in (`Read(p []byte)` pattern), avoid `interface{}` in hot paths.

## 21.9 Internals of Core Data Structures (summary)

### Slice
`{array, len, cap}`; `growslice` handles growth and size-class rounding. Slices of zero-size elements point to `zerobase`.

### String
`{ptr, len}`; immutable; concatenation allocates via `concatstrings` (with a small stack buffer optimization for non-escaping results ≤32 bytes).

### Map (Swiss table, Go 1.24+)
```
Map
 ├─ directory []*table   (extendible hashing: top bits of hash choose table)
 │    table: up to 1024 slots in groups
 │      group: 8 control bytes (ctrl word) + 8 slots (key,elem)
 │         ctrl byte: empty(0x80) / deleted(0xFE) / full(0b0hhhhhhh = H2, low 7 hash bits)
 └─ small maps (≤8 entries): a single group, no table
Lookup: hash → H1 (high 57 bits) selects probe start group; H2 compared against all 8 ctrl
bytes at once using bitwise tricks (SWAR/SIMD) → candidate slots → full key compare;
quadratic probing across groups until a group with an empty slot is seen.
Growth: a table exceeding 7/8 load doubles; tables >1024 slots split, so growth cost is bounded.
Iteration: random start offset; iterators handle concurrent growth semantics.
```
Hash function: AES-based (AESENC instructions when available) `memhash`, with a per-map random seed to resist hash-flooding.

Pre-1.24 (classic, still asked): `hmap{count, B (log2 buckets), buckets, oldbuckets, nevacuate, hash0}`, `bmap` buckets with 8 `tophash` bytes + 8 keys + 8 values + overflow pointer; load factor 6.5; doubling or same-size growth (too many overflow buckets); incremental evacuation during writes.

### Channel
`hchan` + `sudog` wait queues; see Q104. `select` (`selectgo`): shuffles cases (pollorder), locks all channels in address order (lockorder, avoids deadlock), checks readiness; if none, enqueues a sudog on every channel and parks; when woken, dequeues from the others.

### Interface
`iface{tab *itab, data}` / `eface{_type, data}`; `itabTable` global cache; type assertions to concrete types compare `_type` pointers (cheap); to interfaces need itab lookup (cached, and since 1.22 per-callsite caches).

### Mutex
`sync.Mutex{state int32; sema uint32}`: state bits = locked, woken, starving, waiter count. Fast path: single CAS. Slow path: spin briefly (if multicore and others running), then `runtime_SemacquireMutex` which parks the G on a semaphore table (treap keyed by address). Starvation mode after 1ms waiting.

### WaitGroup
64-bit state: counter (high 32) + waiter count (low 32), plus a semaphore.

### defer
Open-coded (inlined at exits with a bitmask tracking which defers are active) for ≤8 defers without loops; otherwise `_defer` records chained on the G. `panic` walks the defer chain; with open-coded defers it uses funcdata to find and run them.

### panic/recover
`gopanic` creates `_panic` on the G, runs deferred calls; `recover` sets `recovered`, and the runtime resumes execution at the deferring function's return sequence (`recovery`) by adjusting SP/PC.

## 21.10 Summary Diagram
```
                       ┌───────────────────── Go Process ─────────────────────┐
  Goroutines  G G G G G G G G G G G G G G G G G G G G G (millions possible)    │
                 │ scheduled onto                                            │
  Ps  [P0: runq|mcache] [P1: runq|mcache] ... [Pn]  (n = GOMAXPROCS)         │
                 │ executed by                                               │
  Ms   M0  M1  M2  M3 ... (OS threads; extra ones for syscalls)  + sysmon    │
                 │                                                           │
  Runtime services: netpoller(epoll) · timers · allocator(mcache→mcentral→   │
                    mheap) · GC (concurrent tri-color, write barrier, pacer)  │
                    · stack growth/copy · signal handling · scavenger        │
                       └──────────────────────────────────────────────────────┘
                                         │ syscalls
                                      OS Kernel
```

---
# PART IV — WEB DEVELOPMENT: MVC & FRAMEWORKS

## 22.1 How `net/http` Works

Go's standard library ships a production-grade HTTP/1.1 + HTTP/2 server and client.

### Core abstractions
```go
type Handler interface { ServeHTTP(ResponseWriter, *Request) }
type HandlerFunc func(ResponseWriter, *Request)   // adapter
type ResponseWriter interface {
    Header() Header
    Write([]byte) (int, error)
    WriteHeader(statusCode int)
}
```

### Server internals
`http.Server.ListenAndServe` → `net.Listen("tcp")` → `Serve(l)` loops on `l.Accept()`; **each connection gets its own goroutine** (`go c.serve(ctx)`), which reads requests (keep-alive loop), builds `*Request` with a context, and calls `Handler.ServeHTTP`. Because the netpoller makes blocking reads cheap, the goroutine-per-connection model scales to tens of thousands of connections. A panic in a handler is recovered per connection and logged. HTTP/2 is enabled automatically over TLS.

**Always configure timeouts** (the zero values mean no timeout → slowloris risk):
```go
srv := &http.Server{
    Addr:              ":8080",
    Handler:           mux,
    ReadHeaderTimeout: 5 * time.Second,
    ReadTimeout:       10 * time.Second,
    WriteTimeout:      15 * time.Second,
    IdleTimeout:       60 * time.Second,
    MaxHeaderBytes:    1 << 20,
}
```

### Enhanced routing (Go 1.22+ ServeMux)
```go
mux := http.NewServeMux()
mux.HandleFunc("GET /users", listUsers)
mux.HandleFunc("GET /users/{id}", getUser)
mux.HandleFunc("POST /users", createUser)
mux.HandleFunc("DELETE /users/{id}", deleteUser)
mux.HandleFunc("GET /static/{path...}", serveStatic) // wildcard remainder
mux.HandleFunc("GET /{$}", home)                      // exact root

func getUser(w http.ResponseWriter, r *http.Request) {
    id := r.PathValue("id")
    ...
}
```
Method matching, path parameters, and most-specific-wins precedence — for many services a third-party router is no longer necessary.

### Middleware pattern
Middleware = function `func(http.Handler) http.Handler`:
```go
func Logging(next http.Handler) http.Handler {
    return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
        start := time.Now()
        rw := &statusRecorder{ResponseWriter: w, status: 200}
        next.ServeHTTP(rw, r)
        slog.Info("req", "method", r.Method, "path", r.URL.Path, "status", rw.status, "dur", time.Since(start))
    })
}
type statusRecorder struct { http.ResponseWriter; status int }
func (s *statusRecorder) WriteHeader(c int) { s.status = c; s.ResponseWriter.WriteHeader(c) }

func Recoverer(next http.Handler) http.Handler {
    return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
        defer func() {
            if rec := recover(); rec != nil {
                slog.Error("panic", "err", rec, "stack", string(debug.Stack()))
                http.Error(w, "internal error", http.StatusInternalServerError)
            }
        }()
        next.ServeHTTP(w, r)
    })
}

func Chain(h http.Handler, m ...func(http.Handler) http.Handler) http.Handler {
    for i := len(m) - 1; i >= 0; i-- { h = m[i](h) }
    return h
}
handler := Chain(mux, Recoverer, Logging, CORS, Auth)
```

### Writing JSON helpers
```go
func writeJSON(w http.ResponseWriter, status int, v any) {
    w.Header().Set("Content-Type", "application/json")
    w.WriteHeader(status)
    _ = json.NewEncoder(w).Encode(v)
}
func readJSON(w http.ResponseWriter, r *http.Request, dst any) error {
    r.Body = http.MaxBytesReader(w, r.Body, 1<<20)
    dec := json.NewDecoder(r.Body)
    dec.DisallowUnknownFields()
    return dec.Decode(dst)
}
```

### HTTP client best practices
Reuse a single `http.Client` (it pools connections via `Transport`), set `Timeout`, always `defer resp.Body.Close()` and drain the body for connection reuse, use `NewRequestWithContext`. Never use `http.DefaultClient` in production without timeout.

## 22.2 What is MVC?

**Model–View–Controller** separates an application into three responsibilities:

| Layer | Responsibility | In Go |
|---|---|---|
| **Model** | Data structures, business rules, persistence | structs + repository/service packages |
| **View** | Presentation | `html/template` files, or JSON serialization for APIs |
| **Controller** | Receives HTTP requests, validates input, calls model/services, chooses view | handler functions/structs |

```
 Browser ──HTTP──► Router ──► Middleware ──► Controller ──► Service ──► Repository ──► DB
                                                 │                (Model layer)
                                                 └──► View (template / JSON) ──► Response
```

Go doesn't impose MVC; most Go APIs use a **layered / clean architecture** that is MVC-inspired: `handler` (controller) → `service` (business logic) → `repository` (data access) with `model`/`domain` structs. For server-rendered sites, templates form the View.

## 22.3 Complete MVC Application (standard library + templates)

### Project layout
```
bookstore/
├── cmd/web/main.go
├── internal/
│   ├── config/config.go
│   ├── models/book.go           # Model: entity + validation
│   ├── repository/book_repo.go  # Model: persistence
│   ├── service/book_service.go  # Model: business logic
│   ├── controllers/book_controller.go
│   ├── middleware/middleware.go
│   └── routes/routes.go
├── views/
│   ├── layouts/base.html
│   └── books/index.html  show.html  form.html
├── migrations/001_create_books.sql
└── go.mod
```

### Model
```go
// internal/models/book.go
package models

import ("errors"; "strings"; "time")

type Book struct {
    ID        int64     `json:"id"`
    Title     string    `json:"title"`
    Author    string    `json:"author"`
    Price     float64   `json:"price"`
    CreatedAt time.Time `json:"created_at"`
}

var ErrNotFound = errors.New("book not found")

type ValidationErrors map[string]string

func (b *Book) Validate() ValidationErrors {
    errs := ValidationErrors{}
    if strings.TrimSpace(b.Title) == "" { errs["title"] = "title is required" }
    if strings.TrimSpace(b.Author) == "" { errs["author"] = "author is required" }
    if b.Price < 0 { errs["price"] = "price must be >= 0" }
    if len(errs) == 0 { return nil }
    return errs
}
```

### Repository (data access)
```go
// internal/repository/book_repo.go
package repository

type BookRepository interface {
    List(ctx context.Context, limit, offset int) ([]models.Book, error)
    Get(ctx context.Context, id int64) (*models.Book, error)
    Create(ctx context.Context, b *models.Book) error
    Update(ctx context.Context, b *models.Book) error
    Delete(ctx context.Context, id int64) error
}

type PostgresBookRepo struct{ db *sql.DB }

func NewPostgresBookRepo(db *sql.DB) *PostgresBookRepo { return &PostgresBookRepo{db: db} }

func (r *PostgresBookRepo) List(ctx context.Context, limit, offset int) ([]models.Book, error) {
    rows, err := r.db.QueryContext(ctx,
        `SELECT id, title, author, price, created_at FROM books ORDER BY id LIMIT $1 OFFSET $2`,
        limit, offset)
    if err != nil { return nil, err }
    defer rows.Close()
    var books []models.Book
    for rows.Next() {
        var b models.Book
        if err := rows.Scan(&b.ID, &b.Title, &b.Author, &b.Price, &b.CreatedAt); err != nil {
            return nil, err
        }
        books = append(books, b)
    }
    return books, rows.Err()
}

func (r *PostgresBookRepo) Get(ctx context.Context, id int64) (*models.Book, error) {
    var b models.Book
    err := r.db.QueryRowContext(ctx,
        `SELECT id, title, author, price, created_at FROM books WHERE id = $1`, id).
        Scan(&b.ID, &b.Title, &b.Author, &b.Price, &b.CreatedAt)
    if errors.Is(err, sql.ErrNoRows) { return nil, models.ErrNotFound }
    return &b, err
}

func (r *PostgresBookRepo) Create(ctx context.Context, b *models.Book) error {
    return r.db.QueryRowContext(ctx,
        `INSERT INTO books (title, author, price) VALUES ($1, $2, $3) RETURNING id, created_at`,
        b.Title, b.Author, b.Price).Scan(&b.ID, &b.CreatedAt)
}

func (r *PostgresBookRepo) Update(ctx context.Context, b *models.Book) error {
    res, err := r.db.ExecContext(ctx,
        `UPDATE books SET title=$1, author=$2, price=$3 WHERE id=$4`, b.Title, b.Author, b.Price, b.ID)
    if err != nil { return err }
    if n, _ := res.RowsAffected(); n == 0 { return models.ErrNotFound }
    return nil
}

func (r *PostgresBookRepo) Delete(ctx context.Context, id int64) error {
    res, err := r.db.ExecContext(ctx, `DELETE FROM books WHERE id=$1`, id)
    if err != nil { return err }
    if n, _ := res.RowsAffected(); n == 0 { return models.ErrNotFound }
    return nil
}
```

### Service (business logic)
```go
// internal/service/book_service.go
package service

type BookService struct{ repo repository.BookRepository }

func NewBookService(r repository.BookRepository) *BookService { return &BookService{repo: r} }

func (s *BookService) Create(ctx context.Context, b *models.Book) (models.ValidationErrors, error) {
    if errs := b.Validate(); errs != nil { return errs, nil }
    b.Title = strings.TrimSpace(b.Title)
    return nil, s.repo.Create(ctx, b)
}
func (s *BookService) List(ctx context.Context, page int) ([]models.Book, error) {
    const size = 20
    if page < 1 { page = 1 }
    return s.repo.List(ctx, size, (page-1)*size)
}
func (s *BookService) Get(ctx context.Context, id int64) (*models.Book, error) { return s.repo.Get(ctx, id) }
func (s *BookService) Delete(ctx context.Context, id int64) error           { return s.repo.Delete(ctx, id) }
```

### View (templates)
```html
<!-- views/layouts/base.html -->
{{define "base"}}
<!doctype html>
<html><head><title>{{block "title" .}}Bookstore{{end}}</title></head>
<body>
  <nav><a href="/books">Books</a> | <a href="/books/new">Add</a></nav>
  <main>{{template "content" .}}</main>
</body></html>
{{end}}

<!-- views/books/index.html -->
{{define "title"}}All Books{{end}}
{{define "content"}}
<h1>Books</h1>
<ul>
{{range .Books}}
  <li><a href="/books/{{.ID}}">{{.Title}}</a> by {{.Author}} — ${{printf "%.2f" .Price}}</li>
{{else}}
  <li>No books yet.</li>
{{end}}
</ul>
{{end}}

<!-- views/books/form.html -->
{{define "content"}}
<form method="POST" action="/books">
  <input name="title" value="{{.Book.Title}}"> {{with .Errors.title}}<span>{{.}}</span>{{end}}
  <input name="author" value="{{.Book.Author}}"> {{with .Errors.author}}<span>{{.}}</span>{{end}}
  <input name="price" value="{{.Book.Price}}">
  <input type="hidden" name="csrf_token" value="{{.CSRF}}">
  <button>Save</button>
</form>
{{end}}
```
`html/template` contextually auto-escapes output (HTML, JS, CSS, URL contexts) to prevent XSS.

```go
// template renderer with per-page cache
type Renderer struct{ pages map[string]*template.Template }

func NewRenderer(fsys fs.FS) (*Renderer, error) {
    pages := map[string]*template.Template{}
    for _, p := range []string{"books/index.html", "books/show.html", "books/form.html"} {
        t, err := template.New("base").Funcs(template.FuncMap{"upper": strings.ToUpper}).
            ParseFS(fsys, "views/layouts/base.html", "views/"+p)
        if err != nil { return nil, err }
        pages[p] = t
    }
    return &Renderer{pages}, nil
}

func (r *Renderer) Render(w http.ResponseWriter, status int, page string, data any) {
    t, ok := r.pages[page]
    if !ok { http.Error(w, "template missing", 500); return }
    var buf bytes.Buffer                     // render to buffer first so errors don't send half a page
    if err := t.ExecuteTemplate(&buf, "base", data); err != nil {
        http.Error(w, err.Error(), 500); return
    }
    w.WriteHeader(status)
    buf.WriteTo(w)
}
```

### Controller
```go
// internal/controllers/book_controller.go
package controllers

type BookController struct {
    svc  *service.BookService
    view *Renderer
}

func NewBookController(s *service.BookService, v *Renderer) *BookController { return &BookController{s, v} }

func (c *BookController) Index(w http.ResponseWriter, r *http.Request) {
    page, _ := strconv.Atoi(r.URL.Query().Get("page"))
    books, err := c.svc.List(r.Context(), page)
    if err != nil { serverError(w, err); return }
    c.view.Render(w, http.StatusOK, "books/index.html", map[string]any{"Books": books})
}

func (c *BookController) Show(w http.ResponseWriter, r *http.Request) {
    id, err := strconv.ParseInt(r.PathValue("id"), 10, 64)
    if err != nil { http.NotFound(w, r); return }
    b, err := c.svc.Get(r.Context(), id)
    if errors.Is(err, models.ErrNotFound) { http.NotFound(w, r); return }
    if err != nil { serverError(w, err); return }
    c.view.Render(w, http.StatusOK, "books/show.html", map[string]any{"Book": b})
}

func (c *BookController) New(w http.ResponseWriter, r *http.Request) {
    c.view.Render(w, 200, "books/form.html", map[string]any{"Book": models.Book{}})
}

func (c *BookController) Create(w http.ResponseWriter, r *http.Request) {
    if err := r.ParseForm(); err != nil { http.Error(w, "bad form", 400); return }
    price, _ := strconv.ParseFloat(r.PostForm.Get("price"), 64)
    b := &models.Book{Title: r.PostForm.Get("title"), Author: r.PostForm.Get("author"), Price: price}
    verrs, err := c.svc.Create(r.Context(), b)
    if err != nil { serverError(w, err); return }
    if verrs != nil {
        c.view.Render(w, http.StatusUnprocessableEntity, "books/form.html",
            map[string]any{"Book": b, "Errors": verrs}); return
    }
    http.Redirect(w, r, fmt.Sprintf("/books/%d", b.ID), http.StatusSeeOther) // PRG pattern
}

// JSON API variant of the same controller
func (c *BookController) APIList(w http.ResponseWriter, r *http.Request) {
    books, err := c.svc.List(r.Context(), 1)
    if err != nil { writeJSON(w, 500, map[string]string{"error": "internal"}); return }
    writeJSON(w, 200, books)
}

func serverError(w http.ResponseWriter, err error) {
    slog.Error("server error", "err", err)
    http.Error(w, "internal server error", http.StatusInternalServerError)
}
```

### Routes & main (wiring / dependency injection)
```go
// internal/routes/routes.go
func Register(bc *controllers.BookController) http.Handler {
    mux := http.NewServeMux()
    mux.HandleFunc("GET /books", bc.Index)
    mux.HandleFunc("GET /books/new", bc.New)
    mux.HandleFunc("POST /books", bc.Create)
    mux.HandleFunc("GET /books/{id}", bc.Show)
    mux.HandleFunc("GET /api/books", bc.APIList)
    mux.Handle("GET /static/", http.StripPrefix("/static/", http.FileServer(http.Dir("./static"))))
    return middleware.Chain(mux, middleware.Recoverer, middleware.Logging, middleware.SecureHeaders)
}

// cmd/web/main.go
func main() {
    cfg := config.Load()
    db, err := sql.Open("pgx", cfg.DatabaseURL)
    if err != nil { log.Fatal(err) }
    db.SetMaxOpenConns(25); db.SetMaxIdleConns(25); db.SetConnMaxLifetime(5 * time.Minute)
    ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
    defer cancel()
    if err := db.PingContext(ctx); err != nil { log.Fatal(err) }
    defer db.Close()

    renderer, err := controllers.NewRenderer(os.DirFS("."))
    if err != nil { log.Fatal(err) }

    repo := repository.NewPostgresBookRepo(db)
    svc := service.NewBookService(repo)
    bc := controllers.NewBookController(svc, renderer)

    srv := &http.Server{Addr: cfg.Addr, Handler: routes.Register(bc),
        ReadHeaderTimeout: 5 * time.Second, WriteTimeout: 15 * time.Second, IdleTimeout: 60 * time.Second}
    // graceful shutdown as in C8
    log.Fatal(srv.ListenAndServe())
}
```
Note how dependencies flow **downward** and every layer depends on interfaces, enabling unit tests with fake repositories.

## 22.4 Popular Go Web Frameworks

| Framework | Style | Built on | Highlights |
|---|---|---|---|
| **Gin** | Router + middleware | net/http, radix tree router (httprouter) | Most popular; fast; binding/validation; `gin.Context` |
| **Echo** | Router + middleware | net/http | Clean API, returns errors from handlers, centralized error handler |
| **Fiber** | Express.js-like | **fasthttp** (not net/http) | Very fast, low alloc; not compatible with net/http middleware; context objects are reused (don't keep references) |
| **Chi** | Lightweight router | net/http, 100% compatible | Idiomatic, composable middleware, route groups |
| **Gorilla/mux** | Router | net/http | Classic; regex routes (project maintained again by community) |
| **Beego** | Full MVC framework | net/http | Rails/Django-like: ORM, controllers, views, CLI `bee`, sessions, caching |
| **Revel** | Full-stack MVC | | Convention-based MVC, hot reload (less active) |
| **Buffalo** | Full-stack | | Rails-like generator (archived/less active) |
| **Iris** | Full-featured | | MVC package, sessions, websockets |
| **Huma / go-kit / go-zero / Kratos** | API/microservice | | OpenAPI generation, microservice toolkits |

**Choosing:** For APIs, standard library (1.22+ mux) or Chi keeps you idiomatic; Gin/Echo add ergonomics (binding, validation, grouping). Fiber for raw throughput if fasthttp constraints are acceptable. Beego/Iris for batteries-included MVC.

### Gin example (REST CRUD in MVC style)
```go
package main

import ("net/http"; "strconv"; "github.com/gin-gonic/gin")

type CreateBookReq struct {
    Title  string  `json:"title"  binding:"required,min=1,max=200"`
    Author string  `json:"author" binding:"required"`
    Price  float64 `json:"price"  binding:"gte=0"`
}

type BookHandler struct{ svc *service.BookService }

func (h *BookHandler) Create(c *gin.Context) {
    var req CreateBookReq
    if err := c.ShouldBindJSON(&req); err != nil {
        c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()}); return
    }
    b := &models.Book{Title: req.Title, Author: req.Author, Price: req.Price}
    if _, err := h.svc.Create(c.Request.Context(), b); err != nil {
        c.JSON(http.StatusInternalServerError, gin.H{"error": "internal"}); return
    }
    c.JSON(http.StatusCreated, b)
}

func (h *BookHandler) Get(c *gin.Context) {
    id, err := strconv.ParseInt(c.Param("id"), 10, 64)
    if err != nil { c.JSON(400, gin.H{"error": "bad id"}); return }
    b, err := h.svc.Get(c.Request.Context(), id)
    if errors.Is(err, models.ErrNotFound) { c.JSON(404, gin.H{"error": "not found"}); return }
    if err != nil { c.JSON(500, gin.H{"error": "internal"}); return }
    c.JSON(200, b)
}

func AuthMiddleware() gin.HandlerFunc {
    return func(c *gin.Context) {
        token := c.GetHeader("Authorization")
        if token == "" { c.AbortWithStatusJSON(401, gin.H{"error": "unauthorized"}); return }
        c.Set("userID", "42") // after verifying JWT
        c.Next()              // continue chain; code after Next runs post-handler
    }
}

func main() {
    r := gin.New()
    r.Use(gin.Logger(), gin.Recovery())
    h := &BookHandler{svc: newService()}
    api := r.Group("/api/v1")
    {
        api.GET("/books/:id", h.Get)
        auth := api.Group("/", AuthMiddleware())
        auth.POST("/books", h.Create)
    }
    r.Run(":8080")
}
```
Gin internals: a **radix tree (compressed trie) per HTTP method** for zero-allocation routing; `gin.Context` objects pooled via `sync.Pool`; handlers chain is a slice with an index advanced by `c.Next()`; `c.Abort()` sets index past the end. Validation via `go-playground/validator` struct tags.

### Echo example
```go
e := echo.New()
e.Use(middleware.Logger(), middleware.Recover())
e.GET("/books/:id", func(c echo.Context) error {
    b, err := svc.Get(c.Request().Context(), parseID(c.Param("id")))
    if errors.Is(err, models.ErrNotFound) { return echo.NewHTTPError(404, "not found") }
    if err != nil { return err } // centralized HTTPErrorHandler
    return c.JSON(200, b)
})
e.Logger.Fatal(e.Start(":8080"))
```

### Chi example
```go
r := chi.NewRouter()
r.Use(middleware.RequestID, middleware.RealIP, middleware.Logger, middleware.Recoverer)
r.Use(middleware.Timeout(30 * time.Second))
r.Route("/books", func(r chi.Router) {
    r.Get("/", listBooks)
    r.Post("/", createBook)
    r.Route("/{id}", func(r chi.Router) {
        r.Use(BookCtx) // load book into context
        r.Get("/", getBook)
        r.Delete("/", deleteBook)
    })
})
http.ListenAndServe(":8080", r)
// chi.URLParam(r, "id")
```

### Fiber example
```go
app := fiber.New()
app.Get("/books/:id", func(c *fiber.Ctx) error {
    id, _ := c.ParamsInt("id")
    return c.JSON(fiber.Map{"id": id})
})
app.Listen(":8080")
```
⚠ Values from `fiber.Ctx` (e.g., `c.Params`) are only valid within the handler because buffers are reused; copy them if needed.

### Beego (classic MVC)
```go
type BookController struct{ web.Controller }
func (c *BookController) Get() {
    c.Data["Books"] = models.AllBooks()
    c.TplName = "books/index.tpl" // View
}
func main() {
    web.Router("/books", &BookController{})
    web.Run()
}
```
Beego provides controllers with `Get/Post/Put` methods mapped automatically, an ORM (`client/orm`), config, sessions, cache, and `bee` CLI scaffolding (`bee new`, `bee generate scaffold`).

## 22.5 Cross-Cutting Web Concerns

- **Authentication**: sessions (`alexedwards/scs`, gorilla/sessions) with secure cookies; JWT (`golang-jwt/jwt/v5`) — validate `alg`, expiry, signature; OAuth2 (`golang.org/x/oauth2`). Password hashing with `bcrypt` or `argon2id` (`golang.org/x/crypto`).
- **CSRF**: `gorilla/csrf`, or Go 1.25's `http.CrossOriginProtection` (Fetch-metadata based).
- **CORS**: `rs/cors` or framework middleware.
- **Validation**: `go-playground/validator`.
- **Config**: env vars (`caarlos0/env`, `kelseyhightower/envconfig`), Viper.
- **Observability**: `log/slog`, Prometheus `client_golang`, OpenTelemetry tracing, `pprof`.
- **Rate limiting**: `x/time/rate` per client IP with a map + cleanup.
- **WebSockets**: `coder/websocket` (formerly nhooyr), `gorilla/websocket`.
- **Security headers**: CSP, HSTS, X-Content-Type-Options.
- **API docs**: `swaggo/swag`, `huma`, `oapi-codegen` (OpenAPI → Go server/client).
- **Testing handlers**: `httptest.NewRecorder`, `httptest.NewServer`.

## 22.6 Clean / Hexagonal Architecture vs MVC

```
          ┌────────── adapters (outer) ──────────┐
          │  HTTP handlers (Gin/Chi) · gRPC ·   │
          │  CLI · Postgres repo · Redis · Kafka │
          │     ┌────── application ───────┐     │
          │     │  use cases / services     │     │
          │     │    ┌──── domain ────┐     │     │
          │     │    │ entities, rules,│     │     │
          │     │    │ repo interfaces │     │     │
          │     │    └─────────────────┘     │     │
          │     └───────────────────────────┘     │
          └──────────────────────────────────────┘
      Dependencies point INWARD. Domain has no imports of frameworks/DB.
```
MVC maps to it naturally: Controller = inbound adapter, Model = domain + services + repositories, View = presenters/templates/JSON DTOs. In interviews, emphasize that Go favors **explicit wiring in `main`**, small interfaces owned by the consumer (service defines `BookRepository`), and avoiding framework lock-in in the domain.

---
# PART V — DATABASE IMPLEMENTATION

## 23.1 Options Overview

| Approach | Library | Pros | Cons |
|---|---|---|---|
| Std lib | `database/sql` + driver | Portable, pooled, explicit SQL | Verbose scanning, nullable handling |
| Native Postgres | `jackc/pgx/v5` (+ `pgxpool`) | Fastest, Postgres types, COPY, LISTEN/NOTIFY, batching | Postgres-only |
| Helpers | `jmoiron/sqlx` | Struct scanning, named params, `IN` expansion | Still hand SQL |
| Code generation | `sqlc` | Write SQL, get type-safe Go; compile-time checked | Dynamic queries harder |
| Query builders | `squirrel`, `goqu`, `bun` | Dynamic queries | Another DSL |
| ORM | `GORM`, `ent` (Meta, schema-as-code, codegen), `bun`, Beego ORM | Rapid CRUD, associations, hooks, migrations | Magic, hidden queries/N+1, reflection overhead |
| NoSQL | `mongo-driver`, `go-redis/v9`, `gocql` | | |

Common drivers: `github.com/jackc/pgx/v5/stdlib` (Postgres), `github.com/go-sql-driver/mysql`, `modernc.org/sqlite` (pure Go) / `mattn/go-sqlite3` (cgo), `github.com/microsoft/go-mssqldb`.

## 23.2 `database/sql` Architecture

```
 your code ──► *sql.DB (connection POOL, goroutine-safe, long-lived)
                  │  acquires *driverConn
                  ▼
             driver.Driver / driver.Connector  (registered via sql.Register in the driver's init)
                  │
                  ▼
             driver.Conn ─► driver.Stmt / driver.Tx / driver.Rows
                  │
                  ▼
              network socket to DB server
```
- `sql.Open` **does not connect**; it validates args and creates the pool. Call `PingContext` to verify.
- `*sql.DB` is meant to be created **once** and shared.
- Pool: idle connections list + open count; a request waits for a free connection when `MaxOpenConns` reached. Bad connections (`driver.ErrBadConn`) are retried transparently.
- `*sql.Rows` holds its connection until **closed or fully iterated** — forgetting `rows.Close()` leaks connections.
- `*sql.Tx` pins a single connection for its lifetime.
- Prepared statements (`db.Prepare`) are transparently re-prepared on each connection they run on.

### Pool configuration
```go
db.SetMaxOpenConns(25)                  // cap concurrency against DB (default unlimited!)
db.SetMaxIdleConns(25)                  // keep warm connections (default 2)
db.SetConnMaxLifetime(30 * time.Minute) // recycle (LBs, failovers)
db.SetConnMaxIdleTime(5 * time.Minute)
stats := db.Stats()                     // WaitCount, WaitDuration, InUse, Idle → export as metrics
```
Sizing rule of thumb: total connections across all instances ≤ DB `max_connections` minus headroom; for Postgres often ~ (cores × 2) + effective spindles per DB, and use PgBouncer for many instances.

## 23.3 CRUD with `database/sql`

### Connect
```go
import (
    "database/sql"
    _ "github.com/jackc/pgx/v5/stdlib" // registers "pgx"
)

func OpenDB(dsn string) (*sql.DB, error) {
    db, err := sql.Open("pgx", dsn) // "postgres://user:pass@localhost:5432/app?sslmode=disable"
    if err != nil { return nil, err }
    db.SetMaxOpenConns(25); db.SetMaxIdleConns(25); db.SetConnMaxIdleTime(5 * time.Minute)
    ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
    defer cancel()
    if err := db.PingContext(ctx); err != nil { db.Close(); return nil, err }
    return db, nil
}
```

### Schema
```sql
CREATE TABLE users (
    id         BIGSERIAL PRIMARY KEY,
    email      TEXT NOT NULL UNIQUE,
    name       TEXT NOT NULL,
    bio        TEXT,                 -- nullable
    balance    NUMERIC(12,2) NOT NULL DEFAULT 0,
    version    INT NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

### Insert, Query one, Query many, Update, Delete
```go
type User struct {
    ID        int64
    Email     string
    Name      string
    Bio       sql.NullString // or *string, or sql.Null[string] (1.22 generic)
    CreatedAt time.Time
}

func CreateUser(ctx context.Context, db *sql.DB, u *User) error {
    const q = `INSERT INTO users (email, name, bio) VALUES ($1, $2, $3) RETURNING id, created_at`
    return db.QueryRowContext(ctx, q, u.Email, u.Name, u.Bio).Scan(&u.ID, &u.CreatedAt)
}
// MySQL: use ? placeholders and res.LastInsertId()

func GetUser(ctx context.Context, db *sql.DB, id int64) (*User, error) {
    var u User
    err := db.QueryRowContext(ctx,
        `SELECT id, email, name, bio, created_at FROM users WHERE id = $1`, id).
        Scan(&u.ID, &u.Email, &u.Name, &u.Bio, &u.CreatedAt)
    switch {
    case errors.Is(err, sql.ErrNoRows): return nil, ErrNotFound
    case err != nil:                    return nil, fmt.Errorf("get user %d: %w", id, err)
    }
    return &u, nil
}

func ListUsers(ctx context.Context, db *sql.DB, afterID int64, limit int) ([]User, error) {
    rows, err := db.QueryContext(ctx,
        `SELECT id, email, name, bio, created_at FROM users WHERE id > $1 ORDER BY id LIMIT $2`,
        afterID, limit) // keyset pagination: faster than OFFSET for deep pages
    if err != nil { return nil, err }
    defer rows.Close()
    users := make([]User, 0, limit)
    for rows.Next() {
        var u User
        if err := rows.Scan(&u.ID, &u.Email, &u.Name, &u.Bio, &u.CreatedAt); err != nil { return nil, err }
        users = append(users, u)
    }
    return users, rows.Err() // ALWAYS check rows.Err()
}

func UpdateName(ctx context.Context, db *sql.DB, id int64, name string) error {
    res, err := db.ExecContext(ctx, `UPDATE users SET name = $1 WHERE id = $2`, name, id)
    if err != nil { return err }
    n, err := res.RowsAffected()
    if err != nil { return err }
    if n == 0 { return ErrNotFound }
    return nil
}

func DeleteUser(ctx context.Context, db *sql.DB, id int64) error {
    _, err := db.ExecContext(ctx, `DELETE FROM users WHERE id = $1`, id)
    return err
}
```

### SQL injection — always use placeholders
```go
// ❌ NEVER
db.Query("SELECT * FROM users WHERE email = '" + email + "'")
// ✅
db.QueryContext(ctx, "SELECT ... WHERE email = $1", email)
```
Identifiers (table/column names, ORDER BY direction) can't be parameters — whitelist them:
```go
allowed := map[string]string{"name": "name", "created": "created_at"}
col, ok := allowed[sortParam]; if !ok { col = "id" }
q := "SELECT ... ORDER BY " + col
```

### Handling NULLs
`sql.NullString/NullInt64/NullTime/NullBool`, generic `sql.Null[T]`, pointers `*string`, or `COALESCE(bio, '')` in SQL.

### Custom types: `driver.Valuer` and `sql.Scanner`
```go
type Tags []string
func (t Tags) Value() (driver.Value, error) { return json.Marshal(t) }
func (t *Tags) Scan(src any) error {
    b, ok := src.([]byte)
    if !ok { return fmt.Errorf("tags: unexpected %T", src) }
    return json.Unmarshal(b, t)
}
```

## 23.4 Transactions

```go
func Transfer(ctx context.Context, db *sql.DB, from, to int64, amount float64) (err error) {
    tx, err := db.BeginTx(ctx, &sql.TxOptions{Isolation: sql.LevelSerializable})
    if err != nil { return err }
    defer func() {
        if err != nil { _ = tx.Rollback() } // Rollback after Commit is a harmless no-op error
    }()

    var bal float64
    // lock row to prevent concurrent overdraw
    if err = tx.QueryRowContext(ctx,
        `SELECT balance FROM users WHERE id = $1 FOR UPDATE`, from).Scan(&bal); err != nil { return err }
    if bal < amount { return ErrInsufficientFunds }

    if _, err = tx.ExecContext(ctx, `UPDATE users SET balance = balance - $1 WHERE id = $2`, amount, from); err != nil {
        return err
    }
    if _, err = tx.ExecContext(ctx, `UPDATE users SET balance = balance + $1 WHERE id = $2`, amount, to); err != nil {
        return err
    }
    return tx.Commit()
}
```
**Reusable helper:**
```go
func WithTx(ctx context.Context, db *sql.DB, fn func(*sql.Tx) error) (err error) {
    tx, err := db.BeginTx(ctx, nil)
    if err != nil { return err }
    defer func() {
        if p := recover(); p != nil { _ = tx.Rollback(); panic(p) }
        if err != nil { _ = tx.Rollback(); return }
        err = tx.Commit()
    }()
    return fn(tx)
}
```
Key facts:
- Use the `tx` object for all statements inside the transaction — using `db` inside would grab a different connection (and can deadlock if pool size is 1).
- A tx's context cancellation rolls it back.
- **Isolation levels**: Read Uncommitted, Read Committed (Postgres default), Repeatable Read (MySQL InnoDB default), Serializable. Serializable may fail with serialization errors (SQLSTATE 40001) → **retry loop**.
- **Deadlocks**: acquire locks in consistent order (e.g., lower ID first).
- **Optimistic locking** with a version column:
```sql
UPDATE users SET name=$1, version=version+1 WHERE id=$2 AND version=$3
-- RowsAffected == 0 → concurrent modification → retry or 409 Conflict
```

### Detecting specific DB errors (pgx)
```go
var pgErr *pgconn.PgError
if errors.As(err, &pgErr) {
    switch pgErr.Code {
    case "23505": return ErrDuplicateEmail  // unique_violation
    case "40001": /* serialization_failure → retry */
    case "23503": /* foreign_key_violation */
    }
}
```
MySQL: `*mysql.MySQLError` with `Number == 1062` for duplicates.

## 23.5 pgx Native (recommended for Postgres)

```go
import "github.com/jackc/pgx/v5/pgxpool"

pool, err := pgxpool.New(ctx, os.Getenv("DATABASE_URL")) // pool_max_conns=20 in DSN
if err != nil { log.Fatal(err) }
defer pool.Close()

// Query into structs with generic helpers
rows, _ := pool.Query(ctx, `SELECT id, email, name FROM users WHERE id > $1`, 0)
users, err := pgx.CollectRows(rows, pgx.RowToStructByName[User]) // matches `db:"..."` tags / field names

// Batch: many statements, one round trip
batch := &pgx.Batch{}
for _, u := range newUsers { batch.Queue(`INSERT INTO users(email,name) VALUES($1,$2)`, u.Email, u.Name) }
br := pool.SendBatch(ctx, batch)
defer br.Close()

// COPY for bulk loading (fastest)
_, err = pool.CopyFrom(ctx, pgx.Identifier{"users"}, []string{"email", "name"},
    pgx.CopyFromSlice(len(newUsers), func(i int) ([]any, error) {
        return []any{newUsers[i].Email, newUsers[i].Name}, nil
    }))

// Transactions
err = pgx.BeginFunc(ctx, pool, func(tx pgx.Tx) error {
    _, err := tx.Exec(ctx, `UPDATE ...`)
    return err
})
```
pgx uses the Postgres binary protocol, caches prepared statements automatically, supports arrays, JSONB, UUID, ranges, LISTEN/NOTIFY.

## 23.6 sqlx

```go
db := sqlx.MustConnect("pgx", dsn)
type User struct {
    ID    int64  `db:"id"`
    Email string `db:"email"`
    Name  string `db:"name"`
}
var u User
err := db.GetContext(ctx, &u, `SELECT id, email, name FROM users WHERE id=$1`, 1)
var list []User
err = db.SelectContext(ctx, &list, `SELECT id, email, name FROM users LIMIT 10`)
_, err = db.NamedExecContext(ctx, `INSERT INTO users (email, name) VALUES (:email, :name)`, &u)
q, args, _ := sqlx.In(`SELECT * FROM users WHERE id IN (?)`, []int{1, 2, 3})
q = db.Rebind(q) // ? → $1,$2,$3 for Postgres
```

## 23.7 sqlc (type-safe code generation)

`sqlc.yaml`:
```yaml
version: "2"
sql:
  - engine: "postgresql"
    queries: "db/queries"
    schema: "db/migrations"
    gen:
      go: { package: "store", out: "internal/store", sql_package: "pgx/v5", emit_json_tags: true }
```
`db/queries/users.sql`:
```sql
-- name: GetUser :one
SELECT * FROM users WHERE id = $1;

-- name: ListUsers :many
SELECT * FROM users ORDER BY id LIMIT $1 OFFSET $2;

-- name: CreateUser :one
INSERT INTO users (email, name) VALUES ($1, $2) RETURNING *;

-- name: DeleteUser :exec
DELETE FROM users WHERE id = $1;
```
Run `sqlc generate` → Go code:
```go
q := store.New(pool)
u, err := q.CreateUser(ctx, store.CreateUserParams{Email: "a@b.c", Name: "Ann"})
// transactions:
tx, _ := pool.Begin(ctx); qtx := q.WithTx(tx); ...; tx.Commit(ctx)
```
SQL is validated against the schema at generation time — popular for teams that like writing SQL but want type safety without ORM magic.

## 23.8 GORM (ORM)

```go
import ("gorm.io/gorm"; "gorm.io/driver/postgres")

type User struct {
    gorm.Model                        // ID, CreatedAt, UpdatedAt, DeletedAt (soft delete)
    Email   string `gorm:"uniqueIndex;not null"`
    Name    string `gorm:"size:100"`
    Orders  []Order                   // has-many
    Profile Profile                   // has-one
}
type Order struct {
    gorm.Model
    UserID uint
    Total  float64
    Items  []Product `gorm:"many2many:order_items;"`
}
type Profile struct { gorm.Model; UserID uint; Bio string }
type Product struct { gorm.Model; Name string; Price float64 }

db, err := gorm.Open(postgres.Open(dsn), &gorm.Config{PrepareStmt: true})
sqlDB, _ := db.DB(); sqlDB.SetMaxOpenConns(25) // underlying pool

db.AutoMigrate(&User{}, &Order{}, &Profile{}, &Product{}) // dev only; use real migrations in prod

// Create
u := User{Email: "a@x.com", Name: "Ann"}
db.WithContext(ctx).Create(&u) // u.ID populated

// Read
var got User
db.First(&got, u.ID)                                      // by PK
db.Where("email = ?", "a@x.com").First(&got)
var users []User
db.Where("name LIKE ?", "A%").Order("id desc").Limit(10).Offset(0).Find(&users)
if errors.Is(err, gorm.ErrRecordNotFound) {}

// Update
db.Model(&got).Update("name", "Anna")
db.Model(&got).Updates(User{Name: "Anna"})                // zero fields ignored with struct!
db.Model(&got).Updates(map[string]any{"name": ""})         // map to set zero values

// Delete
db.Delete(&got)             // soft delete (sets deleted_at) because of gorm.Model
db.Unscoped().Delete(&got)  // hard delete

// Associations & N+1 prevention
db.Preload("Orders.Items").Preload("Profile").Find(&users)
db.Joins("Profile").Find(&users)

// Transactions
err = db.Transaction(func(tx *gorm.DB) error {
    if err := tx.Create(&Order{UserID: u.ID, Total: 10}).Error; err != nil { return err } // rollback
    return nil // commit
})

// Raw SQL
db.Raw("SELECT id, name FROM users WHERE id = ?", 1).Scan(&got)

// Hooks
func (u *User) BeforeCreate(tx *gorm.DB) error { u.Email = strings.ToLower(u.Email); return nil }

// Scopes (reusable query fragments)
func Paginate(page, size int) func(*gorm.DB) *gorm.DB {
    return func(db *gorm.DB) *gorm.DB { return db.Offset((page - 1) * size).Limit(size) }
}
db.Scopes(Paginate(2, 20)).Find(&users)
```
**GORM pitfalls (interview favorites):** struct `Updates` ignore zero values; missing `Where` on bulk update/delete is blocked by default (ErrMissingWhereClause); forgetting `Preload` → N+1 queries; `AutoMigrate` never drops columns; always check `.Error`; use `WithContext`; enable logger in dev to see SQL.

### ent (brief)
Schema defined in Go code → generates type-safe graph-style API:
```go
client.User.Query().Where(user.EmailEQ("a@x")).WithOrders().Only(ctx)
```

## 23.9 Migrations

Tools: **golang-migrate**, **goose**, **atlas** (declarative schema diffing), **tern**.
```bash
migrate create -ext sql -dir migrations -seq create_users
migrate -path migrations -database "$DATABASE_URL" up
migrate -path migrations -database "$DATABASE_URL" down 1
```
`000001_create_users.up.sql` / `.down.sql`. Embed and run at startup:
```go
//go:embed migrations/*.sql
var migFS embed.FS
src, _ := iofs.New(migFS, "migrations")
m, _ := migrate.NewWithSourceInstance("iofs", src, dsn)
if err := m.Up(); err != nil && !errors.Is(err, migrate.ErrNoChange) { log.Fatal(err) }
```
Best practices: forward-only, backward-compatible changes (expand → migrate data → contract), never edit applied migrations, run in CI, add indexes `CONCURRENTLY` in Postgres.

## 23.10 Repository Pattern, Unit of Work & Testing

```go
// domain owns the interface
type UserRepository interface {
    Create(ctx context.Context, u *User) error
    ByEmail(ctx context.Context, email string) (*User, error)
}

// a DBTX abstraction lets repos work with both *sql.DB and *sql.Tx
type DBTX interface {
    ExecContext(context.Context, string, ...any) (sql.Result, error)
    QueryContext(context.Context, string, ...any) (*sql.Rows, error)
    QueryRowContext(context.Context, string, ...any) *sql.Row
}
type pgUserRepo struct{ db DBTX }
func NewUserRepo(db DBTX) UserRepository { return &pgUserRepo{db} }

// service uses a transaction spanning two repositories (Unit of Work)
func (s *Service) Register(ctx context.Context, u *User) error {
    return WithTx(ctx, s.db, func(tx *sql.Tx) error {
        if err := NewUserRepo(tx).Create(ctx, u); err != nil { return err }
        return NewAuditRepo(tx).Log(ctx, "register", u.ID)
    })
}
```

**Testing strategies:**
1. **Unit tests** with fake/mocked repository interfaces (fast, no DB).
2. **sqlmock** (`DATA-DOG/go-sqlmock`) to assert generated SQL.
3. **Integration tests** against a real DB using **testcontainers-go** (spin up Postgres in Docker), wrap each test in a transaction that's rolled back, or truncate tables.
```go
func TestUserRepo(t *testing.T) {
    if testing.Short() { t.Skip() }
    ctx := context.Background()
    pg, err := postgres.Run(ctx, "postgres:16-alpine",
        postgres.WithDatabase("test"), postgres.WithUsername("u"), postgres.WithPassword("p"),
        postgres.BasicWaitStrategies())
    require.NoError(t, err)
    t.Cleanup(func() { _ = pg.Terminate(ctx) })
    dsn, _ := pg.ConnectionString(ctx, "sslmode=disable")
    db := mustOpenAndMigrate(t, dsn)
    repo := NewUserRepo(db)
    require.NoError(t, repo.Create(ctx, &User{Email: "t@x", Name: "T"}))
}
```

## 23.11 Redis & MongoDB Quick Reference

```go
// Redis (github.com/redis/go-redis/v9) — cache-aside pattern
rdb := redis.NewClient(&redis.Options{Addr: "localhost:6379"})
func GetUserCached(ctx context.Context, id int64) (*User, error) {
    key := fmt.Sprintf("user:%d", id)
    if b, err := rdb.Get(ctx, key).Bytes(); err == nil {
        var u User; if json.Unmarshal(b, &u) == nil { return &u, nil }
    } else if !errors.Is(err, redis.Nil) { slog.Warn("redis", "err", err) }
    u, err := repo.Get(ctx, id)
    if err != nil { return nil, err }
    b, _ := json.Marshal(u)
    rdb.Set(ctx, key, b, 10*time.Minute)
    return u, nil
}

// MongoDB (go.mongodb.org/mongo-driver/v2)
client, _ := mongo.Connect(options.Client().ApplyURI("mongodb://localhost:27017"))
coll := client.Database("shop").Collection("users")
_, _ = coll.InsertOne(ctx, bson.M{"email": "a@x", "name": "Ann"})
var res bson.M
_ = coll.FindOne(ctx, bson.M{"email": "a@x"}).Decode(&res)
```

## 23.12 Database Interview Q&A

**Q: Why is `sql.Open` not failing with a wrong password?** It doesn't connect; use `PingContext`.

**Q: Why do I get "too many connections" / "sql: database is closed"?** No `SetMaxOpenConns`; opening a new `*sql.DB` per request; calling `db.Close()` early; leaking `rows` without `Close`.

**Q: `Query` vs `QueryRow` vs `Exec`?** `Query` → multiple rows (must close); `QueryRow` → single row, error deferred to `Scan` (`sql.ErrNoRows`); `Exec` → no rows (INSERT/UPDATE/DELETE), returns `Result`.

**Q: How to avoid N+1 queries?** JOINs, `WHERE id = ANY($1)` batch loading, GORM `Preload`, dataloader pattern for GraphQL.

**Q: How to do bulk inserts efficiently?** Multi-row `VALUES` in batches (mind parameter limit 65535 in Postgres), `pgx.CopyFrom`, `pgx.Batch`, transactions around loops.

**Q: How do you handle context timeouts in DB calls?** Pass request context with deadline to `QueryContext`; the driver cancels the query server-side (pgx sends a cancel request).

**Q: How to implement pagination?** OFFSET/LIMIT (simple, slow on deep pages) vs keyset/cursor pagination (`WHERE (created_at, id) < ($1,$2) ORDER BY created_at DESC, id DESC LIMIT n`).

**Q: ORM or raw SQL?** Trade-off: productivity & associations (GORM/ent) vs control, performance, transparency (sqlc/pgx). Many Go teams prefer sqlc/pgx for core services.

**Q: How do you ensure idempotency on writes?** Unique constraints + `INSERT ... ON CONFLICT DO NOTHING/UPDATE`, idempotency-key table, upserts.

**Q: What is connection pool starvation?** Long transactions or unclosed rows hold connections; other goroutines block in `db.conn()` waiting — monitor `db.Stats().WaitCount`.

---

# APPENDIX — CHEAT SHEET & INTERVIEW TIPS

## Quick reference

| Topic | Remember |
|---|---|
| Zero values | `0, "", false, nil`; structs zeroed field-wise |
| `new` vs `make` | `new(T)` → `*T` zeroed; `make` for slice/map/chan |
| Slice header | `{ptr, len, cap}`; always `s = append(s, …)` |
| Map | not thread-safe; random order; nil map write panics; Swiss tables since 1.24 |
| Strings | immutable bytes; `range` yields runes; `len` = bytes |
| Interfaces | implicit; `(type, value)` pair; nil-interface gotcha |
| Method sets | `T`: value methods; `*T`: both |
| defer | args evaluated immediately; LIFO; can modify named results |
| panic/recover | recover only in deferred func of the same goroutine |
| Channels | nil blocks; closed send panics; closed recv zero+false; sender closes |
| select | random among ready; `default` non-blocking; nil case disabled |
| Sync | Mutex, RWMutex, WaitGroup (Add before go), Once, Pool, Cond, Map, atomic |
| Context | first param; `defer cancel()`; tree cancellation; values for request-scoped data |
| Scheduler | G-M-P; work stealing; netpoller; async preemption (1.14) |
| Stack | 2 KB, grows by copying |
| GC | concurrent tri-color mark-sweep, hybrid write barrier, GOGC/GOMEMLIMIT |
| Escape | `-gcflags=-m`; pointers returned/stored in interfaces escape |
| Generics | `[T constraint]`, `~T`, `comparable`, GC-shape stenciling + dictionaries |
| Loop var | per-iteration since 1.22 |
| Tools | `go vet`, `-race`, pprof, trace, benchstat, golangci-lint |

## Version timeline highlights
- **1.0** (2012) compatibility promise
- **1.5** Go-written compiler/runtime, concurrent GC, GOMAXPROCS=NumCPU
- **1.7** `context` in stdlib, SSA backend
- **1.8** hybrid write barrier (sub-ms pauses)
- **1.11/1.13** modules, error wrapping (`%w`, `errors.Is/As`)
- **1.14** async preemption, open-coded defers
- **1.16** `embed`, modules default
- **1.17** register ABI
- **1.18** generics, fuzzing, workspaces
- **1.19** `GOMEMLIMIT`, atomic types, revised memory model
- **1.20** `errors.Join`, PGO preview
- **1.21** `slices`, `maps`, `cmp`, `log/slog`, `min/max/clear` built-ins, PGO GA
- **1.22** per-iteration loop vars, range over int, enhanced ServeMux, `math/rand/v2`
- **1.23** range-over-func iterators (`iter`), timer changes, `unique`
- **1.24** Swiss-table maps, generic type aliases, `testing.B.Loop`, `os.Root`, `weak`, `runtime.AddCleanup`, tool deps in go.mod
- **1.25** container-aware GOMAXPROCS, `sync.WaitGroup.Go`, `testing/synctest`, experimental Green Tea GC & json/v2, `http.CrossOriginProtection`
- **1.26** Green Tea GC default, `new(expr)`

## How to answer in an interview
1. **Define** the concept in one sentence.
2. **Explain how it works** (mention internals when relevant: slice header, hchan, GMP, itab).
3. **Show a tiny code example.**
4. **Mention pitfalls & best practices** (leaks, races, nil interface, append aliasing).
5. **Relate to real-world experience** ("in our service we used errgroup with SetLimit to…").

## Top 15 most-asked Go questions (revise these first)
1. Goroutines vs threads; GMP scheduler
2. Buffered vs unbuffered channels; channel state table
3. `select` behaviour; timeouts
4. WaitGroup, Mutex, RWMutex, Once, atomic
5. Context cancellation and propagation
6. Slices internals and `append` aliasing
7. Maps: implementation, concurrency safety, iteration order
8. Interfaces: implicit satisfaction, internals, nil interface gotcha
9. Value vs pointer receivers; method sets
10. `defer` rules; `panic`/`recover`
11. Error handling: wrapping, `errors.Is/As`, custom errors
12. Garbage collector and escape analysis
13. Generics: constraints, when to use
14. Goroutine leaks & race detector
15. Designing a REST API / project structure / DB access in Go

---

*End of guide. Practice by writing small programs for each section, run everything with `go test -race`, and read the Go spec, Effective Go, and the Go memory model — they are short and interviewers love questions drawn from them.*
