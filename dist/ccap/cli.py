import subprocess
import sys
import os
import platform

def main():
    # Determine binary name based on platform
    system = platform.system().lower()
    if system == "windows":
        bin_name = "ccap-windows-x64.exe"
    elif system == "darwin":
        bin_name = "ccap-macos-x64"
    else:
        bin_name = "ccap-linux-x64"
    
    # Path to binaries (assuming they are in the package dir)
    bin_path = os.path.join(os.path.dirname(__file__), "binaries", bin_name)
    
    if not os.path.exists(bin_path):
        print(f"❌ CCAP binary not found at {bin_path}.")
        sys.exit(1)
    
    result = subprocess.run([bin_path] + sys.argv[1:])
    sys.exit(result.returncode)

if __name__ == "__main__":
    main()
