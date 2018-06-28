const buildClearStage = ctx => () =>
  ctx.fillRect(0, 0, ctx.canvas.width, ctx.canvas.height)

const createContext = (w, h) => {
  const canvas = document.createElement('canvas');
  canvas.width = w;
  canvas.height = h;

  return canvas.getContext('2d');
}

const buildPlayer = () => {
  // @FIXME : draw a nicer object for the player, or use a spacecraft image
  const ctx = createContext(20, 20)

  ctx.fillStyle = 'green'
  ctx.fillRect(0, 0, ctx.canvas.width, ctx.canvas.height)

  return ctx
}

const buildEnemy = () => {
   const ctx = createContext(20, 20)

   ctx.fillStyle = 'red';
   ctx.beginPath();
   ctx.arc(
     ctx.canvas.width / 2,
     ctx.canvas.height / 2,
     ctx.canvas.width / 2,
     0,
     2 * Math.PI
   );
   ctx.fill();

   return ctx
}

const buildBullet = () => {
    const ctx = createContext(2, 5)

    ctx.fillStyle = 'yellow';
    ctx.fillRect(0, 0, ctx.canvas.width, ctx.canvas.height)

    return ctx
}

const buildResources = () => ({
  player: buildPlayer(),
  enemy: buildEnemy(),
  bullet: buildBullet()
})

// buildImports :: CanvasRenderingContext2D -> Object
// the functions to import into the wasm
const buildImports = (ctx, resources) => ({
  clear_stage: buildClearStage(ctx),
})

const loadWasm = imports => fetch('./sh00t3r.gc.wasm')
  .then(response => response.arrayBuffer())
  .then(bytes => WebAssembly.instantiate(bytes, { env: imports }))
  .then(results => {
    // the exported functions from the wasm
    const exports = results.instance.exports

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
