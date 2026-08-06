# The Complete TypeScript Guide
### Interview Questions with Detailed Answers + Full Theory + Complete Tutorial

---

## Table of Contents

**Part A — Interview Questions**
1. [TypeScript Fundamentals](#1-typescript-fundamentals)
2. [Basic & Built-in Types](#2-basic--built-in-types)
3. [Interfaces & Type Aliases](#3-interfaces--type-aliases)
4. [Functions in TypeScript](#4-functions-in-typescript)
5. [Classes & OOP](#5-classes--oop)
6. [Generics](#6-generics)
7. [Union, Intersection & Literal Types](#7-union-intersection--literal-types)
8. [Type Narrowing & Type Guards](#8-type-narrowing--type-guards)
9. [Advanced Types: Mapped, Conditional & Template Literal Types](#9-advanced-types-mapped-conditional--template-literal-types)
10. [Utility Types](#10-utility-types)
11. [Enums](#11-enums)
12. [Modules & Namespaces](#12-modules--namespaces)
13. [Decorators](#13-decorators)
14. [Type Inference, Assertions & `any`/`unknown`/`never`](#14-type-inference-assertions--anyunknownnever)
15. [`tsconfig.json` & Compiler Configuration](#15-tsconfigjson--compiler-configuration)
16. [TypeScript with React & Node.js](#16-typescript-with-react--nodejs)
17. [Testing TypeScript Code](#17-testing-typescript-code)
18. [Best Practices & Common Pitfalls](#18-best-practices--common-pitfalls)

**Part B — Complete Theory**
19. [TypeScript Theoretical Deep Dive](#19-typescript-theoretical-deep-dive)

**Part C — Full Tutorial**
20. [Complete Tutorial: Building a Typed Full-Stack App](#20-complete-tutorial-building-a-typed-full-stack-app)

---

# Part A — Interview Questions

## 1. TypeScript Fundamentals

### Q1. What is TypeScript, and why would you choose it over plain JavaScript?
TypeScript is a **superset of JavaScript** developed by Microsoft that adds optional **static typing**, compiled (transpiled) down to plain JavaScript via the TypeScript compiler (`tsc`). Every valid JavaScript program is also valid TypeScript (with types simply inferred as `any` where unannotated).

**Why choose it:**
- **Catch errors at compile time** instead of at runtime in production — typos, wrong argument types, `undefined` access, etc.
- **Better editor tooling** — accurate autocompletion, inline documentation, safe refactoring (rename symbol, find all references).
- **Self-documenting code** — function signatures and interfaces describe expected shapes without needing separate docs.
- **Safer refactoring at scale** — changing a shared type immediately surfaces every place that needs updating, which is invaluable in large codebases/teams.
- **Gradual adoption** — you can incrementally add types to an existing JS codebase file by file.

### Q2. Is TypeScript a compiled or interpreted language?
TypeScript itself has **no runtime** — the TypeScript compiler (`tsc`) **transpiles** `.ts` files into plain `.js` files (erasing all type annotations in the process), which then run on any standard JavaScript engine (V8, browsers, Node.js). TypeScript's type system exists purely at **compile time** for developer tooling and error-catching — it provides zero runtime type checking or performance overhead by itself.
```typescript
function add(a: number, b: number): number {
    return a + b;
}
// compiles down to (roughly):
function add(a, b) {
    return a + b;
}
// the types are completely GONE at runtime - this is called "type erasure"
```

### Q3. What is "type erasure," and what practical implication does it have?
Type erasure means all type annotations, interfaces, and generic parameters are removed during compilation — they have **zero footprint in the emitted JavaScript**. Practical implication: you cannot check a TypeScript type at runtime (e.g., `if (x instanceof SomeInterface)` doesn't work — interfaces don't exist at runtime). For runtime type checks, you need actual JS constructs: `typeof`, `instanceof` (with real classes), or explicit validation libraries (e.g., Zod, io-ts).
```typescript
interface User { name: string; }
function isUser(obj: any): obj is User {
    // must check ACTUAL properties at runtime — the User interface itself doesn't exist here
    return typeof obj === "object" && obj !== null && "name" in obj;
}
```

### Q4. How do you install and set up a basic TypeScript project?
```bash
npm install -g typescript          # or as a devDependency: npm install --save-dev typescript
tsc --init                          # generates a tsconfig.json with sensible defaults
```
```typescript
// hello.ts
function greet(name: string): string {
    return `Hello, ${name}!`;
}
console.log(greet("World"));
```
```bash
tsc hello.ts        # compiles to hello.js
node hello.js         # runs the compiled JavaScript

# or run directly without a separate compile step during development:
npx ts-node hello.ts
```

### Q5. What is structural typing ("duck typing"), and how does it differ from nominal typing?
TypeScript uses **structural typing** — two types are considered compatible if they have the same *shape* (matching properties/methods), regardless of their declared names or explicit inheritance relationships. This is different from **nominal typing** (used in languages like Java/C#), where types are compatible only if they're explicitly declared as related (via `implements`/`extends`).
```typescript
interface Point { x: number; y: number; }

function printPoint(p: Point) {
    console.log(`${p.x}, ${p.y}`);
}

const obj = { x: 10, y: 20, z: 30 };   // extra property, never explicitly declared as a Point
printPoint(obj);                          // Valid! obj has AT LEAST the shape of Point
```
This is a core TypeScript design decision — it mirrors JavaScript's own dynamic, shape-based nature rather than forcing a class-hierarchy-based type system on top of it.

---

## 2. Basic & Built-in Types

### Q6. What are the basic primitive types in TypeScript?
```typescript
let isDone: boolean = false;
let count: number = 42;              // TS has a single `number` type (no int/float distinction)
let username: string = "Alice";
let notAssigned: undefined = undefined;
let empty: null = null;
let id: symbol = Symbol("id");
let big: bigint = 100n;
```

### Q7. How do you type arrays and tuples?
```typescript
let nums: number[] = [1, 2, 3];
let names: Array<string> = ["a", "b"];      // generic syntax, equivalent to string[]

// Tuple - a FIXED-LENGTH array where each position has a specific type
let point: [number, number] = [10, 20];
let entry: [string, number, boolean] = ["age", 30, true];

point = [10, 20, 30];    // Error: source has 3 elements but target allows only 2

// Named tuples (improves readability, especially with function returns)
let namedPoint: [x: number, y: number] = [10, 20];

// Rest elements in tuples
let stringNumberBooleans: [string, number, ...boolean[]] = ["a", 1, true, false, true];
```

### Q8. What is the difference between `any`, `unknown`, `never`, and `void`?
```typescript
let anything: any = 5;
anything = "now a string";      // no type checking at all - defeats the purpose of TS!
anything.foo.bar.baz;              // compiles fine, but crashes at runtime

let notSure: unknown = 5;
notSure = "now a string";           // fine to REASSIGN
notSure.toUpperCase();                 // Error! must narrow the type first before using it:
if (typeof notSure === "string") {
    notSure.toUpperCase();               // OK now, TS knows it's a string here
}

function throwError(): never {           // never returns (throws, or infinite loop)
    throw new Error("Always fails");
}

function logMessage(msg: string): void {    // returns nothing meaningful
    console.log(msg);
}
```
`unknown` is the type-safe counterpart to `any` — it accepts any value, but forces you to narrow the type before performing any operations on it, preserving type safety. **Prefer `unknown` over `any`** whenever the actual type is genuinely not known upfront (e.g., parsing JSON, catching errors).

### Q9. What are object types, and how do you type object literals?
```typescript
let user: { name: string; age: number } = { name: "Alice", age: 30 };

// Optional properties
let config: { debug?: boolean } = {};        // debug can be omitted

// Readonly properties
let point: { readonly x: number; readonly y: number } = { x: 0, y: 0 };
point.x = 10;      // Error: Cannot assign to 'x' because it is a read-only property

// Index signatures - for objects with dynamic/unknown keys
let scores: { [studentName: string]: number } = {};
scores["Alice"] = 95;
```

### Q10. How do type annotations differ from type inference — when should you rely on each?
```typescript
let age: number = 30;         // explicit annotation
let name = "Alice";              // inferred as `string` automatically - annotation is redundant here

function add(a: number, b: number) {   // parameters MUST be explicitly typed (no inference possible)
    return a + b;                          // return type INFERRED as number - no need to annotate
}
```
**Best practice**: let TypeScript infer types wherever it can do so accurately (local variables, simple return types) — this keeps code less verbose. Explicitly annotate function **parameters** (always required), public API boundaries (exported function signatures, for clarity and to avoid inference surprises when the implementation changes), and anywhere inference would be too broad or ambiguous.

---

## 3. Interfaces & Type Aliases

### Q11. What is the difference between `interface` and `type`, and when should you use each?
```typescript
// interface
interface User {
    name: string;
    age: number;
}
interface User {              // DECLARATION MERGING - automatically combined into one interface
    email: string;
}
// User now requires: name, age, AND email

// type alias
type Point = {
    x: number;
    y: number;
};
// type Point = { z: number };   // Error! Cannot redeclare - no merging for type aliases

// type aliases can represent things interfaces CANNOT:
type ID = string | number;                     // union types
type Callback = (data: string) => void;         // function types
type Tuple = [number, string];                    // tuples
```
**Key differences**: interfaces support **declaration merging** (useful for extending third-party library types) and are generally preferred for defining the shape of objects/classes (`implements` works with interfaces). Type aliases are more flexible — they can represent unions, primitives, tuples, and mapped types that interfaces cannot express. **Modern convention**: use `interface` for object shapes/class contracts, `type` for everything else (unions, function signatures, utility type compositions).

### Q12. How do interfaces extend other interfaces, and how does this compare to type intersection?
```typescript
interface Animal {
    name: string;
}
interface Dog extends Animal {
    breed: string;
}
const rex: Dog = { name: "Rex", breed: "Labrador" };

// interfaces can extend MULTIPLE interfaces
interface Swimmer { swim(): void; }
interface Flyer { fly(): void; }
interface Duck extends Swimmer, Flyer { quack(): void; }

// type aliases achieve similar composition via intersection (&)
type AnimalType = { name: string };
type DogType = AnimalType & { breed: string };
```

### Q13. What are optional and readonly properties in interfaces?
```typescript
interface Product {
    readonly id: number;              // cannot be reassigned after object creation
    name: string;
    description?: string;               // optional - may be omitted entirely
}

const p: Product = { id: 1, name: "Widget" };   // valid, description omitted
p.id = 2;                                          // Error: read-only property
```

### Q14. What are index signatures in interfaces, and how do they enable dynamic-key objects?
```typescript
interface StringDictionary {
    [key: string]: string;
}
const translations: StringDictionary = {
    hello: "hola",
    goodbye: "adios",
};

// Combining a known property with an index signature
interface Config {
    name: string;                    // known, required property
    [key: string]: string | number;    // catch-all for additional dynamic properties
                                         // (must be compatible with the known properties' types)
}
```

### Q15. How do interfaces describe function types and callable/constructable objects?
```typescript
interface MathOperation {
    (a: number, b: number): number;    // callable signature
}
const add: MathOperation = (a, b) => a + b;

interface ClockConstructor {
    new (hour: number, minute: number): { tick(): void };   // constructable signature
}
```

---

## 4. Functions in TypeScript

### Q16. How do you type function parameters, return values, and optional/default parameters?
```typescript
function greet(name: string, greeting: string = "Hello"): string {   // default parameter
    return `${greeting}, ${name}!`;
}

function log(message: string, userId?: number): void {                 // optional parameter
    console.log(userId ? `[User ${userId}] ${message}` : message);
}

function sum(...nums: number[]): number {                                 // rest parameters
    return nums.reduce((a, b) => a + b, 0);
}
```
Optional parameters (`?`) must come after required parameters; default parameters can be typed via inference from their default value.

### Q17. What are function overloads, and when would you use them?
```typescript
function getLength(value: string): number;
function getLength(value: unknown[]): number;
function getLength(value: string | unknown[]): number {     // implementation signature
    return value.length;
}

getLength("hello");     // 5, matched the first overload
getLength([1, 2, 3]);      // 3, matched the second overload
```
Overloads let you express multiple valid call signatures for a function whose behavior/return type genuinely differs based on the input type in a way a single union signature can't express clearly — the actual implementation signature (last one) is not visible to callers, only the overload signatures are.

### Q18. What is a function type expression vs a call signature interface?
```typescript
type AddFn = (a: number, b: number) => number;         // function type expression (type alias)

interface AddFnInterface {                                  // call signature (interface)
    (a: number, b: number): number;
}

const add: AddFn = (a, b) => a + b;
```
Both describe the same shape; `type` with an arrow-function syntax is generally more concise/common for simple callback types, while interfaces are preferred when the function type needs additional properties attached (rare) or when extending other interfaces.

### Q19. How do you type `this` inside a function?
```typescript
interface Button {
    label: string;
    onClick(this: Button, event: Event): void;   // `this` parameter - NOT a real parameter at call time,
}                                                    // purely for type-checking `this` inside the function body

const button: Button = {
    label: "Submit",
    onClick(event) {
        console.log(this.label);     // TS knows `this` is Button here, catches misuse
    },
};
```

### Q20. What is the difference between a function declaration and an arrow function in terms of typing `this`?
Just like in plain JavaScript, arrow functions don't have their own `this` — they inherit it lexically. TypeScript adds no special typing behavior here beyond what JS already does, but it will correctly type-check `this` usage inside arrow functions based on the enclosing context, catching bugs where `this` doesn't refer to what you expect.

---

## 5. Classes & OOP

### Q21. How do access modifiers (`public`, `private`, `protected`) work in TypeScript?
```typescript
class BankAccount {
    public accountHolder: string;         // accessible from anywhere (default if omitted)
    private balance: number;                // accessible ONLY within this class
    protected accountType: string;            // accessible within this class AND subclasses

    constructor(accountHolder: string, initialBalance: number) {
        this.accountHolder = accountHolder;
        this.balance = initialBalance;
        this.accountType = "standard";
    }

    public deposit(amount: number): void {
        this.balance += amount;
    }

    private validateAmount(amount: number): boolean {   // internal helper, not part of the public API
        return amount > 0;
    }
}

const account = new BankAccount("Alice", 100);
account.balance;         // Error: Property 'balance' is private
```
**Important nuance**: TypeScript's access modifiers are **compile-time only** — like all TS types, they're erased during compilation, so `private`/`protected` provide no actual runtime enforcement (unlike true private fields, see next question). They exist purely to catch misuse during development via the type checker.

### Q22. What are ECMAScript private fields (`#field`), and how do they differ from TypeScript's `private` keyword?
```typescript
class Counter {
    #count = 0;                 // TRUE runtime privacy - a native JS feature (ES2022)

    increment() { this.#count++; }
    getValue() { return this.#count; }
}

const c = new Counter();
c.#count;   // SyntaxError at compile time, AND genuinely inaccessible at runtime too
```
`#field` privacy is enforced by the **JavaScript runtime itself** (not just TypeScript's compiler) — even reflection/dynamic property access cannot reach it from outside the class. TypeScript's `private` keyword is purely a compile-time construct with zero runtime protection (accessible via `obj["balance"]` bracket notation, or plain JS callers ignoring TS entirely).

### Q23. What are abstract classes, and how do they differ from interfaces?
```typescript
abstract class Shape {
    abstract area(): number;             // no implementation - MUST be implemented by subclasses
    describe(): string {                    // concrete method - shared implementation
        return `This shape has an area of ${this.area()}`;
    }
}

class Circle extends Shape {
    constructor(private radius: number) { super(); }
    area(): number { return Math.PI * this.radius ** 2; }
}

// const s = new Shape();   // Error: Cannot create an instance of an abstract class
const c = new Circle(5);
console.log(c.describe());
```
Abstract classes can provide **partial implementation** (shared concrete methods alongside abstract ones) and can hold constructor logic/state — interfaces cannot do either (pure shape/contract, no implementation, no instantiation ever). Use abstract classes when subclasses share meaningful common behavior; use interfaces for pure contracts, especially across unrelated class hierarchies.

### Q24. Explain parameter property shorthand in constructors.
```typescript
// Verbose version
class Point {
    x: number;
    y: number;
    constructor(x: number, y: number) {
        this.x = x;
        this.y = y;
    }
}

// Shorthand: declaring an access modifier directly on a constructor parameter
// automatically creates AND assigns the corresponding class property
class PointShorthand {
    constructor(public x: number, public y: number) {}
}

const p = new PointShorthand(10, 20);
console.log(p.x, p.y);   // 10, 20
```

### Q25. How do you implement interfaces with classes, and can a class implement multiple interfaces?
```typescript
interface Flyable { fly(): void; }
interface Swimmable { swim(): void; }

class Duck implements Flyable, Swimmable {
    fly() { console.log("Flying"); }
    swim() { console.log("Swimming"); }
}
```
A class can `implement` multiple interfaces (unlike `extends`, which only supports single inheritance for classes) — TypeScript requires the class to satisfy every member of every implemented interface.

### Q26. What are static members in TypeScript classes?
```typescript
class Counter {
    static instanceCount = 0;      // shared across ALL instances, not per-instance

    constructor() {
        Counter.instanceCount++;      // accessed via the class name, not `this`
    }

    static reset(): void {
        Counter.instanceCount = 0;
    }
}
new Counter(); new Counter();
console.log(Counter.instanceCount);   // 2
```

---

## 6. Generics

### Q27. What are generics, and why are they essential in TypeScript?
Generics allow you to write reusable code that works with **multiple types** while still preserving full type safety and information — avoiding both the code duplication of type-specific functions and the type-safety loss of using `any`.
```typescript
function identity<T>(value: T): T {
    return value;
}

identity<string>("hello");    // T is explicitly string, returns string
identity(42);                    // T is INFERRED as number, returns number

// Without generics, you'd either duplicate this function per type,
// or use `any` and lose all type safety:
function identityAny(value: any): any { return value; }
const result = identityAny("hello");   // result is typed `any` - no autocomplete, no safety
```

### Q28. How do you use generics with interfaces and classes?
```typescript
interface Box<T> {
    contents: T;
}
const stringBox: Box<string> = { contents: "hello" };
const numberBox: Box<number> = { contents: 42 };

class Stack<T> {
    private items: T[] = [];
    push(item: T): void { this.items.push(item); }
    pop(): T | undefined { return this.items.pop(); }
    peek(): T | undefined { return this.items[this.items.length - 1]; }
}
const numberStack = new Stack<number>();
numberStack.push(1);
numberStack.push(2);
```

### Q29. What are generic constraints (`extends`), and why are they useful?
```typescript
interface HasLength { length: number; }

function logLength<T extends HasLength>(item: T): T {
    console.log(item.length);      // safe - TS knows T has AT LEAST a .length property
    return item;
}

logLength("hello");       // OK - strings have .length
logLength([1, 2, 3]);        // OK - arrays have .length
logLength(42);                 // Error: number doesn't have a .length property
```
Constraints restrict a generic type parameter to types that satisfy a certain shape, letting you safely use properties/methods on the generic value inside the function while still supporting multiple compatible types.

### Q30. What is `keyof`, and how is it commonly combined with generics?
```typescript
function getProperty<T, K extends keyof T>(obj: T, key: K): T[K] {
    return obj[key];
}

const user = { name: "Alice", age: 30 };
getProperty(user, "name");    // "Alice", correctly typed as string
getProperty(user, "age");       // 30, correctly typed as number
getProperty(user, "email");       // Error: "email" is not a key of `user`'s type
```
`keyof T` produces a union of all property names (as string/number/symbol literals) of type `T` — combined with an indexed access type `T[K]`, this creates fully type-safe generic property accessors, a pattern used extensively in libraries and utility types.

### Q31. What are default generic type parameters?
```typescript
interface ApiResponse<T = unknown> {
    data: T;
    status: number;
}

const response: ApiResponse = { data: "anything", status: 200 };      // T defaults to unknown
const typedResponse: ApiResponse<{ id: number }> = { data: { id: 1 }, status: 200 };
```

### Q32. How do multiple generic type parameters work, and give a practical example?
```typescript
function merge<T, U>(obj1: T, obj2: U): T & U {          // returns the INTERSECTION type
    return { ...obj1, ...obj2 };
}
const merged = merge({ name: "Alice" }, { age: 30 });
// merged is typed as { name: string } & { age: number }
console.log(merged.name, merged.age);
```

---

## 7. Union, Intersection & Literal Types

### Q33. What are union types, and how do they differ from intersection types?
```typescript
type StringOrNumber = string | number;      // UNION - value can be EITHER type
let value: StringOrNumber = "hello";
value = 42;                                    // also valid

type Named = { name: string };
type Aged = { age: number };
type Person = Named & Aged;                  // INTERSECTION - value must satisfy BOTH shapes
const p: Person = { name: "Alice", age: 30 };   // must have ALL properties from both types
```
Union (`|`) means "one of these types"; intersection (`&`) means "all of these types combined into one." A common mnemonic: union **widens** what's allowed (OR), intersection **narrows/combines** requirements (AND).

### Q34. What are literal types, and how are they used to model precise values?
```typescript
type Direction = "up" | "down" | "left" | "right";     // string literal union

function move(direction: Direction) { /* ... */ }
move("up");        // OK
move("diagonal");    // Error: not assignable to type 'Direction'

type DiceRoll = 1 | 2 | 3 | 4 | 5 | 6;                    // numeric literal union

type Config = {
    mode: "development" | "production";     // exact allowed string values, not just `string`
};
```
Literal types let you model a finite, exact set of allowed values — far more precise than a general `string`/`number` type, and TypeScript will catch typos or invalid values at compile time (e.g., catching `"Up"` capitalized wrong).

### Q35. What is a discriminated union, and why is it a powerful pattern?
```typescript
interface Circle {
    kind: "circle";           // the "discriminant" property - a common literal-typed field
    radius: number;
}
interface Square {
    kind: "square";
    sideLength: number;
}
type Shape = Circle | Square;

function getArea(shape: Shape): number {
    switch (shape.kind) {
        case "circle":
            return Math.PI * shape.radius ** 2;    // TS knows shape is Circle here!
        case "square":
            return shape.sideLength ** 2;             // TS knows shape is Square here!
    }
}
```
Discriminated unions (also called "tagged unions") let TypeScript **narrow** the exact variant within a `switch`/`if` based on a shared literal-typed field — this is one of the most powerful and commonly used patterns in real-world TypeScript for modeling state machines, API response variants (success/error), and Redux-style actions.

### Q36. How do template literal types work?
```typescript
type EventName = "click" | "hover" | "focus";
type EventHandlerName = `on${Capitalize<EventName>}`;
// resulting type: "onClick" | "onHover" | "onFocus"

type CSSProperty = `--${string}`;        // any string prefixed with "--"
const customProp: CSSProperty = "--main-color";   // valid
```
Template literal types (TS 4.1+) let you construct new string literal types by combining/transforming existing ones — useful for generating precise typed APIs from a smaller set of base literals (e.g., typed event names, typed CSS custom properties, typed route strings).

---

## 8. Type Narrowing & Type Guards

### Q37. What is type narrowing, and what triggers it?
Type narrowing is TypeScript's process of refining a broader type (like a union) to a more specific type within a certain code branch, based on runtime checks.
```typescript
function process(value: string | number) {
    if (typeof value === "string") {
        value.toUpperCase();      // TS knows value is `string` here
    } else {
        value.toFixed(2);           // TS knows value is `number` here (only option left)
    }
}
```

### Q38. What are the different kinds of type guards in TypeScript?
```typescript
// 1. typeof guard - for primitives
function example1(x: string | number) {
    if (typeof x === "string") { /* x is string */ }
}

// 2. instanceof guard - for classes
class Dog {}
class Cat {}
function example2(animal: Dog | Cat) {
    if (animal instanceof Dog) { /* animal is Dog */ }
}

// 3. `in` operator guard - checking for property existence
interface Bird { fly(): void; }
interface Fish { swim(): void; }
function move(animal: Bird | Fish) {
    if ("fly" in animal) { animal.fly(); }       // animal is Bird
    else { animal.swim(); }                          // animal is Fish
}

// 4. Custom type predicate functions (user-defined type guards)
function isFish(pet: Bird | Fish): pet is Fish {
    return (pet as Fish).swim !== undefined;
}
function example4(pet: Bird | Fish) {
    if (isFish(pet)) { pet.swim(); }       // narrowed via the custom guard
}

// 5. Discriminated union narrowing (see Q35)
// 6. Equality narrowing
function example6(x: string | null) {
    if (x !== null) { x.toUpperCase(); }    // narrowed to string
}
```

### Q39. What is the `is` keyword used for in custom type guards, and why is it necessary?
```typescript
function isString(value: unknown): value is string {     // `value is string` = type predicate
    return typeof value === "string";
}
```
Without the `value is string` return type annotation, TypeScript would only see the function as returning a plain `boolean` — it wouldn't know to actually **narrow** the type of `value` in the calling code after the check. The type predicate explicitly tells the compiler "if this returns true, treat the argument as this narrower type going forward."

### Q40. What is exhaustiveness checking with `never`, and why is it valuable?
```typescript
type Shape = Circle | Square | Triangle;    // assume Triangle was added later

function getArea(shape: Shape): number {
    switch (shape.kind) {
        case "circle": return Math.PI * shape.radius ** 2;
        case "square": return shape.sideLength ** 2;
        default:
            const _exhaustiveCheck: never = shape;    // Error if a case (e.g. "triangle") was missed!
            throw new Error(`Unhandled shape kind`);
    }
}
```
If a new variant is added to the `Shape` union but a corresponding `case` isn't added to the switch, `shape` in the `default` branch won't be assignable to `never`, causing a **compile-time error** — this is a powerful safety net that catches forgotten cases immediately when a union type grows, rather than silently failing at runtime.

---

## 9. Advanced Types: Mapped, Conditional & Template Literal Types

### Q41. What are mapped types, and how do you build a custom one?
```typescript
type User = { name: string; age: number; email: string };

// Built-in style mapped type: make every property optional
type PartialUser = { [K in keyof User]?: User[K] };
// equivalent to the built-in Partial<User>

// Make every property readonly
type ReadonlyUser = { readonly [K in keyof User]: User[K] };

// Custom: make every property nullable
type Nullable<T> = { [K in keyof T]: T[K] | null };
type NullableUser = Nullable<User>;
// { name: string | null; age: number | null; email: string | null }
```
Mapped types let you programmatically derive a new type by transforming every property of an existing type — this is exactly how TypeScript's own built-in utility types (`Partial`, `Readonly`, `Pick`, etc.) are implemented internally.

### Q42. What are conditional types, and how do they work?
```typescript
type IsString<T> = T extends string ? true : false;

type A = IsString<string>;    // true
type B = IsString<number>;      // false

// Practical example: extracting a function's return type (like the built-in ReturnType<T>)
type MyReturnType<T> = T extends (...args: any[]) => infer R ? R : never;

function getUser() { return { name: "Alice", age: 30 }; }
type UserType = MyReturnType<typeof getUser>;   // { name: string; age: number }
```
Conditional types (`T extends U ? X : Y`) let you express type-level branching logic — combined with the `infer` keyword, they can extract/derive types from within other types, powering many advanced utility types.

### Q43. What is the `infer` keyword, and what problem does it solve?
```typescript
type ArrayElement<T> = T extends (infer E)[] ? E : never;

type Elem = ArrayElement<string[]>;    // string
type Elem2 = ArrayElement<number[]>;     // number

type UnwrapPromise<T> = T extends Promise<infer U> ? U : T;
type Result = UnwrapPromise<Promise<string>>;   // string
```
`infer` lets you declare a placeholder type variable **within** a conditional type's `extends` clause, which TypeScript infers/extracts from the structure being matched — essential for building generic utilities that need to "reach into" a type (extract a Promise's resolved value, a function's return type, an array's element type, etc.).

### Q44. What are distributive conditional types?
```typescript
type ToArray<T> = T extends any ? T[] : never;

type Result = ToArray<string | number>;
// distributes over the union: ToArray<string> | ToArray<number>
// = string[] | number[]     (NOT (string | number)[] !)
```
When a conditional type's checked type is a **naked type parameter** and you pass in a union, TypeScript automatically distributes the conditional over each union member individually — a subtle but important behavior that trips up many developers when it's unintended (wrap `T` in a tuple, e.g., `[T] extends [any]`, to opt out of distribution).

### Q45. Give a full, practical mapped + conditional type example combined.
```typescript
type ApiEndpoints = {
    getUser: (id: number) => { name: string };
    createUser: (data: { name: string }) => { id: number };
    deleteUser: (id: number) => void;
};

// Extract only the endpoints whose return type is NOT void
type NonVoidEndpoints = {
    [K in keyof ApiEndpoints as ReturnType<ApiEndpoints[K]> extends void ? never : K]: ApiEndpoints[K];
};
// resulting keys: "getUser" | "createUser"  (deleteUser is filtered out)
```
This demonstrates **key remapping** (`as` clause in mapped types, TS 4.1+) combined with conditional types to filter which properties survive the transformation — a genuinely advanced but real pattern seen in typed API client generators and ORMs.

---

## 10. Utility Types

### Q46. What are TypeScript's most commonly used built-in utility types?
```typescript
interface User {
    id: number;
    name: string;
    email: string;
    age?: number;
}

Partial<User>;          // all properties become optional
Required<User>;           // all properties become required (opposite of Partial)
Readonly<User>;             // all properties become readonly
Pick<User, "id" | "name">;   // { id: number; name: string }  - select a subset of properties
Omit<User, "email">;           // User without the `email` property
Record<string, User>;             // { [key: string]: User }  - object type with dynamic keys, fixed value type

type UserKeys = keyof User;              // "id" | "name" | "email" | "age"
type UserId = User["id"];                   // number  (indexed access type)

function getUser(): User { /* ... */ }
type UserReturn = ReturnType<typeof getUser>;      // User

function updateUser(id: number, data: Partial<User>) { /* ... */ }
type UpdateUserParams = Parameters<typeof updateUser>;   // [number, Partial<User>]

type NonNull = NonNullable<string | null | undefined>;   // string
```

### Q47. Give a practical example combining `Pick`, `Omit`, and `Partial` for a real use case.
```typescript
interface Product {
    id: number;
    name: string;
    price: number;
    description: string;
    createdAt: Date;
}

// For CREATING a product - no id (auto-generated), no createdAt (server-set)
type CreateProductDto = Omit<Product, "id" | "createdAt">;

// For UPDATING a product - all fields optional (partial update), id required to target the right one
type UpdateProductDto = Partial<Omit<Product, "id" | "createdAt">> & { id: number };

// For a PRODUCT SUMMARY view - only a subset of fields
type ProductSummary = Pick<Product, "id" | "name" | "price">;
```
This pattern (deriving request/response DTOs from a single source-of-truth entity type using utility types) is extremely common in real backend/frontend TypeScript code — it keeps types DRY and automatically stays in sync when the base `Product` type changes.

### Q48. How would you implement `Pick<T, K>` and `Omit<T, K>` yourself, to demonstrate understanding of mapped types?
```typescript
type MyPick<T, K extends keyof T> = { [P in K]: T[P] };

type MyOmit<T, K extends keyof T> = MyPick<T, Exclude<keyof T, K>>;
// Exclude<T, U> removes from T all types assignable to U -> here, removes the omitted keys from `keyof T`
```

---

## 11. Enums

### Q49. What are enums, and what are the different kinds?
```typescript
// Numeric enum - members auto-increment from 0 by default
enum Direction {
    Up,       // 0
    Down,      // 1
    Left,       // 2
    Right,       // 3
}
let dir: Direction = Direction.Up;

// String enum - each member must be explicitly initialized, no auto-increment
enum Status {
    Active = "ACTIVE",
    Inactive = "INACTIVE",
    Pending = "PENDING",
}
let status: Status = Status.Active;    // "ACTIVE"

// Const enum - fully inlined at compile time, no runtime object generated (more performant)
const enum Color { Red, Green, Blue }
let c = Color.Red;   // compiles directly to `let c = 0;` - the enum itself doesn't exist at runtime
```

### Q50. What are the drawbacks of enums, and what alternatives do many TypeScript style guides recommend?
Numeric enums allow **any number** to be assigned to a variable of that enum type without error (a type-safety hole):
```typescript
enum Direction { Up, Down }
let d: Direction = 99;    // no error! any number is assignable to a numeric enum type
```
Enums also generate actual runtime JavaScript objects (except `const enum`), adding to bundle size, and don't tree-shake as cleanly as plain object literals. Many style guides (including parts of the TypeScript team's own guidance) now recommend a **union of string literals** or an **object literal with `as const`** instead:
```typescript
// Literal union - simpler, no runtime footprint, safer (impossible to assign an invalid value)
type Direction = "up" | "down" | "left" | "right";

// as const object - if you want dot-notation access AND type safety
const Direction = {
    Up: "up",
    Down: "down",
} as const;
type Direction = typeof Direction[keyof typeof Direction];   // "up" | "down"
```

---

## 12. Modules & Namespaces

### Q51. How do ES modules work in TypeScript, and what's the recommended module system today?
```typescript
// math.ts
export function add(a: number, b: number): number { return a + b; }
export default class Calculator { }

// main.ts
import Calculator, { add } from "./math";
```
TypeScript supports standard ES module syntax (`import`/`export`), which is the **recommended** approach for organizing code today — it compiles down to whatever target module system you configure (`ESNext`, `CommonJS`, etc.) via `tsconfig.json`'s `module` option, matching your runtime environment (browsers/bundlers typically want ESM, older Node.js setups may want CommonJS).

### Q52. What are namespaces, and when (rarely) would you still use them?
```typescript
namespace Validation {
    export interface StringValidator {
        isValid(s: string): boolean;
    }
    export class EmailValidator implements StringValidator {
        isValid(s: string): boolean { return s.includes("@"); }
    }
}
const validator = new Validation.EmailValidator();
```
Namespaces were TypeScript's original (pre-ES-modules) way to organize code and avoid global namespace pollution. They're now largely a **legacy feature** — ES modules are strongly preferred for virtually all modern code. Namespaces occasionally still appear when authoring global type declarations for older non-module libraries (`.d.ts` files) or merging with global ambient types.

### Q53. What is `import type`, and why was it introduced?
```typescript
import type { User } from "./types";     // explicitly a TYPE-ONLY import
import { fetchUser } from "./api";          // a regular value import

// or mixed in one line (TS 4.5+):
import { type User, fetchUser } from "./api";
```
`import type` guarantees the import is **completely erased** at compile time (since it's only used for type-checking, never at runtime) — this helps bundlers/compilers correctly tree-shake and avoid accidentally importing a module purely for its side effects when only a type was needed, and is required in some stricter build configurations (`isolatedModules`).

---

## 13. Decorators

### Q54. What are decorators, and what are the different kinds?
Decorators are a (historically experimental, now increasingly standardized) feature that lets you attach metadata or modify the behavior of classes, methods, properties, or parameters via a special `@expression` syntax — most famously used in frameworks like Angular and NestJS.
```typescript
// Class decorator
function Logger(constructor: Function) {
    console.log(`Class created: ${constructor.name}`);
}

@Logger
class Person {
    constructor(public name: string) {}
}

// Method decorator
function LogMethod(target: any, propertyKey: string, descriptor: PropertyDescriptor) {
    const original = descriptor.value;
    descriptor.value = function (...args: any[]) {
        console.log(`Calling ${propertyKey} with`, args);
        return original.apply(this, args);
    };
}

class Calculator {
    @LogMethod
    add(a: number, b: number) { return a + b; }
}
```
Enable in `tsconfig.json` with `"experimentalDecorators": true` (the legacy/widely-used version) — note that a newer, standardized decorators proposal (Stage 3 in TC39) has a slightly different API and is becoming the long-term direction, so check which version a given project/framework targets.

### Q55. Give a practical real-world example of decorator usage (e.g., in NestJS-style code).
```typescript
// This is illustrative of the PATTERN used by frameworks like NestJS (actual implementation differs internally)
function Controller(prefix: string) {
    return function (constructor: Function) {
        Reflect.defineMetadata("prefix", prefix, constructor);
    };
}

function Get(path: string) {
    return function (target: any, propertyKey: string) {
        Reflect.defineMetadata("route", { method: "GET", path }, target, propertyKey);
    };
}

@Controller("/users")
class UserController {
    @Get("/")
    getAllUsers() { /* ... */ }

    @Get("/:id")
    getUserById() { /* ... */ }
}
```
Decorators combined with the `reflect-metadata` library let frameworks build declarative APIs (routing, dependency injection, validation) by attaching metadata to classes/methods at definition time, then reading that metadata at runtime to wire up behavior automatically.

---

## 14. Type Inference, Assertions & `any`/`unknown`/`never`

### Q56. What is a type assertion, and how does it differ from type casting in other languages?
```typescript
const input = document.getElementById("username") as HTMLInputElement;
input.value = "Alice";        // TS now knows this is an HTMLInputElement, not just Element

// Alternative angle-bracket syntax (NOT usable in .tsx files - conflicts with JSX)
const input2 = <HTMLInputElement>document.getElementById("username");
```
A type assertion tells the compiler "trust me, treat this as type X" — it performs **zero runtime conversion or validation** (unlike casting in languages like Java/C#, which can throw at runtime if invalid). It's purely a compile-time instruction; if you're wrong, you'll get a runtime error/bug, not a caught type error. Use assertions sparingly, and only when you genuinely know more about a value's type than TypeScript can infer (e.g., DOM queries, external library results).

### Q57. What is the `satisfies` operator (TS 4.9+), and what problem does it solve?
```typescript
type Colors = "red" | "green" | "blue";

// Problem with a plain type annotation: loses the specific literal types
const palette1: Record<Colors, string> = {
    red: "#FF0000",
    green: "#00FF00",
    blue: "#0000FF",
};
palette1.red.toUpperCase();   // works, but palette1.red is typed as generic `string`

// Problem with NO annotation: loses validation that all keys are covered/valid
const palette2 = {
    red: [255, 0, 0],
    green: [0, 255, 0],
    blue: [0, 0, 255],
    // purple: [128, 0, 128],   // no error even though "purple" isn't a valid Color!
};

// `satisfies` gives you BOTH: full validation AND preserves the precise inferred literal types
const palette3 = {
    red: [255, 0, 0],
    green: [0, 255, 0],
    blue: [0, 0, 255],
} satisfies Record<Colors, number[]>;
palette3.red;   // typed as number[] (precise), while still validated against Record<Colors, number[]>
```
`satisfies` validates that a value matches a type **without changing the value's inferred type** — the best of both worlds compared to a type annotation (which widens the type) or no annotation (which loses validation).

### Q58. What is the non-null assertion operator (`!`), and when is it appropriate?
```typescript
function processUser(user: User | null) {
    console.log(user!.name);      // asserts user is NOT null/undefined here - USE SPARINGLY
}
```
Tells the compiler to ignore that a value might be `null`/`undefined`. Only use this when you have external certainty the compiler can't derive (e.g., you've already checked elsewhere in a way TS can't track) — misuse causes real runtime crashes that proper null checks (`if (user)`) or optional chaining (`user?.name`) would have caught safely.

### Q59. Explain contextual typing and how TypeScript infers types for callback parameters.
```typescript
const numbers = [1, 2, 3];
numbers.map((n) => n * 2);    // `n` is automatically inferred as `number`, no annotation needed
// TS infers this from the KNOWN type of `numbers` and the signature of Array.prototype.map
```
Contextual typing means TypeScript infers a value's type based on the *context/location* where it's used (a callback's expected signature, an assignment's target type) rather than purely from the expression itself — this is why callback parameters usually don't need explicit annotations inside well-typed call sites.

---

## 15. `tsconfig.json` & Compiler Configuration

### Q60. What are the most important `tsconfig.json` compiler options to know for interviews?
```json
{
  "compilerOptions": {
    "target": "ES2020",              // JS version to compile down to
    "module": "ESNext",                // module system for output (CommonJS, ESNext, etc.)
    "strict": true,                      // enables ALL strict type-checking flags (see below)
    "esModuleInterop": true,               // smoother interop between CommonJS and ES modules
    "skipLibCheck": true,                    // skip type-checking of .d.ts files (faster builds)
    "outDir": "./dist",                        // compiled output location
    "rootDir": "./src",                          // source files location
    "moduleResolution": "bundler",                 // how imports are resolved (Node/bundler/classic)
    "declaration": true,                             // emit .d.ts files alongside .js (for libraries)
    "sourceMap": true,                                 // emit .map files for debugging
    "noUnusedLocals": true,                              // error on unused local variables
    "noUnusedParameters": true                             // error on unused function parameters
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules", "dist"]
}
```

### Q61. What does `"strict": true` actually enable, and why should you almost always use it?
`strict` is a shorthand that enables a whole family of stricter checks, most notably:
- `strictNullChecks` — `null`/`undefined` are NOT automatically assignable to every type; you must handle them explicitly (this alone catches an enormous class of real-world bugs).
- `noImplicitAny` — parameters/variables without an inferable type raise an error instead of silently defaulting to `any`.
- `strictFunctionTypes` — stricter checking of function parameter type compatibility (contravariance).
- `strictPropertyInitialization` — class properties must be initialized in the constructor or explicitly marked optional/definite-assignment.
- `alwaysStrict` — emits `"use strict"` and parses in JS strict mode.

**Why use it**: without `strictNullChecks` in particular, TypeScript's safety guarantees are dramatically weaker — `null`/`undefined` bugs (the "billion dollar mistake") remain just as likely as in plain JS. Virtually all modern TypeScript projects and style guides mandate `strict: true`.

### Q62. What is the difference between `"target"` and `"lib"` in `tsconfig.json`?
```json
{
  "compilerOptions": {
    "target": "ES2017",                       // what SYNTAX to downlevel-compile to (e.g., arrow functions -> function expressions for old targets)
    "lib": ["ES2020", "DOM", "DOM.Iterable"]     // what APIs/GLOBALS TypeScript assumes are AVAILABLE (Promise, Array.flat, document, etc.)
  }
}
```
`target` controls **syntax transformation** (e.g., whether `async`/`await` gets compiled down to generator-based polyfills for older environments); `lib` controls which **type declarations** for built-in APIs (like `Array.prototype.flat`, `Promise`, browser DOM APIs) TypeScript assumes exist and type-checks against — these can be configured independently (e.g., targeting older JS syntax output while still using modern lib APIs, if you know a polyfill is present at runtime).

---

## 16. TypeScript with React & Node.js

### Q63. How do you type React function components and their props?
```tsx
interface ButtonProps {
    label: string;
    onClick: () => void;
    variant?: "primary" | "secondary";
}

function Button({ label, onClick, variant = "primary" }: ButtonProps) {
    return <button className={variant} onClick={onClick}>{label}</button>;
}

// Typing useState
const [count, setCount] = useState<number>(0);
const [user, setUser] = useState<User | null>(null);    // common pattern for "not loaded yet" state

// Typing event handlers
function handleChange(event: React.ChangeEvent<HTMLInputElement>) {
    console.log(event.target.value);
}

// Typing useRef
const inputRef = useRef<HTMLInputElement>(null);
```

### Q64. How do you type custom React hooks?
```tsx
function useFetch<T>(url: string): { data: T | null; loading: boolean; error: string | null } {
    const [data, setData] = useState<T | null>(null);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        fetch(url)
            .then(res => res.json())
            .then((json: T) => setData(json))
            .catch((err: Error) => setError(err.message))
            .finally(() => setLoading(false));
    }, [url]);

    return { data, loading, error };
}

// Usage with full type inference
const { data: user, loading } = useFetch<User>("/api/user/1");
```
Generic hooks let consumers specify the exact shape of the data they expect, while the hook implementation stays reusable across any data type.

### Q65. How do you type an Express.js route handler in TypeScript (Node backend)?
```typescript
import { Request, Response, NextFunction } from "express";

interface CreateUserBody {
    name: string;
    email: string;
}

app.post(
    "/users",
    (req: Request<{}, {}, CreateUserBody>, res: Response) => {
        const { name, email } = req.body;    // fully typed, autocomplete works
        res.status(201).json({ name, email });
    }
);

function errorHandler(err: Error, req: Request, res: Response, next: NextFunction) {
    res.status(500).json({ error: err.message });
}
```
Express's generic `Request<Params, ResBody, ReqBody, ReqQuery>` type lets you precisely type route params, response body, request body, and query strings for each individual route handler.

---

## 17. Testing TypeScript Code

### Q66. How do you set up Jest for a TypeScript project?
```bash
npm install --save-dev jest ts-jest @types/jest typescript
npx ts-jest config:init
```
```typescript
// math.ts
export function add(a: number, b: number): number { return a + b; }

// math.test.ts
import { add } from "./math";

describe("add()", () => {
    test("adds two numbers", () => {
        expect(add(2, 3)).toBe(5);
    });
});
```
`ts-jest` transpiles TypeScript on the fly during test runs, and also **type-checks** your test files, catching type errors in your tests themselves (not just runtime assertion failures).

### Q67. How do you mock a typed dependency in a TypeScript test?
```typescript
interface UserRepository {
    findById(id: number): Promise<User | null>;
}

const mockRepo: jest.Mocked<UserRepository> = {
    findById: jest.fn(),
};

test("returns user when found", async () => {
    mockRepo.findById.mockResolvedValue({ id: 1, name: "Alice" });
    const result = await mockRepo.findById(1);
    expect(result?.name).toBe("Alice");
});
```
`jest.Mocked<T>` gives you a fully-typed mock object matching the shape of the real interface — autocomplete and type-checking work on `.mockResolvedValue()`/`.mockReturnValue()` calls, catching mismatches between mock setup and the real interface's return types.

---

## 18. Best Practices & Common Pitfalls

### Q68. What's wrong with overusing `any`, and what should you do instead?
```typescript
function processData(data: any) {      // BAD - defeats the entire purpose of TypeScript
    return data.whatever.you.want;        // no error, but crashes at runtime if wrong
}
```
`any` silently disables type checking for that value and everything derived from it (it's "contagious" — operations on an `any` value produce more `any`s). Prefer `unknown` (forces narrowing before use), precise types/interfaces, or generics. Reserve `any` for genuine escape hatches (e.g., interfacing with untyped legacy JS) and isolate/wrap it immediately behind a properly-typed boundary.

### Q69. Why should you avoid excessive type assertions (`as`), and what's a safer alternative?
```typescript
// RISKY - no runtime verification, just tells TS "trust me"
const user = JSON.parse(jsonString) as User;

// SAFER - actually validate at runtime, e.g., with a validation library
import { z } from "zod";
const UserSchema = z.object({ name: z.string(), age: z.number() });
const user = UserSchema.parse(JSON.parse(jsonString));   // throws if shape doesn't match, and the RESULT is properly typed
```
Type assertions are a compile-time-only promise — if the actual runtime data doesn't match, you get silent bugs rather than caught errors. For data crossing a genuine trust boundary (API responses, `JSON.parse`, form input), prefer runtime validation libraries (Zod, io-ts, Yup) that both validate AND infer the correct static type from the schema.

### Q70. What is the difference between structural compatibility issues with excess properties, and how does "excess property checking" work?
```typescript
interface Point { x: number; y: number; }

function printPoint(p: Point) { console.log(p.x, p.y); }

printPoint({ x: 1, y: 2, z: 3 });     // Error! Excess property 'z' - but ONLY for object LITERALS

const obj = { x: 1, y: 2, z: 3 };
printPoint(obj);                          // OK - structural typing allows this via a variable (see Q5)
```
TypeScript applies **stricter excess-property checking** specifically for object literals assigned/passed directly (catching likely typos), while still allowing structurally-compatible variables with extra properties to pass through — a deliberate compromise between catching common mistakes and preserving TypeScript's fundamentally structural type system.

### Q71. What are common TypeScript interview red flags/pitfalls to avoid in your own code?
- Using `as any` to silence errors instead of fixing the underlying type issue.
- Not enabling `strict` mode (or worse, disabling it partway through a project).
- Overusing type assertions instead of proper type guards/narrowing.
- Defining overly broad types (`string` when a literal union would be more precise) that let invalid values slip through.
- Ignoring `readonly` for values that should never be mutated (arrays/objects passed as function params, in particular).
- Not leveraging utility types (`Partial`, `Pick`, `Omit`) and instead manually duplicating near-identical interfaces, causing drift over time.

---

# Part B — Complete Theory

## 19. TypeScript Theoretical Deep Dive

### 19.1 What TypeScript Actually Is: A Compile-Time Layer, Not a New Language Runtime
TypeScript is best understood as **JavaScript plus a static analysis tool**. There is no "TypeScript runtime" — the `tsc` compiler's entire job is to (a) type-check your code against the rules of the type system, and (b) strip out all type syntax to emit plain, runnable JavaScript. This single fact explains nearly every design decision and limitation in the language: types can't be checked at runtime (erasure), types have zero performance cost (they don't exist post-compilation), and TypeScript can always interoperate seamlessly with existing JS libraries and codebases.

### 19.2 The Structural Type System, In Depth
Unlike nominally-typed languages (Java, C#) where `class Dog implements Animal` is what makes `Dog` an `Animal`, TypeScript asks only: **"does this value have the required shape?"** This is sometimes called "duck typing with static verification." Structural typing has deep implications:
- Two independently-defined, unrelated interfaces with identical members are fully interchangeable.
- Function type compatibility is checked parameter-by-parameter and return-type-wise (with some nuance around variance).
- It makes TypeScript naturally fit onto existing, untyped JavaScript patterns, since JS itself has always been shape-based (duck-typed) at runtime.

### 19.3 The Type-Checking Pipeline
```
.ts source
    │
    ▼
Parser → Abstract Syntax Tree (AST)
    │
    ▼
Binder — resolves symbols/scopes, builds a symbol table
    │
    ▼
Type Checker — walks the AST, infers/verifies types, reports diagnostics (errors/warnings)
    │
    ▼
Emitter — strips types, downlevels syntax per `target`, writes out .js (and optionally .d.ts, .map)
```
Editors (VS Code, WebStorm) use the **TypeScript Language Service** — essentially the same binder/checker machinery running incrementally in the background — to power autocompletion, inline errors, and refactoring tools in real time as you type, without a full separate compile.

### 19.4 Type Inference: How Much TypeScript Figures Out On Its Own
TypeScript's inference engine works via several mechanisms: **best common type** inference (inferring the union/supertype across multiple array elements or return statements), **contextual typing** (inferring a value's type from where it's used — e.g., a callback parameter's type from the expected function signature), and **control flow analysis** (narrowing a variable's type differently in different branches based on `if`/`typeof`/`instanceof` checks). This is why well-written TypeScript often needs surprisingly few explicit annotations — the compiler does substantial work silently, which is part of why "let inference work, annotate boundaries" is a widely-adopted style guideline.

### 19.5 Variance: Why Function Parameter Types Behave Differently From Return Types
```typescript
type Animal = { name: string };
type Dog = { name: string; breed: string };

let dogHandler: (d: Dog) => void;
let animalHandler: (a: Animal) => void;

animalHandler = dogHandler;    // Error under strictFunctionTypes (contravariant parameters)
dogHandler = animalHandler;      // OK - a function accepting the WIDER type can safely handle the narrower one
```
Function parameters are checked **contravariantly** (a function accepting a broader type can be used where a function accepting a narrower type is expected — because it can still handle any narrower-type argument passed to it), while return types are checked **covariantly** (a function returning a narrower/more specific type can be used where a broader return type is expected). This variance model exists to preserve genuine type safety at every call site, and is a frequent source of "why doesn't this function type match" confusion in interviews and real code alike.

### 19.6 Declaration Files (`.d.ts`) and the DefinitelyTyped Ecosystem
Many JavaScript libraries are written in plain JS with no types at all. TypeScript solves this via **declaration files** (`.d.ts`) — files containing only type information (no implementation) describing a library's shape.
```typescript
// example.d.ts
declare module "some-untyped-library" {
    export function doSomething(x: number): string;
}
```
The community-maintained **DefinitelyTyped** repository (installed via `@types/package-name`, e.g., `npm install --save-dev @types/lodash`) provides type definitions for thousands of popular JS-only libraries, letting you get full type safety and autocomplete even for packages that never shipped TypeScript themselves.

### 19.7 The Relationship Between TypeScript's Type System and Set Theory
A useful mental model: types in TypeScript behave like **sets of possible values**. `string` is the (infinite) set of all string values. A union `A | B` is the set union. An intersection `A & B` is the set intersection. `never` is the empty set (no possible values — hence why it's the correct type for a function that never returns, or the impossible branch in exhaustiveness checking). `unknown` is the universal set (contains everything). This framing makes many advanced type operations (`Exclude`, `Extract`, conditional type distribution) far more intuitive than treating types as arbitrary syntactic constructs.

### 19.8 Soundness: Why TypeScript Is "Intentionally Unsound"
A fully **sound** type system would guarantee zero type-related runtime errors are ever possible if code compiles. TypeScript deliberately sacrifices some soundness for practicality and JS interoperability — e.g., `any` exists as an explicit escape hatch, array index access (`arr[i]`) isn't automatically typed as possibly `undefined` unless `noUncheckedIndexedAccess` is enabled, and type assertions let you override the checker entirely. Understanding this tradeoff — TypeScript optimizes for catching the vast majority of real-world bugs with minimal friction, not mathematical proof of correctness — explains why enabling additional strictness flags (`noUncheckedIndexedAccess`, `exactOptionalPropertyTypes`) can catch even more classes of bugs at the cost of extra verbosity.

### 19.9 Why TypeScript Won: Ecosystem and Adoption Context
TypeScript's dominance in the modern JS ecosystem (default in Angular, near-universal in serious React/Vue projects, native support in Deno/Bun) stems from a combination of: gradual/incremental adoptability (drop it into an existing JS codebase file by file), zero runtime cost, extremely strong editor tooling (co-developed by the VS Code and TypeScript teams at Microsoft), and a large, mature ecosystem of typed libraries. This context matters for understanding *why* companies invest in migrating JS codebases to TypeScript — the ROI is primarily in reduced runtime bugs and dramatically improved refactoring safety/velocity at scale, not in any performance benefit (there is none — again, types are fully erased).

---

# Part C — Full Tutorial

## 20. Complete Tutorial: Building a Typed Full-Stack App

We'll build a **Bookshelf API + Client** — a Node.js/Express backend written entirely in TypeScript with strict typing, generics, and interfaces, plus a small typed frontend consuming it. This tutorial deliberately touches nearly every concept covered in Part A so you see them working together in a real project.

### 20.1 Project Setup

```bash
mkdir bookshelf && cd bookshelf
npm init -y
npm install express cors
npm install --save-dev typescript ts-node-dev @types/node @types/express @types/cors
npx tsc --init
```

```json
// tsconfig.json (key settings for this project)
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "CommonJS",
    "moduleResolution": "node",
    "outDir": "./dist",
    "rootDir": "./src",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true
  },
  "include": ["src/**/*"]
}
```

```json
// package.json (add scripts)
{
  "scripts": {
    "dev": "ts-node-dev --respawn src/server.ts",
    "build": "tsc",
    "start": "node dist/server.js"
  }
}
```

Project structure:
```
bookshelf/
├── src/
│   ├── types.ts            # shared interfaces & type aliases
│   ├── store.ts             # generic in-memory data store
│   ├── validation.ts          # type guards for request validation
│   ├── bookService.ts           # business logic layer
│   ├── bookRoutes.ts              # Express routes, fully typed
│   └── server.ts                    # app entrypoint
├── tsconfig.json
└── package.json
```

### 20.2 Shared Types — The Single Source of Truth

```typescript
// src/types.ts

// Discriminated union modeling a book's availability state (Q35 pattern)
export type BookAvailability =
    | { status: "available" }
    | { status: "borrowed"; borrowerName: string; dueDate: string }
    | { status: "lost" };

export interface Book {
    id: number;
    title: string;
    author: string;
    genre: BookGenre;
    publishedYear: number;
    availability: BookAvailability;
}

export type BookGenre =
    | "fiction"
    | "non-fiction"
    | "sci-fi"
    | "biography"
    | "history";

// Utility-type-derived DTOs (Q47 pattern) - single source of truth is `Book`
export type CreateBookDto = Omit<Book, "id" | "availability">;
export type UpdateBookDto = Partial<CreateBookDto>;

// Generic API response wrapper (Q28/Q31 pattern - generics + default type param)
export interface ApiResponse<T = unknown> {
    success: boolean;
    data?: T;
    error?: string;
}
```

### 20.3 A Generic In-Memory Store (Demonstrating Generics + Constraints)

```typescript
// src/store.ts

// Generic constraint: T must have at least an `id: number` (Q29 pattern)
interface HasId {
    id: number;
}

export class InMemoryStore<T extends HasId> {
    private items: T[] = [];
    private nextId = 1;

    getAll(): T[] {
        return [...this.items];              // return a copy - avoid exposing internal mutable array
    }

    getById(id: number): T | undefined {
        return this.items.find(item => item.id === id);
    }

    create(data: Omit<T, "id">): T {
        const newItem = { ...data, id: this.nextId++ } as T;
        this.items.push(newItem);
        return newItem;
    }

    update(id: number, updates: Partial<Omit<T, "id">>): T | undefined {
        const item = this.getById(id);
        if (!item) return undefined;
        Object.assign(item, updates);
        return item;
    }

    delete(id: number): boolean {
        const initialLength = this.items.length;
        this.items = this.items.filter(item => item.id !== id);
        return this.items.length < initialLength;
    }
}
```
This single generic class can back a store for `Book`, `User`, or any future entity — a concrete demonstration of why generics matter (Q27): one reusable, fully type-safe implementation instead of duplicating CRUD logic per entity type.

### 20.4 Runtime Validation with Type Guards (Demonstrating Q37–Q39)

```typescript
// src/validation.ts
import { CreateBookDto, BookGenre } from "./types";

const VALID_GENRES: BookGenre[] = ["fiction", "non-fiction", "sci-fi", "biography", "history"];

// Custom type predicate (Q39) - validates unknown request bodies at the API boundary,
// which TypeScript's compile-time types alone CANNOT do (Q3 - type erasure)
export function isCreateBookDto(body: unknown): body is CreateBookDto {
    if (typeof body !== "object" || body === null) return false;

    const b = body as Record<string, unknown>;
    return (
        typeof b.title === "string" && b.title.trim().length > 0 &&
        typeof b.author === "string" && b.author.trim().length > 0 &&
        typeof b.publishedYear === "number" &&
        typeof b.genre === "string" && VALID_GENRES.includes(b.genre as BookGenre)
    );
}
```

### 20.5 Business Logic Layer

```typescript
// src/bookService.ts
import { InMemoryStore } from "./store";
import { Book, CreateBookDto, UpdateBookDto } from "./types";

const bookStore = new InMemoryStore<Book>();

export const bookService = {
    getAllBooks(genre?: string): Book[] {
        const all = bookStore.getAll();
        return genre ? all.filter(b => b.genre === genre) : all;
    },

    getBookById(id: number): Book | undefined {
        return bookStore.getById(id);
    },

    createBook(dto: CreateBookDto): Book {
        return bookStore.create({ ...dto, availability: { status: "available" } });
    },

    updateBook(id: number, dto: UpdateBookDto): Book | undefined {
        return bookStore.update(id, dto);
    },

    borrowBook(id: number, borrowerName: string, dueDate: string): Book | undefined {
        return bookStore.update(id, {
            // demonstrating the discriminated union in action (Q35)
        }) ?? this.updateAvailability(id, { status: "borrowed", borrowerName, dueDate });
    },

    updateAvailability(id: number, availability: Book["availability"]): Book | undefined {
        const book = bookStore.getById(id);
        if (!book) return undefined;
        book.availability = availability;
        return book;
    },

    deleteBook(id: number): boolean {
        return bookStore.delete(id);
    },
};
```

### 20.6 Fully Typed Express Routes (Demonstrating Q65)

```typescript
// src/bookRoutes.ts
import { Router, Request, Response } from "express";
import { bookService } from "./bookService";
import { isCreateBookDto } from "./validation";
import { ApiResponse, Book } from "./types";

export const bookRouter = Router();

bookRouter.get("/", (req: Request, res: Response<ApiResponse<Book[]>>) => {
    const genre = typeof req.query.genre === "string" ? req.query.genre : undefined;
    const books = bookService.getAllBooks(genre);
    res.json({ success: true, data: books });
});

bookRouter.get("/:id", (req: Request<{ id: string }>, res: Response<ApiResponse<Book>>) => {
    const book = bookService.getBookById(Number(req.params.id));
    if (!book) {
        return res.status(404).json({ success: false, error: "Book not found" });
    }
    res.json({ success: true, data: book });
});

bookRouter.post("/", (req: Request, res: Response<ApiResponse<Book>>) => {
    if (!isCreateBookDto(req.body)) {
        return res.status(400).json({ success: false, error: "Invalid book payload" });
    }
    const book = bookService.createBook(req.body);      // req.body is now narrowed to CreateBookDto
    res.status(201).json({ success: true, data: book });
});

bookRouter.patch("/:id", (req: Request<{ id: string }>, res: Response<ApiResponse<Book>>) => {
    const book = bookService.updateBook(Number(req.params.id), req.body);
    if (!book) {
        return res.status(404).json({ success: false, error: "Book not found" });
    }
    res.json({ success: true, data: book });
});

bookRouter.post("/:id/borrow", (req: Request<{ id: string }>, res: Response<ApiResponse<Book>>) => {
    const { borrowerName, dueDate } = req.body as { borrowerName: string; dueDate: string };
    const book = bookService.updateAvailability(Number(req.params.id), {
        status: "borrowed",
        borrowerName,
        dueDate,
    });
    if (!book) {
        return res.status(404).json({ success: false, error: "Book not found" });
    }
    res.json({ success: true, data: book });
});

bookRouter.delete("/:id", (req: Request<{ id: string }>, res: Response<ApiResponse<null>>) => {
    const deleted = bookService.deleteBook(Number(req.params.id));
    if (!deleted) {
        return res.status(404).json({ success: false, error: "Book not found" });
    }
    res.status(204).end();
});
```

### 20.7 App Entrypoint

```typescript
// src/server.ts
import express, { Request, Response, NextFunction } from "express";
import cors from "cors";
import { bookRouter } from "./bookRoutes";

const app = express();

app.use(cors());
app.use(express.json());
app.use("/api/books", bookRouter);

app.get("/health", (req: Request, res: Response) => {
    res.json({ status: "ok" });
});

// Centralized, TYPED error handler
app.use((err: Error, req: Request, res: Response, next: NextFunction) => {
    console.error(err);
    res.status(500).json({ success: false, error: "Internal server error" });
});

const PORT = 4000;
app.listen(PORT, () => console.log(`Bookshelf API running on http://localhost:${PORT}`));
```

### 20.8 Running the Project

```bash
npm run dev
# Bookshelf API running on http://localhost:4000

curl -X POST http://localhost:4000/api/books \
  -H "Content-Type: application/json" \
  -d '{"title":"Dune","author":"Frank Herbert","genre":"sci-fi","publishedYear":1965}'

curl http://localhost:4000/api/books

curl -X POST http://localhost:4000/api/books/1/borrow \
  -H "Content-Type: application/json" \
  -d '{"borrowerName":"Alice","dueDate":"2026-09-01"}'
```

### 20.9 A Typed Frontend Consumer (Sharing Types Across the Stack)

In a real monorepo, `types.ts` would live in a shared package imported by both server and client, guaranteeing the frontend and backend never drift out of sync on data shape.

```typescript
// client/api.ts
import type { ApiResponse, Book, CreateBookDto } from "../shared/types";

const BASE_URL = "http://localhost:4000/api/books";

export async function fetchBooks(): Promise<Book[]> {
    const res = await fetch(BASE_URL);
    const json: ApiResponse<Book[]> = await res.json();
    if (!json.success || !json.data) throw new Error(json.error ?? "Failed to fetch books");
    return json.data;
}

export async function createBook(dto: CreateBookDto): Promise<Book> {
    const res = await fetch(BASE_URL, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(dto),
    });
    const json: ApiResponse<Book> = await res.json();
    if (!json.success || !json.data) throw new Error(json.error ?? "Failed to create book");
    return json.data;
}
```
```tsx
// client/BookList.tsx (React + TypeScript, demonstrating Q63/Q64)
import { useEffect, useState } from "react";
import type { Book } from "../shared/types";
import { fetchBooks } from "./api";

function BookList() {
    const [books, setBooks] = useState<Book[]>([]);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        fetchBooks().then(setBooks).catch(err => setError((err as Error).message));
    }, []);

    if (error) return <p>Error: {error}</p>;

    return (
        <ul>
            {books.map(book => (
                <li key={book.id}>
                    {book.title} by {book.author}
                    {" — "}
                    {book.availability.status === "borrowed"
                        ? `borrowed by ${book.availability.borrowerName}`   // narrowed via discriminated union!
                        : book.availability.status}
                </li>
            ))}
        </ul>
    );
}

export default BookList;
```
Notice how `book.availability.borrowerName` is only accessible inside the `status === "borrowed"` branch — TypeScript's discriminated union narrowing (Q35) makes this completely safe without any manual casting.

### 20.10 What This Tutorial Demonstrates (Mapping Back to the Concepts Above)

| Concept | Where it's used in the project |
|---|---|
| Discriminated unions (Q35) | `BookAvailability` type and its narrowing in `BookList.tsx` |
| Generics + constraints (Q27, Q29) | `InMemoryStore<T extends HasId>` |
| Utility types (Q46, Q47) | `CreateBookDto`, `UpdateBookDto` derived via `Omit`/`Partial` |
| Custom type guards (Q39) | `isCreateBookDto()` validating `unknown` request bodies |
| `unknown` over `any` (Q8) | Every place raw request data is handled before narrowing |
| Generic API wrapper type (Q28, Q31) | `ApiResponse<T = unknown>` |
| Typed Express request/response (Q65) | Every route handler in `bookRoutes.ts` |
| Type-only imports (Q53) | `import type { Book } from "../shared/types"` in the client |
| `strict` mode (Q61) | Enabled throughout via `tsconfig.json`, catching null/undefined issues everywhere |

### 20.11 Taking It Further (Production Checklist)

1. **Replace the in-memory store** with a real database + a typed ORM (Prisma generates fully-typed models automatically from your schema — an excellent next step after this tutorial).
2. **Add a proper validation library** (Zod) instead of hand-written type guards, and derive TypeScript types directly from the schema (`z.infer<typeof schema>`) so validation and types can never drift apart.
3. **Share the `types.ts` file** between client and server via a real monorepo setup (npm/pnpm workspaces) instead of relative imports across folders.
4. **Add authentication** with typed JWT payloads (`interface JwtPayload { userId: number }`).
5. **Write tests** with `ts-jest`, using `jest.Mocked<T>` for typed mocks of the service layer.
6. **Enable additional strictness**: `noUncheckedIndexedAccess`, `exactOptionalPropertyTypes` for even stronger guarantees (Q71 area).
7. **Generate OpenAPI docs from types** using a library like `zod-to-openapi`, keeping documentation, validation, and types all derived from one source of truth.

This tutorial deliberately threads generics, discriminated unions, utility types, and type guards through a single cohesive, runnable project — exactly the depth of applied understanding interviewers look for beyond isolated syntax knowledge.
