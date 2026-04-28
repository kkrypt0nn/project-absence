<div align="center">

# Project Absence

[![Discord Server Badge](https://img.shields.io/discord/1358456011316396295?logo=discord)](https://discord.gg/xj6y5ZaTMr)
[![Crates.io Badge](https://img.shields.io/crates/v/project-absence.svg?color=fe7d37)](https://crates.io/crates/project-absence)
[![Docker Badge](https://img.shields.io/docker/v/kkrypt0nn/project-absence?logo=docker)](https://hub.docker.com/r/kkrypt0nn/project-absence)
[![CI Badge](https://github.com/kkrypt0nn/project-absence/actions/workflows/ci.yaml/badge.svg)](https://github.com/kkrypt0nn/project-absence/actions)
[![Dependency Status Badge](https://deps.rs/repo/github/kkrypt0nn/project-absence/status.svg)](https://deps.rs/repo/github/kkrypt0nn/project-absence)

[![Last Commit Badge](https://img.shields.io/github/last-commit/kkrypt0nn/project-absence)](https://github.com/kkrypt0nn/project-absence/commits/main)
[![Conventional Commits Badge](https://img.shields.io/badge/Conventional%20Commits-1.0.0-%23FE5196?logo=conventionalcommits&logoColor=white)](https://conventionalcommits.org/en/v1.0.0/)

</div>

---

> [!CAUTION]
> This is a WIP tool that may **not fully optimised**, use at your own care! This README will also be reworked.

### 👁️ Uncover the unseen

Project Absence is a domain and server OSINT tool for system administrators and security engineers. It supports a wide range of modules - for example subdomains discovery, files discovery, DNS data, etc. - and its default capabilities can be extended via Lua scripting.

To maintain an OSINT-only approach, the tool contacts each discovered domain or server only once, to extract information based on the returned content - valuable data that can help and lead to further discoveries.

## Getting Started

### Rust Features

Project Absence has the following [Rust features](https://doc.rust-lang.org/cargo/reference/features.html) available:

* `clipboard`: Will let you use the `--clipboard/-C` command line argument, only necessary if you want to copy the result to your clipboard (the result is saved in a file at `~/.absence/result.{json,md}` either way)

### Installation

To install Project Absence, you can use one of the following methods:

#### Cargo

You need to have [Rust](https://rustup.rs) installed. You can then install using:

```bash
cargo install project-absence
```

#### Docker

You can run the tool from the published [Docker image](https://hub.docker.com/r/kkrypt0nn/project-absence) using:

```bash
docker run -v ~/.absence:/root/.absence -it kkrypt0nn/project-absence
```

#### Build from source

You need to have [Rust](https://rustup.rs) installed. After cloning this repository you can build it using:

```bash
cargo build --release
```

> [!NOTE]
> On **Linux** systems, you have to install the following packages if you want to use the `--clipboard/-C` CLI argument:
>
> - `libxcb1-dev`
> - `libxcb-render0-dev`
> - `libxcb-shape0-dev`
> - `libxcb-xfixes0-dev`
>
> They are required for the [`arboard`](https://crates.io/crates/arboard) crate to work properly. The usage of the crate has been put behind a feature in the future so that you are not forced to install these packages.

### Example Usage

Using the tool is straightforward. You may look at the [documentation](https://absence.krypton.ninja/docs/) website for the config and CLI arguments that you can pass.

After editing the config as you wish, running the tool with no specific CLI arguments is as simple as doing

```bash
project-absence -d <your-domain>
```

## Documentation

Full documentation is available [here](https://absence.krypton.ninja/docs/). It includes detailed explanations of arguments and configurations.

## Troubleshooting

If you encounter issues while using Project Absence, consider the following:

- Ensure you are running the latest version
- Report issues: Use the [GitHub issue tracker](https://github.com/kkrypt0nn/project-absence/issues)

## Disclaimer

Use responsibly and lawfully. Project Absence is a tool designed to assist cybersecurity professionals, engineers, and system administrators in identifying publicly available information related to their digital infrastructure. Do not use it against systems you do not own or explicitly have permission to test.

By using this tool, you agree to comply with all applicable laws and abide by the [Terms of Use](./TERMS_OF_USE.md).

## Contributing

People may contribute by following the [Contributing Guidelines](https://github.com/kkrypt0nn/project-absence/blob/main/CONTRIBUTING.md) and the [Code of Conduct](https://github.com/kkrypt0nn/project-absence/blob/main/CODE_OF_CONDUCT.md)

## License

This project was made with 💜 by [Krypton](https://github.com/kkrypt0nn) and is under the [MIT License](https://github.com/kkrypt0nn/project-absence/blob/main/LICENSE.md).
