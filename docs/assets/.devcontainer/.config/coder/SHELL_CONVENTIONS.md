# Shell Development Conventions

## Code Style
- Follow Google Shell Style Guide
- Use 2 spaces for indentation
- Maximum line length of 80 characters
- Group functions by related functionality
- Use clear function names in lowercase with underscores
- Add comments for non-obvious code sections

## Error Handling
- Always check command return values
- Use set -e for strict error handling
- Create error handling functions
- Log errors to stderr
- Provide meaningful error messages

## Testing
- *Never* write tests in main script file
    - always write them in `tests/` directory as `test_*.sh`
- Integration tests go in `tests/integration` directory
- Use `assert` functions for test cases
- Test both success and failure paths
- Run tests in isolated environments
- Use fixtures for test data

## Project Structure
- bin/ for executable scripts
- lib/ for shared functions
- conf/ for configuration files
- One function per logical unit
- Group related functions in modules
- Keep scripts focused and simple
- Use subcommands for complex tools

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
