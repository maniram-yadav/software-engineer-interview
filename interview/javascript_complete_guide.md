# The Complete JavaScript Guide
### Interview Questions with Detailed Answers + Full Theory + Step-by-Step Web App Tutorial

---

## Table of Contents

**Part A — Interview Questions**
1. [JavaScript Fundamentals](#1-javascript-fundamentals)
2. [Variables, Scope & Hoisting](#2-variables-scope--hoisting)
3. [Data Types & Type Coercion](#3-data-types--type-coercion)
4. [Functions & Closures](#4-functions--closures)
5. [`this`, `call`/`apply`/`bind` & Arrow Functions](#5-this-callapplybind--arrow-functions)
6. [Objects & Prototypes](#6-objects--prototypes)
7. [Arrays & Array Methods](#7-arrays--array-methods)
8. [Asynchronous JavaScript](#8-asynchronous-javascript)
9. [The Event Loop & Concurrency Model](#9-the-event-loop--concurrency-model)
10. [ES6+ Modern Features](#10-es6-modern-features)
11. [DOM Manipulation & Events](#11-dom-manipulation--events)
12. [Error Handling](#12-error-handling)
13. [Modules](#13-modules)
14. [Advanced Patterns](#14-advanced-patterns)
15. [Node.js Essentials](#15-nodejs-essentials)
16. [Testing JavaScript](#16-testing-javascript)
17. [Performance & Best Practices](#17-performance--best-practices)

**Part B — Complete Theory**
18. [JavaScript Theoretical Deep Dive](#18-javascript-theoretical-deep-dive)

**Part C — Full Tutorial**
19. [Complete Tutorial: Building a Web App from Scratch](#19-complete-tutorial-building-a-web-app-from-scratch)

---

# Part A — Interview Questions

## 1. JavaScript Fundamentals

### Q1. What is JavaScript, and what makes it unique among programming languages?
JavaScript is a high-level, interpreted (or JIT-compiled), multi-paradigm programming language originally designed to make web pages interactive. Key distinguishing traits:
- **Single-threaded with an event loop** — non-blocking async behavior via callbacks/Promises rather than OS threads.
- **Dynamically and weakly typed** — types are determined at runtime, and implicit coercion happens between types.
- **Prototype-based OOP** — objects inherit directly from other objects (no classes at the engine level, though `class` syntax exists as sugar).
- **First-class functions** — functions are values, can be passed around, returned, and have properties.
- **Runs everywhere** — browsers (via engines like V8, SpiderMonkey), servers (Node.js, Deno, Bun), mobile apps, IoT.

### Q2. What is the difference between JavaScript and ECMAScript?
**ECMAScript (ES)** is the language *specification* standardized by ECMA International (TC39 committee). **JavaScript** is the most popular *implementation* of that specification — engines like V8 (Chrome/Node), SpiderMonkey (Firefox), and JavaScriptCore (Safari) all implement ECMAScript, plus browser/runtime-specific APIs (DOM, `fetch`, `setTimeout`) that aren't part of the ECMAScript spec itself.

### Q3. Is JavaScript compiled or interpreted?
Modern JavaScript engines (like V8) use a **JIT (Just-In-Time) compilation** pipeline: source code is first parsed into an AST, executed initially via an interpreter (e.g., V8's Ignition), and "hot" (frequently executed) code paths are compiled to optimized machine code on the fly (via V8's TurboFan). So it's neither purely interpreted nor purely ahead-of-time compiled — it's a hybrid, optimized at runtime based on actual usage patterns.

### Q4. What is the difference between `null` and `undefined`?
```javascript
let a;
console.log(a);          // undefined -> declared but not assigned

let b = null;
console.log(b);            // null -> explicitly assigned "no value"

typeof undefined;    // "undefined"
typeof null;           // "object"  <- famous long-standing JS bug/quirk

undefined == null;    // true  (loose equality, both considered "empty")
undefined === null;   // false (different types)
```
`undefined` means a variable has been declared but has no assigned value (or a function returned nothing, or an object property doesn't exist). `null` is an intentional, explicit representation of "no value" that a developer assigns.

### Q5. What are JavaScript's primitive data types?
Seven primitives (as of ES2020): `string`, `number`, `bigint`, `boolean`, `undefined`, `symbol`, and `null`. Everything else (arrays, objects, functions, dates) is an `object`.
```javascript
typeof "hello"      // "string"
typeof 42             // "number"
typeof 42n            // "bigint"
typeof true            // "boolean"
typeof undefined        // "undefined"
typeof Symbol("id")      // "symbol"
typeof null               // "object" (quirk)
typeof {}                  // "object"
typeof []                   // "object"
typeof function(){}          // "function"
```

### Q6. Primitive vs Reference types — how does JavaScript handle assignment and comparison?
```javascript
// Primitives: copied BY VALUE
let a = 10;
let b = a;
b = 20;
console.log(a);   // 10, unaffected

// Objects/Arrays: copied BY REFERENCE
let obj1 = { x: 1 };
let obj2 = obj1;
obj2.x = 99;
console.log(obj1.x);   // 99 -> both variables point to the SAME object

// Comparison follows the same rule:
{} === {}          // false, different objects in memory
let o = {};
o === o             // true, same reference
```
This is a very common interview trip-up: mutating a nested object/array received as a function argument mutates the caller's original data too, since objects are passed by reference (technically "pass by value of the reference").

---

## 2. Variables, Scope & Hoisting

### Q7. `var` vs `let` vs `const` — explain the differences in detail.
```javascript
var x = 1;      // function-scoped, hoisted & initialized as undefined, can be redeclared
let y = 2;       // block-scoped, hoisted but NOT initialized (Temporal Dead Zone)
const z = 3;      // block-scoped, must be initialized, binding cannot be reassigned

function scopeDemo() {
    if (true) {
        var varVal = "I leak out of the block";
        let letVal = "I stay in the block";
    }
    console.log(varVal);    // "I leak out of the block" -> var ignores block scope
    console.log(letVal);     // ReferenceError: letVal is not defined
}
```
- **`var`**: function-scoped (or globally-scoped if outside any function), can be redeclared, hoisted and auto-initialized to `undefined`.
- **`let`**: block-scoped `{}`, cannot be redeclared in the same scope, hoisted but stays uninitialized until its declaration line (the "Temporal Dead Zone").
- **`const`**: same scoping as `let`, but the **binding** (the variable itself) cannot be reassigned — note this does NOT make objects/arrays immutable, only the reference:
```javascript
const arr = [1, 2, 3];
arr.push(4);        // valid! mutating contents is fine
arr = [5, 6];         // TypeError: Assignment to constant variable
```

### Q8. What is hoisting? Explain with `var`, `let`/`const`, and function declarations.
Hoisting is JavaScript's behavior of moving declarations to the top of their scope during the compilation phase, before code execution.
```javascript
console.log(a);   // undefined (var is hoisted AND initialized to undefined)
var a = 5;

console.log(b);   // ReferenceError: Cannot access 'b' before initialization (TDZ)
let b = 10;

sayHi();            // "Hi!" -> function declarations are FULLY hoisted (name + body)
function sayHi() {
    console.log("Hi!");
}

sayBye();            // TypeError: sayBye is not a function (only the `var` is hoisted, not the assignment)
var sayBye = function () {
    console.log("Bye!");
};
```

### Q9. What is the Temporal Dead Zone (TDZ)?
The TDZ is the period between entering a scope and the actual `let`/`const` declaration being executed, during which accessing the variable throws a `ReferenceError` rather than returning `undefined`. It exists to catch bugs that `var`'s "hoist to undefined" behavior would otherwise hide silently.

### Q10. Explain lexical scope and scope chains.
JavaScript uses **lexical (static) scoping** — a variable's scope is determined by where it's *written* in the source code, not by how/where the function is called.
```javascript
const globalVar = "global";

function outer() {
    const outerVar = "outer";
    function inner() {
        const innerVar = "inner";
        console.log(innerVar, outerVar, globalVar);   // all accessible via the scope chain
    }
    inner();
}
outer();
```
When a variable is referenced, the JS engine looks up the **scope chain**: current scope → enclosing scope(s) → global scope, until the variable is found or a `ReferenceError` is thrown.

### Q11. What's the difference between global scope, function scope, and block scope?
```javascript
let globalVar = "I'm global";               // global scope: accessible everywhere

function myFunc() {
    let functionVar = "I'm function-scoped";  // accessible only inside myFunc
}

{
    let blockVar = "I'm block-scoped";          // accessible only inside these {}
}
```
Polluting the global scope is a common source of bugs in larger apps (name collisions across scripts) — modern JS favors modules and block scoping (`let`/`const`) specifically to minimize this.

---

## 3. Data Types & Type Coercion

### Q12. `==` vs `===` — explain type coercion rules.
```javascript
"5" == 5          // true  -> string coerced to number before comparing
"5" === 5          // false -> different types, no coercion
0 == false          // true  -> false coerced to 0
0 == ""              // true  -> "" coerced to 0
null == undefined      // true  -> special case, both loosely equal to each other
null === undefined      // false
NaN == NaN                // false -> NaN is never equal to anything, including itself!
```
**Best practice**: always use `===`/`!==` (strict equality) unless you have a specific, well-understood reason to rely on coercion — `==` coercion rules are notoriously confusing and a common source of bugs.

### Q13. How do you correctly check if a value is `NaN`?
```javascript
NaN === NaN            // false
isNaN("hello")            // true -> BUT isNaN() coerces its argument first, unreliable
isNaN(NaN)                  // true
Number.isNaN(NaN)             // true -> PREFERRED, no coercion
Number.isNaN("hello")           // false -> correctly identifies "hello" is not NaN (it's a string)
```

### Q14. What is implicit type coercion, and give tricky examples.
```javascript
1 + "1"        // "11"  -> number coerced to string (string concatenation wins)
1 - "1"         // 0     -> string coerced to number (arithmetic wins for -, *, /)
"5" + 3           // "53"
"5" - 3            // 2
[] + []             // ""    -> arrays coerced to strings, both empty -> ""
[] + {}              // "[object Object]"
{} + []               // 0 (in a statement context, {} is parsed as an empty block!)
true + true            // 2     -> booleans coerced to numbers (1 + 1)
```
The `+` operator is special: if either operand is a string, it performs concatenation; otherwise (for `-`, `*`, `/`), operands are coerced to numbers.

### Q15. What are truthy and falsy values in JavaScript?
Falsy values (there are exactly 8 in JS): `false`, `0`, `-0`, `0n` (BigInt zero), `""` (empty string), `null`, `undefined`, `NaN`. **Everything else is truthy**, including `"0"`, `"false"` (non-empty strings), `[]` (empty array), and `{}` (empty object) — a very common gotcha.
```javascript
if ([]) console.log("arrays are truthy!");    // logs
if ({}) console.log("objects are truthy!");    // logs
if ("0") console.log("non-empty strings are truthy!");   // logs
```

### Q16. Explain `Number`, `String`, and `Boolean` explicit conversion functions.
```javascript
Number("42")       // 42
Number("42px")      // NaN -> strict parsing
parseInt("42px")     // 42  -> parses until it hits a non-numeric character
parseFloat("3.14abc") // 3.14

String(42)             // "42"
String(null)             // "null"
String(undefined)          // "undefined"

Boolean(0)                  // false
Boolean("")                   // false
Boolean("false")                // true (non-empty string!)
```

---

## 4. Functions & Closures

### Q17. What is a closure? Give a practical, real-world example.
A closure is a function that retains access to variables from its enclosing lexical scope even after the outer function has returned.
```javascript
function createCounter() {
    let count = 0;                 // private state, not accessible from outside
    return {
        increment() { return ++count; },
        decrement() { return --count; },
        reset() { count = 0; },
    };
}

const counter = createCounter();
counter.increment();   // 1
counter.increment();   // 2
counter.decrement();   // 1
// `count` cannot be accessed directly - counter.count is undefined
```
**Real-world uses**: data privacy/encapsulation (module pattern), memoization/caching, event handler factories, and debounce/throttle implementations (see Advanced Patterns section).

### Q18. What is the classic closure-in-a-loop pitfall, and how do you fix it?
```javascript
// BUGGY: all three timeouts log "3"
for (var i = 0; i < 3; i++) {
    setTimeout(() => console.log(i), 100);
}
// because `var` is function-scoped - all closures share the SAME `i`,
// which has already reached 3 by the time the timeouts fire

// FIX 1: use `let` (block-scoped - a new binding per iteration)
for (let i = 0; i < 3; i++) {
    setTimeout(() => console.log(i), 100);   // logs 0, 1, 2
}

// FIX 2 (pre-ES6 style): wrap in an IIFE to capture the value per iteration
for (var i = 0; i < 3; i++) {
    (function (capturedI) {
        setTimeout(() => console.log(capturedI), 100);
    })(i);
}
```

### Q19. Function declarations vs function expressions vs arrow functions.
```javascript
// Function declaration - hoisted fully, has its own `this`, can be called before defined
function add(a, b) { return a + b; }

// Function expression - not hoisted (the variable is, but not the assignment)
const subtract = function (a, b) { return a - b; };

// Arrow function - concise syntax, NO own `this`/`arguments`/`super` (inherits from enclosing scope)
const multiply = (a, b) => a * b;
```

### Q20. What is an IIFE (Immediately Invoked Function Expression)?
```javascript
(function () {
    console.log("Runs immediately!");
})();

// Common historical use: creating a private scope to avoid polluting globals,
// before ES6 modules and block scoping existed
const myModule = (function () {
    let privateVar = 0;
    return {
        increment: () => ++privateVar,
    };
})();
```

### Q21. What is currying, and how do you implement it?
Currying transforms a function taking multiple arguments into a sequence of functions each taking a single argument.
```javascript
function curry(fn) {
    return function curried(...args) {
        if (args.length >= fn.length) {
            return fn.apply(this, args);
        }
        return (...moreArgs) => curried.apply(this, [...args, ...moreArgs]);
    };
}

function add3(a, b, c) { return a + b + c; }
const curriedAdd = curry(add3);

curriedAdd(1)(2)(3);      // 6
curriedAdd(1, 2)(3);       // 6
curriedAdd(1, 2, 3);        // 6
```
Useful for creating specialized/partially-applied functions (e.g., `multiplyBy(2)` derived from a generic `multiply(a, b)`) and functional-programming pipelines.

### Q22. What are default parameters and rest parameters?
```javascript
function greet(name = "Guest", greeting = "Hello") {
    return `${greeting}, ${name}!`;
}
greet();                    // "Hello, Guest!"
greet("Alice");               // "Hello, Alice!"

function sum(...numbers) {      // rest parameter - collects remaining args into an array
    return numbers.reduce((total, n) => total + n, 0);
}
sum(1, 2, 3, 4);   // 10
```

---

## 5. `this`, `call`/`apply`/`bind` & Arrow Functions

### Q23. How is `this` determined in JavaScript? Explain the four binding rules.
`this` is determined by **how a function is called** (call-site), not where it's defined (except for arrow functions).

```javascript
// 1. Default binding - plain function call -> `this` is undefined (strict mode) or global object
function show() { console.log(this); }
show();                     // undefined (strict mode) / window (non-strict, browser)

// 2. Implicit binding - called as a method -> `this` is the object before the dot
const obj = {
    name: "Alice",
    greet() { console.log(this.name); }
};
obj.greet();                  // "Alice"

const detachedGreet = obj.greet;
detachedGreet();                // undefined - lost its `this` context!

// 3. Explicit binding - call/apply/bind
function greet2() { console.log(this.name); }
greet2.call({ name: "Bob" });      // "Bob"
greet2.apply({ name: "Carol" });     // "Carol"
const bound = greet2.bind({ name: "Dave" });
bound();                                // "Dave"

// 4. `new` binding - constructor call -> `this` is the newly created object
function Person(name) { this.name = name; }
const p = new Person("Eve");
console.log(p.name);   // "Eve"
```

### Q24. `call()` vs `apply()` vs `bind()` — differences and use cases.
```javascript
function introduce(greeting, punctuation) {
    return `${greeting}, I'm ${this.name}${punctuation}`;
}

const user = { name: "Zoe" };

introduce.call(user, "Hi", "!");            // args passed individually
introduce.apply(user, ["Hi", "!"]);          // args passed as an array
const boundIntroduce = introduce.bind(user);  // returns a NEW function with `this` permanently bound
boundIntroduce("Hi", "!");                      // can be called later, `this` stays fixed
```
`call`/`apply` invoke the function **immediately** with a given `this`; `bind` returns a **new function** with `this` permanently fixed, to be called later — commonly used for event handlers in class components or callbacks that need a specific `this`.

### Q25. How do arrow functions handle `this` differently from regular functions?
Arrow functions **do not have their own `this`** — they lexically inherit `this` from their enclosing scope at the time they're defined, and it cannot be changed via `call`/`apply`/`bind`.
```javascript
const obj = {
    name: "Alice",
    regularMethod: function () {
        setTimeout(function () {
            console.log(this.name);   // undefined - `this` inside a plain callback is NOT obj
        }, 100);
    },
    arrowFixedMethod: function () {
        setTimeout(() => {
            console.log(this.name);   // "Alice" - arrow function inherits `this` from arrowFixedMethod
        }, 100);
    },
};
```
This is precisely why arrow functions became the idiomatic choice for callbacks inside methods after ES6 — they eliminate the classic `var self = this` / `.bind(this)` workarounds needed with regular functions.

### Q26. What is the difference between `this` in a regular method and an arrow function defined directly as an object property?
```javascript
const obj = {
    name: "Alice",
    regular() { return this.name; },        // `this` = obj when called as obj.regular()
    arrow: () => { return this.name; },        // `this` = enclosing (often global/module) scope, NOT obj!
};
obj.regular();   // "Alice"
obj.arrow();       // undefined - common mistake, arrow functions should not be used as object methods
                     // that need to reference the object itself
```

---

## 6. Objects & Prototypes

### Q27. How does prototypal inheritance work in JavaScript?
Every object has an internal `[[Prototype]]` link (accessible via `Object.getPrototypeOf()` or the deprecated `__proto__`) to another object, forming a **prototype chain**. Property/method lookups walk up this chain until found or the chain ends at `null`.

```javascript
const animal = {
    eat() { console.log(`${this.name} is eating`); }
};

const dog = Object.create(animal);   // dog's prototype is `animal`
dog.name = "Rex";
dog.eat();   // "Rex is eating" -> found via the prototype chain, not on `dog` itself

console.log(dog.hasOwnProperty("eat"));   // false - inherited, not own property
console.log(Object.getPrototypeOf(dog) === animal);   // true
```

### Q28. How do ES6 `class` and constructor functions relate to prototypes?
`class` syntax is **syntactic sugar** over JavaScript's existing prototype-based inheritance — it doesn't introduce a fundamentally new inheritance model, just a cleaner syntax.
```javascript
// Constructor function style (pre-ES6)
function Animal(name) {
    this.name = name;
}
Animal.prototype.speak = function () {
    console.log(`${this.name} makes a sound`);
};

function Dog(name) {
    Animal.call(this, name);
}
Dog.prototype = Object.create(Animal.prototype);
Dog.prototype.constructor = Dog;
Dog.prototype.speak = function () {
    console.log(`${this.name} barks`);
};

// ES6 class style (equivalent, cleaner)
class AnimalES6 {
    constructor(name) { this.name = name; }
    speak() { console.log(`${this.name} makes a sound`); }
}
class DogES6 extends AnimalES6 {
    speak() { console.log(`${this.name} barks`); }
    superSpeak() { super.speak(); }   // calls the parent's method
}

const rex = new DogES6("Rex");
rex.speak();          // "Rex barks"
rex.superSpeak();      // "Rex makes a sound"
```

### Q29. Object cloning: shallow copy vs deep copy.
```javascript
const original = { a: 1, nested: { b: 2 } };

// Shallow copy (top-level only, nested objects still shared)
const shallow1 = { ...original };
const shallow2 = Object.assign({}, original);
shallow1.nested.b = 99;
console.log(original.nested.b);   // 99 -> nested object was shared!

// Deep copy options
const deep1 = structuredClone(original);         // modern, built-in, handles most cases (2022+)
const deep2 = JSON.parse(JSON.stringify(original)); // older technique, but loses functions/undefined/Dates/Symbols
```
`structuredClone()` is now the recommended built-in for deep cloning; `JSON.parse(JSON.stringify())` was the common workaround before it existed but silently drops functions, `undefined` values, and mishandles `Date`/`Map`/`Set`.

### Q30. What are `Object.freeze()`, `Object.seal()`, and how do they differ?
```javascript
const obj1 = Object.freeze({ a: 1 });
obj1.a = 99;             // fails silently (or throws in strict mode)
obj1.b = 2;                // also fails - cannot add new properties
console.log(obj1);           // { a: 1 } - fully immutable (shallow only!)

const obj2 = Object.seal({ a: 1 });
obj2.a = 99;              // WORKS - existing properties can still be modified
obj2.b = 2;                 // fails - cannot add new properties
console.log(obj2);            // { a: 99 }
```
Both are **shallow** — nested objects inside a frozen/sealed object remain mutable unless recursively frozen.

### Q31. Explain property descriptors and `Object.defineProperty()`.
```javascript
const obj = {};
Object.defineProperty(obj, "id", {
    value: 42,
    writable: false,        // cannot be reassigned
    enumerable: false,        // won't show up in for...in / Object.keys()
    configurable: false,        // cannot be deleted or redefined
});

obj.id = 100;      // silently fails (writable: false)
console.log(obj.id);   // 42
console.log(Object.keys(obj));   // [] - not enumerable
```
This is the low-level mechanism underlying `getter`/`setter` properties, `const`-like object properties, and how frameworks like Vue 2 implemented reactivity (by wrapping properties with custom getters/setters).

### Q32. What are getters and setters?
```javascript
const person = {
    firstName: "John",
    lastName: "Doe",
    get fullName() {
        return `${this.firstName} ${this.lastName}`;
    },
    set fullName(value) {
        [this.firstName, this.lastName] = value.split(" ");
    },
};
console.log(person.fullName);    // "John Doe" -> called like a property, runs like a function
person.fullName = "Jane Smith";
console.log(person.firstName);     // "Jane"
```

---

## 7. Arrays & Array Methods

### Q33. `map()` vs `forEach()` vs `filter()` vs `reduce()` — explain and demonstrate.
```javascript
const nums = [1, 2, 3, 4, 5];

nums.forEach(n => console.log(n));            // no return value, just side effects

const doubled = nums.map(n => n * 2);          // [2, 4, 6, 8, 10] -> new array, same length
const evens = nums.filter(n => n % 2 === 0);     // [2, 4] -> new array, filtered
const sum = nums.reduce((acc, n) => acc + n, 0);  // 15 -> single accumulated value

// reduce is the most general - map/filter can both be implemented using reduce
const mappedViaReduce = nums.reduce((acc, n) => [...acc, n * 2], []);
```
`forEach` is for side effects (no chaining, returns `undefined`). `map`, `filter` return new arrays (chainable, don't mutate the original). `reduce` collapses an array to any single value (number, object, string, even another array).

### Q34. How do you find elements in an array — `find`, `findIndex`, `includes`, `some`, `every`?
```javascript
const users = [{ id: 1, name: "A" }, { id: 2, name: "B" }];

users.find(u => u.id === 2);        // { id: 2, name: "B" } -> first match, or undefined
users.findIndex(u => u.id === 2);     // 1 -> index of first match, or -1
[1, 2, 3].includes(2);                  // true -> simple value existence check
[1, 2, 3].some(n => n > 2);               // true -> at least one matches
[1, 2, 3].every(n => n > 0);                // true -> all match
```

### Q35. Array mutating methods vs non-mutating methods — why does this distinction matter?
```javascript
const arr = [3, 1, 2];

// MUTATING - modify the original array in place
arr.push(4);       // [3, 1, 2, 4]
arr.pop();           // [3, 1, 2]
arr.sort();            // [1, 2, 3] -> mutates AND returns the same array!
arr.splice(1, 1);        // removes 1 element at index 1

// NON-MUTATING - return a NEW array, original untouched
const arr2 = [3, 1, 2];
const sorted = [...arr2].sort();       // spread first to avoid mutating arr2
const sliced = arr2.slice(0, 2);         // [3, 1] -> arr2 unchanged
const concatenated = arr2.concat([4, 5]);  // new array
```
This matters enormously in frameworks like React, where state should never be mutated directly — always prefer non-mutating methods (`slice`, `map`, `filter`, spread `...`) when working with state that triggers re-renders.

### Q36. What's the difference between `Array.from()`, spread syntax, and `Array.of()`?
```javascript
Array.from({ length: 5 }, (_, i) => i * 2);   // [0, 2, 4, 6, 8] -> from array-like + mapping fn
Array.from("hello");                             // ['h', 'e', 'l', 'l', 'o'] -> from any iterable
Array.from(document.querySelectorAll("div"));      // NodeList -> real array (common DOM use case)

[...new Set([1, 2, 2, 3])];    // [1, 2, 3] -> spread works on any iterable too

Array.of(7);        // [7] -> vs new Array(7) which creates an array of length 7 (empty slots!)
new Array(7).length;  // 7, but all empty slots -> a classic gotcha
```

### Q37. How do you flatten a nested array?
```javascript
const nested = [1, [2, 3], [4, [5, 6]]];

nested.flat();          // [1, 2, 3, 4, [5, 6]]  -> flattens 1 level deep by default
nested.flat(2);           // [1, 2, 3, 4, 5, 6]   -> depth argument
nested.flat(Infinity);      // fully flattens any depth

// flatMap = map() + flat(1) combined, common for one-to-many transformations
[1, 2, 3].flatMap(n => [n, n * 2]);   // [1, 2, 2, 4, 3, 6]
```

### Q38. How does `Array.prototype.sort()` work, and what's the classic gotcha?
```javascript
[10, 1, 21, 2].sort();                        // [1, 10, 2, 21] -> WRONG! default sort is LEXICOGRAPHIC (string-based)
[10, 1, 21, 2].sort((a, b) => a - b);           // [1, 2, 10, 21] -> CORRECT, explicit numeric comparator
[10, 1, 21, 2].sort((a, b) => b - a);            // [21, 10, 2, 1] -> descending
```
Always pass an explicit comparator function when sorting numbers — the default sort converts elements to strings first, causing unexpected ordering.

---

## 8. Asynchronous JavaScript

### Q39. Explain the evolution: callbacks → Promises → async/await.
```javascript
// 1. Callbacks (pre-ES6) - prone to "callback hell" with nested async operations
function getUser(id, callback) {
    setTimeout(() => callback(null, { id, name: "Alice" }), 100);
}
getUser(1, (err, user) => {
    if (err) return console.error(err);
    console.log(user);
});

// 2. Promises (ES6) - chainable, better error handling, avoids deep nesting
function getUserPromise(id) {
    return new Promise((resolve, reject) => {
        setTimeout(() => resolve({ id, name: "Alice" }), 100);
    });
}
getUserPromise(1)
    .then(user => console.log(user))
    .catch(err => console.error(err));

// 3. async/await (ES2017) - synchronous-LOOKING syntax over Promises
async function main() {
    try {
        const user = await getUserPromise(1);
        console.log(user);
    } catch (err) {
        console.error(err);
    }
}
main();
```
`async`/`await` is syntactic sugar over Promises — it doesn't replace them, it makes Promise-based code read more like synchronous code while still being fully non-blocking under the hood.

### Q40. What are the three states of a Promise?
A Promise is always in exactly one of: **pending** (initial, neither fulfilled nor rejected), **fulfilled** (operation succeeded, has a resolved value), or **rejected** (operation failed, has a reason/error). Once settled (fulfilled or rejected), a Promise's state and value are **immutable** — it cannot transition again.
```javascript
const p = new Promise((resolve, reject) => {
    // resolve(value)  -> transitions to fulfilled
    // reject(error)     -> transitions to rejected
    setTimeout(() => resolve("done"), 1000);
});
console.log(p);   // Promise { <pending> } immediately after creation
```

### Q41. `Promise.all()` vs `Promise.allSettled()` vs `Promise.race()` vs `Promise.any()`.
```javascript
const p1 = Promise.resolve(1);
const p2 = new Promise((_, reject) => setTimeout(() => reject("fail"), 100));
const p3 = Promise.resolve(3);

// Promise.all - rejects immediately if ANY promise rejects (fail-fast)
Promise.all([p1, p2, p3]).catch(err => console.log("all rejected:", err));

// Promise.allSettled - ALWAYS resolves, giving status of every promise (never short-circuits)
Promise.allSettled([p1, p2, p3]).then(results => console.log(results));
// [{status:"fulfilled", value:1}, {status:"rejected", reason:"fail"}, {status:"fulfilled", value:3}]

// Promise.race - settles as soon as the FIRST promise settles (fulfilled or rejected)
Promise.race([p1, p2, p3]).then(val => console.log("first to settle:", val));

// Promise.any - settles as soon as the FIRST promise FULFILLS, ignores rejections
// (only rejects if ALL promises reject, with an AggregateError)
Promise.any([p2, p3]).then(val => console.log("first success:", val));
```
Use `Promise.all` for "all must succeed" scenarios (e.g., loading required data), `Promise.allSettled` when you want results regardless of individual failures (e.g., batch operations where partial failure is OK), `Promise.race` for timeouts, and `Promise.any` for "first successful response wins" (e.g., querying redundant servers).

### Q42. How do you run async operations sequentially vs concurrently?
```javascript
async function fetchAll() {
    // SEQUENTIAL - each await blocks until the previous resolves (slower, ~300ms total)
    const a = await fetchData("a");   // ~100ms
    const b = await fetchData("b");    // ~100ms, starts only AFTER a finishes
    const c = await fetchData("c");     // ~100ms
    return [a, b, c];
}

async function fetchAllConcurrent() {
    // CONCURRENT - all three start immediately, total time ~= slowest one (~100ms)
    const [a, b, c] = await Promise.all([
        fetchData("a"),
        fetchData("b"),
        fetchData("c"),
    ]);
    return [a, b, c];
}
```
A very common interview/code-review issue: unnecessarily `await`-ing independent operations one at a time inside a loop, serializing work that could run in parallel via `Promise.all`.

### Q43. What happens if you don't handle a Promise rejection?
```javascript
async function risky() {
    throw new Error("Oops");
}
risky();   // unhandled promise rejection! logs a warning/error to console,
             // and in Node.js can even crash the process depending on version/config

// Always handle it:
risky().catch(err => console.error(err));
// or wrap the await in try/catch inside an async function
```

### Q44. Explain `async function` return values.
Any `async function` **always returns a Promise**, even if you `return` a plain value inside it (it gets auto-wrapped).
```javascript
async function getValue() {
    return 42;
}
getValue();                  // Promise { 42 }, NOT 42 directly
getValue().then(v => console.log(v));   // 42

async function getValueThatThrows() {
    throw new Error("fail");
}
getValueThatThrows().catch(e => console.log(e.message));   // "fail" -> becomes a rejected Promise
```

### Q45. What is a callback, and what is "callback hell"? How do Promises solve it?
```javascript
// Callback hell - deeply nested, hard to read/maintain, hard to handle errors consistently
getUser(1, (err, user) => {
    getOrders(user.id, (err, orders) => {
        getOrderDetails(orders[0].id, (err, details) => {
            console.log(details);
            // pyramid of doom keeps growing rightward...
        });
    });
});

// Promise chaining flattens this significantly
getUserPromise(1)
    .then(user => getOrdersPromise(user.id))
    .then(orders => getOrderDetailsPromise(orders[0].id))
    .then(details => console.log(details))
    .catch(err => console.error(err));    // single error handler for the ENTIRE chain

// async/await flattens it even further, reads top-to-bottom
async function main() {
    const user = await getUserPromise(1);
    const orders = await getOrdersPromise(user.id);
    const details = await getOrderDetailsPromise(orders[0].id);
    console.log(details);
}
```

---

## 9. The Event Loop & Concurrency Model

### Q46. Explain JavaScript's concurrency model: the call stack, event loop, and queues.
JavaScript is **single-threaded** — only one operation runs at a time on the **call stack**. Async behavior is achieved through the **event loop**, which coordinates between the call stack and two main queues:
- **Macrotask (task) queue**: `setTimeout`, `setInterval`, I/O callbacks, UI rendering.
- **Microtask queue**: Promise callbacks (`.then`/`.catch`/`.finally`), `queueMicrotask()`, `async`/`await` continuations.

**The event loop's algorithm**: after each macrotask finishes and the call stack is empty, the event loop **drains the entire microtask queue** (running all pending microtasks, including any new ones they schedule) before picking up the next macrotask.

```javascript
console.log("1: sync");

setTimeout(() => console.log("2: macrotask (setTimeout)"), 0);

Promise.resolve().then(() => console.log("3: microtask (Promise)"));

console.log("4: sync");

// Output order: 1, 4, 3, 2
// Sync code always runs first, then ALL microtasks, then the next macrotask
```

### Q47. Why does the microtask queue take priority over the macrotask queue?
This design ensures Promise-based code behaves predictably and finishes "as soon as possible" relative to other scheduled work — e.g., ensuring a chain of `.then()` calls fully resolves before the browser proceeds to the next rendering frame or timer callback, avoiding inconsistent intermediate UI states.

### Q48. What is the difference between `setTimeout(fn, 0)` and `queueMicrotask(fn)`?
```javascript
console.log("start");
setTimeout(() => console.log("timeout"), 0);
queueMicrotask(() => console.log("microtask"));
Promise.resolve().then(() => console.log("promise"));
console.log("end");

// Output: start, end, microtask, promise, timeout
// setTimeout(fn, 0) does NOT run immediately - it still waits for the call stack
// to clear AND the entire microtask queue to drain first
```

### Q49. What is "starvation" of the macrotask queue, and how can it happen?
If microtasks keep scheduling more microtasks recursively (e.g., a `.then()` that resolves and chains another `.then()` indefinitely), the event loop can get stuck endlessly draining the microtask queue, **starving** macrotasks (like `setTimeout` callbacks or rendering) from ever running — a subtle bug to be aware of in Promise-heavy code with recursive chains.

### Q50. How does the event loop relate to browser rendering?
Browsers typically render a new frame **between** macrotasks (not between microtasks) — this is why microtask-heavy work (all Promise callbacks) completes before the browser gets a chance to repaint, while `setTimeout`-scheduled work can be interleaved with rendering frames. This matters for animations and perceived UI responsiveness — heavy synchronous work anywhere blocks rendering entirely since JS execution and rendering share the same thread.

---

## 10. ES6+ Modern Features

### Q51. Destructuring — objects, arrays, defaults, nested, and renaming.
```javascript
// Array destructuring
const [first, second, ...rest] = [1, 2, 3, 4];    // first=1, second=2, rest=[3,4]

// Object destructuring
const { name, age = 30 } = { name: "Alice" };       // age defaults to 30 since not present
const { name: userName } = { name: "Bob" };            // renaming: userName = "Bob"

// Nested destructuring
const { address: { city } } = { address: { city: "NYC" } };   // city = "NYC"

// Function parameter destructuring - extremely common in React props
function greet({ name, age }) {
    return `${name} is ${age}`;
}
greet({ name: "Carol", age: 25 });

// Swapping variables without a temp variable
let a = 1, b = 2;
[a, b] = [b, a];   // a=2, b=1
```

### Q52. Spread (`...`) vs Rest (`...`) — same syntax, opposite purposes.
```javascript
// Spread - EXPANDS an iterable into individual elements
const arr1 = [1, 2, 3];
const arr2 = [...arr1, 4, 5];       // [1, 2, 3, 4, 5]
const merged = { ...obj1, ...obj2 };   // merges objects, later keys override earlier ones
Math.max(...[1, 5, 3]);                    // spreads array as individual arguments

// Rest - COLLECTS remaining elements into an array/object
function sum(...nums) { return nums.reduce((a, b) => a + b); }
const { a, ...restProps } = { a: 1, b: 2, c: 3 };   // restProps = { b: 2, c: 3 }
```

### Q53. Template literals — beyond simple interpolation.
```javascript
const name = "Alice";
const greeting = `Hello, ${name}!`;              // basic interpolation

const multiline = `Line 1
Line 2`;                                            // native multi-line strings, no \n needed

// Tagged templates - powerful, used by libraries like styled-components
function highlight(strings, ...values) {
    return strings.reduce((acc, str, i) =>
        `${acc}${str}${values[i] ? `<b>${values[i]}</b>` : ""}`, "");
}
highlight`Name: ${name}, Age: ${30}`;
// "Name: <b>Alice</b>, Age: <b>30</b>"
```

### Q54. Optional chaining (`?.`) and nullish coalescing (`??`).
```javascript
const user = { profile: { name: "Alice" } };

user.profile?.name;              // "Alice"
user.address?.city;                // undefined -> no error, short-circuits instead of throwing
user.getName?.();                    // safely calls a method only if it exists

const value = null ?? "default";       // "default" -> only falls back for null/undefined
const value2 = 0 ?? "default";           // 0 -> NOT "default"! (0 is not null/undefined)
const value3 = 0 || "default";             // "default" -> `||` falls back for ANY falsy value, a key difference
```
`??` is specifically for `null`/`undefined` fallbacks, whereas `||` triggers on any falsy value (`0`, `""`, `false`) — a crucial distinction when `0` or `""` are legitimate valid values you don't want overridden.

### Q55. What are Symbols and what are they used for?
```javascript
const id1 = Symbol("id");
const id2 = Symbol("id");
id1 === id2;   // false -> every Symbol is guaranteed unique, even with the same description

const obj = {
    [id1]: "value1",     // used as a "hidden"/collision-proof object key
};
```
Symbols are commonly used internally (e.g., `Symbol.iterator` to make objects iterable) and for defining "private-ish" or collision-free object keys that won't show up in normal `for...in`/`Object.keys()` enumeration.

### Q56. What are Map and Set, and how do they differ from plain objects/arrays?
```javascript
const map = new Map();
map.set("key1", "value1");
map.set({ id: 1 }, "objectAsKey!");    // Maps allow ANY type as a key, including objects
map.get("key1");                          // "value1"
map.size;                                    // 2
for (const [key, value] of map) { }            // directly iterable, insertion order guaranteed

const set = new Set([1, 2, 2, 3]);
set.add(4);
set.has(2);        // true
[...set];             // [1, 2, 3, 4] -> automatic deduplication
```
`Map` is preferable to plain objects when keys aren't strings, when insertion order matters strictly, or when you need frequent additions/removals (better performance characteristics for that use case). `Set` is the idiomatic way to deduplicate values or perform set operations.

### Q57. What are generators, and how do they relate to iterators?
```javascript
function* idGenerator() {
    let id = 1;
    while (true) {
        yield id++;      // pauses here, resumes on next .next() call
    }
}

const gen = idGenerator();
gen.next().value;   // 1
gen.next().value;    // 2
gen.next().value;     // 3

// Generators implement the iterable protocol - usable directly in for...of
function* range(start, end) {
    for (let i = start; i <= end; i++) yield i;
}
for (const n of range(1, 3)) console.log(n);   // 1, 2, 3
```
Generators are useful for lazy sequences, custom iteration logic, and were historically used (pre-`async`/`await`) to write asynchronous code that looked synchronous via libraries like `co`.

---

## 11. DOM Manipulation & Events

### Q58. How do you select and manipulate DOM elements?
```javascript
document.getElementById("myId");                  // single element by ID
document.querySelector(".myClass");                 // first match (CSS selector)
document.querySelectorAll("div.item");                // NodeList of ALL matches

const el = document.querySelector("#box");
el.textContent = "New text";           // sets text (safe, no HTML parsing)
el.innerHTML = "<b>Bold text</b>";        // sets HTML (XSS risk with untrusted input!)
el.classList.add("active");
el.classList.toggle("hidden");
el.setAttribute("data-id", "42");
el.style.backgroundColor = "blue";
```
**Security note**: `innerHTML` with untrusted/user-provided content is a classic XSS vulnerability vector — prefer `textContent` for plain text, or sanitize HTML with a library (e.g., DOMPurify) when rich HTML input is genuinely needed.

### Q59. Event bubbling vs event capturing — explain the DOM event flow.
```javascript
// DOM events flow in 3 phases: CAPTURING (root -> target) -> TARGET -> BUBBLING (target -> root)

parent.addEventListener("click", () => console.log("parent (bubbling)"));      // default: bubbling phase
parent.addEventListener("click", () => console.log("parent (capturing)"), true); // capturing phase

child.addEventListener("click", () => console.log("child clicked"));

// Clicking child logs: "parent (capturing)", "child clicked", "parent (bubbling)"
```
By default, `addEventListener` listens during the **bubbling** phase (event travels from the clicked element up to the root). Passing `true` (or `{capture: true}`) as the third argument listens during the **capturing** phase instead (event travels from the root down to the target first).

### Q60. What is event delegation, and why is it useful?
```javascript
// Instead of attaching a listener to EVERY list item (wasteful, breaks for dynamically added items):
document.querySelectorAll("li").forEach(li => li.addEventListener("click", handleClick));

// Attach ONE listener to a common ancestor, and use event.target to identify what was clicked
document.querySelector("ul").addEventListener("click", (event) => {
    if (event.target.tagName === "LI") {
        console.log("Clicked:", event.target.textContent);
    }
});
```
Event delegation leverages bubbling to handle events from many (including dynamically added) child elements with a single listener — better performance and automatically works for elements added after the listener was attached.

### Q61. `preventDefault()` vs `stopPropagation()` — what's the difference?
```javascript
form.addEventListener("submit", (event) => {
    event.preventDefault();      // stops the browser's DEFAULT action (e.g., page reload on submit)
});

child.addEventListener("click", (event) => {
    event.stopPropagation();      // stops the event from BUBBLING further up to ancestors
});
```

### Q62. What is the difference between synchronous and `defer`/`async` script loading?
```html
<script src="script.js"></script>                  <!-- blocks HTML parsing until downloaded AND executed -->
<script src="script.js" defer></script>              <!-- downloads in parallel, executes AFTER HTML parsing, in order -->
<script src="script.js" async></script>                <!-- downloads in parallel, executes IMMEDIATELY when ready (order not guaranteed) -->
```
`defer` is generally preferred for scripts that need the full DOM and must run in a specific order (e.g., app initialization code); `async` suits independent scripts with no DOM dependency (e.g., analytics).

---

## 12. Error Handling

### Q63. `try`/`catch`/`finally` with synchronous and asynchronous code.
```javascript
try {
    JSON.parse("{invalid json}");
} catch (error) {
    console.error("Parsing failed:", error.message);
} finally {
    console.log("Always runs, e.g., for cleanup");
}

async function fetchData() {
    try {
        const response = await fetch("/api/data");
        if (!response.ok) throw new Error(`HTTP ${response.status}`);
        return await response.json();
    } catch (error) {
        console.error("Fetch failed:", error.message);
        throw error;      // re-throw if the caller needs to know too
    }
}
```
Note: `try`/`catch` around a `fetch()` call only catches network-level failures (DNS, connection refused) — a 404 or 500 response is still a "successful" fetch from JS's perspective; you must manually check `response.ok`/`response.status`.

### Q64. How do you create and use custom Error classes?
```javascript
class ValidationError extends Error {
    constructor(message, field) {
        super(message);
        this.name = "ValidationError";
        this.field = field;
    }
}

function validateAge(age) {
    if (age < 0) {
        throw new ValidationError("Age cannot be negative", "age");
    }
}

try {
    validateAge(-5);
} catch (error) {
    if (error instanceof ValidationError) {
        console.log(`Validation failed on field: ${error.field}`);
    } else {
        throw error;    // unexpected error type, let it propagate
    }
}
```
Custom error classes let calling code distinguish between different failure types via `instanceof`, enabling more precise error handling than string-matching `error.message`.

### Q65. What is the difference between operational errors and programmer errors?
**Operational errors** are expected runtime failures in valid code paths (network timeout, invalid user input, file not found) — these should be caught and handled gracefully. **Programmer errors** are actual bugs (calling a function with the wrong argument types, undefined is not a function) — these generally should NOT be silently caught; let them surface (crash loudly in dev, get logged/alerted in production) so they get fixed rather than papered over.

---

## 13. Modules

### Q66. ES Modules (`import`/`export`) vs CommonJS (`require`/`module.exports`).
```javascript
// CommonJS (Node.js default historically) - synchronous, dynamic
// math.js
module.exports = { add: (a, b) => a + b };
// main.js
const { add } = require("./math");

// ES Modules (modern standard, works in browsers natively via type="module") - static, async-capable
// math.js
export function add(a, b) { return a + b; }
export default function multiply(a, b) { return a * b; }
// main.js
import multiply, { add } from "./math.js";
```
Key differences: ES Modules are **statically analyzable** (imports/exports resolved at parse time, enabling tree-shaking by bundlers) and support top-level `await`; CommonJS is dynamic and resolved at runtime, historically simpler for Node.js but less optimizable.

### Q67. Named exports vs default exports — when to use which?
```javascript
// Named exports - multiple per file, must match name on import (or alias with `as`)
export const PI = 3.14159;
export function square(x) { return x * x; }
import { PI, square as sq } from "./math.js";

// Default export - one per file, imported name is arbitrary
export default class Calculator { }
import Calc from "./Calculator.js";        // any name works here
```
Named exports are generally preferred in larger codebases for better refactoring support (IDEs can rename/find-usages reliably) and explicit imports; default exports are common for a file's single primary export (e.g., a React component).

### Q68. What is tree-shaking, and how do ES Modules enable it?
Tree-shaking is a bundler optimization (webpack, Rollup, esbuild) that eliminates unused exported code from the final bundle. Because ES Modules have a **static** import/export structure (unlike CommonJS's dynamic `require()` calls, which can be conditional/computed at runtime), bundlers can statically determine which exports are actually used and strip out the rest — resulting in smaller production bundles.

---

## 14. Advanced Patterns

### Q69. Implement `debounce` and explain when to use it.
```javascript
function debounce(fn, delay) {
    let timeoutId;
    return function (...args) {
        clearTimeout(timeoutId);
        timeoutId = setTimeout(() => fn.apply(this, args), delay);
    };
}

const debouncedSearch = debounce((query) => {
    console.log("Searching for:", query);
}, 300);

// Rapid keystrokes only trigger ONE search call, 300ms after the LAST keystroke
searchInput.addEventListener("input", (e) => debouncedSearch(e.target.value));
```
**Debounce** delays execution until a pause in activity — ideal for search-as-you-type, resize handlers, or auto-save, where you only care about the final state after rapid-fire events settle.

### Q70. Implement `throttle` and explain how it differs from debounce.
```javascript
function throttle(fn, limit) {
    let inThrottle = false;
    return function (...args) {
        if (!inThrottle) {
            fn.apply(this, args);
            inThrottle = true;
            setTimeout(() => { inThrottle = false; }, limit);
        }
    };
}

const throttledScroll = throttle(() => {
    console.log("Scroll position:", window.scrollY);
}, 200);

window.addEventListener("scroll", throttledScroll);
```
**Throttle** guarantees execution at most once per interval, regardless of how many events fire — ideal for scroll/mousemove handlers where you want *regular* updates during continuous activity, not just at the end.

### Q71. Implement memoization.
```javascript
function memoize(fn) {
    const cache = new Map();
    return function (...args) {
        const key = JSON.stringify(args);
        if (cache.has(key)) return cache.get(key);
        const result = fn.apply(this, args);
        cache.set(key, result);
        return result;
    };
}

const slowSquare = (n) => { for (let i = 0; i < 1e8; i++) {} return n * n; };
const fastSquare = memoize(slowSquare);
fastSquare(5);   // slow the first time
fastSquare(5);   // instant - served from cache
```

### Q72. Explain the module pattern and the revealing module pattern.
```javascript
const BankAccount = (function () {
    let balance = 0;      // truly private, only accessible via closures below

    function deposit(amount) { balance += amount; }
    function withdraw(amount) { if (amount <= balance) balance -= amount; }
    function getBalance() { return balance; }

    return { deposit, withdraw, getBalance };   // reveals only the public API
})();

BankAccount.deposit(100);
console.log(BankAccount.getBalance());   // 100
console.log(BankAccount.balance);          // undefined - truly private!
```
Historically important for encapsulation before ES6 classes/modules existed with proper private fields; still a useful pattern for singleton-style state with a clean public API.

### Q73. What is the difference between shallow and deep equality checks, and how would you implement `deepEqual`?
```javascript
function deepEqual(a, b) {
    if (a === b) return true;
    if (typeof a !== "object" || typeof b !== "object" || a === null || b === null) return false;

    const keysA = Object.keys(a), keysB = Object.keys(b);
    if (keysA.length !== keysB.length) return false;

    return keysA.every(key => deepEqual(a[key], b[key]));
}

deepEqual({ a: 1, b: { c: 2 } }, { a: 1, b: { c: 2 } });   // true
({ a: 1 } === { a: 1 });                                       // false, reference comparison
```

### Q74. Explain the Observer pattern / a simple pub-sub implementation in JS.
```javascript
class EventEmitter {
    constructor() { this.listeners = {}; }
    on(event, callback) {
        (this.listeners[event] ??= []).push(callback);
    }
    emit(event, ...args) {
        (this.listeners[event] || []).forEach(cb => cb(...args));
    }
    off(event, callback) {
        this.listeners[event] = (this.listeners[event] || []).filter(cb => cb !== callback);
    }
}

const emitter = new EventEmitter();
emitter.on("userLoggedIn", (user) => console.log(`${user} logged in`));
emitter.emit("userLoggedIn", "Alice");
```
This pattern underlies Node's built-in `EventEmitter`, DOM events, and many state management libraries — decoupling the code that triggers an event from the code that reacts to it.

---

## 15. Node.js Essentials

### Q75. What is Node.js, and how does it differ from browser JavaScript?
Node.js is a JavaScript runtime built on Chrome's V8 engine, running JavaScript **outside the browser** — on servers, CLIs, and build tooling. Key differences: no DOM/`window` object (Node has `global` instead), access to filesystem/OS/network APIs via built-in modules (`fs`, `http`, `path`), and uses CommonJS modules by default (though ES Modules are now supported too).

### Q76. What is `npm`/`package.json`, and what do `dependencies` vs `devDependencies` mean?
```json
{
  "name": "my-app",
  "version": "1.0.0",
  "dependencies": { "express": "^4.18.0" },
  "devDependencies": { "jest": "^29.0.0", "eslint": "^8.0.0" },
  "scripts": { "start": "node index.js", "test": "jest" }
}
```
`dependencies` are needed at runtime in production (e.g., `express`); `devDependencies` are only needed during development/testing/building (e.g., test runners, linters, bundlers) and are typically excluded from production installs (`npm install --production`).

### Q77. What is the Node.js event loop, and how does it differ from the browser's?
Conceptually similar (single-threaded, non-blocking I/O via an event loop), but Node's event loop has distinct **phases** (timers, pending callbacks, poll, check, close callbacks) and uses **libuv** under the hood to hand off I/O operations (file system, network) to a thread pool, notifying the main thread via callbacks once complete — this is how Node achieves non-blocking I/O despite JavaScript itself running single-threaded.

### Q78. How do you build a basic HTTP server with Node's built-in `http` module (no framework)?
```javascript
const http = require("http");

const server = http.createServer((req, res) => {
    if (req.url === "/" && req.method === "GET") {
        res.writeHead(200, { "Content-Type": "application/json" });
        res.end(JSON.stringify({ message: "Hello World" }));
    } else {
        res.writeHead(404);
        res.end("Not Found");
    }
});

server.listen(3000, () => console.log("Server running on port 3000"));
```

### Q79. Basic Express.js example — the most common Node.js web framework.
```javascript
const express = require("express");
const app = express();
app.use(express.json());

let items = [];

app.get("/items", (req, res) => res.json(items));

app.post("/items", (req, res) => {
    const item = { id: items.length + 1, ...req.body };
    items.push(item);
    res.status(201).json(item);
});

app.use((err, req, res, next) => {           // centralized error-handling middleware
    console.error(err);
    res.status(500).json({ error: "Internal server error" });
});

app.listen(3000, () => console.log("Server listening on port 3000"));
```

### Q80. What are streams in Node.js, and why do they matter?
Streams process data in **chunks** rather than loading it entirely into memory — critical for handling large files or data transfers efficiently.
```javascript
const fs = require("fs");

// BAD for large files - loads the ENTIRE file into memory at once
fs.readFile("huge-file.txt", (err, data) => { /* ... */ });

// GOOD - processes the file chunk by chunk, constant memory usage
const readStream = fs.createReadStream("huge-file.txt");
const writeStream = fs.createWriteStream("copy.txt");
readStream.pipe(writeStream);
```

---

## 16. Testing JavaScript

### Q81. How do you write unit tests with Jest?
```javascript
// math.js
function add(a, b) { return a + b; }
module.exports = { add };

// math.test.js
const { add } = require("./math");

describe("add()", () => {
    test("adds two positive numbers", () => {
        expect(add(2, 3)).toBe(5);
    });

    test("handles negative numbers", () => {
        expect(add(-1, -1)).toBe(-2);
    });
});
```
```bash
npx jest
```

### Q82. How do you mock functions and modules in Jest?
```javascript
const fetchData = jest.fn();
fetchData.mockResolvedValue({ id: 1, name: "Alice" });

test("fetches user data", async () => {
    const data = await fetchData();
    expect(data.name).toBe("Alice");
    expect(fetchData).toHaveBeenCalledTimes(1);
});

// Mocking an entire module
jest.mock("./api");
import { getUser } from "./api";
getUser.mockResolvedValue({ id: 1 });
```

### Q83. What is the difference between unit, integration, and end-to-end (E2E) tests?
- **Unit tests**: test a single function/component in isolation, with dependencies mocked (fast, focused).
- **Integration tests**: test multiple units working together (e.g., a route handler + real database), catching interaction bugs unit tests miss.
- **E2E tests**: test the entire application flow as a real user would (via tools like Playwright/Cypress), simulating actual browser interactions — slowest but highest confidence.

A healthy test suite typically follows the "testing pyramid": many unit tests, fewer integration tests, and a small number of E2E tests covering critical user flows.

### Q84. How do you test asynchronous code in Jest?
```javascript
test("async data fetch resolves correctly", async () => {
    const data = await fetchUserData(1);
    expect(data).toEqual({ id: 1, name: "Alice" });
});

test("rejected promise throws", async () => {
    await expect(fetchUserData(-1)).rejects.toThrow("Invalid ID");
});
```

---

## 17. Performance & Best Practices

### Q85. What causes memory leaks in JavaScript, and how do you avoid them?
Common causes:
```javascript
// 1. Forgotten event listeners / timers keeping references alive
element.addEventListener("click", handler);
// ... element removed from DOM, but listener reference still held elsewhere -> leak
// FIX: element.removeEventListener("click", handler) when done

// 2. Closures unintentionally retaining large objects
function setup() {
    const hugeData = new Array(1000000).fill("data");
    return function () {
        console.log("hi");   // doesn't use hugeData, but the closure still keeps it alive!
    };
}

// 3. Uncleared intervals
const id = setInterval(() => { /* ... */ }, 1000);
// FIX: clearInterval(id) when the component/feature is no longer needed

// 4. Global variables accumulating data indefinitely (e.g., a cache with no eviction)
```

### Q86. What's the difference between `requestAnimationFrame` and `setTimeout` for animations?
```javascript
function animate() {
    // update animation state here
    requestAnimationFrame(animate);   // syncs with the browser's repaint cycle (~60fps)
}
requestAnimationFrame(animate);
```
`requestAnimationFrame` is optimized for animations: it's synchronized with the browser's actual repaint cycle (avoiding wasted work between frames), automatically pauses when the tab is inactive (saving battery/CPU), and provides smoother results than a manually-tuned `setTimeout` interval.

### Q87. How do you optimize a long list rendering many DOM elements?
- **Virtualization/windowing** — only render the DOM elements currently visible in the viewport (libraries: `react-window`, `react-virtualized`).
- **Document fragments** — batch DOM insertions to avoid multiple reflows:
```javascript
const fragment = document.createDocumentFragment();
items.forEach(item => {
    const li = document.createElement("li");
    li.textContent = item;
    fragment.appendChild(li);
});
list.appendChild(fragment);   // single reflow instead of one per item
```
- **Debounce/throttle** expensive handlers tied to scroll/resize on long lists.

### Q88. What is the difference between reflow and repaint, and why do they matter for performance?
**Reflow (layout)**: the browser recalculates element geometry/position (triggered by changes to DOM structure, size, or certain CSS properties like `width`). **Repaint**: the browser redraws pixels without recalculating layout (triggered by visual-only changes like `color` or `background`). Reflow is more expensive than repaint, and both block the main thread — batching DOM reads/writes (avoiding "layout thrashing," where you interleave reads and writes causing repeated forced reflows) is a key performance technique.

### Q89. What are Web Workers, and when would you use them?
```javascript
// worker.js
self.onmessage = (event) => {
    const result = heavyComputation(event.data);
    self.postMessage(result);
};

// main.js
const worker = new Worker("worker.js");
worker.postMessage(largeDataSet);
worker.onmessage = (event) => console.log("Result:", event.data);
```
Web Workers run JavaScript on a **separate thread**, allowing genuinely parallel execution of CPU-intensive tasks (image processing, large computations) without blocking the main UI thread. They communicate with the main thread via message passing (no shared memory by default), and cannot access the DOM directly.

### Q90. General JavaScript performance/best-practice checklist for interviews.
- Minimize DOM access/manipulation; batch reads and writes separately.
- Use `const`/`let` (never `var`) for clearer scoping and to avoid accidental global leaks.
- Avoid deeply nested callbacks; prefer `async`/`await` and `Promise.all` for concurrent independent work.
- Debounce/throttle high-frequency event handlers (scroll, resize, input).
- Lazy-load images/routes/components not needed immediately (code-splitting).
- Avoid memory leaks: clean up listeners, timers, and subscriptions when no longer needed.
- Prefer strict equality (`===`) to avoid coercion bugs.
- Use `Array`/`Object` methods that return new data over mutating methods when working with shared/reactive state (React, Vue).

---

# Part B — Complete Theory

## 18. JavaScript Theoretical Deep Dive

### 18.1 The JavaScript Engine — From Source to Execution
```
Source Code
    │
    ▼
Parser → Abstract Syntax Tree (AST)
    │
    ▼
Interpreter (e.g., V8's Ignition) — generates bytecode, executes immediately
    │
    ▼
Profiler identifies "hot" (frequently run) functions
    │
    ▼
JIT Compiler (e.g., V8's TurboFan) — compiles hot code to optimized machine code
    │
    ▼
Deoptimization — if assumptions made during optimization turn out wrong
    (e.g., a variable's type changes unexpectedly), the engine falls back
    to the interpreter and re-optimizes
```
This pipeline explains why JS performance can be inconsistent for code that changes shape/types dynamically (breaking the engine's optimizations) — writing consistent, predictable object shapes and avoiding mixing types in the same variable/array helps engines optimize more effectively.

### 18.2 Execution Context and the Call Stack
Every time a function is invoked, JavaScript creates an **execution context** containing: the variable environment (local variables, function declarations), the scope chain (for resolving outer variables), and the value of `this`. Execution contexts are pushed onto the **call stack**; when a function returns, its context is popped off.
```javascript
function a() { b(); }
function b() { c(); }
function c() { console.log(new Error().stack); }
a();
// Stack (top to bottom): c, b, a, (global)
```
A **stack overflow** occurs when the call stack grows beyond its limit — typically from unbounded recursion without a proper base case.

### 18.3 Memory: The Stack and The Heap
- **Stack**: stores primitive values and references (pointers) to objects — fixed-size, fast access, automatically managed as functions are called/return.
- **Heap**: stores objects, arrays, and functions — larger, dynamically allocated, unordered memory region.

```javascript
let x = 10;              // primitive value 10 stored directly on the stack
let obj = { a: 1 };        // a REFERENCE to the object is on the stack; the object itself lives on the heap
```

### 18.4 Garbage Collection
JavaScript uses automatic garbage collection — primarily **mark-and-sweep**: the engine periodically traces all reachable objects starting from "roots" (global object, currently executing functions' local variables), marks them as reachable, and sweeps away (frees) everything unmarked (unreachable). Modern engines like V8 use a **generational** approach — dividing the heap into "young generation" (frequently collected, since most objects die young) and "old generation" (collected less often), improving GC efficiency.

Circular references are handled correctly by mark-and-sweep (unlike naive reference counting) since reachability, not reference count, determines whether memory is freed.

### 18.5 The Prototype Chain in Depth
```javascript
const arr = [1, 2, 3];
// arr.__proto__ === Array.prototype
// Array.prototype.__proto__ === Object.prototype
// Object.prototype.__proto__ === null   <- end of the chain

arr.hasOwnProperty(0);       // true - inherited from Object.prototype, but checking own property "0"
arr.map;                       // inherited from Array.prototype
```
Every function in JS automatically gets a `prototype` property, which becomes the `[[Prototype]]` of objects created via `new FunctionName()`. This single mechanism underlies inheritance for built-in types (`Array`, `Object`, `Function`) and user-defined constructor functions/classes alike.

### 18.6 The Full Event Loop Model (Consolidated)
```
┌───────────────────────────┐
│         Call Stack           │  <- synchronous code executes here, one frame at a time
└───────────────┬───────────┘
                │ (empty stack triggers the event loop tick)
                ▼
┌───────────────────────────┐
│      Microtask Queue          │  <- Promise callbacks, queueMicrotask — FULLY drained every tick
└───────────────┬───────────┘
                ▼
┌───────────────────────────┐
│      Macrotask Queue          │  <- setTimeout, setInterval, I/O — ONE task processed per tick
└───────────────────────────┘
                (repeat)
```
This single mental model explains virtually every "what's the output order" interview question involving `setTimeout`, Promises, and synchronous code together.

### 18.7 Functions as First-Class Citizens & Higher-Order Functions
Because functions in JS are values (can be assigned, passed, returned, stored in data structures), JavaScript naturally supports **higher-order functions** (functions that take/return other functions) — the foundation for `map`/`filter`/`reduce`, decorators/wrappers (like `debounce`), and functional composition patterns widely used across the ecosystem (React hooks, Redux middleware, Express middleware all lean on this).

### 18.8 Type Coercion Rules — The Full Picture
JavaScript's `+` operator first calls `ToPrimitive()` on both operands (invoking `valueOf()` then `toString()` on objects). For all other arithmetic operators, both operands go through `ToNumber()`. Equality (`==`) has its own detailed algorithm (the Abstract Equality Comparison) with special-cased rules for `null`/`undefined` — which is precisely why `===` (no coercion, straightforward type + value comparison) is recommended as the default in virtually all style guides.

### 18.9 Where JavaScript Fits: Browser vs Server vs Everywhere
JavaScript began as a browser-only scripting language (1995) but is now a genuinely universal language: **Node.js** (2009) brought it server-side using V8 + libuv for non-blocking I/O; **React Native**/**Ionic** brought it to mobile; **Electron** brought it to desktop apps; and **Deno**/**Bun** are newer runtimes addressing some of Node's early design limitations (built-in TypeScript support, better security defaults, modern module resolution). This "one language everywhere" property is one of JavaScript's biggest practical advantages for full-stack development.

---

# Part C — Full Tutorial

## 19. Complete Tutorial: Building a Web App from Scratch

We'll build a **Task Tracker Web App** — vanilla HTML/CSS/JavaScript on the frontend (no framework, so the fundamentals are crystal clear), talking to a small **Node.js + Express** backend API, with data persistence. This mirrors real full-stack JS patterns end to end.

### 19.1 Project Setup

```bash
mkdir task-tracker && cd task-tracker
mkdir server client
cd server
npm init -y
npm install express cors
npm install --save-dev nodemon
cd ..
```

Project structure:
```
task-tracker/
├── server/
│   ├── package.json
│   ├── server.js
│   └── data.json          (simple file-based storage for this tutorial)
└── client/
    ├── index.html
    ├── style.css
    └── app.js
```

### 19.2 Backend: A Small Express API

```javascript
// server/server.js
const express = require("express");
const cors = require("cors");
const fs = require("fs");
const path = require("path");

const app = express();
const DATA_FILE = path.join(__dirname, "data.json");

app.use(cors());
app.use(express.json());

// --- Helper functions for our simple file-based "database" ---
function readTasks() {
    if (!fs.existsSync(DATA_FILE)) return [];
    return JSON.parse(fs.readFileSync(DATA_FILE, "utf-8"));
}

function writeTasks(tasks) {
    fs.writeFileSync(DATA_FILE, JSON.stringify(tasks, null, 2));
}

// --- Routes ---
app.get("/api/tasks", (req, res) => {
    res.json(readTasks());
});

app.post("/api/tasks", (req, res) => {
    const { title } = req.body;
    if (!title || !title.trim()) {
        return res.status(400).json({ error: "Title is required" });
    }
    const tasks = readTasks();
    const newTask = { id: Date.now(), title: title.trim(), completed: false };
    tasks.push(newTask);
    writeTasks(tasks);
    res.status(201).json(newTask);
});

app.patch("/api/tasks/:id", (req, res) => {
    const tasks = readTasks();
    const task = tasks.find(t => t.id === Number(req.params.id));
    if (!task) return res.status(404).json({ error: "Task not found" });

    Object.assign(task, req.body);
    writeTasks(tasks);
    res.json(task);
});

app.delete("/api/tasks/:id", (req, res) => {
    let tasks = readTasks();
    const exists = tasks.some(t => t.id === Number(req.params.id));
    if (!exists) return res.status(404).json({ error: "Task not found" });

    tasks = tasks.filter(t => t.id !== Number(req.params.id));
    writeTasks(tasks);
    res.status(204).end();
});

// --- Centralized error handler ---
app.use((err, req, res, next) => {
    console.error(err);
    res.status(500).json({ error: "Internal server error" });
});

const PORT = 4000;
app.listen(PORT, () => console.log(`API running on http://localhost:${PORT}`));
```

```json
// server/package.json (add a "scripts" section)
{
  "scripts": {
    "dev": "nodemon server.js"
  }
}
```

Run the backend:
```bash
cd server
npm run dev
# API listening on http://localhost:4000
```

### 19.3 Frontend: HTML Structure

```html
<!-- client/index.html -->
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Task Tracker</title>
    <link rel="stylesheet" href="style.css">
</head>
<body>
    <main class="app">
        <h1>Task Tracker</h1>

        <form id="task-form">
            <input type="text" id="task-input" placeholder="What needs to be done?" required>
            <button type="submit">Add Task</button>
        </form>

        <p id="error-message" class="error hidden"></p>

        <ul id="task-list"></ul>

        <p id="empty-state" class="hidden">No tasks yet — add one above!</p>
    </main>

    <script src="app.js"></script>
</body>
</html>
```

### 19.4 Frontend: Styling

```css
/* client/style.css */
* { box-sizing: border-box; margin: 0; padding: 0; }

body {
    font-family: system-ui, sans-serif;
    background: #f4f6f8;
    display: flex;
    justify-content: center;
    padding: 40px 20px;
}

.app {
    background: white;
    width: 100%;
    max-width: 480px;
    padding: 24px;
    border-radius: 12px;
    box-shadow: 0 4px 12px rgba(0,0,0,0.08);
}

h1 { margin-bottom: 20px; font-size: 1.5rem; }

#task-form { display: flex; gap: 8px; margin-bottom: 16px; }

#task-input {
    flex: 1;
    padding: 10px 12px;
    border: 1px solid #ddd;
    border-radius: 8px;
    font-size: 1rem;
}

button {
    padding: 10px 16px;
    background: #4f46e5;
    color: white;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    font-size: 1rem;
}
button:hover { background: #4338ca; }

#task-list { list-style: none; }

.task-item {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 0;
    border-bottom: 1px solid #eee;
}

.task-item span { flex: 1; }
.task-item.completed span { text-decoration: line-through; color: #999; }

.delete-btn {
    background: #ef4444;
    padding: 4px 10px;
    font-size: 0.85rem;
}
.delete-btn:hover { background: #dc2626; }

.error { color: #dc2626; margin-bottom: 12px; }
.hidden { display: none; }

```

### 19.5 Frontend: Application Logic (this is where most of the JS concepts come together)

```javascript
// client/app.js
const API_URL = "http://localhost:4000/api/tasks";

const form = document.getElementById("task-form");
const input = document.getElementById("task-input");
const taskList = document.getElementById("task-list");
const errorMessage = document.getElementById("error-message");
const emptyState = document.getElementById("empty-state");

// --- API layer: isolates all fetch() calls, using async/await + error handling ---
const api = {
    async getTasks() {
        const res = await fetch(API_URL);
        if (!res.ok) throw new Error("Failed to load tasks");
        return res.json();
    },
    async createTask(title) {
        const res = await fetch(API_URL, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ title }),
        });
        if (!res.ok) {
            const { error } = await res.json();
            throw new Error(error || "Failed to create task");
        }
        return res.json();
    },
    async updateTask(id, updates) {
        const res = await fetch(`${API_URL}/${id}`, {
            method: "PATCH",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(updates),
        });
        if (!res.ok) throw new Error("Failed to update task");
        return res.json();
    },
    async deleteTask(id) {
        const res = await fetch(`${API_URL}/${id}`, { method: "DELETE" });
        if (!res.ok) throw new Error("Failed to delete task");
    },
};

// --- Rendering: pure function that takes state and produces DOM ---
function renderTasks(tasks) {
    taskList.innerHTML = "";                     // clear previous render
    emptyState.classList.toggle("hidden", tasks.length > 0);

    // event delegation - ONE set of listeners on the list, not per-item
    tasks.forEach(task => {
        const li = document.createElement("li");
        li.className = `task-item${task.completed ? " completed" : ""}`;
        li.dataset.id = task.id;

        const checkbox = document.createElement("input");
        checkbox.type = "checkbox";
        checkbox.checked = task.completed;

        const span = document.createElement("span");
        span.textContent = task.title;               // textContent - safe against XSS

        const deleteBtn = document.createElement("button");
        deleteBtn.textContent = "Delete";
        deleteBtn.className = "delete-btn";

        li.append(checkbox, span, deleteBtn);
        taskList.appendChild(li);
    });
}

function showError(message) {
    errorMessage.textContent = message;
    errorMessage.classList.remove("hidden");
    setTimeout(() => errorMessage.classList.add("hidden"), 3000);   // auto-dismiss
}

// --- Load & render tasks on page load ---
async function loadTasks() {
    try {
        const tasks = await api.getTasks();
        renderTasks(tasks);
    } catch (err) {
        showError(err.message);
    }
}

// --- Event: submitting the "add task" form ---
form.addEventListener("submit", async (event) => {
    event.preventDefault();     // stop the browser's default page-reload-on-submit behavior

    const title = input.value.trim();
    if (!title) return;

    try {
        await api.createTask(title);
        input.value = "";
        await loadTasks();       // re-fetch and re-render to reflect the new state
    } catch (err) {
        showError(err.message);
    }
});

// --- Event delegation: handle checkbox toggling AND delete clicks with ONE listener ---
taskList.addEventListener("click", async (event) => {
    const li = event.target.closest(".task-item");
    if (!li) return;
    const id = Number(li.dataset.id);

    try {
        if (event.target.matches("input[type='checkbox']")) {
            await api.updateTask(id, { completed: event.target.checked });
            await loadTasks();
        } else if (event.target.matches(".delete-btn")) {
            await api.deleteTask(id);
            await loadTasks();
        }
    } catch (err) {
        showError(err.message);
        await loadTasks();      // resync UI with server state on failure
    }
});

// --- Initial load ---
loadTasks();
```

### 19.6 Running the Full Stack App

```bash
# Terminal 1: start the backend
cd server
npm run dev

# Terminal 2: serve the frontend (any static server works)
cd client
npx serve .
# or simply open client/index.html directly in a browser for this simple example
```

### 19.7 What This Tutorial Demonstrates (Mapping Back to the Concepts Above)

| Concept | Where it's used in the app |
|---|---|
| `async`/`await` + `fetch` | Every API call in the `api` object |
| Error handling (`try`/`catch`) | Wrapping every async operation with user-facing error messages |
| Event delegation | Single listener on `taskList` handling clicks for all (including future) task items |
| `preventDefault()` | Stopping the form's default page-reload submit behavior |
| Closures | The `api` object's methods capturing `API_URL` from the enclosing scope |
| Safe rendering | Using `textContent` (not `innerHTML`) to avoid XSS from task titles |
| Array methods | `tasks.forEach()` for rendering, `dataset` for storing IDs on DOM nodes |
| Destructuring | `const { error } = await res.json()` when parsing error responses |
| Guard clauses | `if (!li) return;` guarding against clicks outside task items |

### 19.8 Taking It Further (Production Checklist)

To evolve this into a more robust app:
1. **Replace file-based storage** with a real database (PostgreSQL/MongoDB) on the backend.
2. **Add input validation** more thoroughly on both client and server (never trust client-side validation alone).
3. **Add optimistic UI updates** — update the DOM immediately on user action, then roll back if the API call fails, instead of always waiting for a round-trip before re-rendering.
4. **Add a build step** (Vite/webpack) for bundling, minification, and modern JS features with broader browser support via transpilation (Babel).
5. **Introduce a framework** (React/Vue/Svelte) once the app's state management grows complex enough that manual DOM diffing becomes error-prone — the vanilla-JS patterns learned here (event handling, async data flow, rendering-from-state) transfer directly.
6. **Add authentication** (JWT-based) if tasks should be scoped per user.
7. **Write tests** — unit test the `api` module functions with mocked `fetch`, and add E2E tests (Playwright/Cypress) covering the full add/complete/delete flow.
8. **Add loading states** (spinners/skeletons) during async operations for better UX.

This tutorial deliberately avoids a framework so every DOM interaction, event handler, and async call is visible and traceable — exactly the kind of fundamentals-first understanding interviewers probe for before assuming framework-specific knowledge (React, Vue, Angular) on top.
