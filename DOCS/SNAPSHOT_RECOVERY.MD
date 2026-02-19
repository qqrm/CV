# Snapshot recovery guide

The `collapse-history` workflow stores an encrypted repository snapshot at:

- `.snapshots/previous.bundle.7z`

This file is a 7-Zip archive containing a Git bundle (`previous.bundle`) for the pre-collapse commit state.

## 1) Decrypt the snapshot archive

Provide the password interactively (do not put it in shell history):

```bash
7z x previous.bundle.7z
```

If you need to pass a password explicitly, use a local placeholder and your own secure handling:

```bash
7z x -p<PASSWORD> previous.bundle.7z
```

## 2) Verify and inspect the recovered bundle

```bash
git clone <repo> temp
cd temp
git bundle verify ../previous.bundle
```

## 3) Restore the previous state into a branch

```bash
git fetch ../previous.bundle <PREV_SHA>:recovered
git checkout recovered
```

You can then inspect, diff, or restore files from `recovered` as needed.
