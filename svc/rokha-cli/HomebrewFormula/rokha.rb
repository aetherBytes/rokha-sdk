# Rokha CLI Homebrew formula.
#
# This file is the source of truth. To publish, mirror it to
# https://github.com/aetherBytes/homebrew-tap/Formula/rokha.rb on each release.
#
# On release, replace the version + sha256 placeholders with values
# from the corresponding GH release `.sha256` sidecar files. The
# release workflow can do this via a follow-up tap-bump job (see
# `svc/rokha-cli/HomebrewFormula/README.md`).
#
# After mirroring:
#   brew tap aetherBytes/tap
#   brew install rokha
# or one-shot:
#   brew install aetherBytes/tap/rokha

class Rokha < Formula
  desc "Rokha CLI — picks and shovels for the agentic economy"
  homepage "https://rokha.ai"
  version "0.2.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/aetherBytes/rokha-sdk/releases/download/cli-v#{version}/ro-#{version}-aarch64-apple-darwin.tar.gz"
      sha256 "REPLACE_WITH_SHA256_DARWIN_ARM64"
    end
    on_intel do
      url "https://github.com/aetherBytes/rokha-sdk/releases/download/cli-v#{version}/ro-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "REPLACE_WITH_SHA256_DARWIN_X86_64"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/aetherBytes/rokha-sdk/releases/download/cli-v#{version}/ro-#{version}-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "REPLACE_WITH_SHA256_LINUX_ARM64"
    end
    on_intel do
      url "https://github.com/aetherBytes/rokha-sdk/releases/download/cli-v#{version}/ro-#{version}-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "REPLACE_WITH_SHA256_LINUX_X86_64"
    end
  end

  def install
    bin.install "ro"
  end

  test do
    assert_match(/ro #{version}/, shell_output("#{bin}/ro version"))
  end
end
