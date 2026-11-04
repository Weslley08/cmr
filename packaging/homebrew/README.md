# Homebrew distribution (planned)

After the first tagged GitHub Release, create a separate public `Weslley08/homebrew-tap` repository containing `Formula/cmr.rb`. Use checksums computed from the **actual** release archives (see `SHA256SUMS`).

Example for a Linux x64-only formula (extend with macOS and Linux ARM64 platforms before publishing):

```ruby
class Cmr < Formula
  desc "Claude Model Router companion CLI"
  homepage "https://github.com/Weslley08/cmr"
  url "https://github.com/Weslley08/cmr/releases/download/v0.1.0/cmr-x86_64-unknown-linux-gnu.tar.gz"
  sha256 "REPLACE_WITH_ACTUAL_CHECKSUM"
  license "MIT"

  def install
    bin.install "cmr"
  end

  test do
    assert_match "0.1.0", shell_output("#{bin}/cmr version")
  end
end
```

The placeholder formula is **documentation**, not an installable package. The future tap should support Intel and Apple Silicon Macs and Linux architectures, and run formula tests on each platform.

WinGet is a separate Windows-only package manager.
