# Homebrew formula for `ro`

This directory is the source of truth for the formula. The published
tap lives at **`aetherBytes/homebrew-tap`** (separate repo, must exist
before `brew install` works).

## Publishing a new release

1. Tag and push: `git tag cli-v0.2.0 && git push --tags`. The
   `cli-release.yml` workflow builds tarballs and uploads `.sha256`
   sidecar files to the GH Release.
2. After the release is live, run the bump helper:

   ```bash
   ./scripts/bump-tap.sh 0.2.0
   ```

   (Helper script is TODO — for now, the manual recipe is below.)

3. Manually: download the four `.sha256` files from the release,
   copy them into `rokha.rb` in place of the `REPLACE_WITH_SHA256_*`
   tokens, bump the `version` line, and commit to
   `aetherBytes/homebrew-tap/Formula/rokha.rb`.

## One-time setup (creates the tap repo)

The tap is a separate GitHub repo. Create it once:

```bash
gh repo create aetherBytes/homebrew-tap --public \
  --description "Homebrew tap for Rokha"
cd $(mktemp -d) && gh repo clone aetherBytes/homebrew-tap && cd homebrew-tap
mkdir -p Formula
cp /path/to/this/rokha.rb Formula/rokha.rb
git add Formula/rokha.rb && git commit -m "feat: rokha formula" && git push
```

After that, users can install:

```bash
brew install aetherBytes/tap/rokha
```

## Why a separate tap repo?

Homebrew taps need a specific naming convention (`homebrew-<name>`)
and live at the root of a repo. We keep the source of truth here in
the main repo so this formula lives next to the CLI it installs;
the tap repo is just the publication channel.
