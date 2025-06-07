set -x

#dx serve --verbose --hot-reload true --trace  --platform web
dx serve --verbose --hot-reload true --trace  --platform web --addr 0.0.0.0 --port 3000 --features web
#--interactive true
