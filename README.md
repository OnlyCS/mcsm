# mcsm
Self-hosted Minecraft Server Manager, written in Rust and Docker, for Arch Linux.

# run
Start a nodemon instance for instant scss recompilation
```sh
nodemon -w './styles' --exec 'sass ./styles/index.scss > public/css/index.css' -e 'scss'

# in a different terminal, start dioxus instance
dioxus serve

# build
dioxus build
```

# contributions
Send a pr.