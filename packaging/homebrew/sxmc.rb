class Sxmc < Formula
  desc "AI-agnostic Skills x MCP x CLI pipeline"
  homepage "https://github.com/aihxp/sxmc"
  url "https://github.com/aihxp/sxmc/archive/refs/tags/v0.2.37.tar.gz"
  sha256 "9c68ff4f4e2c1059288d3f62c96aa1ef6b08b1a1f701b31e01cc4a118b731144"
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
