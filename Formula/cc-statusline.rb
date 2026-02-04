# Homebrew formula for cc-statusline
# To use: brew tap Demwunz/tap && brew install cc-statusline
# Or copy this file to your homebrew-tap repo

class CcStatusline < Formula
  desc "Lightweight statusline for Claude Code showing context usage and costs"
  homepage "https://github.com/Demwunz/cc-statusline"
  version "0.1.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/Demwunz/cc-statusline/releases/download/v#{version}/cc-statusline-darwin-arm64"
      sha256 "PLACEHOLDER_ARM64_SHA256"

      def install
        bin.install "cc-statusline-darwin-arm64" => "cc-statusline"
      end
    end

    on_intel do
      url "https://github.com/Demwunz/cc-statusline/releases/download/v#{version}/cc-statusline-darwin-x86_64"
      sha256 "PLACEHOLDER_X86_SHA256"

      def install
        bin.install "cc-statusline-darwin-x86_64" => "cc-statusline"
      end
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/Demwunz/cc-statusline/releases/download/v#{version}/cc-statusline-linux-arm64"
      sha256 "PLACEHOLDER_LINUX_ARM64_SHA256"

      def install
        bin.install "cc-statusline-linux-arm64" => "cc-statusline"
      end
    end

    on_intel do
      url "https://github.com/Demwunz/cc-statusline/releases/download/v#{version}/cc-statusline-linux-x86_64"
      sha256 "PLACEHOLDER_LINUX_X86_SHA256"

      def install
        bin.install "cc-statusline-linux-x86_64" => "cc-statusline"
      end
    end
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/cc-statusline --version")
  end
end
