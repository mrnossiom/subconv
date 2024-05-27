# subconv

<p align="center"><strong>
Easily convert and transform subtitle files
</strong></p>

<p align="center">
  <a href="https://wakatime.com/badge/github/mrnossiom/subconv">
    <img alt="Time spent" src="https://wakatime.com/badge/github/mrnossiom/subconv.svg" />
  </a>
</p>

Only supported format right now is `SubRip` (`.srt`).

# Features

- Any processed subtitle comes out `UTF-8`, useful for normalizing files that could be misread on TVs.

- When provided the `--parse` option, `subconv` parses incoming subtitle and can perform actions on the subtitle.

  - `--shift` lets you shift timecodes by an offset. It take a simili-timestamp as an input: `±HH:MM:SS,XXX`.

  - …more to come?

- Backup the current file with the `--backup` flag in case you are not sure of the operation and don't want to lose the original subtitle.

## Installation

<details>
  <summary>With <code>cargo</code></summary>

Install from repository with `cargo`:

```sh
cargo install --git https://github.com/mrnossiom/subconv
```
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

