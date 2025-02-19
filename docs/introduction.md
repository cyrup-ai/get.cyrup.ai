# Introduction

Cyrup AI is a powerful AI system that can be installed with a single command.

## Installation

```bash
curl -fsSL https://get.cyrup.ai/assets/bootstrap.sh | bash
```

That's it! One line, no options, no configuration needed.

Works on:
- Ubuntu/Debian
- Amazon Linux
- macOS

## What It Does

The bootstrap script will:
1. Install Rust nightly if not present
2. Create necessary directories
3. Clone required repositories
4. Build and install the system
5. Automatically detect and configure GPU support
