---
agent: 'agent'
description: 'Generate comprehensive pull request descriptions based on code changes and git diff'
---

# Task

As an agent, your task is to compare this branch to the target branch (typically `main` or `staging`) and generate a PR description with the PR template filled.

Please note:

1. You should just fill the PR template, don't generate any kind of scripts to do that.
2. Please don't try to fetch remote, just use local info.
3. Generate the md doc in the `.github/docs/temp` folder if requested to save to file.

## Instructions

### Input

- The changes compared to the target branch
- The actual code changes (git diff)
- The repository context (optional)

### Output Format

The output should be compatible with the repository's Default PR template located at `.github/PULL_REQUEST_TEMPLATE/default.md`. Note that Release PRs should be written by humans and not auto-generated.

IMPORTANT: The output should be in markdown format for the user to directly copy/paste into the PR description on GitHub.
It should not be formatted in the chat. Add "```" before and after the output.

#### Template Format

```text
# 🚀 Pull Request Summary

## 📌 JIRA Ticket(s)

<!-- Extract ticket ID from branch name if available (e.g., feature/TICKET-001-description) -->
<!-- Update the URL pattern below to match your issue tracker -->

- [TICKET-001](https://your-org.atlassian.net/browse/TICKET-001)

---

## 🧠 Context

[Why this change is being made - the problem being solved and value it brings]

---

## 🛠 Description

[How the task is being accomplished - approach and key design decisions]

---

## 🧾 Changes in the Codebase

- [Key change 1]
- [Key change 2]
- [Key change 3]

---

## 🌍 Changes Outside the Codebase

[Any database, environment, third-party service, or infrastructure changes]
[For Rust projects: Cargo.toml dependency changes, feature flag changes, MSRV updates]

---

## 📎 Additional Information

[Any extra context that will help reviewers]
[For Rust projects: clippy warnings addressed, unsafe code justification, performance considerations]

---

## ✅ Checklist

- [ ] I have tested the changes locally
- [ ] I have added/updated necessary documentation
- [ ] I have followed naming conventions and code structure guidelines
- [ ] I have included relevant unit/integration tests
```

For simpler changes, you can also use this condensed format:

```text
## Summary
[A brief 1-2 sentence overview of the changes]

## Changes
- [Key change 1]
- [Key change 2]
- [Key change 3]

## Testing
[Brief note on testing performed or needed]

## Notes
[Any additional information, concerns, or special considerations]
```

## Guidelines for Analyzing Changes

### Determining Context (Why)

- Look at the overall theme of the changes
- Consider what problem the code is solving
- Think about the business or technical value
- Reference any ticket IDs found in branch names or commit messages

### Determining Description (How)

- Identify the approach and architecture decisions
- Note any patterns or idioms used
- Explain non-obvious implementation choices
- For Rust projects: mention traits implemented, error handling strategy, ownership patterns

### Categorizing Changes

Group changes by type:

- **Features**: New functionality added
- **Bug Fixes**: Issues resolved
- **Refactoring**: Code improvements without behavior changes
- **Dependencies**: Cargo.toml changes, version updates
- **Configuration**: Build settings, CI/CD, tooling
- **Documentation**: README, doc comments, examples
- **Tests**: New or updated test cases

### Identifying External Changes

Look for changes in:

- `Cargo.toml` (dependencies, features, metadata)
- `.env` files or environment configuration
- CI/CD configuration (`.github/workflows/`, `Makefile.toml`)
- Database migrations or schema files
- Docker or deployment configurations

## Example

### Input (Git Log)

```text
commit a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9
Author: Developer Name <developer@example.com>
Date: Mon Jun 19 10:30:45 2023 -0700

feat(parser): implement custom error types with thiserror

- Added ParseError enum with descriptive variants
- Implemented Display and Error traits via thiserror
- Replaced panic! with proper Result returns
- Added unit tests for error cases
```

### Input (Git Diff)

```diff
diff --git a/src/error.rs b/src/error.rs
new file mode 100644
index 0000000..abcdefg
--- /dev/null
+++ b/src/error.rs
@@ -0,0 +1,25 @@
+use thiserror::Error;
+
+#[derive(Debug, Error)]
+pub enum ParseError {
+    #[error("Invalid syntax at line {line}: {message}")]
+    InvalidSyntax { line: usize, message: String },
+
+    #[error("Unexpected token: expected {expected}, found {found}")]
+    UnexpectedToken { expected: String, found: String },
+
+    #[error("I/O error: {0}")]
+    Io(#[from] std::io::Error),
+}
+
+pub type Result<T> = std::result::Result<T, ParseError>;

diff --git a/src/parser.rs b/src/parser.rs
index 1234567..fedcba9
--- a/src/parser.rs
+++ b/src/parser.rs
@@ -1,10 +1,12 @@
+use crate::error::{ParseError, Result};
+
 pub struct Parser {
     input: String,
     position: usize,
 }

 impl Parser {
-    pub fn parse(&mut self) -> String {
+    pub fn parse(&mut self) -> Result<String> {
         if self.input.is_empty() {
-            panic!("Empty input");
+            return Err(ParseError::InvalidSyntax {
+                line: 1,
+                message: "Empty input".to_string(),
+            });
         }
-        self.input.clone()
+        Ok(self.input.clone())
     }
 }

diff --git a/Cargo.toml b/Cargo.toml
index abcdefg..1234567
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -6,3 +6,6 @@ edition = "2024"

 [dependencies]
+thiserror = "1.0"

diff --git a/src/lib.rs b/src/lib.rs
index 1234567..abcdefg
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -1 +1,2 @@
+pub mod error;
 pub mod parser;
```

### Output (Generated PR Description)

```text
# 🚀 Pull Request Summary

## 📌 JIRA Ticket(s)

<!-- No ticket ID found in branch name -->

- N/A

---

## 🧠 Context

The parser module was using `panic!` for error conditions, which is not idiomatic for library code and makes error handling difficult for consumers. This change introduces proper error types using the `thiserror` crate, enabling callers to handle errors gracefully and receive descriptive error messages.

---

## 🛠 Description

Implemented a custom error type hierarchy using the `thiserror` crate for derive macros. The approach follows Rust best practices:

- Created a dedicated `error` module with a `ParseError` enum
- Used `#[from]` attribute for automatic `std::io::Error` conversion
- Defined a `Result<T>` type alias for convenience
- Replaced all `panic!` calls with `Result` returns

---

## 🧾 Changes in the Codebase

- Added new `src/error.rs` module with `ParseError` enum containing `InvalidSyntax`, `UnexpectedToken`, and `Io` variants
- Updated `Parser::parse()` method to return `Result<String>` instead of `String`
- Replaced `panic!("Empty input")` with proper `Err(ParseError::InvalidSyntax {...})` return
- Exported `error` module from `src/lib.rs`

---

## 🌍 Changes Outside the Codebase

- Added `thiserror = "1.0"` dependency to `Cargo.toml`

---

## 📎 Additional Information

This is a **breaking change** for any code that calls `Parser::parse()` directly, as the return type has changed from `String` to `Result<String>`. Callers will need to handle the `Result` appropriately.

The `thiserror` crate is a well-maintained, zero-runtime-cost derive macro for implementing `std::error::Error`.

---

## ✅ Checklist

- [x] I have tested the changes locally
- [x] I have added/updated necessary documentation
- [x] I have followed naming conventions and code structure guidelines
- [ ] I have included relevant unit/integration tests
```

## Tips for Quality PR Descriptions

1. **Be specific**: Mention exact files, functions, or modules changed
2. **Explain the "why"**: Context is more valuable than just listing changes
3. **Highlight breaking changes**: Make these obvious for reviewers
4. **Note dependencies**: Any new crates or version bumps should be mentioned
5. **Consider security**: Note any changes to unsafe code, authentication, or sensitive data
6. **Link related work**: Reference related PRs, issues, or documentation
7. **Be honest about testing**: Mark checklist items accurately
