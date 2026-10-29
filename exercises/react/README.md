# React exercises

A component-exercise track in TypeScript JSX (React 19). Each exercise is a pair of files — the component and its test — where the test is the executable spec.

## Layout

- `exercises/react/<exercise>/` — blank worksheet: a stub component + the spec test.
- `solutions/react/<exercise>/` — reference solution: the implemented component + the same spec test.

Each directory contains `Component.tsx` and `Component.test.tsx` (for example `counter/Counter.tsx` and `counter/Counter.test.tsx`).

## How to work through one

From the repository root, run the worksheet's test and watch it fail, then implement until green:

```powershell
pnpm exec vitest run exercises/react/counter
```

Compare with the reference solution and confirm it passes:

```powershell
pnpm exec vitest run solutions/react/counter
```

Run the whole reference suite or type-check everything:

```powershell
pnpm test
pnpm exec tsc --noEmit
```

## Exercise list

| Exercise | Component | Concepts |
|---|---|---|
| counter | `Counter` | `useState`, event handlers |
| greeting | `Greeting` | props |
| toggle | `Toggle` | conditional rendering |
| todo-list | `TodoList` | lists, keys, empty state |
| controlled-form | `ControlledForm` | controlled inputs, form submit |
| accordion | `Accordion` | state, expand/collapse |
| fetch-user | `FetchUser` | `useEffect`, async, loading/error |
| search-filter | `SearchFilter` | derived state, `useMemo` |
| debounced-search | `DebouncedSearch` | effect cleanup, timers |
| theme-context | `ThemeToggle` | context, `useContext` |
| counter-reducer | `CounterReducer` | `useReducer` |
| tic-tac-toe | `TicTacToe` | composition, win logic |

Suggested order: top to bottom.
