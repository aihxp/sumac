class Sxmc < Formula
  desc "AI-agnostic Skills x MCP x CLI pipeline"
  homepage "https://github.com/aihxp/sxmc"
  url "https://github.com/aihxp/sxmc/archive/refs/tags/v0.2.25.tar.gz"
  sha256 "5c4a9b60c1a9d6089cd583938c99bb6d680770cc31e6b4611d591478c08c7588"
  license "MIT"
  head "https://github.com/aihxp/sxmc.git", branch: "master"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args(path: ".")
  end

  test do
    assert_match "sxmc", shell_output("#{bin}/sxmc --version")
  end
end
