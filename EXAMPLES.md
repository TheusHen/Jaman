# Jaman Usage Examples

This document provides detailed examples of using Jaman for various scenarios.

## Table of Contents
- [First Time Setup](#first-time-setup)
- [Daily Usage](#daily-usage)
- [Project-Specific Versions](#project-specific-versions)
- [Managing Multiple Versions](#managing-multiple-versions)
- [Maintenance and Cleanup](#maintenance-and-cleanup)
- [Advanced Usage](#advanced-usage)

## First Time Setup

### Installing Jaman

**Windows:**
```powershell
irm https://raw.githubusercontent.com/TheusHen/jaman/main/install.ps1 | iex
```

**macOS/Linux:**
```bash
curl -fsSL https://raw.githubusercontent.com/TheusHen/jaman/main/install.sh | bash
```

### Initial Configuration

After installation, scan your system for existing Java installations:

```bash
jaman scan
```

Check your current status:

```bash
jaman
# or
jaman status
```

## Daily Usage

### Listing Versions

```bash
# List installed versions
jaman list

# List available LTS versions
jaman list --available --lts

# List all available versions
jaman list --available

# Find specific version
jaman list 21
```

### Installing Java

```bash
# Interactive installation (shows picker)
jaman install

# Install specific version
jaman install 21
jaman install 17.0.1

# Install latest LTS
jaman list --available --lts
jaman install 21
```

### Switching Versions

```bash
# Interactive selection
jaman activate

# Activate specific version
jaman activate 21
jaman activate 17

# Verify active version
java -version
echo $JAVA_HOME  # Unix
echo %JAVA_HOME% # Windows
```

## Project-Specific Versions

### Legacy Project (Java 8)

```bash
cd ~/projects/legacy-app
jaman activate 8
java -version
# openjdk version "8.0...."

# Build your project
./gradlew build
```

### Modern Project (Java 21)

```bash
cd ~/projects/modern-app
jaman activate 21
java -version
# openjdk version "21.0...."

# Run your application
./mvnw spring-boot:run
```

### Microservices with Different Versions

```bash
# Service A requires Java 11
cd ~/projects/service-a
jaman activate 11
./gradlew bootRun

# Service B requires Java 17
cd ~/projects/service-b
jaman activate 17
./gradlew bootRun
```

## Managing Multiple Versions

### Installing Multiple LTS Versions

```bash
# Install all LTS versions
jaman install 8
jaman install 11
jaman install 17
jaman install 21

# Verify installations
jaman list
```

### Testing Against Multiple Versions

```bash
# Test against Java 11
jaman activate 11
./gradlew test

# Test against Java 17
jaman activate 17
./gradlew test

# Test against Java 21
jaman activate 21
./gradlew test
```

### Quick Switching During Development

```bash
# Working with Java 17
jaman activate 17
code .  # Open in VSCode

# Need to test with Java 21
jaman activate 21
./gradlew test

# Back to Java 17
jaman activate 17
```

## Maintenance and Cleanup

### Regular Maintenance

```bash
# Check system health
jaman doctor

# Update tracked versions
jaman scan

# View configuration
jaman config --show
```

### Cleaning Up Unused Versions

```bash
# Remove versions not used in 90 days
jaman clean

# Remove versions not used in 30 days
jaman clean --days 30

# Force cleanup without confirmation
jaman clean --days 60 --force
```

### Checking Disk Usage

```bash
# List all installed versions with paths
jaman list

# Clean command shows space to be freed
jaman clean
# Shows: "Total space: 2.5 GB"
```

## Advanced Usage

### Custom Installation Directory

```bash
# Set custom directory
jaman config --set-install-dir "D:\Java\JDKs"

# Verify
jaman config --show
```

### Scripting with Jaman

**Build Script (Unix):**
```bash
#!/bin/bash
# build.sh

echo "Building with Java 17..."
jaman activate 17
./gradlew clean build

echo "Testing with Java 21..."
jaman activate 21
./gradlew test

echo "Done!"
```

**Build Script (Windows):**
```powershell
# build.ps1

Write-Host "Building with Java 17..."
jaman activate 17
.\gradlew.bat clean build

Write-Host "Testing with Java 21..."
jaman activate 21
.\gradlew.bat test

Write-Host "Done!"
```

### CI/CD Integration

**GitHub Actions:**
```yaml
- name: Setup Java with Jaman
  run: |
    curl -fsSL https://raw.githubusercontent.com/TheusHen/jaman/main/install.sh | bash
    jaman install 17
    jaman activate 17
    java -version
```

**GitLab CI:**
```yaml
setup_java:
  script:
    - curl -fsSL https://raw.githubusercontent.com/TheusHen/jaman/main/install.sh | bash
    - jaman install 17
    - jaman activate 17
    - java -version
```

### Automated Java Setup for Teams

**Team Setup Script:**
```bash
#!/bin/bash
# setup-java-env.sh

echo "Setting up team Java environment..."

# Install Jaman
curl -fsSL https://raw.githubusercontent.com/TheusHen/jaman/main/install.sh | bash

# Install required versions
jaman install 17  # Main version
jaman install 21  # Testing version

# Set default
jaman activate 17

echo "Setup complete! Run 'jaman list' to verify."
```

### Working with GraalVM

```bash
# List GraalVM versions
jaman list --available --graalvm

# Install GraalVM
jaman install <graalvm-version>

# Activate GraalVM
jaman activate <graalvm-version>

# Verify native-image is available
native-image --version
```

### Troubleshooting

**Issue: Java not found after activation**
```bash
# Check if jaman managed to update PATH
jaman doctor

# Restart terminal
exit
# Open new terminal

# Verify
java -version
```

**Issue: Multiple Java installations conflicting**
```bash
# Scan and import all installations
jaman scan

# Check what's active
jaman status

# Explicitly set desired version
jaman activate 17

# Run diagnostics
jaman doctor
```

**Issue: Download failed**
```bash
# Check internet connection
ping api.adoptium.net

# Try again
jaman install 21

# If persistent, check available versions
jaman list --available
```

## Tips and Tricks

### Quick Version Check
```bash
# See active version without full status
jaman list | grep "●"
```

### Find Java Installation Path
```bash
jaman list
# Shows paths for all versions
```

### Temporary Version Switch (Unix)
```bash
# Use specific Java version for one command
JAVA_HOME=/path/to/java21 java -version

# Or create alias
alias java21='JAVA_HOME=/path/to/java21 java'
java21 -version
```

### Batch Operations
```bash
# Install multiple versions at once
for version in 11 17 21; do
    jaman install $version
done

# Test against all versions
for version in 11 17 21; do
    jaman activate $version
    ./gradlew test
done
```

## Common Workflows

### New Project Setup
1. `jaman list --available --lts` - Check available versions
2. `jaman install 21` - Install desired version
3. `jaman activate 21` - Set as active
4. Create your project
5. Build and run

### Switching Between Projects
1. `cd project-a && jaman activate 17`
2. Work on project A
3. `cd ../project-b && jaman activate 21`
4. Work on project B

### System Maintenance
1. `jaman doctor` - Check health
2. `jaman scan` - Update tracked versions
3. `jaman clean --days 90` - Remove old versions
4. `jaman list` - Verify remaining versions

---

For more information, run `jaman --help` or visit the [GitHub repository](https://github.com/TheusHen/jaman).
