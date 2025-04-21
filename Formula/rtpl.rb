class Rtpl < Formula
  desc "Command-line tool for rendering Jinja2 templates with data from various sources"
  homepage "https://github.com/fabiomontefuscolo/rtpl"
  url "https://github.com/fabiomontefuscolo/rtpl/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "0000000000000000000000000000000000000000000000000000000000000000" # Replace with actual SHA256 when you have a release
  license "MIT"
  head "https://github.com/fabiomontefuscolo/rtpl.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "build", "--release", "--bin", "rtpl"
    bin.install "target/release/rtpl"
  end

  test do
    # Create a simple template file for testing
    (testpath/"test.j2").write("Hello, {{ name }}!")
    
    # Test rendering the template
    assert_match "Hello, Homebrew!", shell_output("#{bin}/rtpl -t #{testpath}/test.j2 -d '{\"name\":\"Homebrew\"}'")
  end
end

