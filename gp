#!/bin/bash

# Loop over all git remotes
for remote in $(git remote); do
  echo "Pushing to remote: $remote"
  git push "$remote"
done