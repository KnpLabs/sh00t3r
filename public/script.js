const buildClearStage = ctx => () =>
  ctx.fillRect(0, 0, ctx.canvas.width, ctx.canvas.height)

// buildImports :: CanvasRenderingContext2D -> Object
// the functions to import into the wasm
const buildImports = (ctx, resources) => ({
  clear_stage: buildClearStage(ctx),
})

const buildPlayer = () => {
  const canvas = document.createElement('canvas')
  canvas.width = 20
  canvas.height = 20

  // @FIXME : draw a nicer object for the player, or use a spacecraft image
  const ctx = canvas.getContext('2d')
  ctx.fillStyle = 'white'
  ctx.fillRect(0, 0, canvas.width, canvas.height)

  return canvas
}

const buildResources = () => ({
  player: buildPlayer(),
})

const buildKeyBindings = exports => {
  const handleKey = (key, enabled) => {
    switch (key) {
      case 'ArrowUp':
        exports.toggle_move_up(enabled)
        break
      case 'ArrowDown':
        exports.toggle_move_down(enabled)
        break
      case 'ArrowLeft':
        exports.toggle_move_left(enabled)
        break
      case 'ArrowRight':
        exports.toggle_move_right(enabled)
        break
      case ' ':
        exports.toggle_shoot(enabled)
        break
      default:
        break
    }
  }

  document.addEventListener('keydown', e => handleKey(e.key, true));
  document.addEventListener('keyup', e => handleKey(e.key, false));
}

const loadWasm = imports => fetch('./sh00t3r.gc.wasm')
  .then(response => response.arrayBuffer())
  .then(bytes => WebAssembly.instantiate(bytes, { env: imports }))
  .then(results => {
    // the exported functions from the wasm
    const exports = results.instance.exports

    buildKeyBindings(exports)
    exports.init_game()
    exports.render()

    // @TODO : make requestAnimationFrame loop
  })

const runGame = () => {
  const ctx = document.getElementById('canvas').getContext('2d')
  ctx.fillStyle = 'black'

  const resources = buildResources()
  const imports = buildImports(ctx, resources)

  loadWasm(imports)
}

runGame()
