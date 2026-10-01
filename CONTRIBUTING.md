# Contributing to Remora

Thanks for your interest! Remora is a large systems project; this guide keeps contributions predictable.

## Ground rules
- Be kind and follow the [Code of Conduct](CODE_OF_CONDUCT.md).
- Security issues go through [SECURITY.md](SECURITY.md), never public issues.
- Significant changes (protocol, public CLI/config, new transports, new API families) start as an [RFC](rfcs/README.md).

## Developer Certificate of Origin
Every commit must be signed off (`git commit -s`), certifying the [DCO](https://developercertificate.org/):

```
Signed-off-by: Your Name <you@example.com>
```

CI rejects pull requests with unsigned commits.

## Building
Requirements: Rust (version pinned in `rust-toolchain.toml`, installed automatically by rustup), Node.js 20+ for the docs site. A GPU and the CUDA Toolkit are **not** needed for most work: tests run against a mock server.

```sh
cargo build --workspace
cargo xtask ci          # fmt + clippy + tests, exactly as CI
cd docs && npm ci && npm run dev   # documentation site
```

Install the hooks once: `pre-commit install`.

## NVIDIA headers and binaries
Do not commit CUDA headers, libraries or any NVIDIA binaries — their license does not allow redistribution. The code generator reads headers from a local CUDA Toolkit installation; only our annotations and generated signature descriptions are committed.

## Pull requests
- One logical change per PR, with tests. Keep `cargo xtask ci` green.
- Use [Conventional Commits](https://www.conventionalcommits.org/) for PR titles (`feat(proto): ...`, `fix(shim): ...`); PRs are squash-merged.
- User-visible changes get an entry in `CHANGELOG.md` under `Unreleased`.
- Documentation is in English (`docs/en`) with a Russian translation (`docs/ru`); update both or mark the translation as outdated in the PR.

## Code style
- `rustfmt` and `clippy -D warnings` (pedantic) are enforced.
- Every `unsafe` block has a `// SAFETY:` comment.
- The server never trusts the client: validate every length, pointer and handle that crosses the wire.
