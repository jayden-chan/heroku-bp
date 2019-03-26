# Heroku Boilerplate

This repo contains some boilerplate for setting up a Heroku Rust app with a Postgres hobby-dev addon.
Useful for programming competitions that require a cloud hosted backend (most competitions).

The app is provisioned and managed using Terraform.

## Usage

* `git clone https://github.com/jayden-chan/heroku-bp`
* `cd heroku-bp`
* `./bootstrap.sh <app_name>`

That's it. Test the app out at https://app_name.herokuapp.com/ping

## Deploying subsequent builds

Since the app is managed with Terraform you will use that for deploys as well. Edit the code in `/app`,
make sure it compiles, then simply re-run `terraform apply`. Easy.

If you need to create more addons or otherwise update the infrastructure in any way,
make sure to do it from Terraform and NOT from the Heroku dashboard. If you make 
edits from the Heroku dashboard directly, the Terraform state will be out of sync 
and it will not work anymore.
