#!/usr/bin/env python3
"""Rewrite the current branch to a single snapshot commit."""

from __future__ import annotations

import os
import subprocess
import sys
from typing import List


def run(command: List[str], **kwargs) -> None:
    print(f"+ {' '.join(command)}", flush=True)
    subprocess.run(command, check=True, **kwargs)


def git_output(command: List[str]) -> str:
    result = subprocess.run(command, check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    return result.stdout.decode().strip()


def has_parent_commit() -> bool:
    try:
        subprocess.run(["git", "rev-parse", "HEAD^"], check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        return True
    except subprocess.CalledProcessError:
        return False


def main() -> None:
    if not has_parent_commit():
        print("Repository already reduced to a single commit; nothing to do.", flush=True)
        return

    snapshot_message = f"Snapshot: {os.environ.get('GITHUB_SHA', 'unknown')}"
    tree_sha = git_output(["git", "rev-parse", "HEAD^{tree}"])
    new_commit = git_output(["git", "commit-tree", tree_sha, "-m", snapshot_message])

    print(f"Pushing rewritten history based on commit {new_commit}.", flush=True)
    run(["git", "push", "origin", f"{new_commit}:main", "--force-with-lease"])


if __name__ == "__main__":
    try:
        main()
    except subprocess.CalledProcessError as error:
        print(error.stderr.decode() if error.stderr else str(error), file=sys.stderr)
        raise
