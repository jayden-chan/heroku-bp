#!/usr/bin/env zsh

# Exit on any command failure
set -e

if [ $# -eq 0 ]
  then
    echo "No project name provided. Aborting."
    exit 1
fi

if [ -d "../$1" ]; then
    echo "Folder with project ID already exists in parent folder. Aborting."
    exit 1
fi

if ! [ -x "$(command -v terraform)" ]; then
  echo "Terraform is not installed. Aborting." >&2
  exit 1
fi

if ! [ -x "$(command -v git)" ]; then
  echo "Git is not installed (wut??). Aborting." >&2
  exit 1
fi

# Show the commands as they're being executed
set -v

# Rename this folder to the project name provided
cd ..
mv heroku-bp $1

cd $1

# Nuke the old git stuff and start fresh
rm -rf .git
git init

cd app
carg build --release

# Let Terraform do the rest of the project setup
cd ../terraform
terraform init
terraform apply

# Self destruct. If we made it this far we know the script succeeded
# thanks to set -e
cd ..
rm bootstrap.sh

echo "Setup complete!"

set +ev
