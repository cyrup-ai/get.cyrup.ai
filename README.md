# Install Cyrup AI

## `cyrup` bootstrap

```shell
curl -fsSL https://get.cyrup.ai/bootstrap.sh | bash
```
## cyrup binaries

- `cyrup` your main interactive cli and desktop binary
- `cyrupd` daemon for background work
  - ambient agent runtime
  - LLM execution environment mgt
  - automated RAG and fine tuning
  - MirrorMark: doubles every code file with a natural language equivalent
- `cysec` @ Secret.rust :: local dev secrets fully compatible with AWS Secrets Manager Agent

- `cysec`

This crate extends aws secrets manager to local environments providing:

- SurrealDB based Encrypted Vault
  - Local File-Based Security
  - Zeroize Process Launcher
  - simple cli for configuring your api secrets and other runtime config
  - full compatibility with `aws-secrets-manager-agent`

#### Namespacing

Namespaces and wilcards allow you to organize secets. All secrets are one of three namespace types:

    - `user`
    - `team`
    - `org`

You work with these implicitly but it's worth noting to better understand the trust model.

`cysec sync AWSTEST/*` is by default `user/kloudsamurai/AWSTEST/*`

#### `cysec trust`

`cysec trust` provides grant operations to share secrets with users, teams and providers.

##### P2P `trust`

  - P2P Share with a phone number: `cysec trust --phone +17654321098 OPENAI_API_KEY`
  - P2P Share with an email: `cysec trust --email devpal@secret.rust ENV/TEST/POSTGRES/*`
  - User2Org Share: `cysec trust --org cyrup-ai PROD/SURREALDB/*`
  - User2Team Share: `cysec trust --org cyrup-ai--team cyrupdev VEGAS2025/*`

##### `team`

Creating a new team:

```shell
cysec team new --name 'Cyrup.ai Dev Team' --id 'cyrupdev'
# this creates a new team with id `cyrupdev` and name `Cyrup.ai Dev Team`

cysec trust --team cyrupdev VEGAS2025/*
# this shares `user/kloudsamurai/VEGAS2025/*` with `team/cyrupdev`
```


##### `cysec team list`

`cysec team list` lists all teams.

```shell
cysec team list
```

##### `cysec team delete`

`cysec team delete --id 'cyrupdev'`
