# Code Conventions

## Rust Code Style

1. Error Handling
   - Use anyhow::Result for error propagation
   - Provide context with .context() or .with_context()
   - Create specific error types for domain-specific errors

2. Async/Await
   - Use tokio runtime for async operations
   - Prefer async/await over manual Future implementations
   - Use proper error propagation in async contexts

3. Configuration
   - Use const strings for configuration templates
   - Maintain configuration in structured formats
   - Document all configuration options

4. Module Organization
   - One primary feature per module
   - Clear separation of concerns
   - Public interfaces should be well-documented

5. Testing
   - Unit tests alongside implementation
   - Integration tests for cross-module functionality
   - Platform-specific test cases

## Project Structure

1. Source Organization
   - src/
     * Main application code
     * Module-specific implementations
   - docs/
     * User documentation
     * API documentation
   - assets/
     * Static resources
   - theme/
     * UI/documentation theming

2. Documentation
   - Maintain comprehensive README.md
   - Keep ARCHITECTURE.md up to date
   - Document all public interfaces
   - Include examples in documentation

3. Configuration Files
   - Keep templates in const strings
   - Document all configuration options
   - Maintain backward compatibility

## Git Workflow

1. Commit Messages
   - Use descriptive commit messages
   - Reference issues/tickets when applicable
   - Separate subject from body

2. Branching
   - Feature branches for new features
   - Hotfix branches for urgent fixes
   - Clean merge history

3. Code Review
   - Review all changes before merge
   - Ensure CI passes
   - Check for documentation updates

## Dependencies

1. Version Management
   - Use specific versions in Cargo.toml
   - Document major dependency updates
   - Keep dependencies up to date

2. Feature Flags
   - Use minimal feature sets
   - Document feature dependencies
   - Test all feature combinations
