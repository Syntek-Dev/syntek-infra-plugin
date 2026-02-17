# Coding Principles

These principles apply to **all code** in this project. Read and apply them
before writing or reviewing any code.

---

## Rob Pike's 5 Rules of Programming

Rob Pike is the co-creator of Go. These rules govern when and how to optimise.

**Rule 1 — Don't guess where the bottleneck is**
You can't tell where a programme is going to spend its time. Bottlenecks occur
in surprising places, so don't try to second-guess and put in a speed hack until
you know that is where the bottleneck is.

**Rule 2 — Measure before you tune**
Don't tune for speed until you've measured, and even then don't unless one part
of the code overwhelms the rest.

**Rule 3 — Fancy algorithms are slow when N is small**
N is usually small. Fancy algorithms have big constants. Until you know that N
is frequently going to be large, don't get fancy. Even if N does get large, use
Rule 2 first.

**Rule 4 — Fancy algorithms are buggier**
Fancy algorithms are harder to implement. Use simple, reusable, easy-to-maintain
algorithms and simple data structures.

**Rule 5 — Data dominates**
If you have chosen the right data structures and organised things well, the
algorithms will almost always be self-evident. Data structures are central to
programming — not algorithms.

---

## Linus Torvalds' Coding Principles

Derived from Linus Torvalds' coding style, talks, and mailing list
contributions. Focused on efficiency, simplicity, readability, and a deep
understanding of data structures.

**Rule 1 — Data structures over algorithms**
*"Show me your flowcharts and conceal your tables, and I shall continue to be
mystified. Show me your tables, and I won't usually need your flowcharts;
they'll be obvious."*
Focus on how data is organised — the code (logic) will naturally follow. A
solid data model often eliminates the need for complex, messy code.

**Rule 2 — "Good taste" in coding**
- Remove special cases: good code eliminates edge cases rather than adding `if`
  statements for them.
- Simplify logic: avoid tricky expressions or complex, nested control flows.
- Reduce branches: fewer conditional statements make code faster (CPU branch
  prediction) and easier to reason about.

**Rule 3 — Readability and maintainability**
- Short functions: functions do one thing, are short, and fit on one or two
  screenfuls of text.
- Descriptive names: variables and functions should be descriptive but concise.
- Avoid excessive indentation: deep nesting makes code hard to read, especially
  after looking at it for 20 hours.

**Rule 4 — Code structure and style**
Avoid multiple assignments on a single line. One action per statement.

**Rule 5 — Favour stability over complexity**
Doing something clever is not a virtue. Stability and predictability matter more
than doing something cool.

**Rule 6 — The bad code principle**
Make it work, then make it better. Don't over-optimise — get it working first,
then optimise. All code should be maintainable by anyone, not just yourself.
