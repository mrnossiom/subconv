# subconv

<p align="center"><strong>
Easily convert and transform subtitle files
</strong></p>

<p align="center">
  <img alt="Nix Flake" src="https://img.shields.io/badge/Nix-Flake-blue?logo=nixos" />
  <a href="https://wakatime.com/badge/github/mrnossiom/subconv">
    <img alt="Time spent" src="https://wakatime.com/badge/github/mrnossiom/subconv.svg" />
  </a>
</p>

Only supported format right now is `SubRip` (`.srt`).

# Features

- Detects input subtitle encoding and converts it to `UTF-8` internally. Any processed subtitle comes out `UTF-8`, useful for normalizing files that could be misread on TVs.

- When provided the `--parse` option, `subconv` parses incoming subtitle and can perform actions on the subtitle.

  - `--shift` lets you shift timecodes by an offset. It take a timestamp-like as an input: `±HH:MM:SS,XXX`.

  - …more to come?

- Backup the current file with the `--backup` flag in case you are not sure of the operation and don't want to lose the original subtitle.

# Installation

<details>
  <summary>With <code>cargo</code></summary>

Install from repository with `cargo`:

```sh
cargo install --git https://github.com/mrnossiom/subconv
```
</details>

<details>
  <summary>With <code>nix</code> flakes</summary>

A `flake.nix` is available which means that you can use `github:mrnossiom/subconv` as a flake identifier, so you can:

- import this repository in your flake inputs

  ```nix
  {
    git-leave.url = "github:mrnossiom/subconv";
    git-leave.inputs.nixpkgs.follows = "nixpkgs";
  }
  ```

  Then add the package to your [NixOS](https://nixos.org) or [Home Manager](https://github.com/nix-community/home-manager) packages depending on your installation.

- use with `nix shell`/`nix run` for temporary testing

  e.g. `nix shell github:mrnossiom/subconv`

- use with `nix profile` for imperative installation

  e.g. `nix profile install github:mrnossiom/subconv`

Package is reachable through `packages.${system}.default` or `packages.${system}.subconv`.

</details>

# Examples

- Normalize a (`SubRip`) subtitle in-place to `UTF-8`

  ```sh
  subconv --input subtitle.srt
  ```

- Shift a subtitle in-place by adding ten seconds

  ```sh
  subconv --input subtitle.srt --parse --shift +00:00:10,000
  ```

---

Work is licensed under [`CECILL-2.1`](https://choosealicense.com/licenses/cecill-2.1/), a French OSS license that allows modification and distribution of the software while requiring the same license for derived works.

