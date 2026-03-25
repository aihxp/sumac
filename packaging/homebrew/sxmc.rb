class Sxmc < Formula
  desc "Sumac: bring out what your tools can do (Skills x MCP x CLI)"
  homepage "https://github.com/aihxp/sumac"
  url "https://github.com/aihxp/sumac/archive/refs/tags/v1.0.3.tar.gz"
  sha256 "7a856bc3c0236c8f4fdf76907f7698552a2296a7133caffdead5b8f8c1f43f77"
  license "MIT"
  head "https://github.com/aihxp/sumac.git", branch: "master"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args(path: ".")
  end

  test do
    assert_match "sxmc", shell_output("#{bin}/sxmc --version")
  end
end
