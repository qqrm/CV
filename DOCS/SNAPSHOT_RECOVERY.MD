# Snapshot Recovery

The collapse workflow stores one encrypted snapshot archive at:

- `.snapshots/previous.repo.7z`

Use it to recover the previous repository working tree.

## Restore Steps

1. Copy `.snapshots/previous.repo.7z` to a safe location.
2. Run `7z x previous.repo.7z` and enter the archive password when prompted.
3. Delete current repository files in the target working directory, then copy the extracted files into the repository root.

## Notes

- The archive is encrypted with 7-Zip AES and header encryption.
- Keep your password source private (for example, a secure secret manager).
