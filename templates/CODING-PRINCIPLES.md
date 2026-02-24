# Coding Principles

These principles apply to **all code** in this project. Read and apply them
before writing or reviewing any code.

---

## Length of Coding Files

Each coding file should be a maximum of 750 lines with a grace of 50 lines,
including comments. If a file exceeds 750 lines, split it into modules and
import them into a central file.

In Nix: extract repeated patterns into a module under `modules/` and import it.
In Rust: extract into a submodule file and `pub use` from the parent.

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
_"Show me your flowcharts and conceal your tables, and I shall continue to be
mystified. Show me your tables, and I won't usually need your flowcharts;
they'll be obvious."_
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
- Avoid excessive indentation: deep nesting makes code hard to read.

**Rule 4 — Code structure and style**
Avoid multiple assignments on a single line. One action per statement.

**Rule 5 — Favour stability over complexity**
Doing something clever is not a virtue. Stability and predictability matter more
than doing something cool.

**Rule 6 — The bad code principle**
Make it work, then make it better. Don't over-optimise — get it working first,
then optimise. All code should be maintainable by anyone, not just yourself.

---

## Error Handling

Prefer explicit error handling over silent failures. Never swallow an error
without logging it — silent failures are the hardest class of bug to diagnose.

- Use custom error types over generic ones. An error that says `ConfigLoadError`
  with a path and reason is more actionable than a bare `std::io::Error`.
- Every error message should answer three questions: **what** went wrong,
  **why** it happened, and **what to do** about it.
- In Rust, propagate errors with `?` and attach context at the caller boundary
  using `context()` / `with_context()` from `anyhow` or `thiserror`. Never
  lose the original error — wrap it, don't replace it.
- In Nix, surface configuration errors during evaluation rather than allowing
  them to fail silently at activation or service startup time. Use
  `lib.mkAssert` or `lib.mkIf (lib.assertMsg ...)` for invariant checks.
- Do not return `Option` where an error is the more honest type. Use `Result`
  when the absence of a value is unexpected or requires explanation.

---

## Naming Conventions

Beyond Linus's "descriptive but concise" rule, follow these concrete
conventions across all languages in this project:

- **Booleans** read as questions: `is_active`, `has_secret`, `can_deploy`.
- **Functions** are verbs: `build_config`, `validate_module`, `rotate_key`.
- **Avoid abbreviations** unless universally understood (`url`, `id`, `cfg`,
  `nix` are acceptable; `usr`, `mgr`, `svc` are not).
- **No single-letter variables** except in tight loops (`i`, `j`) or clear
  mathematical contexts.
- **Nix attribute names** follow `camelCase` for module options and
  `kebab-case` for service names, secret names, and package names — matching
  NixOS and nixpkgs conventions.
- **Rust** follows `snake_case` for variables, functions, and modules;
  `PascalCase` for types and traits; `SCREAMING_SNAKE_CASE` for constants.
- **NixOS hostnames and Wireguard peer names** use `kebab-case` consistent
  with the naming conventions in CLAUDE.md.

---

## Testing

Every public function, module, and NixOS service requires tests. See
**[TESTING.md](TESTING.md)** for the full testing guide, patterns, and examples
adapted to this project's stack (Rust and NixOS).

Summary of requirements:

- Every public Rust function has at least one unit test.
- Every Rust CLI command has integration tests covering the happy path, error
  paths, and invalid argument cases.
- Every new NixOS module has a `nix flake check` assertion or a `nixosTest`
  smoke check verifying the module evaluates cleanly.
- Tests are independent — no test relies on another having run first.
- Test names describe the scenario: `test_vault_connect_fails_with_bad_token`
  not `test_vault_2`.

---

## Comments and Documentation

Comments explain **why**, not **what**. If code needs a comment to explain what
it does, rewrite the code to be clearer instead.

- **Docstrings** are mandatory on all public APIs. In Rust, use `///` doc
  comments. In Nix, document every module option with `description`,
  `type`, and default behaviour using `lib.mdDoc "..."`.
- **TODO comments** must include a name or ticket reference:
  `# TODO(sam): remove after STORY-042 deploys`.
- **NixOS module option descriptions** explain the option's purpose, accepted
  values, and any operational notes.
- Avoid commented-out code in committed files. Delete it; git history is the
  recovery mechanism.
- Multi-line block comments in Nix explain architectural decisions or
  operational constraints — not restate the configuration in prose.

---

## Security

- **Never hardcode** secrets, API keys, or credentials in any file committed to
  this repository. All secrets live in Hashicorp Vault and are encrypted with
  agenix for declarative NixOS secret management.
- **Always validate and sanitise** user input at system boundaries. Assume all
  external input is hostile until proven otherwise.
- **Principle of least privilege**: every service, user, container, and token
  has only the permissions it needs and nothing more.
- **Pin all dependencies** explicitly. In Rust: `Cargo.lock`. In Nix:
  `flake.lock`. Unpinned dependencies are a supply-chain risk.
- See **[SECURITY.md](SECURITY.md)** for detailed security patterns and the
  compliance checklist relevant to NixOS, Wireguard, Vault, and Vaultwarden.

---

## Dependencies

Don't add a dependency for something you can write correctly in under 50 lines.
Before adding any dependency, answer all five questions:

1. Can this be implemented simply without it? If yes, write it.
2. Is it actively maintained? (Recent commits, issues acknowledged and resolved)
3. Does it have a clean security track record? (Check CVE databases)
4. Is the licence compatible? (MIT, Apache 2.0, MPL 2.0 are acceptable; GPL
   requires careful review)
5. Is the version pinned explicitly? Never use unbounded version ranges.

In Rust, pin in `Cargo.toml` with a specific version. In Nix, all inputs are
pinned via `flake.lock` — run `nix flake update` deliberately, never as part
of an automated process without review.

---

## Git and Version Control

- **Atomic commits**: each commit does exactly one thing. Mixed concerns belong
  in separate commits.
- **Conventional Commits format**: `feat:`, `fix:`, `refactor:`, `docs:`,
  `chore:`. Subject line under 72 characters. Body explains the reasoning and
  context, not the diff.
- **Never commit** generated files, secrets, `.env` files, `flake.lock`
  changes without review, or `result` symlinks.
- **Branch naming** follows `<story-id>/<short-description>`:
  `us042/wireguard-mullvad`.
- **Pull requests** require a description explaining what changed and why,
  with a reference to the story ID.
- Force-push is only permitted on personal feature branches before a PR is
  opened, never after.

---

## Code Review Checklist

Before submitting code for review or marking a task complete, verify:

- [ ] Errors are handled explicitly — no silent failures or unchecked `unwrap()`
- [ ] All public functions and module options have tests
- [ ] Test names describe the scenario being tested
- [ ] The code follows existing patterns in the codebase
- [ ] A stranger could understand this code in six months without context
- [ ] No secrets, credentials, or API keys are present in the diff
- [ ] No new dependency was added without evaluation (see Dependencies above)
- [ ] Every modified file stays within the 750-line limit
- [ ] `nix flake check` passes without errors or warnings
- [ ] `cargo clippy -- -D warnings` passes for all Rust crates
- [ ] Relevant documentation has been updated (CLAUDE.md, DEVELOPMENT.md, etc.)

---

## DRY vs WET — The Rule of Three

Don't abstract prematurely. Duplication is acceptable the first and second time
you write something. On the **third occurrence**, refactor into a shared
abstraction.

The wrong abstraction is worse than duplication: a premature abstraction forces
every future use into a shape that doesn't quite fit. Three clear, slightly
repetitive implementations are preferable to one clever abstraction that
obscures intent.

In Nix, extract a reusable module when a pattern appears in three or more
service configurations. In Rust, extract a shared function or trait when the
same logic appears in three or more places.

---

## Logging

Log at the appropriate level for the audience and severity:

| Level     | Use for                                                        |
| --------- | -------------------------------------------------------------- |
| `DEBUG`   | Development detail — Nix evaluation paths, Vault request details |
| `INFO`    | Significant state changes — module built, secret rotated       |
| `WARNING` | Recoverable issues — retry attempted, config fallback used     |
| `ERROR`   | Failures requiring attention — deploy failed, Vault unreachable |

Rules:

- Include enough context to diagnose the issue without re-running: include
  hostnames, paths, module names, and relevant values alongside the error.
- Never log sensitive data: Vault tokens, private keys, Wireguard PSKs, or PII.
- **Structured logging (JSON)** is preferred over free-text in production Rust
  tools. Use the `tracing` crate with a JSON subscriber.
- In NixOS services, direct logs to `journald`. Do not write to flat files
  unless the service specifically requires it.
- Log at `ERROR` when an error propagates to the top of the call stack
  unhandled. Log at `WARNING` or `DEBUG` when it is caught and recovered from.
