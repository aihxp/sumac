class Sxmc < Formula
  desc "AI-agnostic Skills x MCP x CLI pipeline"
  homepage "https://github.com/aihxp/sxmc"
  url "https://github.com/aihxp/sxmc/archive/refs/tags/v0.2.21.tar.gz"
  sha256 "385ca025dc6302d4d81e06946132bae9b68f54cf167a753435b1c063c5a4f197"
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
