
variable "fly_region" {
  type = string
  default = "ams"
}

variable "fly_app_name" {
  type = string
  default = "picturepro-rewrite"
}

variable "tikv-image" {
  type = string
  default = "redis:latest"
}

resource "fly_app" "picturepro" {
  name = var.fly_app_name
}

resource "fly_ip" "tikv-ip" {
  app = fly_app.picturepro.name
  type = "v6"
}

resource "fly_volume" "tikv-volume" {
  app = fly_app.picturepro.name
  name = "tikv_volume"
  region = var.fly_region
  size = 10
}
  
resource "fly_machine" "tikv-machine" {
  app = fly_app.picturepro.name
  image = var.tikv-image
  region = var.fly_region
  name = "tikv"

  cpu_type = "shared"
  cpus = 2
  memory = 2048

  mounts = [
    {
      path = "/data"
      volume = fly_volume.tikv-volume.id
    }
  ]
}

