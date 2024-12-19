
provider "fly" {}

provider "aws" {
  region = "eu-north-1"
}

provider "nix" {}

terraform {
  required_providers {
    fly = {
      source = "andrewbaxter/fly"
      version = "0.1.18"
    }
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
    nix = {
      source = "krostar/nix"
      version = "0.0.8"
    }
  }
}

