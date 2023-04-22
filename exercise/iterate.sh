#!/bin/bash

find_git=$(find . -name ".git" -type d)

for git_dir in $find_git; do
	echo "removing: $git_dir"
	rm -rf "$git_dir"
done
