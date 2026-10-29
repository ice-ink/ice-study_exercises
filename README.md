# ice-study_exercises

A collection of coding exercises for ongoing programming practice and software engineering interview preparation.

## Purpose

This repository serves as a staging area for practicing coding exercises, primarily generated with the assistance of LLMs.

The main practice languages are **TypeScript** and **Rust**. **PHP** and **Python** are also included as lower-priority languages, and there is a separate **React** (TypeScript/JSX) component-exercise track.

Exercises are organized by difficulty:

- **Easy** — Fundamental programming concepts and straightforward problems.
- **Medium** — More involved algorithms, data structures, and problem-solving.
- **Hard** — Challenging problems requiring more advanced reasoning.

The goal is to attempt each exercise independently, test the implementation, review alternative solutions, and improve programming skills over time.

## Repository Structure

```text
.
├── exercises/                  # blank worksheets — write your solutions here
│   ├── typescript/
│   │   ├── easy/
│   │   ├── medium/
│   │   └── hard/
│   ├── php/
│   │   ├── easy/
│   │   ├── medium/
│   │   └── hard/
│   ├── python/
│   │   ├── easy/
│   │   ├── medium/
│   │   └── hard/
│   ├── react/                  # component exercises (no difficulty tiers)
│   └── rust/
│       ├── Cargo.toml          # worksheet workspace manifest
│       ├── Cargo.lock
│       ├── easy/
│       ├── medium/
│       └── hard/
├── solutions/                  # reference solutions (fully implemented, tested)
│   ├── typescript/
│   │   ├── easy/
│   │   ├── medium/
│   │   └── hard/
│   ├── php/
│   │   ├── easy/
│   │   ├── medium/
│   │   └── hard/
│   ├── python/
│   │   ├── easy/
│   │   ├── medium/
│   │   └── hard/
│   ├── react/                  # reference component implementations
│   └── rust/
│       ├── Cargo.toml          # solutions workspace manifest
│       ├── Cargo.lock
│       ├── easy/
│       ├── medium/
│       └── hard/
├── problems/                   # problem/solution/syntax documents
│   ├── README.md               # catalog (indexed by difficulty and topic)
│   └── ...
├── learning/
│   ├── problem-solving-guide.md
│   └── syntax-guide.md
├── package.json
├── pnpm-lock.yaml
├── pnpm-workspace.yaml
├── tsconfig.json
├── .gitignore
└── README.md
```

Two directories do two different jobs:

- **`exercises/`** — blank **worksheets**. Each file is a stub (a function signature with a `TODO`). This is where you write your own solution.
- **`solutions/`** — **reference solutions**. Fully implemented, tested, and runnable. Compare your work against these after you attempt a problem.
- **React** exercises use a *component + test* pair rather than a single script (see [React](#react)).

Every problem is documented in [`problems/`](problems/README.md) with the statement, a reference solution, and a syntax explanation. The directory tree above is illustrative; it grows as problems are added.

## Problem Documents

Every documented exercise lives in [`problems/`](problems/README.md) as a self-contained markdown file with three sections:

- **Problem** — statement, examples, constraints, and prompts to think through before coding.
- **Solution** — approach, TypeScript and Rust implementations, and likely interviewer follow-ups.
- **Syntax Explanation** — the idea in plain steps, each language explained piece by piece, a worked trace, and a TypeScript↔Rust comparison table.

See the [problems catalog](problems/README.md) for the full, indexed list grouped by difficulty and topic.

For guidance on *how to think about and prepare for* these problems, see the [learning guide](learning/problem-solving-guide.md). For the TypeScript and Rust syntax each difficulty tier assumes — from `Map` lookups at Easy to `VecDeque` and binary-search invariants at Hard — see the [syntax guide](learning/syntax-guide.md).

## Prerequisites

Install the following tools:

- [Node.js](https://nodejs.org/)
- [pnpm](https://pnpm.io/)
- [Rust and Cargo](https://www.rust-lang.org/tools/install)

Run commands from the repository root unless otherwise specified.

## Running the code

Run **every** command below from the **repository root** — the directory that contains `package.json`, `exercises/`, and `solutions/`. The relative paths in these commands are all relative to the root, so running them from a subdirectory will not find the files.

| Language | Run a worksheet (`exercises/`) | Run a reference solution (`solutions/`) | Check / test |
|---|---|---|---|
| TypeScript | `pnpm exec tsx exercises/typescript/easy/two_sum.ts` | `pnpm exec tsx solutions/typescript/easy/two_sum.ts` | `pnpm exec tsc --noEmit` |
| Rust | `cargo run --manifest-path exercises/rust/Cargo.toml -p two-sum` | `cargo run --manifest-path solutions/rust/Cargo.toml -p two-sum` | `cargo test --manifest-path solutions/rust/Cargo.toml` |
| PHP | `php exercises/php/easy/two_sum.php` | `php solutions/php/easy/two_sum.php` | `php -l solutions/php/easy/two_sum.php` |
| Python | `python exercises/python/easy/two_sum.py` | `python solutions/python/easy/two_sum.py` | `python -m py_compile solutions/python/easy/two_sum.py` |
| React | `pnpm exec vitest run exercises/react/counter` | `pnpm exec vitest run solutions/react/counter` | `pnpm test` |

- Substitute `two_sum` (TypeScript / PHP / Python) or `two-sum` (Rust) with the problem you are working on.
- `exercises/` holds the blank worksheets; `solutions/` holds the completed reference implementations.
- TypeScript needs its dependencies installed first — run `pnpm install` once (see [TypeScript](#typescript)).
- Rust package names are hyphenated (`two-sum`), matching the `name` field in each package's `Cargo.toml`; the other languages use the underscore file name (`two_sum`).
- React exercises are component + test pairs verified with Vitest (see [React](#react)).
- Per-language details are in the sections below.

## TypeScript

TypeScript dependencies and configuration are managed from the repository root. Individual worksheets and solutions do not need their own `package.json` or TypeScript installation.

Each TypeScript worksheet is a standalone `.ts` stub; each TypeScript solution is a standalone `.ts` file with the completed implementation and quick checks.

### Initial setup

Install the project dependencies:

```powershell
pnpm install
```

If the TypeScript tooling has not yet been installed, run:

```powershell
pnpm add -D typescript tsx @types/node
```

If `tsconfig.json` does not exist, initialize it:

```powershell
pnpm exec tsc --init
```

These initialization commands only need to be run when setting up or repairing the project, not for each new exercise.

### Running a worksheet

Run your worksheet with `tsx`, from the repository root:

```powershell
pnpm exec tsx exercises/typescript/easy/two_sum.ts
```

### Running a reference solution

Run the corresponding reference solution the same way, from the repository root:

```powershell
pnpm exec tsx solutions/typescript/easy/two_sum.ts
```

Replace the path with the problem you are working on.

### Type-checking

Check the whole TypeScript project — worksheets and solutions alike — for type errors without generating JavaScript files:

```powershell
pnpm exec tsc --noEmit
```

Ensure `tsconfig.json` includes the exercise directories and appropriate compiler options.

## Rust

Rust code is split across two Cargo workspaces:

- **Worksheets:** `exercises/rust/Cargo.toml` — blank stubs you fill in.
- **Solutions:** `solutions/rust/Cargo.toml` — completed, tested reference implementations.

Each exercise is a separate Cargo package containing its own `Cargo.toml` and `src/main.rs`, so each can be compiled, run, and tested independently. The two workspaces share the same package names, so the same `-p` flag works for either.

### Workspace configuration

Each workspace manifest uses an **explicit list of members**. For example, `exercises/rust/Cargo.toml` may contain:

```toml
[workspace]
resolver = "3"
members = [
    "easy/valid-brackets",
    "easy/two-sum",
]
```

Each entry corresponds to a package directory relative to that workspace's root. The package name used with Cargo is defined by the package's own `Cargo.toml`.

When a new exercise is created, its worksheet directory must be added to `exercises/rust/Cargo.toml`, and its solution directory (once written) to `solutions/rust/Cargo.toml`.

### Creating a new exercise

Create a Cargo package inside the appropriate difficulty directory of each workspace. For example:

```powershell
cargo new exercises/rust/easy/reverse-string
```

This generates a package similar to:

```text
exercises/rust/easy/reverse-string/
├── Cargo.toml
└── src/
    └── main.rs
```

Next, add the package path to `exercises/rust/Cargo.toml` (and, when you write the solution, to `solutions/rust/Cargo.toml`):

```toml
[workspace]
resolver = "3"
members = [
    "easy/valid-brackets",
    "easy/two-sum",
    "easy/reverse-string",
]
```

Keep the existing members in the list when adding a new one. Do not replace them with only the newest exercise.

### Running a worksheet or solution

From the repository root, run a Rust package by specifying the workspace manifest and package name:

```powershell
# Your worksheet (blank stub):
cargo run --manifest-path exercises/rust/Cargo.toml -p valid-brackets

# The reference solution:
cargo run --manifest-path solutions/rust/Cargo.toml -p valid-brackets
```

The `-p` argument identifies the package by the `name` field in that exercise's `Cargo.toml`.

### Other useful Cargo commands

Compile a package without running it, and run its tests:

```powershell
cargo check --manifest-path solutions/rust/Cargo.toml -p valid-brackets
cargo test  --manifest-path solutions/rust/Cargo.toml -p valid-brackets
```

Run the full test suite for either workspace:

```powershell
cargo test --manifest-path exercises/rust/Cargo.toml
cargo test --manifest-path solutions/rust/Cargo.toml
```

Check that Cargo can load a workspace:

```powershell
cargo metadata --manifest-path solutions/rust/Cargo.toml --no-deps
```

Replace `valid-brackets` with the relevant package name when working on a different exercise.

## PHP

PHP is a lower-priority third language. Worksheets live in `exercises/php/` and reference solutions in `solutions/php/`, organized by difficulty as standalone `.php` scripts (no package manifest).

Run a PHP worksheet or solution with the `php` CLI, from the repository root:

```powershell
php exercises/php/easy/two_sum.php
php solutions/php/easy/two_sum.php
```

Syntax-check a file with:

```powershell
php -l solutions/php/easy/two_sum.php
```

PHP is included for completeness but is not the focus of the problem documents, which cover TypeScript and Rust.

## Python

Python is a lower-priority language, like PHP. Worksheets live in `exercises/python/` and reference solutions in `solutions/python/`, organized by difficulty as standalone `.py` scripts (no package manifest).

Run a Python worksheet or solution with the `python` CLI, from the repository root:

```powershell
python exercises/python/easy/two_sum.py
python solutions/python/easy/two_sum.py
```

Syntax-check a file with:

```powershell
python -m py_compile solutions/python/easy/two_sum.py
```

Python is included for completeness but is not the focus of the problem documents, which cover TypeScript and Rust.

## React

React is a separate **component-exercise track** (TypeScript JSX), not part of the algorithm problems. Each exercise is a pair of files — a component and its test — where the test is the executable spec.

| | Location |
|---|---|
| Worksheets | `exercises/react/<exercise>/` |
| Reference solutions | `solutions/react/<exercise>/` |

Each exercise directory contains:

- `Component.tsx` — the component (a stub in `exercises/`, implemented in `solutions/`).
- `Component.test.tsx` — the tests (identical in both places).

Run the tests for one exercise from the repository root:

```powershell
# Your worksheet (expected to fail until you implement it):
pnpm exec vitest run exercises/react/counter

# The reference solution (should pass):
pnpm exec vitest run solutions/react/counter
```

Run the whole React reference-solution suite, or type-check everything:

```powershell
pnpm test
pnpm exec tsc --noEmit
```

The initial exercises cover `counter`, `greeting`, `toggle`, `todo-list`, `controlled-form`, `accordion`, `fetch-user`, `search-filter`, `debounced-search`, `theme-context`, `counter-reducer`, and `tic-tac-toe`. See [`exercises/react/README.md`](exercises/react/README.md) for the full list.

## Working with Exercises

A typical practice session follows this workflow:

1. Select an exercise from the [problems catalog](problems/README.md) appropriate to the desired difficulty.
2. Read the **Problem** section and identify the expected inputs, outputs, and constraints.
3. Implement a solution independently in the matching blank worksheet under `exercises/`.
4. Run your worksheet (from the repository root) against representative examples and edge cases.
5. Compare your work against the reference solution in `solutions/` and the doc's **Solution** and **Syntax Explanation** sections.
6. Review correctness, readability, and time and space complexity, and record useful observations.

## Conventions

- Organize worksheets and solutions by language and difficulty.
- Use descriptive, consistent names across languages (snake_case for docs and TypeScript files, kebab-case for Rust packages).
- Prefer one self-contained TypeScript file per exercise.
- Use one Cargo package per Rust exercise.
- PHP and Python exercises are standalone scripts (`.php` / `.py`) with no package manifest.
- React exercises are component + test pairs under `exercises/react/` and `solutions/react/`.
- Keep `exercises/` (worksheets) and `solutions/` (reference) in sync: same names, same signatures.
- Run commands from the repository root.
- Keep `package.json`, lockfiles, Cargo manifests, and other project configuration files under version control.
- Exclude generated output and dependency directories such as `node_modules/`, `dist/`, and Rust's `target/` from version control.

## Future Expansion

Other languages may be added in the future using a similar language-and-difficulty organization.


test