# Jaman Quick Start Guide

Get up and running with Jaman in 5 minutes! 🚀

## Installation (30 seconds)

### Windows
Open PowerShell and run:
```powershell
irm https://raw.githubusercontent.com/TheusHen/jaman/main/install.ps1 | iex
```

### macOS / Linux
Open Terminal and run:
```bash
curl -fsSL https://raw.githubusercontent.com/TheusHen/jaman/main/install.sh | bash
```

**Restart your terminal after installation!**

## Verify Installation

```bash
jaman --version
```

You should see something like: `jaman 0.1.0`

## First Steps (2 minutes)

### 1. Check Your Status

```bash
jaman
```

This shows your current setup and available commands.

### 2. Scan for Existing Java

```bash
jaman scan
```

This finds any Java installations already on your system.

### 3. See What's Available

```bash
jaman list --available --lts
```

This shows all Long Term Support (LTS) versions you can download.

### 4. Install Java

```bash
jaman install 21
```

This downloads and installs Java 21 (the latest LTS version).

### 5. Activate Java

```bash
jaman activate 21
```

This makes Java 21 your active version.

### 6. Verify It Works

```bash
java -version
```

You should see Java 21 information!

## Common Tasks

### Switch Between Versions

```bash
# Install another version
jaman install 17

# Switch to it
jaman activate 17

# Verify
java -version

# Switch back
jaman activate 21
```

### List Your Versions

```bash
jaman list
```

Shows all installed versions with an indicator (●) for the active one.

### Remove Old Versions

```bash
jaman clean
```

Removes Java versions you haven't used in a while.

### Check System Health

```bash
jaman doctor
```

Verifies everything is working correctly.

## Pro Tips 💡

1. **Use Tab Completion**: Type `jaman ` and press Tab to see commands
2. **Interactive Mode**: Just run `jaman install` or `jaman activate` without a version for a picker menu
3. **Filter Versions**: Use `jaman list 21` to see all 21.x versions
4. **Check Before Install**: Run `jaman list --available` to see what's downloadable

## Typical Workflow

```bash
# Morning: Check status
jaman

# See available versions
jaman list --available --lts

# Install what you need
jaman install 21

# Start working
jaman activate 21
java -version

# Switch projects
cd ~/other-project
jaman activate 17

# End of day: Clean up
jaman clean --days 90
```

## Troubleshooting

### Command Not Found
- Restart your terminal
- Check PATH: `echo $PATH` (Unix) or `echo %PATH%` (Windows)
- Reinstall: Run the install command again

### Java Version Not Changing
- Run `jaman doctor` to diagnose
- Restart terminal after activation
- Check `jaman list` to see active version (marked with ●)

### Download Failed
- Check internet connection
- Try again: `jaman install 21`
- Check available versions: `jaman list --available`

## Next Steps

- Read the full [README.md](README.md) for detailed features
- Check [EXAMPLES.md](EXAMPLES.md) for advanced usage
- Run `jaman --help` to see all options

## Need Help?

- Run `jaman --help` for command help
- Run `jaman <command> --help` for specific command help
- Open an issue: https://github.com/TheusHen/jaman/issues

---

**You're all set! Happy coding! 🎉**
