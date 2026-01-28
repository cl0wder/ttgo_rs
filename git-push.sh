#!/bin/bash

# Git commit and push script
# Usage: ./git-push.sh "commit message"

set -e  # Exit on any error

# Check if commit message is provided
if [ $# -eq 0 ]; then
    echo "Error: Please provide a commit message"
    echo "Usage: ./git-push.sh \"your commit message\""
    exit 1
fi

COMMIT_MESSAGE="$1"

echo "🔍 Checking git status..."
git status

echo ""
echo "📦 Adding all changes..."
git add .

echo ""
echo "📝 Creating commit with message: \"$COMMIT_MESSAGE\""
git commit -m "$COMMIT_MESSAGE"

echo ""
echo "⬆️  Pushing to remote repository..."
git push

echo ""
echo "✅ Successfully committed and pushed changes!"