<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/logo/struna-logo-dark.svg">
    <img src="assets/logo/struna-logo-light.svg" width="280" alt="struna">
  </picture>
</p>

<p align="center"><b>English</b> · <a href="README.cs.md">Čeština</a></p>

<p align="center">A lightweight desktop music player for <a href="https://www.navidrome.org/">Navidrome</a> and other (Open)Subsonic servers.</p>

## Goals

- Small native binary — no Electron, no bundled browser
- Rust + [egui](https://github.com/emilk/egui) UI with a warm graphite / copper theme
- Playback via libmpv: gapless, ReplayGain, Opus and everything else ffmpeg can decode
- Optional server-side transcoding (e.g. Opus on a slow connection)
- OS media keys and now-playing integration (MPRIS on Linux, SMTC on Windows)
- Linux AppImage and Windows `.exe`, Flatpak later

## Status

Early development — nothing to run yet.

## Name

*Struna* is Czech for "string" — as in the string of an instrument.
