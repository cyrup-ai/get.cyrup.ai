# Rust Development Conventions

## Table of Contents

- [Code Style](#code-style)
- [Error Handling](#error-handling)
- [Testing](#testing)
- [Project Structure](#project-structure)
- [Development Workflow](#development-workflow-1)
- [Aider Conventions](#aider-conventions)


## Code Style

### Formatting

- Follow Rust official style guide and use `rustfmt`
- Use 4 spaces for indentation
- Maximum line length of 100 characters

### Naming and Organization

- Use clear, descriptive names in `snake_case`
- Group imports in this order:
  1. Standard library (`std`)
  2. External crates
  3. Internal modules
- Separate import groups with blank linest

### Documentation

- Document all public APIs with rustdoc comments
- Include examples in documentation
- Explain complex logic with inline comments


## Error Handling

### General Principles

- Use `Result<T, E>` for operations that can fail
- Create custom error types for domain-specific errors
- Handle all `Result` and `Option` values explicitly

### Best Practices

- Use the `?` operator for error propagation
- Avoid `unwrap()` except in tests or examples
- Provide context when converting errors
- Log errors at appropriate levels


## Testing

### Organization

- Unit tests go in `tests/` directory as `test_*.rs` files
- Integration tests go in `tests/integration/` directory
- Never write unit tests in the same file as code

### Test Structure

- Use `#[test]` attribute for test functions
- Follow arrange-act-assert pattern:
  ```rust
  #[test]
  fn test_something() {
      // Arrange
      let input = setup_test_data();

      // Act
      let result = function_under_test(input);

      // Assert
      assert_eq!(result, expected_output);
  }
  ```

### Coverage and Quality

- Aim for >80% test coverage
- Use test data builders for complex types
- Test edge cases and error conditions
- Write clear test names and descriptions


## Project Structure

### File Organization

- `src/lib.rs` for library crates
- `src/main.rs` for binary crates
- One module per file
- Keep module hierarchies shallow

### Module Layout

```
project/
├── src/
│   ├── lib.rs
│   ├── main.rs
│   └── module/
│       ├── mod.rs
│       └── submodule.rs
├── tests/
│   ├── test_module.rs
│   └── integration/
│       └── test_api.rs
└── examples/
    └── basic_usage.rs
```

### Workspace Structure

- Use workspaces for multi-crate projects
- Keep related crates together
- Share common dependencies


## Development Workflow

### Project Initialization

1. When opening a project, always run:
   ```bash
   cargo check --message-format short --quiet
   ```
2. Document all compiler errors in `TASKLIST.md` under "Current Issues"
3. Categorize errors by type (e.g., compilation, type mismatch, missing implementations)
4. Prioritize errors based on dependency chain and impact

### Continuous Error Checking

- Run `cargo check` after each significant code change
- Update `TASKLIST.md` when:
  - New errors are introduced
  - Existing errors are resolved
  - Error patterns suggest systemic issues
- Track error resolution progress in git commits

### Error Documentation Format

```markdown
## Current Issues

### Compilation Errors

- [ ] Error: "expected 2 arguments, found 1"
  - File: src/example.rs:21
  - Context: Function call missing required argument
  - Priority: High
  - Dependencies: None

### Type Errors

- [ ] Error: "mismatched types"
  - File: src/lib.rs:45
  - Context: Return type doesn't match function signature
  - Priority: Medium
  - Dependencies: Depends on issue #1

### Implementation Errors

- [ ] Error: "not all trait items implemented"
  - File: src/trait_impl.rs:17
  - Context: Missing required trait method
  - Priority: High
  - Dependencies: None
```


## Aider Conventions

### Prompt Guidelines

- Be specific and explicit
- One change request per prompt
- Reference specific files/functions
- Include context when needed
- Define clear success criteria

### Code Changes

- Make atomic, focused changes
- Follow existing code style
- Include tests for changes
- Update documentation
- Write clear commit messages

### Development Workflow

1. Start with failing tests
2. Make minimal changes to pass
3. Refactor after tests pass
4. Commit logical units
5. Review diffs before committing

### Best Practices

- Use `--edit` for targeted changes
- Save work frequently
- Keep git history clean
- Document significant changes
- Test changes incrementally


---

**Note**: These conventions are guidelines. Use judgment when applying them and prioritize code clarity and maintainability.
