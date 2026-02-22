# RAG Assistant for Zotero v0.4.3

## ⚠️ IMPORTANT: Name Change & Migration Required

**This release includes a name change to comply with Zotero's trademark requirements.** The application is now called **"RAG Assistant for Zotero"** instead of "ZoteroRAG".

### Why This Change?

Zotero's trademark guidelines require third-party apps to use the format "for Zotero" rather than starting with "Zotero". We've updated:
- Application display name → "RAG Assistant for Zotero"
- Executable name → `rag-assistant`
- Package name → `rag-assistant`
- Repository → `RAG-Assistant-for-Zotero`

### 🔧 Migration Steps (One-Time Only)

**Your profiles, vector databases, and settings will be preserved** if you run these commands before launching the new version:

#### macOS

```bash
# Close the app if running, then rename the data folder:
mv ~/Library/Application\ Support/ZoteroRAG ~/Library/Application\ Support/RAG\ Assistant
```

#### Windows

```powershell
# Close the app if running, then rename the data folder:
# Open PowerShell and run:
Rename-Item -Path "$env:APPDATA\ZoteroRAG" -NewName "RAG Assistant"
```

Or using Command Prompt (cmd.exe):
```cmd
rename "%APPDATA%\ZoteroRAG" "RAG Assistant"
```

#### Linux (Debian/Ubuntu/AppImage)

```bash
# Close the app if running, then rename the config folder:
mv ~/.config/zotero-rag-assistant ~/.config/rag-assistant
```

### What Happens If I Don't Migrate?

If you skip the migration and launch v0.4.3 directly:
- The app will create a **new** data folder with the new name
- You'll see empty profiles and need to re-index your library
- Your old data will remain in the old folder (safe to manually migrate later)

---

## 📦 Installation

Download the installer for your platform:

### macOS
- **Apple Silicon**: `RAG-Assistant-0.4.3-mac-arm64.dmg`
- **Intel**: `RAG-Assistant-0.4.3-mac-x64.dmg`

**Note**: Run this command after installation to bypass macOS Gatekeeper:
```bash
xattr -dr com.apple.quarantine "/Applications/RAG Assistant.app"
```

### Windows
- **64-bit**: `RAG-Assistant-0.4.3-win-x64.exe`

**Note**: Windows SmartScreen may warn about an unrecognized app. Click "More info" → "Run anyway". The app is not code-signed.

### Linux
- **Debian/Ubuntu**: `RAG-Assistant-0.4.3-linux-amd64.deb`
  ```bash
  sudo apt install ./RAG-Assistant-0.4.3-linux-amd64.deb
  ```
- **Portable**: `RAG-Assistant-0.4.3-linux-x64.AppImage`
  ```bash
  chmod +x RAG-Assistant-0.4.3-linux-x64.AppImage
  ./RAG-Assistant-0.4.3-linux-x64.AppImage
  ```

---

## 🎯 What's New in 0.4.3

### Changed

- **Trademark Compliance Rebrand**: All references updated to "RAG Assistant for Zotero"
  - Updated display names across UI
  - Updated installer filenames for all platforms
  - Updated 50+ documentation files
  - Updated GitHub repository URLs

### Technical

- New installer naming pattern: `RAG-Assistant-{version}-{platform}-{arch}.{ext}`
- Linux executable renamed: `zotero-rag-assistant` → `rag-assistant`
- Linux package renamed: `zotero-rag-assistant` → `rag-assistant`
- Repository moved: `aahepburn/Zotero-RAG-Assistant` → `aahepburn/RAG-Assistant-for-Zotero`

---

## 📚 Recent Highlights (v0.4.0-0.4.2)

### Dynamic Retrieval Scaling (v0.4.2)
- Automatically adjusts retrieval limits based on model context size
- Large-context models (Gemini 1.5 Pro, Claude Opus) retrieve 3-5x more snippets
- Conservative defaults for local models (Ollama, LM Studio)

### Linux Remote Desktop Support (v0.4.1)
- Fixed renderer crashes in Remote Desktop environments
- Added `--disable-gpu-sandbox` flag for compatibility
- Extended backend startup timeout for slower sessions

### Metadata Filtering System (v0.4.0)
- Filter retrieved chunks by publication date, tags, and collections
- Supports complex multi-criteria filtering
- JSON-based configuration for flexibility

### UI Improvements
- Chat focus mode with auto-scroll
- Real-time sync status indicators
- 4+ provider integrations (Ollama, LM Studio, Anthropic, Google AI)

---

## 🐛 Bug Fixes & Improvements

See [CHANGELOG.md](CHANGELOG.md) for complete version history.

---

## 📖 Documentation

- **Quick Start**: [README.md](README.md)
- **Prompting Guide**: [PROMPTING_QUICKSTART.md](docs/PROMPTING_QUICKSTART.md)
- **Provider Setup**: [provider_guide.md](docs/provider_guide.md)
- **Build Instructions**: [BUILD_CHECKLIST.md](docs/BUILD_CHECKLIST.md)

---

## 🙏 Thank You

Thank you to the Zotero team for their trademark guidance, and to all users for your patience during this transition!

If you encounter any issues with the migration, please open an issue at: https://github.com/aahepburn/RAG-Assistant-for-Zotero/issues
