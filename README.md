# sh00t3r

A WebAssembly 2D shooter game written in rust.

![screenshot](/doc/screenshot.png)

## Build

```bash
$ docker-compose run --rm compiler make build
$ docker-compose up -d devserver
```

Then go to http://localhost:8080 with your web browser to play the game.

## Credits

This game was inspired by [rocket_wasm](https://github.com/aochagavia/rocket_wasm)
