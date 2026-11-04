# Homebrew tap (planned)

CMR is a native Rust executable. After a verified GitHub Release, create `Weslley08/homebrew-tap` with `Formula/cmr.rb`, choosing the appropriate macOS/Linux architectures and SHA256 digests from the actual release archive.

`brew install weslley08/tap/cmr` is **not currently published**. A Homebrew install would place the `cmr` binary on PATH; users still run `cmr init -g` to opt into native hooks.
