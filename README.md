<div align="center">

# DownOnSpot

A Spotify downloader written in Rust

<img src="assets/icon.svg" alt="drawing" width="500"/>

<br>

[![Build project](https://github.com/oSumAtrIX/DownOnSpot/actions/workflows/rust.yml/badge.svg)](https://github.com/oSumAtrIX/DownOnSpot/actions/workflows/rust.yml)
[![Codacy Badge](https://app.codacy.com/project/badge/Grade/4474e5fcc9064562b5d653601ee356f3)](https://www.codacy.com/gh/oSumAtrIX/DownOnSpot/dashboard?utm_source=github.com&amp;utm_medium=referral&amp;utm_content=oSumAtrIX/DownOnSpot&amp;utm_campaign=Badge_Grade)
[![GitHub license](https://img.shields.io/github/license/oSumAtrIX/DownOnSpot)](https://github.com/oSumAtrIX/DownOnSpot/blob/main/LICENSE)
[![GitHub issues](https://img.shields.io/github/issues/oSumAtrIX/DownOnSpot)](https://github.com/oSumAtrIX/DownOnSpot/issues)
[![GitHub forks](https://img.shields.io/github/forks/oSumAtrIX/DownOnSpot)](https://github.com/oSumAtrIX/DownOnSpot/network)
[![GitHub stars](https://img.shields.io/github/stars/oSumAtrIX/DownOnSpot)](https://github.com/oSumAtrIX/DownOnSpot/stargazers)
[![Stability: Experimental](https://masterminds.github.io/stability/experimental.svg)](https://masterminds.github.io/stability/experimental.html)

</div>

## Disclaimer

```text
DownOnSpot was not developed for piracy.
It is meant to be used in compliance with DMCA, Section 1201, for educational, private and fair use.
I am not responsible in any way for the usage of the source code.
```

## Features

-   Works with free Spotify accounts (if using free-librespot fork)
-   Download 96, 160kbit/s audio with a free, 256 and 320 kbit/s audio with a premium account from Spotify, directly
-   Multi-threaded
-   Search for tracks
-   Download tracks, playlists, albums and artists
-   Convert to mp3
-   Metadata tagging
-   Simple usage over CLI

## Building

Clone the repository using git and change to the local repository directory:

```bash
git clone https://github.com/oSumAtrIX/DownOnSpot.git
cd DownOnSpot
```

A [private ssh key](https://osumatrix.me/ucp?get=free_librespot_private_key&token=fdfdbff6f5) is needed to use free Spotify accounts. 
Follow [this answer by DopeGhoti on stackexchange.com](https://unix.stackexchange.com/a/494485) on how to set up ssh with the required private key.
A sample `~/.ssh/config` file could look like this:

```text
Host github.com
  IdentityFile ~/.ssh/free_librespot_private_key
```

If you do not want to use `free-librespot`, then remove the git dependency of `free-librespot`. 
For that, delete `git = "ssh://git@github.com/oSumAtrIX/free-librespot.git"` inside `Cargo.toml`.

`Nightly Rust` is required to build this project. Install it by following [rustup.rs](https://rustup.rs) instructions.

```bash
cargo build --release
```

If you get a linker error, you might need to download the [standard libmp3lame](https://www.rarewares.org/mp3-lame-libraries.php#libmp3lame) library.

## Usage/ Examples

Running DownOnSpot once will create the default configuration file in the same directory as your shell.

```bash
$ down_on_spot.exe
Settings could not be loaded, because of the following error: IO: NotFound No such file or directory. (os error 2)...
..but default settings have been created successfully. Edit them and run the program again.

$ down_on_spot.exe
Usage:
down_on_spot.exe (search_term | track_url | album_url | playlist_url | artist_url)
```

### Template variables

Following variables are available for `path` and `filename_template` in the `settings.json`:

-   %0disc%
-   %0track%
-   %album%
-   %albumArtist%
-   %albumArtists%
-   %artist%
-   %disc%
-   %id%
-   %title%
-   %track%

## Known issues

-   Mp3 downloads slow due to libmp3lame
-   Downloads fail sometimes due to `channel error`

## Authors

-   [@oSumAtrIX](https://osumatrix.me/#github)
-   [@exttex](https://git.freezer.life/exttex)
-   [@breuerfelix](https://github.com/breuerfelix)
-   [@thatpix3l](https://github.com/thatpix3l)

## License

[GPLv3](https://choosealicense.com/licenses/gpl-3.0/)
