const { execSync } = require("child_process");
const os = require("os");

const platform = os.platform();
if (platform === "win32") {
  // For Window
  execSync(
    "npm link ./target/x86_64-pc-windows-gnu/debug/analyze-packages.exe"
  );
} else {
  // For macOS and Linux
  execSync("npm link ./bin/analyze-packages");
}
