# The Complete React Guide
### Interview Questions with Detailed Answers + Full Theory + Complete Functional/Hooks Tutorial

---

## Table of Contents

**Part A — Interview Questions**
1. [React Fundamentals & JSX](#1-react-fundamentals--jsx)
2. [Components, Props & State](#2-components-props--state)
3. [The Complete Hooks Reference (Every Built-in Hook)](#3-the-complete-hooks-reference-every-built-in-hook)
4. [Custom Hooks](#4-custom-hooks)
5. [Event Handling](#5-event-handling)
6. [Conditional Rendering, Lists & Keys](#6-conditional-rendering-lists--keys)
7. [Forms: Controlled vs Uncontrolled](#7-forms-controlled-vs-uncontrolled)
8. [Context API & Global State](#8-context-api--global-state)
9. [Performance Optimization](#9-performance-optimization)
10. [Refs & the DOM](#10-refs--the-dom)
11. [Error Boundaries](#11-error-boundaries)
12. [React Router](#12-react-router)
13. [State Management Libraries](#13-state-management-libraries)
14. [Older Patterns: HOCs & Render Props (and why Hooks replaced them)](#14-older-patterns-hocs--render-props)
15. [React 18/19: Concurrent Features](#15-react-1819-concurrent-features)
16. [Testing React Applications](#16-testing-react-applications)
17. [Best Practices & Common Pitfalls](#17-best-practices--common-pitfalls)

**Part B — Complete Theory**
18. [React Theoretical Deep Dive](#18-react-theoretical-deep-dive)

**Part C — Full Tutorial**
19. [Complete Tutorial: Building a Fully Functional, Hooks-Only Web App](#19-complete-tutorial-building-a-fully-functional-hooks-only-web-app)

---

# Part A — Interview Questions

## 1. React Fundamentals & JSX

### Q1. What is React, and what problem was it designed to solve?
React is a JavaScript library (not a full framework) for building user interfaces, created by Facebook/Meta. It solves the problem of **efficiently updating the UI in response to changing data** by introducing a **declarative, component-based** model: instead of manually mutating the DOM step by step (imperative), you describe *what* the UI should look like for a given state, and React figures out the minimal set of DOM changes needed (via the Virtual DOM and reconciliation) to get there.

Key ideas:
- **Declarative** — describe the desired UI, not the steps to mutate it.
- **Component-based** — UI is built from small, reusable, composable pieces.
- **Unidirectional data flow** — data flows down via props; changes flow up via callbacks/events.
- **Learn once, write anywhere** — the same mental model powers React DOM (web), React Native (mobile), and other renderers.

### Q2. What is JSX, and how does it relate to actual JavaScript?
JSX (JavaScript XML) is a syntax extension that lets you write HTML-like markup directly inside JavaScript. It's **not** understood by browsers natively — it's compiled (via Babel or the TypeScript compiler) into plain `React.createElement()` calls.
```jsx
const element = <h1 className="greeting">Hello, {name}!</h1>;

// compiles down to (roughly):
const element = React.createElement(
    "h1",
    { className: "greeting" },
    "Hello, ",
    name,
    "!"
);
```
Modern React (17+) uses an updated JSX transform that doesn't even require `import React` to be in scope, compiling instead to calls from `react/jsx-runtime`.

### Q3. What is the Virtual DOM, and how does it improve performance?
The Virtual DOM is a lightweight, in-memory JavaScript representation of the actual DOM. When state changes, React:
1. Builds a new Virtual DOM tree reflecting the updated UI.
2. **Diffs** it against the previous Virtual DOM tree (the "reconciliation" algorithm).
3. Computes the minimal set of actual DOM mutations needed.
4. Applies only those specific changes to the real DOM (batched together).

Because real DOM operations are comparatively expensive (triggering layout/reflow/repaint), and JS object diffing is cheap, this approach is generally much faster than naively re-rendering/re-creating DOM nodes on every change — though it's worth noting the Virtual DOM's real value is more about **developer ergonomics** (write simple declarative code, get efficient updates "for free") than raw diffing being inherently faster than all possible hand-optimized DOM code.

### Q4. What are React elements vs React components?
```jsx
// A React ELEMENT - a plain, immutable JS object describing what to render
const element = <h1>Hello</h1>;
// roughly: { type: "h1", props: { children: "Hello" } }

// A React COMPONENT - a function (or class) that RETURNS elements
function Greeting() {
    return <h1>Hello</h1>;      // returns an element
}
```
Elements are cheap, plain description objects; components are the reusable functions/classes that produce those elements, typically based on props and internal state.

### Q5. What is the difference between functional components and class components?
```jsx
// Class component (legacy style, still supported but not recommended for new code)
class Counter extends React.Component {
    constructor(props) {
        super(props);
        this.state = { count: 0 };
    }
    increment = () => this.setState({ count: this.state.count + 1 });
    render() {
        return <button onClick={this.increment}>{this.state.count}</button>;
    }
}

// Functional component with Hooks (modern, RECOMMENDED style)
function Counter() {
    const [count, setCount] = useState(0);
    return <button onClick={() => setCount(count + 1)}>{count}</button>;
}
```
Since React 16.8 introduced Hooks (2019), **functional components can do everything class components can** (state, lifecycle-equivalent effects, context, refs) with less boilerplate, better logic reuse (custom hooks vs HOCs/render props), and no confusing `this` binding issues. The React team and ecosystem have fully shifted toward function components + Hooks as the standard, idiomatic way to write React — this entire guide emphasizes that functional approach throughout.

---

## 2. Components, Props & State

### Q6. What are props, and what are their key characteristics?
```jsx
function Welcome({ name, age }) {          // props destructured directly in the signature
    return <p>{name} is {age} years old</p>;
}
<Welcome name="Alice" age={30} />
```
Props (short for "properties") are **read-only** inputs passed from a parent component to a child. A component must never modify its own props (React enforces this in dev mode) — data flows strictly **downward**; if a child needs to affect a parent, the parent passes down a callback function as a prop for the child to invoke.

### Q7. What is state, and how does it differ from props?
```jsx
function Counter() {
    const [count, setCount] = useState(0);     // state - owned and managed BY this component
    return <button onClick={() => setCount(count + 1)}>{count}</button>;
}
```

| | Props | State |
|---|---|---|
| Owned by | Parent (passed down) | The component itself |
| Mutable? | No (read-only) | Yes, via its setter function |
| Purpose | Configure a component from outside | Track data that changes over the component's lifetime |
| Triggers re-render? | Yes, when the value changes | Yes, when updated |

### Q8. Why is state update via `setState`/hook setters asynchronous (batched), and what does that mean in practice?
```jsx
function Counter() {
    const [count, setCount] = useState(0);

    function handleClick() {
        setCount(count + 1);
        setCount(count + 1);
        setCount(count + 1);
        // count only increases by 1, NOT 3! All three calls see the SAME `count` from this render
    }

    function handleClickFixed() {
        setCount(prev => prev + 1);      // functional updater form - always gets the LATEST state
        setCount(prev => prev + 1);
        setCount(prev => prev + 1);
        // count correctly increases by 3
    }
}
```
React batches state updates (for performance — avoiding unnecessary re-renders between every single `setState` call within the same event handler/tick) and updates are applied based on the state value captured **at the time the closure was created** unless you use the functional updater form (`setCount(prev => ...)`), which always operates on the most current, up-to-date state. As of React 18, this batching happens automatically across `setTimeout`, promises, and native event handlers too (not just React's own synthetic events, as was the case pre-18).

### Q9. What does "lifting state up" mean, and why is it a fundamental React pattern?
```jsx
// Two siblings need to share/sync state - the solution is to lift it to their common parent
function Parent() {
    const [sharedValue, setSharedValue] = useState("");
    return (
        <>
            <Input value={sharedValue} onChange={setSharedValue} />
            <Display value={sharedValue} />
        </>
    );
}
function Input({ value, onChange }) {
    return <input value={value} onChange={e => onChange(e.target.value)} />;
}
function Display({ value }) {
    return <p>You typed: {value}</p>;
}
```
Since data flows strictly downward via props, when two or more sibling components need access to the same changing data, the state must live in their **nearest common ancestor** — "lifted up" from wherever it was originally, so it can be passed down to all components that need it.

### Q10. What is component composition, and why is it preferred over inheritance in React?
```jsx
function Card({ children, title }) {
    return (
        <div className="card">
            <h3>{title}</h3>
            {children}
        </div>
    );
}
function App() {
    return (
        <Card title="Profile">
            <p>Custom content goes here, defined by the CALLER, not the Card component itself</p>
        </Card>
    );
}
```
React explicitly favors **composition over inheritance** — the `children` prop (and more generally, passing components/render functions as props) lets you build flexible, reusable wrapper components without the rigid coupling and fragile-base-class problems that class inheritance hierarchies introduce. The React docs themselves state there's no common use case where a deep component inheritance hierarchy is the right solution.

---

## 3. The Complete Hooks Reference (Every Built-in Hook)

This section lists **every official React Hook**, what problem it solves, and a concrete usage example — the complete functional toolkit for building React apps without classes.

### 3.1 `useState` — Local Component State
```jsx
import { useState } from "react";

function Counter() {
    const [count, setCount] = useState(0);          // initial value, or a lazy initializer function
    const [user, setUser] = useState(() => computeExpensiveInitialUser());   // lazy init - runs ONCE

    return (
        <div>
            <p>{count}</p>
            <button onClick={() => setCount(count + 1)}>+1</button>
            <button onClick={() => setCount(prev => prev - 1)}>-1 (functional update)</button>
        </div>
    );
}
```
**Use for**: any value that changes over time and should trigger a re-render when updated (form inputs, toggles, counters, fetched data). Pass a function to `useState` for expensive-to-compute initial values, so the computation runs only on the first render, not every re-render.

### 3.2 `useEffect` — Side Effects (Data Fetching, Subscriptions, Manual DOM Work)
```jsx
import { useEffect, useState } from "react";

function UserProfile({ userId }) {
    const [user, setUser] = useState(null);

    useEffect(() => {
        let cancelled = false;
        fetch(`/api/users/${userId}`)
            .then(res => res.json())
            .then(data => { if (!cancelled) setUser(data); });

        return () => { cancelled = true; };     // cleanup - prevents setting state on an unmounted/stale component
    }, [userId]);                                   // dependency array - re-runs ONLY when userId changes

    return <p>{user?.name ?? "Loading..."}</p>;
}
```
**Use for**: synchronizing a component with an external system — fetching data, subscribing to events/sockets, manually manipulating a non-React DOM element, setting up timers. The dependency array controls when the effect re-runs: `[]` runs once on mount, omitted runs after every render, `[dep1, dep2]` runs when any listed dependency changes. The returned cleanup function runs before the effect re-runs and when the component unmounts.

### 3.3 `useContext` — Consuming Context Without Prop Drilling
```jsx
import { createContext, useContext, useState } from "react";

const ThemeContext = createContext("light");

function App() {
    const [theme, setTheme] = useState("dark");
    return (
        <ThemeContext.Provider value={theme}>
            <Toolbar />
        </ThemeContext.Provider>
    );
}
function Toolbar() {
    const theme = useContext(ThemeContext);     // reads the nearest Provider's value, NO prop drilling needed
    return <div className={theme}>Toolbar</div>;
}
```
**Use for**: sharing data (theme, authenticated user, locale, feature flags) across many components at different nesting depths without manually threading props through every intermediate level ("prop drilling").

### 3.4 `useReducer` — Complex State Logic
```jsx
import { useReducer } from "react";

function reducer(state, action) {
    switch (action.type) {
        case "increment": return { count: state.count + 1 };
        case "decrement": return { count: state.count - 1 };
        case "reset": return { count: 0 };
        default: throw new Error(`Unknown action: ${action.type}`);
    }
}

function Counter() {
    const [state, dispatch] = useReducer(reducer, { count: 0 });
    return (
        <div>
            <p>{state.count}</p>
            <button onClick={() => dispatch({ type: "increment" })}>+</button>
            <button onClick={() => dispatch({ type: "decrement" })}>-</button>
            <button onClick={() => dispatch({ type: "reset" })}>Reset</button>
        </div>
    );
}
```
**Use for**: state logic involving multiple sub-values, complex transitions, or when the next state depends heavily on the previous one via distinct "actions" (similar to Redux's pattern, but local to a component). Preferred over `useState` when you find yourself writing many related `setX` calls together repeatedly, or when state transitions have non-trivial business logic worth centralizing and testing in isolation.

### 3.5 `useRef` — Mutable Values That Don't Trigger Re-renders, and DOM Access
```jsx
import { useRef, useEffect } from "react";

function TextInputWithFocus() {
    const inputRef = useRef(null);              // DOM ref usage
    const renderCount = useRef(0);                 // mutable "instance variable" usage - does NOT cause re-renders

    useEffect(() => {
        inputRef.current.focus();                     // direct DOM access, e.g., imperative focus management
        renderCount.current += 1;
    });

    return <input ref={inputRef} />;
}
```
**Use for**: (1) accessing a DOM node directly (focus management, measuring size, integrating a non-React library), and (2) storing a mutable value that persists across renders **without** causing a re-render when it changes (unlike state) — e.g., tracking a previous value, a timer ID, or a render count for debugging.

### 3.6 `useMemo` — Memoizing Expensive Computed Values
```jsx
import { useMemo, useState } from "react";

function ProductList({ products, filterText }) {
    const [count, setCount] = useState(0);          // unrelated state - re-renders this component often

    const filteredProducts = useMemo(() => {
        console.log("Filtering...");                    // only logs when `products` or `filterText` actually change
        return products.filter(p => p.name.includes(filterText));
    }, [products, filterText]);

    return (
        <div>
            <button onClick={() => setCount(count + 1)}>Unrelated re-render trigger: {count}</button>
            <ul>{filteredProducts.map(p => <li key={p.id}>{p.name}</li>)}</ul>
        </div>
    );
}
```
**Use for**: avoiding expensive recalculations on every render when the inputs haven't changed. Also commonly used to preserve **referential equality** of derived objects/arrays passed as props to memoized child components (see `React.memo`, Q section 9) — without `useMemo`, a new array/object is created every render, breaking shallow-equality checks even if the contents are identical.

### 3.7 `useCallback` — Memoizing Function References
```jsx
import { useCallback, useState } from "react";

function ParentComponent() {
    const [count, setCount] = useState(0);

    const handleClick = useCallback(() => {
        console.log("Clicked!");
    }, []);      // stable function reference across re-renders, since it has no dependencies

    return <MemoizedChild onClick={handleClick} />;    // MemoizedChild won't re-render unnecessarily
}
```
**Use for**: preventing a new function instance from being created on every render — crucial when passing callbacks to `React.memo`-wrapped children (a new function reference each render would otherwise defeat the memoization) or when a function is a dependency of another hook like `useEffect`. `useCallback(fn, deps)` is functionally equivalent to `useMemo(() => fn, deps)`.

### 3.8 `useLayoutEffect` — Synchronous Effects Before Browser Paint
```jsx
import { useLayoutEffect, useRef, useState } from "react";

function Tooltip({ text }) {
    const tooltipRef = useRef(null);
    const [position, setPosition] = useState({ top: 0, left: 0 });

    useLayoutEffect(() => {
        const rect = tooltipRef.current.getBoundingClientRect();
        setPosition({ top: rect.top - 40, left: rect.left });   // measure & adjust BEFORE the browser paints
    }, [text]);

    return <div ref={tooltipRef} style={position}>{text}</div>;
}
```
**Use for**: DOM measurements/mutations that must happen synchronously **before** the browser paints the screen, to avoid a visible flicker (e.g., measuring an element's size and repositioning it based on that measurement). Unlike `useEffect` (which runs asynchronously after paint), `useLayoutEffect` blocks the browser from painting until it finishes — use sparingly, only when a visible flash from `useEffect`'s timing would otherwise occur, since it can hurt perceived performance if overused.

### 3.9 `useImperativeHandle` — Customizing Exposed Ref Behavior
```jsx
import { useRef, useImperativeHandle, forwardRef } from "react";

const CustomInput = forwardRef((props, ref) => {
    const inputRef = useRef(null);

    useImperativeHandle(ref, () => ({                 // expose a CUSTOM, limited API instead of the raw DOM node
        focus: () => inputRef.current.focus(),
        clear: () => { inputRef.current.value = ""; },
    }));

    return <input ref={inputRef} {...props} />;
});

function Parent() {
    const customInputRef = useRef(null);
    return (
        <>
            <CustomInput ref={customInputRef} />
            <button onClick={() => customInputRef.current.focus()}>Focus</button>
            <button onClick={() => customInputRef.current.clear()}>Clear</button>
        </>
    );
}
```
**Use for**: building reusable component libraries where you want to expose a deliberately limited, controlled imperative API to parent components via `ref` (e.g., `.focus()`, `.reset()`) rather than exposing the raw underlying DOM node, which would let parents bypass your component's intended encapsulation.

### 3.10 `useDebugValue` — Custom Hook Labels in React DevTools
```jsx
import { useDebugValue, useState, useEffect } from "react";

function useOnlineStatus() {
    const [isOnline, setIsOnline] = useState(navigator.onLine);
    useDebugValue(isOnline ? "Online" : "Offline");    // shows a readable label in React DevTools for this custom hook

    useEffect(() => {
        const handler = () => setIsOnline(navigator.onLine);
        window.addEventListener("online", handler);
        window.addEventListener("offline", handler);
        return () => {
            window.removeEventListener("online", handler);
            window.removeEventListener("offline", handler);
        };
    }, []);
    return isOnline;
}
```
**Use for**: purely a developer-experience aid inside custom hooks — displays a readable label next to the hook's entry in React DevTools, rather than showing raw internal state. Has zero effect on your app's actual behavior.

### 3.11 `useDeferredValue` — Deferring Non-Urgent UI Updates
```jsx
import { useDeferredValue, useState } from "react";

function SearchResults({ query }) {
    const deferredQuery = useDeferredValue(query);     // "lags behind" the real query during heavy re-renders
    const results = useMemo(() => expensiveSearch(deferredQuery), [deferredQuery]);

    return <ul>{results.map(r => <li key={r.id}>{r.name}</li>)}</ul>;
}

function SearchPage() {
    const [query, setQuery] = useState("");
    return (
        <>
            <input value={query} onChange={e => setQuery(e.target.value)} />
            <SearchResults query={query} />
        </>
    );
}
```
**Use for**: keeping an input responsive (e.g., typing in a search box) while a computationally expensive re-render (a large filtered list) happens slightly behind, without blocking the urgent keystroke updates — a React 18 concurrent-rendering feature.

### 3.12 `useTransition` — Marking State Updates as Non-Urgent
```jsx
import { useTransition, useState } from "react";

function TabContainer() {
    const [isPending, startTransition] = useTransition();
    const [tab, setTab] = useState("home");

    function selectTab(nextTab) {
        startTransition(() => {                 // marks this state update as a low-priority "transition"
            setTab(nextTab);                       // React can interrupt it for more urgent updates (e.g., clicks)
        });
    }

    return (
        <>
            <button onClick={() => selectTab("home")}>Home</button>
            <button onClick={() => selectTab("profile")}>Profile</button>
            {isPending && <Spinner />}
            <TabContent tab={tab} />
        </>
    );
}
```
**Use for**: preventing a slow state update (one that triggers an expensive re-render) from blocking the UI's responsiveness to urgent interactions (typing, clicking) — React can pause/interrupt/abandon a transition update if something more urgent comes in, and `isPending` lets you show a loading indicator during the transition.

### 3.13 `useId` — Stable Unique IDs for Accessibility Attributes
```jsx
import { useId } from "react";

function LabeledInput({ label }) {
    const id = useId();       // generates a stable, unique ID - consistent between server and client render (SSR-safe)
    return (
        <>
            <label htmlFor={id}>{label}</label>
            <input id={id} />
        </>
    );
}
```
**Use for**: generating unique IDs to link `<label>`/`<input>` pairs or ARIA attributes (`aria-describedby`, etc.) — specifically designed to avoid ID mismatches between server-rendered and client-hydrated HTML, which a naive incrementing counter or `Math.random()` would cause.

### 3.14 `useSyncExternalStore` — Subscribing to External (Non-React) State
```jsx
import { useSyncExternalStore } from "react";

function subscribe(callback) {
    window.addEventListener("resize", callback);
    return () => window.removeEventListener("resize", callback);
}
function getSnapshot() {
    return window.innerWidth;
}

function WindowWidth() {
    const width = useSyncExternalStore(subscribe, getSnapshot);   // safely reads external, mutable state
    return <p>Width: {width}px</p>;
}
```
**Use for**: safely reading and subscribing to state that lives **outside** React (browser APIs, third-party state stores, global mutable stores) in a way that's compatible with React 18's concurrent rendering — this is the low-level primitive that libraries like Redux and Zustand use internally to integrate with React correctly.

### 3.15 `useInsertionEffect` — CSS-in-JS Library Internals (Rarely Used Directly)
```jsx
import { useInsertionEffect } from "react";

function useCSS(rule) {
    useInsertionEffect(() => {
        // inject a <style> tag BEFORE any useLayoutEffect runs, avoiding layout thrashing from CSS-in-JS libraries
        const styleTag = document.createElement("style");
        styleTag.textContent = rule;
        document.head.appendChild(styleTag);
        return () => styleTag.remove();
    }, [rule]);
}
```
**Use for**: an extremely niche hook, intended almost exclusively for **CSS-in-JS library authors** (styled-components, Emotion) to inject styles into the DOM at the correct timing (before layout effects read layout), avoiding a flash of unstyled/incorrectly-styled content. Application developers essentially never need this directly.

### 3.16 React 19: `use()`, `useActionState`, `useFormStatus`, `useOptimistic`
```jsx
// use() - reads a Promise or Context, can be called conditionally (unlike other hooks!)
import { use } from "react";
function Comments({ commentsPromise }) {
    const comments = use(commentsPromise);     // suspends the component until the promise resolves
    return <ul>{comments.map(c => <li key={c.id}>{c.text}</li>)}</ul>;
}

// useActionState - manages state driven by a form action, including pending/error state
import { useActionState } from "react";
function ChangeNameForm() {
    const [error, submitAction, isPending] = useActionState(async (prevState, formData) => {
        const result = await updateName(formData.get("name"));
        if (result.error) return result.error;
        return null;
    }, null);

    return (
        <form action={submitAction}>
            <input name="name" />
            <button disabled={isPending}>Update</button>
            {error && <p>{error}</p>}
        </form>
    );
}

// useFormStatus - lets a CHILD of a <form> read its pending submission status without prop drilling
import { useFormStatus } from "react-dom";
function SubmitButton() {
    const { pending } = useFormStatus();
    return <button disabled={pending}>{pending ? "Submitting..." : "Submit"}</button>;
}

// useOptimistic - shows an optimistic UI state while an async action is in flight
import { useOptimistic } from "react";
function Thread({ messages, sendMessage }) {
    const [optimisticMessages, addOptimisticMessage] = useOptimistic(
        messages,
        (state, newMessage) => [...state, { text: newMessage, sending: true }]
    );
    async function formAction(formData) {
        addOptimisticMessage(formData.get("message"));
        await sendMessage(formData.get("message"));
    }
    return (
        <form action={formAction}>
            {optimisticMessages.map((m, i) => <p key={i}>{m.text} {m.sending && "(sending...)"}</p>)}
            <input name="message" />
        </form>
    );
}
```
**Use for**: React 19's newest hooks are specifically built around the **Actions** pattern (forms and async transitions as a first-class primitive) — `use()` generalizes reading async/context values (even conditionally), `useActionState` centralizes form submission + pending + error state, `useFormStatus` avoids prop-drilling submission status into deeply nested form children, and `useOptimistic` provides built-in optimistic-UI patterns without manually managing rollback logic.

---

## 4. Custom Hooks

### Q11. What is a custom hook, and what are the rules for creating one?
A custom hook is simply a JavaScript function whose name starts with `use` and which calls other hooks internally — it's how React enables **reusable stateful logic** across components, without HOCs or render props.
```jsx
function useLocalStorage(key, initialValue) {
    const [value, setValue] = useState(() => {
        const stored = localStorage.getItem(key);
        return stored ? JSON.parse(stored) : initialValue;
    });

    useEffect(() => {
        localStorage.setItem(key, JSON.stringify(value));
    }, [key, value]);

    return [value, setValue];
}

// Usage - looks and behaves just like useState, but persists automatically
function Settings() {
    const [theme, setTheme] = useLocalStorage("theme", "light");
    return <button onClick={() => setTheme(theme === "light" ? "dark" : "light")}>{theme}</button>;
}
```
The naming convention (`use` prefix) is what allows both the Rules of Hooks linter and React itself (in some cases) to recognize it as a hook and correctly track its internal hook calls across renders.

### Q12. What are the "Rules of Hooks," and why do they exist?
1. **Only call hooks at the top level** — never inside loops, conditions, or nested functions.
2. **Only call hooks from React function components or other custom hooks** — never from regular JS functions or class components.

```jsx
function BadComponent({ condition }) {
    if (condition) {
        const [state, setState] = useState(0);   // VIOLATES rule 1 - conditional hook call
    }
    // ...
}
```
**Why**: React tracks hooks by **call order**, not by name — internally, each `useState`/`useEffect` call corresponds to a slot in an ordered linked list tied to that component's fiber. If hooks are called conditionally, the order can shift between renders, causing React to associate the wrong state/effect with the wrong hook call, leading to subtle, hard-to-debug bugs. The `eslint-plugin-react-hooks` package enforces both rules automatically.

### Q13. Give three more practical custom hook examples showing common real-world patterns.
```jsx
// 1. useFetch - a generic data-fetching hook
function useFetch(url) {
    const [data, setData] = useState(null);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState(null);

    useEffect(() => {
        let cancelled = false;
        setLoading(true);
        fetch(url)
            .then(res => res.json())
            .then(json => { if (!cancelled) { setData(json); setLoading(false); } })
            .catch(err => { if (!cancelled) { setError(err); setLoading(false); } });
        return () => { cancelled = true; };
    }, [url]);

    return { data, loading, error };
}

// 2. useDebounce - debounces a rapidly-changing value
function useDebounce(value, delay) {
    const [debounced, setDebounced] = useState(value);
    useEffect(() => {
        const timer = setTimeout(() => setDebounced(value), delay);
        return () => clearTimeout(timer);
    }, [value, delay]);
    return debounced;
}

// 3. useToggle - a simple boolean toggle
function useToggle(initial = false) {
    const [value, setValue] = useState(initial);
    const toggle = useCallback(() => setValue(v => !v), []);
    return [value, toggle];
}
```
This ability to extract and share stateful logic as plain, composable functions — with zero component-tree nesting overhead — is widely considered Hooks' single biggest advantage over the older HOC/render-props patterns (see Section 14).

---

## 5. Event Handling

### Q14. How does event handling in React differ from vanilla DOM event handling?
```jsx
function Button() {
    function handleClick(event) {
        console.log("Clicked", event);      // event is a SyntheticEvent - a cross-browser wrapper
    }
    return <button onClick={handleClick}>Click me</button>;
}
```
React uses **SyntheticEvents** — a wrapper around native browser events providing a consistent, cross-browser API. Historically (pre-React 17), React attached a single listener at the document root and used event delegation internally for performance; since React 17, listeners are attached to the root DOM container the app is rendered into instead of `document`, improving compatibility with multiple React versions on one page, though the delegation strategy conceptually remains.

### Q15. How do you pass arguments to an event handler?
```jsx
function TodoList({ todos, onDelete }) {
    return (
        <ul>
            {todos.map(todo => (
                <li key={todo.id}>
                    {todo.text}
                    <button onClick={() => onDelete(todo.id)}>Delete</button>   {/* arrow function wrapper */}
                </li>
            ))}
        </ul>
    );
}
```
Wrapping the handler in an inline arrow function is the standard way to pass extra arguments — note this does create a new function on every render (a minor performance consideration only relevant for very large lists or deeply memoized children).

### Q16. What is `event.preventDefault()` used for, and how does it differ from `event.stopPropagation()`?
```jsx
function SearchForm() {
    function handleSubmit(event) {
        event.preventDefault();     // stops the browser's default full-page-reload form submission
        // ... handle the search via JS/fetch instead
    }
    return <form onSubmit={handleSubmit}>...</form>;
}
```
Same underlying DOM concepts as vanilla JS (`preventDefault` stops the default browser action; `stopPropagation` stops the event from bubbling to ancestor elements/handlers) — React's SyntheticEvent exposes both methods identically to native events.

---

## 6. Conditional Rendering, Lists & Keys

### Q17. What are the common patterns for conditional rendering in JSX?
```jsx
function StatusMessage({ isLoggedIn, isLoading, error }) {
    if (isLoading) return <Spinner />;                              // early return pattern
    if (error) return <ErrorMessage error={error} />;

    return (
        <div>
            {isLoggedIn && <p>Welcome back!</p>}                      {/* && short-circuit - renders nothing if false */}
            {isLoggedIn ? <LogoutButton /> : <LoginButton />}            {/* ternary for either/or */}
        </div>
    );
}
```
**Caution with `&&`**: if the left-hand value is `0` (a falsy but non-boolean value), React will actually render the literal `0` on screen instead of nothing — a common gotcha. Use `isLoggedIn && items.length > 0 && <List />` carefully, or explicitly convert to boolean (`!!items.length`) when the count could be zero.

### Q18. Why does React require a `key` prop when rendering lists, and what makes a good key?
```jsx
function TodoList({ todos }) {
    return (
        <ul>
            {todos.map(todo => (
                <li key={todo.id}>{todo.text}</li>     // GOOD - stable, unique identifier
            ))}
        </ul>
    );
}
```
Keys help React's reconciliation algorithm identify **which items changed, were added, or were removed** across re-renders, so it can correctly match old Virtual DOM elements to new ones (preserving component state/DOM identity for unchanged items, rather than needlessly recreating them). A good key is a **stable, unique identifier** intrinsic to the data (a database ID) — not derived from array position.

### Q19. Why is using the array index as a key considered an anti-pattern?
```jsx
{todos.map((todo, index) => <li key={index}>{todo.text}</li>)}    // RISKY if the list can reorder/change
```
If items can be reordered, inserted, or removed from the middle of the list, using the index as a key means React may match the **wrong** old element to a new position — causing incorrect state to persist on the wrong item (e.g., an input's typed value or a checkbox's checked state "sticking" to the wrong row after a reorder), and unnecessary re-renders/DOM churn. Index keys are only safe for lists that are strictly static, never reordered, and never filtered.

---

## 7. Forms: Controlled vs Uncontrolled

### Q20. What is the difference between controlled and uncontrolled form components?
```jsx
// CONTROLLED - React state is the single source of truth for the input's value
function ControlledForm() {
    const [name, setName] = useState("");
    return <input value={name} onChange={e => setName(e.target.value)} />;
}

// UNCONTROLLED - the DOM itself holds the value; React reads it only when needed (via a ref)
function UncontrolledForm() {
    const inputRef = useRef(null);
    function handleSubmit(e) {
        e.preventDefault();
        console.log(inputRef.current.value);     // read directly from the DOM, on demand
    }
    return (
        <form onSubmit={handleSubmit}>
            <input ref={inputRef} defaultValue="" />
        </form>
    );
}
```
**Controlled** components give you full control — instant validation, conditional disabling, formatting-as-you-type — at the cost of a re-render on every keystroke. **Uncontrolled** components are simpler and slightly more performant for large/simple forms where you only need the value at submission time, but you lose the ability to easily react to every change.

### Q21. How do you build a multi-field controlled form efficiently?
```jsx
function SignupForm() {
    const [formData, setFormData] = useState({ username: "", email: "", password: "" });

    function handleChange(event) {
        const { name, value } = event.target;
        setFormData(prev => ({ ...prev, [name]: value }));   // single handler for ALL fields, using `name`
    }

    return (
        <form>
            <input name="username" value={formData.username} onChange={handleChange} />
            <input name="email" value={formData.email} onChange={handleChange} />
            <input name="password" type="password" value={formData.password} onChange={handleChange} />
        </form>
    );
}
```
A single generic `handleChange` keyed off each input's `name` attribute avoids writing a separate handler per field — a very common and scalable pattern for larger forms.

### Q22. How do you handle form validation in React?
```jsx
function SignupForm() {
    const [email, setEmail] = useState("");
    const [errors, setErrors] = useState({});

    function validate() {
        const newErrors = {};
        if (!email.includes("@")) newErrors.email = "Invalid email address";
        setErrors(newErrors);
        return Object.keys(newErrors).length === 0;
    }

    function handleSubmit(e) {
        e.preventDefault();
        if (validate()) {
            // submit
        }
    }

    return (
        <form onSubmit={handleSubmit}>
            <input value={email} onChange={e => setEmail(e.target.value)} />
            {errors.email && <span className="error">{errors.email}</span>}
            <button type="submit">Submit</button>
        </form>
    );
}
```
For anything beyond simple validation, libraries like **React Hook Form** or **Formik** (combined with schema validators like Zod/Yup) are the standard production choice — they minimize re-renders (React Hook Form uses uncontrolled inputs + refs internally for performance) and handle validation-error state, touch/dirty tracking, and submission state boilerplate for you.

---

## 8. Context API & Global State

### Q23. How do you set up and use the Context API end to end?
```jsx
import { createContext, useContext, useState } from "react";

const AuthContext = createContext(undefined);

function AuthProvider({ children }) {
    const [user, setUser] = useState(null);
    const login = (userData) => setUser(userData);
    const logout = () => setUser(null);

    return (
        <AuthContext.Provider value={{ user, login, logout }}>
            {children}
        </AuthContext.Provider>
    );
}

function useAuth() {                              // custom hook wrapper - the idiomatic pattern
    const context = useContext(AuthContext);
    if (context === undefined) {
        throw new Error("useAuth must be used within an AuthProvider");   // catches misuse early
    }
    return context;
}

// Usage anywhere inside <AuthProvider>
function ProfileButton() {
    const { user, logout } = useAuth();
    return user ? <button onClick={logout}>{user.name} (Logout)</button> : null;
}
```
Wrapping `useContext` in a custom hook (`useAuth`) that throws a clear error when used outside its Provider is the idiomatic, production-grade pattern — it gives better error messages than a silent `undefined` and centralizes the "which context" concern behind one importable hook.

### Q24. What is the main performance pitfall of Context, and how do you mitigate it?
```jsx
// PROBLEM: every consumer of AuthContext re-renders whenever ANY value in the context object changes,
// even if a specific consumer only cares about `user`, not `login`/`logout`
<AuthContext.Provider value={{ user, login, logout }}>   // a NEW object every render, if not memoized!
```
**Mitigations**:
```jsx
// 1. Memoize the context value to avoid unnecessary re-renders from new object identity
const value = useMemo(() => ({ user, login, logout }), [user]);

// 2. Split contexts by concern/update-frequency (e.g., separate UserContext and ThemeContext)
//    so a theme change doesn't re-render every component that only cares about the user.

// 3. For high-frequency updates, consider a dedicated state management library
//    (Zustand, Jotai) which supports selective/granular subscriptions out of the box.
```
Context is **not** optimized for high-frequency updates or fine-grained subscriptions by design — every consumer re-renders on any change to the Provider's value, regardless of which specific field it actually reads. This is the single most common reason experienced teams reach for a dedicated state library once an app's global state grows complex.

---

## 9. Performance Optimization

### Q25. What is `React.memo`, and when should you use it?
```jsx
const ExpensiveRow = React.memo(function ExpensiveRow({ item }) {
    console.log("Rendering row:", item.id);
    return <li>{item.name}</li>;
});
```
`React.memo` wraps a component so React **skips re-rendering it** if its props are shallowly equal to the previous render's props — useful for components that render often (due to a parent re-rendering) but whose actual props rarely change, especially "expensive" components (large lists, complex charts). It's a targeted optimization, not a default to apply everywhere — wrapping cheap components in `memo` can add overhead (the comparison itself) without meaningful benefit.

### Q26. Why do `React.memo`, `useMemo`, and `useCallback` often need to be used *together*?
```jsx
function Parent() {
    const [count, setCount] = useState(0);

    // WITHOUT useCallback: a NEW function every render -> breaks MemoizedChild's memoization
    const handleClick = () => console.log("clicked");

    // WITH useCallback: stable reference -> MemoizedChild correctly skips re-rendering
    const handleClickMemoized = useCallback(() => console.log("clicked"), []);

    return <MemoizedChild onClick={handleClickMemoized} />;
}
const MemoizedChild = React.memo(function Child({ onClick }) {
    return <button onClick={onClick}>Click</button>;
});
```
`React.memo`'s shallow comparison checks prop **reference equality** for objects/arrays/functions. If a parent passes a freshly-created function or object as a prop on every render (the default JS behavior), the child's memoization is defeated even though the "logical" value hasn't changed — `useCallback`/`useMemo` exist specifically to give these values a **stable reference** across renders when their actual dependencies haven't changed, making `React.memo` effective.

### Q27. What is code-splitting, and how do you implement it with `React.lazy` and `Suspense`?
```jsx
import { lazy, Suspense } from "react";

const Dashboard = lazy(() => import("./Dashboard"));      // loaded only when actually rendered

function App() {
    return (
        <Suspense fallback={<Spinner />}>
            <Dashboard />
        </Suspense>
    );
}
```
Code-splitting breaks your JS bundle into smaller chunks loaded on demand (rather than one giant bundle downloaded upfront), improving initial page-load performance. `React.lazy` dynamically imports a component; `Suspense` provides a fallback UI to show while that chunk is being fetched over the network.

### Q28. What causes unnecessary re-renders in React, and how do you diagnose/fix them?
Common causes: (1) a parent re-rendering causes all non-memoized children to re-render too, by default; (2) new object/array/function literals created inline as props on every render (breaking memoization); (3) Context value changes triggering every consumer to re-render (Q24); (4) state stored higher in the tree than necessary, causing broad re-render cascades for narrow changes.

**Diagnosis**: React DevTools' "Profiler" tab records renders and highlights *why* each component re-rendered. **Fixes**: `React.memo` + stable prop references (`useMemo`/`useCallback`), splitting components/state to narrow the blast radius of a given state change, and moving state as close as possible to where it's actually used (rather than lifting state higher than genuinely necessary "just in case").

### Q29. What is the "key" trick for forcing a component to fully remount/reset its state?
```jsx
function ProfilePage({ userId }) {
    return <ProfileForm key={userId} userId={userId} />;    // changing `key` forces a full remount
}
```
Since React uses `key` to determine element identity during reconciliation, deliberately changing a component's `key` (e.g., to the current `userId`) forces React to **discard the old instance entirely and mount a fresh one** — resetting all of its internal state. This is a clean way to reset a form/component's state when switching between different underlying data, without manually writing `useEffect` reset logic.

---

## 10. Refs & the DOM

### Q30. What are the main use cases for `useRef` beyond DOM access?
```jsx
function Timer() {
    const [seconds, setSeconds] = useState(0);
    const intervalRef = useRef(null);          // storing a mutable value (interval ID) across renders

    function start() {
        intervalRef.current = setInterval(() => setSeconds(s => s + 1), 1000);
    }
    function stop() {
        clearInterval(intervalRef.current);
    }

    return (
        <div>
            <p>{seconds}s</p>
            <button onClick={start}>Start</button>
            <button onClick={stop}>Stop</button>
        </div>
    );
}
```
Beyond DOM refs, `useRef` is the standard way to hold **any mutable value that should persist across renders without causing a re-render when it changes** — timer/interval IDs, a previous value for comparison, a flag to avoid a duplicate effect run, or any "instance variable" equivalent to what a class component would store on `this`.

### Q31. How do you forward a ref through a component to an underlying DOM element (`forwardRef`)?
```jsx
const FancyInput = forwardRef((props, ref) => {
    return <input className="fancy" ref={ref} {...props} />;
});

function Parent() {
    const inputRef = useRef(null);
    useEffect(() => { inputRef.current.focus(); }, []);
    return <FancyInput ref={inputRef} />;
}
```
By default, function components **cannot** receive a `ref` — attempting `<FancyInput ref={...} />` without `forwardRef` results in the ref being `null` and a console warning. `forwardRef` explicitly opts a component into accepting and forwarding a ref down to one of its own DOM elements or child components. (Note: React 19 allows passing `ref` as a regular prop directly to function components, removing the need for `forwardRef` in most new code going forward.)

### Q32. Why shouldn't you use refs to trigger UI updates?
```jsx
function BadCounter() {
    const countRef = useRef(0);
    function increment() {
        countRef.current += 1;
        // UI does NOT update - refs changing does NOT trigger a re-render!
    }
    return <button onClick={increment}>{countRef.current}</button>;   // always shows stale value 0
}
```
Mutating a ref's `.current` does **not** trigger a re-render, so the UI won't reflect the change until something else causes a re-render for an unrelated reason. If a value needs to be reflected in the rendered UI, it must be `useState` (or `useReducer`), not `useRef`.

---

## 11. Error Boundaries

### Q33. What are Error Boundaries, and why must they still be class components?
```jsx
class ErrorBoundary extends React.Component {
    constructor(props) {
        super(props);
        this.state = { hasError: false };
    }
    static getDerivedStateFromError(error) {
        return { hasError: true };
    }
    componentDidCatch(error, errorInfo) {
        console.error("Caught by ErrorBoundary:", error, errorInfo);
        logErrorToService(error, errorInfo);
    }
    render() {
        if (this.state.hasError) {
            return <h2>Something went wrong.</h2>;
        }
        return this.props.children;
    }
}

function App() {
    return (
        <ErrorBoundary>
            <RiskyComponent />
        </ErrorBoundary>
    );
}
```
Error Boundaries catch JavaScript errors thrown during rendering anywhere in their child component tree, log them, and display a fallback UI instead of crashing the entire app. As of today, **there is no Hook equivalent** (`getDerivedStateFromError`/`componentDidCatch` have no functional-component counterparts) — this remains one of the few legitimate reasons a modern, otherwise-fully-functional codebase still contains a class component, typically written once as a reusable wrapper and rarely touched again. (Libraries like `react-error-boundary` provide a pre-built, hook-friendly wrapper around this class internally.)

### Q34. What do Error Boundaries NOT catch?
Error Boundaries do **not** catch errors in: event handlers (use a regular `try`/`catch` there instead), asynchronous code (`setTimeout`, promises — again, use `try`/`catch`), server-side rendering, or errors thrown in the Error Boundary component itself. They specifically catch errors thrown during the **render phase** of their descendant components.

---

## 12. React Router

### Q35. How do you set up basic client-side routing with React Router (v6+)?
```jsx
import { BrowserRouter, Routes, Route, Link, useNavigate, useParams } from "react-router-dom";

function App() {
    return (
        <BrowserRouter>
            <nav>
                <Link to="/">Home</Link>
                <Link to="/about">About</Link>
            </nav>
            <Routes>
                <Route path="/" element={<Home />} />
                <Route path="/about" element={<About />} />
                <Route path="/users/:userId" element={<UserProfile />} />
                <Route path="*" element={<NotFound />} />     {/* catch-all 404 route */}
            </Routes>
        </BrowserRouter>
    );
}

function UserProfile() {
    const { userId } = useParams();          // reads dynamic URL segments
    const navigate = useNavigate();            // programmatic navigation
    return (
        <div>
            <p>Viewing user {userId}</p>
            <button onClick={() => navigate(-1)}>Go back</button>
        </div>
    );
}
```

### Q36. How do you implement protected/private routes?
```jsx
function ProtectedRoute({ children }) {
    const { user } = useAuth();
    if (!user) return <Navigate to="/login" replace />;
    return children;
}

<Routes>
    <Route path="/dashboard" element={<ProtectedRoute><Dashboard /></ProtectedRoute>} />
</Routes>
```
A wrapper component checks authentication state and either renders the protected content or redirects (`<Navigate>`) to a login page — a straightforward composition-based pattern rather than a special router feature.

### Q37. What is the difference between `BrowserRouter` and `HashRouter`?
`BrowserRouter` uses the HTML5 History API for clean URLs (`/about`) but requires server-side configuration to serve `index.html` for all routes (since the server must handle arbitrary paths). `HashRouter` uses the URL hash (`/#/about`) — the server only ever sees requests for the root path, making it simpler to deploy on static hosts without server-side routing configuration, at the cost of less clean URLs.

---

## 13. State Management Libraries

### Q38. When does an app need a dedicated state management library instead of just `useState`/Context?
Signs it's time: state needs to be accessed/updated from many unrelated parts of the component tree; frequent updates cause Context-related re-render performance issues (Q24); you need advanced features out of the box (time-travel debugging, middleware, persistence, selective/granular subscriptions); or the team wants a more structured, predictable pattern for a large, complex application.

### Q39. How does Redux Toolkit (modern Redux) work, in brief?
```jsx
// counterSlice.js
import { createSlice } from "@reduxjs/toolkit";

const counterSlice = createSlice({
    name: "counter",
    initialState: { value: 0 },
    reducers: {
        increment: (state) => { state.value += 1; },    // looks mutable, but uses Immer internally (safe)
        decrementBy: (state, action) => { state.value -= action.payload; },
    },
});
export const { increment, decrementBy } = counterSlice.actions;
export default counterSlice.reducer;

// Component usage
import { useSelector, useDispatch } from "react-redux";
import { increment } from "./counterSlice";

function Counter() {
    const count = useSelector(state => state.counter.value);    // selective subscription - only re-renders on relevant changes
    const dispatch = useDispatch();
    return <button onClick={() => dispatch(increment())}>{count}</button>;
}
```
Redux centralizes all application state in a single store, updated only via dispatched actions processed by pure reducer functions — providing predictability, powerful DevTools (time-travel debugging), and a well-established pattern for large teams, at the cost of more boilerplate than simpler alternatives (though Redux Toolkit significantly reduced this compared to classic Redux).

### Q40. How does Zustand compare to Redux, and why has it become popular?
```jsx
import { create } from "zustand";

const useCounterStore = create((set) => ({
    count: 0,
    increment: () => set((state) => ({ count: state.count + 1 })),
}));

function Counter() {
    const count = useCounterStore(state => state.count);    // subscribes ONLY to `count`, not the whole store
    const increment = useCounterStore(state => state.increment);
    return <button onClick={increment}>{count}</button>;
}
```
Zustand offers a much smaller API surface (no Provider wrapping required, no action-type boilerplate, no reducers) while still supporting selective subscriptions (a component only re-renders when the specific slice of state it selects changes) and middleware (persistence, devtools). Its popularity reflects a broader ecosystem trend toward lighter-weight state solutions once Redux's original problems (Context's re-render issues, need for predictable global state) were addressed more simply.

---

## 14. Older Patterns: HOCs & Render Props

### Q41. What is a Higher-Order Component (HOC), and how has it been largely superseded by Hooks?
```jsx
function withLoading(WrappedComponent) {
    return function WithLoadingComponent({ isLoading, ...props }) {
        if (isLoading) return <Spinner />;
        return <WrappedComponent {...props} />;
    };
}
const UserListWithLoading = withLoading(UserList);
```
A HOC is a function that takes a component and returns a new, enhanced component — used historically for cross-cutting concerns (loading states, auth checks, data fetching). Downsides: "wrapper hell" (deeply nested component trees in DevTools when multiple HOCs are composed), prop name collisions, and unclear prop origin (hard to trace which HOC injected which prop). A custom hook (`const { isLoading } = useLoading()`) achieves the same logic reuse far more transparently, without adding a wrapper component to the tree at all — this is why HOCs are now considered a legacy pattern for most use cases.

### Q42. What is the render props pattern, and why did Hooks largely replace it too?
```jsx
class MouseTracker extends React.Component {
    state = { x: 0, y: 0 };
    handleMouseMove = (e) => this.setState({ x: e.clientX, y: e.clientY });
    render() {
        return <div onMouseMove={this.handleMouseMove}>{this.props.render(this.state)}</div>;
    }
}
// Usage:
<MouseTracker render={({ x, y }) => <p>Mouse at {x}, {y}</p>} />
```
Render props share logic by passing a **function as a prop** that returns the JSX to render, given some internal state. Like HOCs, this works but creates nesting and can be harder to read/type (especially in TypeScript) compared to a custom hook (`const { x, y } = useMousePosition();`) that returns plain values directly usable in a component's own JSX — no wrapping component or nested function-as-child syntax required.

---

## 15. React 18/19: Concurrent Features

### Q43. What is "Concurrent Rendering," and how does it differ from React's legacy rendering model?
Before React 18, rendering was synchronous and non-interruptible once started — a large update would block the main thread entirely until finished, potentially causing jank on slow devices/large trees. **Concurrent rendering** lets React prepare multiple versions of the UI simultaneously in the background, **interrupt** a render if something more urgent comes in (e.g., a keystroke), and resume/discard work as needed — without ever showing an inconsistent, half-updated UI to the user. This is the foundational capability underlying `useTransition`, `useDeferredValue`, and improved `Suspense` behavior.

### Q44. What is automatic batching in React 18, and what changed from React 17?
```jsx
function handleClick() {
    setTimeout(() => {
        setCount(c => c + 1);      // Before React 18: caused a SEPARATE re-render (not batched, outside React events)
        setFlag(f => !f);            // React 18+: automatically BATCHED into a single re-render, even here
    }, 1000);
}
```
React 18 extends automatic batching of state updates to **all** contexts — promises, `setTimeout`, native event handlers — not just React's own synthetic event handlers as in React 17 and earlier. This reduces unnecessary re-renders by default across the board. (`flushSync()` is available as an escape hatch to force synchronous, unbatched updates in rare cases where needed.)

### Q45. What is Suspense, and how has its scope expanded from just `React.lazy`?
```jsx
<Suspense fallback={<Spinner />}>
    <ProfileDetails />         {/* can "suspend" — i.e., signal it's not ready yet — while data loads */}
</Suspense>
```
`Suspense` lets a component tree declaratively show a fallback while something inside it isn't ready — originally scoped to code-splitting (`React.lazy`), it has expanded (via frameworks like Next.js, and React's own `use()` hook) to also support **data fetching**, letting components "suspend" render until their data dependencies resolve, coordinated declaratively rather than via manual `loading` state juggling in every component.

### Q46. What are React Server Components (RSC), and how do they differ from SSR?
Server Components run **exclusively on the server**, never shipped to the client as JavaScript at all — they can directly access backend resources (databases, filesystem) and their rendered output is streamed to the client, resulting in a **zero-bundle-size** contribution for that component's logic/dependencies. This is a fundamentally different model from traditional Server-Side Rendering (SSR), where the *entire* app (including all its JS) still gets sent to and re-hydrated on the client — RSC lets you mix Server Components (zero client JS) with Client Components (`"use client"` directive, interactive, hydrated normally) within the same tree, primarily used today via frameworks built on this model (Next.js App Router).

---

## 16. Testing React Applications

### Q47. How do you test React components with React Testing Library?
```jsx
import { render, screen, fireEvent } from "@testing-library/react";
import Counter from "./Counter";

test("increments count when button is clicked", () => {
    render(<Counter />);

    const button = screen.getByRole("button", { name: /increment/i });
    fireEvent.click(button);

    expect(screen.getByText("Count: 1")).toBeInTheDocument();
});
```
React Testing Library deliberately encourages testing components the way a **user** interacts with them — querying by visible text/role/label rather than by internal implementation details (component instance state, class names) — making tests resilient to refactors that don't change actual user-facing behavior.

### Q48. How do you test asynchronous behavior (data fetching) in a component test?
```jsx
import { render, screen, waitFor } from "@testing-library/react";

test("displays fetched user name", async () => {
    render(<UserProfile userId={1} />);

    expect(screen.getByText("Loading...")).toBeInTheDocument();

    await waitFor(() => {
        expect(screen.getByText("Alice")).toBeInTheDocument();
    });
});
```
`waitFor` (or the built-in async queries like `findByText`) polls until an assertion passes or times out — necessary because state updates from asynchronous operations (fetch calls, promises) happen after the initial synchronous render.

### Q49. How do you mock API calls or custom hooks in component tests?
```jsx
jest.mock("./api", () => ({
    fetchUser: jest.fn(() => Promise.resolve({ id: 1, name: "Alice" })),
}));

// Or mock a custom hook directly
jest.mock("./useAuth", () => ({
    useAuth: () => ({ user: { name: "Alice" }, logout: jest.fn() }),
}));
```
Mocking at the module boundary (the API layer or a custom hook) keeps component tests fast and deterministic — isolated from real network calls — while still exercising the actual rendering/interaction logic of the component under test.

---

## 17. Best Practices & Common Pitfalls

### Q50. What are the most common React interview red flags/pitfalls to avoid?
- **Mutating state directly** (`state.items.push(x)` instead of `setState([...state.items, x])`) — React relies on detecting new references to know a re-render is needed.
- **Missing or incorrect `useEffect` dependencies** — causes stale closures (using an outdated variable value) or infinite loops (an object/array recreated every render, listed as a dependency).
- **Using array index as `key`** for dynamic lists (Q19).
- **Overusing `useEffect`** for logic that could be computed directly during render (derived state) instead — a very common React anti-pattern; not everything needs an effect.
- **Not cleaning up subscriptions/timers** in an effect's cleanup function, causing memory leaks or "setting state on an unmounted component" warnings.
- **Prop drilling** through many layers instead of using Context or composition (passing components as props/children) when appropriate.
- **Overusing global state** for things that are actually local/component-specific concerns.

### Q51. What is "derived state," and why should you avoid syncing it with `useState` + `useEffect`?
```jsx
// ANTI-PATTERN - syncing derived data via an extra state + effect
function ProductList({ products }) {
    const [visibleProducts, setVisibleProducts] = useState([]);
    useEffect(() => {
        setVisibleProducts(products.filter(p => p.inStock));
    }, [products]);
    // ...
}

// BETTER - compute it directly during render, no extra state/effect needed at all
function ProductList({ products }) {
    const visibleProducts = products.filter(p => p.inStock);   // or useMemo if genuinely expensive
    // ...
}
```
If a value can be **calculated directly from existing props/state during render**, it doesn't need its own `useState` + synchronizing `useEffect` — that pattern adds an unnecessary extra render cycle, a source of bugs (forgetting a dependency, stale data for one render), and unneeded complexity. The React docs explicitly call this out as one of the most common effect-related mistakes.

### Q52. How do you decide where a piece of state should live in the component tree?
Follow the principle of **minimal necessary scope**: state should live in the lowest (most deeply nested) common ancestor of all the components that need to read or write it — no higher. Placing state too high causes unrelated parts of the tree to re-render on every change; placing it too low means siblings that need the data can't access it, forcing an awkward lift-up refactor later. When state genuinely needs to be widely shared with unrelated parts of a large tree, that's the signal to reach for Context or a dedicated state library rather than lifting state all the way to the app root "just in case."

---

# Part B — Complete Theory

## 18. React Theoretical Deep Dive

### 18.1 The Rendering Pipeline: Render Phase vs Commit Phase
React's work on every update splits into two distinct phases:
- **Render phase**: React calls your component functions, builds a new Virtual DOM tree, and diffs it against the previous tree (reconciliation) to compute a list of necessary changes. This phase is **interruptible** in concurrent mode — React can pause, abandon, or restart it if higher-priority work arrives.
- **Commit phase**: React applies the computed changes to the actual DOM, runs layout effects (`useLayoutEffect`) synchronously, then paints, then runs passive effects (`useEffect`) asynchronously afterward. This phase is **always synchronous and non-interruptible** — once React starts committing, it finishes, ensuring the DOM never sits in a half-updated state visible to the user.

This split is precisely why `useEffect` (runs after commit/paint, asynchronously) and `useLayoutEffect` (runs after commit but before paint, synchronously) have their distinct timing guarantees and use cases (Q3.2 vs Q3.8).

### 18.2 Reconciliation: The Diffing Algorithm
Full tree diffing between two arbitrary trees is an O(n³) problem in the general case — far too slow for UI updates. React's reconciler uses a set of **heuristics** to reduce this to roughly O(n):
1. **Different element types produce different trees** — if a `<div>` becomes a `<span>` at the same position, React tears down the old subtree entirely and builds a new one from scratch, rather than trying to diff their children.
2. **Same element type, diff props/attributes** — React updates only the changed attributes on the existing DOM node.
3. **Lists are diffed using `key`s** — this is why keys matter so much (Q18/Q19); without stable keys, React falls back to a slower, more error-prone index-based comparison.

### 18.3 The Fiber Architecture
Since React 16, the reconciler ("React Fiber") represents each component instance as a **Fiber node** — a JavaScript object holding the component's type, props, state, its relationship to other fibers (child/sibling/return pointers forming a linked-list-like tree), and its "effect list" of pending DOM changes. This architecture is what enables:
- **Incremental rendering** — work can be split into units and processed across multiple frames instead of one giant synchronous pass.
- **Prioritization** — different updates can be assigned different priority levels (a click is more urgent than a background data refresh), which is the technical foundation for `useTransition`/`useDeferredValue`.
- **Pausing, aborting, and reusing work** — the render phase's interruptibility (Q18.1) is only possible because Fiber tracks enough state to resume or discard in-progress work cleanly.

### 18.4 How Hooks Actually Work Internally
Each function component's Fiber node holds a **linked list of hook objects**, one per hook call, in the exact order they were called. On every render, React walks this list in lockstep with your component function's hook calls — `useState`'s call reads/writes the next node in the list, `useEffect`'s call reads/writes the next node, and so on. This is precisely *why* the Rules of Hooks (Q12) exist: if hook calls are conditional, the call order shifts between renders, and React ends up reading the wrong slot's data for a given hook call — corrupting state association silently rather than throwing an obvious error in all cases.

### 18.5 Closures and Stale State: The Root Cause of Many Hook Bugs
```jsx
function Timer() {
    const [count, setCount] = useState(0);
    useEffect(() => {
        const id = setInterval(() => {
            setCount(count + 1);   // BUG: `count` is captured from the render this effect was CREATED in,
        }, 1000);                    // and never updates - the interval always adds 1 to the ORIGINAL count (0)
        return () => clearInterval(id);
    }, []);     // empty deps -> effect (and its closure over `count`) never re-created
}
```
Every render of a function component creates an entirely new closure over that render's specific props/state values. An effect (or any callback) created during one render "sees" the values as they were at that exact moment, forever, unless the effect re-runs (capturing a fresh closure) or you use the functional updater form (`setCount(c => c + 1)`, which doesn't depend on a captured value at all). Understanding this single mechanism resolves the majority of "why is my state stale" bugs in real-world Hooks code.

### 18.6 Why Hooks Were Introduced: The Problems They Solve
Before Hooks, React had two major pain points: (1) **stateful logic reuse** required HOCs or render props, both of which caused "wrapper hell" and obscured data flow (Q41/Q42); (2) **class components split related logic across lifecycle methods** (`componentDidMount` and `componentDidUpdate` might both need to run the same data-fetching code, and `componentWillUnmount` needs matching cleanup logic written far away from where the effect was set up) while unrelated logic got crammed together in the same method. Hooks solve both: custom hooks provide composable logic reuse without extra component nesting, and `useEffect` lets you colocate a piece of related setup+cleanup logic together, organized by *concern* rather than by *lifecycle phase*.

### 18.7 The Declarative Model, Formally
React's core mental model can be expressed as: **UI = f(state)**. Your component is a pure function that maps the current state (and props) to a description of the UI; React's job is entirely about efficiently computing and applying the difference between the previous UI description and the new one whenever state changes. This is why side effects (anything that isn't purely computing UI from state — data fetching, subscriptions, DOM mutation) are deliberately pulled out into `useEffect`, kept separate from the pure "render" computation — mixing side effects directly into a component's render logic breaks this model and causes hard-to-predict, timing-dependent bugs.

### 18.8 Where React Fits in the Broader Ecosystem
React is explicitly a **library**, not a framework — it handles the view layer only, and deliberately leaves routing, data fetching, and global state management as separate, swappable concerns (React Router, TanStack Query, Redux/Zustand). This is a deliberate design philosophy distinguishing React from more opinionated, all-in-one frameworks (Angular, older Vue conventions). In practice, most production React apps today are built on top of a **meta-framework** (Next.js, Remix, React Router's framework mode) that layers in routing, SSR/RSC, data loading, and build tooling on top of React's core rendering model — worth mentioning in interviews to show awareness of how React is actually used in the field, not just in isolation.

---

# Part C — Full Tutorial

## 19. Complete Tutorial: Building a Fully Functional, Hooks-Only Web App

We'll build a **Recipe Box** app — a complete, multi-page React application using **only function components and Hooks** (zero classes, except a single unavoidable Error Boundary). It demonstrates routing, Context for global state, `useReducer` for complex local state, custom hooks, memoization, forms, and a mock async API — every major concept from Part A working together.

### 19.1 Project Setup

```bash
npm create vite@latest recipe-box -- --template react
cd recipe-box
npm install react-router-dom
npm install
npm run dev
```

Project structure:
```
recipe-box/
├── src/
│   ├── main.jsx
│   ├── App.jsx
│   ├── api/
│   │   └── recipes.js            # mock async API
│   ├── context/
│   │   └── FavoritesContext.jsx    # global state via Context + useReducer
│   ├── hooks/
│   │   ├── useFetch.js              # generic data-fetching custom hook
│   │   └── useDebounce.js             # debounce custom hook
│   ├── components/
│   │   ├── ErrorBoundary.jsx           # the one unavoidable class component
│   │   ├── RecipeCard.jsx
│   │   ├── SearchBar.jsx
│   │   └── Spinner.jsx
│   └── pages/
│       ├── HomePage.jsx
│       ├── RecipeDetailPage.jsx
│       └── FavoritesPage.jsx
└── package.json
```

### 19.2 Mock API Layer

```javascript
// src/api/recipes.js
const RECIPES = [
    { id: 1, title: "Margherita Pizza", category: "Italian", time: 30, ingredients: ["Dough", "Tomato", "Mozzarella", "Basil"] },
    { id: 2, title: "Chicken Tikka Masala", category: "Indian", time: 45, ingredients: ["Chicken", "Yogurt", "Tomato", "Spices"] },
    { id: 3, title: "Beef Tacos", category: "Mexican", time: 20, ingredients: ["Beef", "Tortilla", "Cheese", "Lettuce"] },
    { id: 4, title: "Pad Thai", category: "Thai", time: 25, ingredients: ["Rice noodles", "Egg", "Peanuts", "Lime"] },
];

function delay(ms) { return new Promise(resolve => setTimeout(resolve, ms)); }

export async function fetchRecipes(searchTerm = "") {
    await delay(500);      // simulate network latency
    return RECIPES.filter(r => r.title.toLowerCase().includes(searchTerm.toLowerCase()));
}

export async function fetchRecipeById(id) {
    await delay(300);
    const recipe = RECIPES.find(r => r.id === Number(id));
    if (!recipe) throw new Error("Recipe not found");
    return recipe;
}
```

### 19.3 Custom Hooks: `useFetch` and `useDebounce`

```javascript
// src/hooks/useFetch.js
import { useState, useEffect } from "react";

export function useFetch(fetchFn, deps) {
    const [data, setData] = useState(null);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState(null);

    useEffect(() => {
        let cancelled = false;
        setLoading(true);
        setError(null);

        fetchFn()
            .then(result => { if (!cancelled) setData(result); })
            .catch(err => { if (!cancelled) setError(err.message); })
            .finally(() => { if (!cancelled) setLoading(false); });

        return () => { cancelled = true; };
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, deps);

    return { data, loading, error };
}
```

```javascript
// src/hooks/useDebounce.js
import { useState, useEffect } from "react";

export function useDebounce(value, delay = 300) {
    const [debounced, setDebounced] = useState(value);

    useEffect(() => {
        const timer = setTimeout(() => setDebounced(value), delay);
        return () => clearTimeout(timer);
    }, [value, delay]);

    return debounced;
}
```

### 19.4 Global State: Favorites via Context + `useReducer`

```jsx
// src/context/FavoritesContext.jsx
import { createContext, useContext, useReducer, useMemo } from "react";

const FavoritesContext = createContext(undefined);

function favoritesReducer(state, action) {
    switch (action.type) {
        case "TOGGLE":
            return state.includes(action.id)
                ? state.filter(id => id !== action.id)
                : [...state, action.id];
        case "CLEAR":
            return [];
        default:
            throw new Error(`Unknown action: ${action.type}`);
    }
}

export function FavoritesProvider({ children }) {
    const [favoriteIds, dispatch] = useReducer(favoritesReducer, []);

    // memoize the context value - avoids unnecessary re-renders of every consumer (see Q24)
    const value = useMemo(() => ({
        favoriteIds,
        toggleFavorite: (id) => dispatch({ type: "TOGGLE", id }),
        clearFavorites: () => dispatch({ type: "CLEAR" }),
        isFavorite: (id) => favoriteIds.includes(id),
    }), [favoriteIds]);

    return (
        <FavoritesContext.Provider value={value}>
            {children}
        </FavoritesContext.Provider>
    );
}

export function useFavorites() {
    const context = useContext(FavoritesContext);
    if (context === undefined) {
        throw new Error("useFavorites must be used within a FavoritesProvider");
    }
    return context;
}
```

### 19.5 The One Unavoidable Class: Error Boundary

```jsx
// src/components/ErrorBoundary.jsx
import { Component } from "react";

// No functional equivalent exists for getDerivedStateFromError/componentDidCatch (Q33)
export class ErrorBoundary extends Component {
    state = { hasError: false };

    static getDerivedStateFromError() {
        return { hasError: true };
    }

    componentDidCatch(error, errorInfo) {
        console.error("Uncaught error:", error, errorInfo);
    }

    render() {
        if (this.state.hasError) {
            return <p role="alert">Something went wrong. Please refresh the page.</p>;
        }
        return this.props.children;
    }
}
```

### 19.6 Reusable UI Components

```jsx
// src/components/Spinner.jsx
export function Spinner() {
    return <p aria-live="polite">Loading...</p>;
}
```

```jsx
// src/components/SearchBar.jsx
import { useState } from "react";

export function SearchBar({ onSearch }) {
    const [input, setInput] = useState("");

    function handleSubmit(event) {
        event.preventDefault();
        onSearch(input);
    }

    return (
        <form onSubmit={handleSubmit}>
            <input
                value={input}
                onChange={e => setInput(e.target.value)}
                placeholder="Search recipes..."
                aria-label="Search recipes"
            />
            <button type="submit">Search</button>
        </form>
    );
}
```

```jsx
// src/components/RecipeCard.jsx
import { memo } from "react";
import { Link } from "react-router-dom";
import { useFavorites } from "../context/FavoritesContext";

// React.memo - avoids re-rendering every card when unrelated state (e.g. search input) changes
export const RecipeCard = memo(function RecipeCard({ recipe }) {
    const { isFavorite, toggleFavorite } = useFavorites();
    const favorite = isFavorite(recipe.id);

    return (
        <div className="recipe-card">
            <h3>
                <Link to={`/recipes/${recipe.id}`}>{recipe.title}</Link>
            </h3>
            <p>{recipe.category} · {recipe.time} min</p>
            <button onClick={() => toggleFavorite(recipe.id)}>
                {favorite ? "★ Favorited" : "☆ Favorite"}
            </button>
        </div>
    );
});
```

### 19.7 Pages

```jsx
// src/pages/HomePage.jsx
import { useState, useCallback, useMemo } from "react";
import { fetchRecipes } from "../api/recipes";
import { useFetch } from "../hooks/useFetch";
import { useDebounce } from "../hooks/useDebounce";
import { SearchBar } from "../components/SearchBar";
import { RecipeCard } from "../components/RecipeCard";
import { Spinner } from "../components/Spinner";

export function HomePage() {
    const [searchTerm, setSearchTerm] = useState("");
    const debouncedSearch = useDebounce(searchTerm, 300);

    const fetchFn = useCallback(() => fetchRecipes(debouncedSearch), [debouncedSearch]);
    const { data: recipes, loading, error } = useFetch(fetchFn, [debouncedSearch]);

    const sortedRecipes = useMemo(
        () => recipes ? [...recipes].sort((a, b) => a.time - b.time) : [],
        [recipes]
    );

    return (
        <div>
            <h1>Recipe Box</h1>
            <SearchBar onSearch={setSearchTerm} />

            {loading && <Spinner />}
            {error && <p role="alert">Error: {error}</p>}
            {!loading && !error && sortedRecipes.length === 0 && <p>No recipes found.</p>}

            <div className="recipe-grid">
                {sortedRecipes.map(recipe => (
                    <RecipeCard key={recipe.id} recipe={recipe} />
                ))}
            </div>
        </div>
    );
}
```

```jsx
// src/pages/RecipeDetailPage.jsx
import { useCallback } from "react";
import { useParams, Link } from "react-router-dom";
import { fetchRecipeById } from "../api/recipes";
import { useFetch } from "../hooks/useFetch";
import { useFavorites } from "../context/FavoritesContext";
import { Spinner } from "../components/Spinner";

export function RecipeDetailPage() {
    const { id } = useParams();
    const fetchFn = useCallback(() => fetchRecipeById(id), [id]);
    const { data: recipe, loading, error } = useFetch(fetchFn, [id]);
    const { isFavorite, toggleFavorite } = useFavorites();

    if (loading) return <Spinner />;
    if (error) return <p role="alert">{error}</p>;

    return (
        <div>
            <Link to="/">&larr; Back to all recipes</Link>
            <h1>{recipe.title}</h1>
            <p>{recipe.category} · {recipe.time} minutes</p>
            <button onClick={() => toggleFavorite(recipe.id)}>
                {isFavorite(recipe.id) ? "★ Remove favorite" : "☆ Add favorite"}
            </button>
            <h2>Ingredients</h2>
            <ul>
                {recipe.ingredients.map(ingredient => <li key={ingredient}>{ingredient}</li>)}
            </ul>
        </div>
    );
}
```

```jsx
// src/pages/FavoritesPage.jsx
import { useCallback } from "react";
import { fetchRecipes } from "../api/recipes";
import { useFetch } from "../hooks/useFetch";
import { useFavorites } from "../context/FavoritesContext";
import { RecipeCard } from "../components/RecipeCard";
import { Spinner } from "../components/Spinner";

export function FavoritesPage() {
    const { favoriteIds, clearFavorites } = useFavorites();
    const fetchFn = useCallback(() => fetchRecipes(), []);
    const { data: allRecipes, loading } = useFetch(fetchFn, []);

    if (loading) return <Spinner />;

    const favoriteRecipes = allRecipes.filter(r => favoriteIds.includes(r.id));

    return (
        <div>
            <h1>Favorites</h1>
            {favoriteRecipes.length === 0 ? (
                <p>No favorites yet — go star some recipes!</p>
            ) : (
                <>
                    <button onClick={clearFavorites}>Clear all favorites</button>
                    <div className="recipe-grid">
                        {favoriteRecipes.map(recipe => <RecipeCard key={recipe.id} recipe={recipe} />)}
                    </div>
                </>
            )}
        </div>
    );
}
```

### 19.8 Wiring It All Together

```jsx
// src/App.jsx
import { BrowserRouter, Routes, Route, Link, NavLink } from "react-router-dom";
import { FavoritesProvider } from "./context/FavoritesContext";
import { ErrorBoundary } from "./components/ErrorBoundary";
import { HomePage } from "./pages/HomePage";
import { RecipeDetailPage } from "./pages/RecipeDetailPage";
import { FavoritesPage } from "./pages/FavoritesPage";

function App() {
    return (
        <ErrorBoundary>
            <FavoritesProvider>
                <BrowserRouter>
                    <nav>
                        <NavLink to="/" end>Home</NavLink>
                        <NavLink to="/favorites">Favorites</NavLink>
                    </nav>
                    <Routes>
                        <Route path="/" element={<HomePage />} />
                        <Route path="/recipes/:id" element={<RecipeDetailPage />} />
                        <Route path="/favorites" element={<FavoritesPage />} />
                        <Route path="*" element={<p>Page not found. <Link to="/">Go home</Link></p>} />
                    </Routes>
                </BrowserRouter>
            </FavoritesProvider>
        </ErrorBoundary>
    );
}

export default App;
```

```jsx
// src/main.jsx
import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./App.jsx";

createRoot(document.getElementById("root")).render(
    <StrictMode>
        <App />
    </StrictMode>
);
```

### 19.9 Running the App

```bash
npm run dev
# Visit http://localhost:5173
```
Try: searching recipes (debounced so it doesn't re-fetch on every keystroke), clicking into a recipe's detail page, favoriting recipes, and navigating to the Favorites page — all state (favorites) persists across navigation because it lives in Context above the router.

### 19.10 What This Tutorial Demonstrates (Mapping Back to the Hooks & Concepts Above)

| Hook / Concept | Where it's used |
|---|---|
| `useState` | `SearchBar`'s input value, `HomePage`'s search term |
| `useEffect` | Inside `useFetch` and `useDebounce` (data fetching + cleanup, debounce timer) |
| `useContext` (via `useFavorites`) | Every component reading/toggling favorites |
| `useReducer` | `FavoritesProvider`'s `favoritesReducer` — action-based state transitions |
| `useMemo` | Memoizing the Context value and the sorted recipe list |
| `useCallback` | Stabilizing `fetchFn` references passed into `useFetch` across renders |
| `React.memo` | `RecipeCard`, to avoid re-rendering every card on unrelated search-input changes |
| Custom hooks | `useFetch`, `useDebounce` — both fully reusable, generic, composable |
| `useParams`/`useNavigate` (React Router) | `RecipeDetailPage` reading the `:id` URL segment |
| Error Boundary (the one class) | Wraps the entire app, catching any uncaught render errors |
| Controlled forms | `SearchBar`'s input |
| Conditional rendering & keys | Loading/error/empty states; `key={recipe.id}` on every mapped list |
| Component composition | `<FavoritesProvider><BrowserRouter>{children}</BrowserRouter></FavoritesProvider>` |

### 19.11 Taking It Further (Production Checklist)

1. **Replace the mock API** with real network calls, and consider **TanStack Query** (React Query) instead of hand-rolled `useFetch` — it adds caching, request deduplication, background refetching, and pagination out of the box.
2. **Persist favorites** to `localStorage` (via a `useEffect` syncing on change, or a custom `useLocalStorage` hook as shown in Section 4) or a real backend, so favorites survive a page reload.
3. **Add `React.lazy` + `Suspense`** to code-split `RecipeDetailPage` and `FavoritesPage`, so the initial bundle only includes `HomePage`.
4. **Add tests** with React Testing Library for `SearchBar` (typing + debounce timing with fake timers), `RecipeCard` (favorite toggling), and the routing flow.
5. **Add TypeScript** to strongly type the `Recipe` shape, `useFetch<T>`'s generic return type, and Context values — see the TypeScript guide's React section for the exact patterns.
6. **Consider `useTransition`** around the search-triggered re-fetch if the recipe list grows very large, to keep the search input maximally responsive.
7. **Migrate to a meta-framework** (Next.js/Remix) if server-side rendering, Server Components, or file-based routing become valuable as the app grows.

This tutorial deliberately uses **every category of hook** covered in Part A (state, effects, context, reducer, memoization, refs indirectly via React Router's internals) inside one small, coherent, fully runnable app — demonstrating the complete "functional way" of building React applications with no class components beyond the single unavoidable Error Boundary.
