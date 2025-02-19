# Python Development Conventions

## Code Style
- Follow PEP 8 style guide and use `black`
- Use 4 spaces for indentation
- Maximum line length of 88 characters
- Group imports by standard library, third-party packages, and local modules
- Use clear variable/function names in snake_case
- Document public APIs with docstrings

## Error Handling
- Use try/except blocks for operations that can fail
- Create custom exception classes for domain-specific errors
- Avoid bare except clauses
- Use context managers with 'with' statements
- Handle all exceptions explicitly

## Testing
- *Never* write unit tests in same file as code
    - always write them in `tests/` directory as `test_*.py`
- Integration tests go in `tests/integration` directory
- Use pytest for test framework
- Follow arrange-act-assert pattern
- Aim for >80% test coverage
- Use factories for complex test data

## Project Structure
- __init__.py in each package directory
- main.py for executable scripts
- One class/function group per module
- Group related functionality in packages
- Keep import hierarchies shallow
- Use virtual environments for dependencies

## Aider Conventions

### Prompts
- Be specific and explicit
- One change request per prompt
- Reference specific files/functions
- Include context when needed
- Use clear success criteria

### Code Changes
- Make atomic, focused changes
- Follow existing code style
- Include tests for changes
- Update documentation
- Add clear commit messages

### Development Flow
- Start with failing tests
- Make minimal changes to pass
- Refactor after tests pass
- Commit logical units
- Review diffs before committing

### Best Practices
- Use --edit for targeted changes
- Save work frequently
- Keep git history clean
- Document significant changes
- Test changes incrementally
