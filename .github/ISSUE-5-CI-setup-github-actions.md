
**Title:** CI: Set up GitHub Actions for Automated Builds and Tests

**Description:**

Create a GitHub Actions workflow to automatically build, lint, and test the application on macOS, Windows, and Linux.

**Acceptance Criteria:**

- A new workflow file is created at `.github/workflows/ci.yml`.
- The workflow triggers on every push to the `main` branch and on pull requests.
- It runs three separate jobs, one for each OS (macos-latest, windows-latest, ubuntu-latest).
- Each job checks out the code, sets up Go, and runs `make lint` and `make test`.
- The build artifact (`camouflage` binary) is optionally uploaded for each OS.
