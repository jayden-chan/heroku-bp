#!/usr/bin/env zsh

# Exit on any command failure
set -e

if [ $# -le 1 ] || [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    echo "Usage:"
    echo "     ./bootstrap.sh <project name> <email>"
    echo
    echo "Dependencies: Git, Terraform, Rust"
    exit 1
fi

if [ -d "../$1" ]; then
    echo "Folder with project ID already exists in parent folder. Aborting."
    exit 1
fi

dependencies=('git' 'terraform' 'cargo')
for d in $dependencies; do
    command -v $d >/dev/null 2>&1 || { echo >&2 "This program requires \`$d\`. Exiting"; exit 1; }
done

# Rename this folder to the project name provided
cd ..
mv heroku-bp $1

cd $1

# Nuke the old git stuff and start fresh
rm -rf .git
git init

cd app
cargo build

# Let Terraform do the rest of the project setup
cd ..
terraform init
terraform apply -var app_name="$1" -var heroku_email="$2"

# Self destruct. If we made it this far we know the script succeeded
# thanks to set -e
rm bootstrap.sh

echo "Setup complete!"
set +e
