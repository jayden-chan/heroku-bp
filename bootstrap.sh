#!/usr/bin/env zsh

# Exit on any command failure
set -e

if [ $# -lt 1 ] || [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    echo "Usage:"
    echo "     ./bootstrap.sh <project name> <email>"
    echo
    echo "Dependencies: Git, Terraform, Rust, Heroku CLI"
    exit 1
fi

if [ -d "../$1" ]; then
    echo "Folder with project ID already exists in parent folder. Aborting."
    exit 1
fi

dependencies=('git' 'terraform' 'cargo' 'heroku')
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

# Create the app
heroku apps:create $1
heroku addons:create heroku-postgresql:hobby-dev -a $1
heroku buildpacks:add https://github.com/lstoll/heroku-buildpack-monorepo -a $1
heroku buildpacks:add emk/rust -a $1
heroku config:set APP_BASE=app -a $1

# Commit and deploy
cd ..
git add app README.md
git commit -m "(bootstrap.sh) Initial commit"

git remote add heroku https://git.heroku.com/$1.git
git push heroku HEAD:master

# Self destruct. If we made it this far we know the script succeeded
# thanks to set -e
rm bootstrap.sh

echo "Setup complete!"
set +e
