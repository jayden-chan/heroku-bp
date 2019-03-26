# Variables for the app name and heroku email address
variable "app_name" {
  type = "string"
}

variable "heroku_email" {
  type = "string"
}

# Setup the Heroku provider. The HEROKU_API_KEY env var needs to be
# set for this to work correctly
provider "heroku" {
  email = "${var.heroku_email}"
}

# Create the app
resource "heroku_app" "default" {
  name = "${var.app_name}"
  region = "us"
}

# Build code & release to the app
resource "heroku_build" "default" {
  app = "${heroku_app.default.name}"
  buildpacks = ["https://github.com/emk/heroku-buildpack-rust.git"]

  source = {
    path = "../app"
    version = "0.0.1"
  }
}

# Launch the app's web process on a free dyno
resource "heroku_formation" "default" {
  app        = "${heroku_app.default.name}"
  type       = "web"
  quantity   = 1
  size       = "Free"
  depends_on = ["heroku_build.default"]
}

# Create a database, and configure the app to use it
resource "heroku_addon" "database" {
  app  = "${heroku_app.default.name}"
  plan = "heroku-postgresql:hobby-dev"
}

output "app_url" {
  value = "https://${heroku_app.default.name}.herokuapp.com/"
}
