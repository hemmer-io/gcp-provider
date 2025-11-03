# Installation Guide

This guide provides detailed instructions for installing the Gcp provider for Hemmer.

---

## Prerequisites

- **Hemmer CLI** (recommended) or manual binary installation
- Supported platforms:
  - macOS (Intel x86_64 or Apple Silicon ARM64)
  - Linux (x86_64 or ARM64)
  - Windows (x86_64)

---

## Method 1: Using Hemmer CLI (Recommended)

The easiest way to install the provider is using the Hemmer CLI:

```bash
hemmer provider install gcp
```

This command will:
1. Detect your platform automatically
2. Download the correct pre-built binary from the latest GitHub release
3. Verify the checksum
4. Install the provider to `~/.hemmer/providers/`

### Verify Installation

```bash
hemmer provider list
```

You should see `gcp` in the list of installed providers.

---

## Method 2: Manual Installation

### Step 1: Download Binary

Download the appropriate binary for your platform from the [Releases](../../releases) page:

| Platform | Binary Name |
|----------|-------------|
| macOS (Intel) | `hemmer-provider-gcp-darwin-amd64` |
| macOS (Apple Silicon) | `hemmer-provider-gcp-darwin-arm64` |
| Linux (x86_64) | `hemmer-provider-gcp-linux-amd64` |
| Linux (ARM64) | `hemmer-provider-gcp-linux-arm64` |
| Windows (x86_64) | `hemmer-provider-gcp-windows-amd64.exe` |

### Step 2: Verify Checksum (Recommended)

Download `checksums.txt` from the same release and verify:

**macOS/Linux:**
```bash
sha256sum -c checksums.txt
```

**Windows (PowerShell):**
```powershell
Get-FileHash -Algorithm SHA256 hemmer-provider-gcp-windows-amd64.exe
# Compare with value in checksums.txt
```

### Step 3: Install Binary

Move the binary to your Hemmer providers directory:

**macOS/Linux:**
```bash
# Create providers directory if it doesn't exist
mkdir -p ~/.hemmer/providers

# Move and rename binary
mv hemmer-provider-gcp-* ~/.hemmer/providers/hemmer-provider-gcp

# Make executable
chmod +x ~/.hemmer/providers/hemmer-provider-gcp
```

**Windows (PowerShell):**
```powershell
# Create providers directory if it doesn't exist
New-Item -ItemType Directory -Force -Path $env:USERPROFILE\.hemmer\providers

# Move binary
Move-Item hemmer-provider-gcp-windows-amd64.exe $env:USERPROFILE\.hemmer\providers\hemmer-provider-gcp.exe
```

### Step 4: Verify Installation

**macOS/Linux:**
```bash
~/.hemmer/providers/hemmer-provider-gcp --version
```

**Windows (PowerShell):**
```powershell
& "$env:USERPROFILE\.hemmer\providers\hemmer-provider-gcp.exe" --version
```

---

## Method 3: Build from Source

If you prefer to build the provider from source:

### Prerequisites

- Rust toolchain (1.70.0 or later)
- Git

### Build Steps

```bash
# Clone the repository
git clone https://github.com/YOUR_ORG/hemmer-provider-gcp.git
cd hemmer-provider-gcp

# Build in release mode
cargo build --release

# The binary will be at:
# - macOS: target/release/libhemmer_gcp_provider.dylib
# - Linux: target/release/libhemmer_gcp_provider.so
# - Windows: target/release/hemmer_gcp_provider.dll
```

### Install

Copy the built binary to your Hemmer providers directory:

**macOS:**
```bash
mkdir -p ~/.hemmer/providers
cp target/release/libhemmer_gcp_provider.dylib \
   ~/.hemmer/providers/hemmer-provider-gcp
chmod +x ~/.hemmer/providers/hemmer-provider-gcp
```

**Linux:**
```bash
mkdir -p ~/.hemmer/providers
cp target/release/libhemmer_gcp_provider.so \
   ~/.hemmer/providers/hemmer-provider-gcp
chmod +x ~/.hemmer/providers/hemmer-provider-gcp
```

**Windows:**
```powershell
New-Item -ItemType Directory -Force -Path $env:USERPROFILE\.hemmer\providers
Copy-Item target\release\hemmer_gcp_provider.dll `
          $env:USERPROFILE\.hemmer\providers\hemmer-provider-gcp.exe
```

---

## Troubleshooting

### Binary Not Found

If Hemmer can't find the provider, ensure:
1. The binary is in `~/.hemmer/providers/` (or `%USERPROFILE%\.hemmer\providers\` on Windows)
2. The binary is executable (`chmod +x` on macOS/Linux)
3. The binary name is correct: `hemmer-provider-gcp`

### Permission Denied (macOS)

macOS may block the binary due to security settings:

```bash
xattr -d com.apple.quarantine ~/.hemmer/providers/hemmer-provider-gcp
```

### Checksum Mismatch

If the checksum doesn't match:
1. Re-download the binary (it may have been corrupted)
2. Ensure you're using the correct checksum from `checksums.txt`
3. Report the issue on GitHub if the problem persists

---

## Upgrading

To upgrade to a newer version:

**Using Hemmer CLI:**
```bash
hemmer provider upgrade gcp
```

**Manual:**
1. Download the new version
2. Replace the existing binary in `~/.hemmer/providers/`
3. Verify the installation

---

## Uninstalling

**Using Hemmer CLI:**
```bash
hemmer provider uninstall gcp
```

**Manual:**
```bash
rm ~/.hemmer/providers/hemmer-provider-gcp
```

---

## Next Steps

- 🚀 [Getting Started Guide](getting-started.md)
- 📚 [Service Documentation](services/)
- ⬅️ [Back to README](../README.md)
