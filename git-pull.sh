#!/bin/bash

# Git pull script
# Usage: ./git-pull.sh [branch-name]

set -e  # Exit on any error

# Default to master if no branch specified
BRANCH_NAME=${1:-master}

echo "⬇️  Pulling latest changes from origin/$BRANCH_NAME..."
git pull origin "$BRANCH_NAME"

echo ""
echo "✅ Successfully pulled changes from $BRANCH_NAME!"

echo ""
echo "🔍 Current status:"
git status