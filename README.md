# babyrs

[![Rust](https://img.shields.io/badge/Rather-Rusty-orange)](https://www.rust-lang.org/)
![Build](https://github.com/cerridwen-io/babyrs/actions/workflows/test.yml/badge.svg)
[![LICENSE](https://img.shields.io/badge/License-GPLv3-blue)](./LICENSE)

A simple, data-privacy conscientious (it's all local) `TUI` application for keeping track of your baby's health.

## About

Let's face it, you've just had a life-changing and probably exhausting experience in the hospital. If it's your first
child then you're bombarded with information about how to keep this tiny human alive. Then they send you home with,
perhaps, a bottle or two, some swaddles you just *can't* get right, and your baby.

These little humans are entirely dependent on you. And you're starting out exhausted. When we fed, how much we fed,
diaper changes, and breastfeeding. All studiously annotated on a printed out spread sheet. Honestly, the exercise was
somewhat cathartic during that state of absolute exhaustion. It was especially helpful when coordinating between
caregivers and reconciling when in fact the baby last had a bowel movement.

`babyrs` came out of this experience. We didn't trust any number of phone apps that all probably know a *little too
much* about you and your baby and sell that to *whoever for whatever reason*. At some point the stack of papers became
unwieldly and so now there's this simple `TUI` application. Maybe it could help you too when it's 1 in the morning, you
haven't had more than an hour of congruent sleep, and you're ready to jump out a window.

## Contribution

Your contributions to this project are welcome and encouraged!

### Testing, Linting, and Formatting

Builds are tested automatically in `github actions`.

```sh
# Running all tests
cargo test

# Running just unit tests
cargo test --lib --bins

# Running just integration tests
cargo test --test '*'
```

This repository also uses [pre-commit](https://pre-commit.com/).

```sh
# Installing the pre-commit hooks
pre-commit install

# Running pre-commit checks manually against e'r'thing
pre-commit run --all-files

# Updating the hooks
pre-commit autoupdate --freeze
```

### Migrations

The following commands use `diesel_cli`:

```sh
# Run migrations
diesel migration run

# Re-apply a migration to check that a `down.sql` works
diesel migration redo
```

### Useful tools

- [diesel_cli](https://github.com/diesel-rs/diesel/tree/master/diesel_cli)
