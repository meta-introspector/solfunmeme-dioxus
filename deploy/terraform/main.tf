terraform {
  required_providers {
    netlify = {
      source  = "netlify/netlify"
      version = "~> 0.2"
    }
  }
}

# ── Netlify ──────────────────────────────────────────────────────
provider "netlify" {
  token = var.netlify_token
}

resource "netlify_site" "solfunmeme" {
  name = "solfunmeme-dioxus"
}

variable "netlify_token" {
  type      = string
  sensitive = true
  default   = ""
}

output "netlify_url" {
  value = "https://${netlify_site.solfunmeme.name}.netlify.app"
}
