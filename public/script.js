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

const loadWasm = imports => fetch('./sh00t3r.gc.wasm')
  .then(response => response.arrayBuffer())
  .then(bytes => WebAssembly.instantiate(bytes, { env: imports }))
  .then(results => {
    // the exported functions from the wasm
    const exports = results.instance.exports

    console.log(exports)

    exports.build_game(800, 600)
    exports.render()
  })

const runGame = () => {
  const ctx = document.getElementById('canvas').getContext('2d')
  ctx.fillStyle = 'black'

  const resources = buildResources()
  const imports = buildImports(ctx, resources)

  loadWasm(imports)
}

runGame()
