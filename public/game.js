const buildClearStage = ctx => () =>
  ctx.fillRect(0, 0, ctx.canvas.width, ctx.canvas.height)

const centerPosToTopLeft = (x, resource) => x - resource.canvas.width / 2

const drawObject = (ctx, resource, dx, dy) =>
  ctx.drawImage(resource.canvas, centerPosToTopLeft(dx, resource), centerPosToTopLeft(dy, resource))

const drawPlayer = (ctx, resources) => (dx, dy) =>
  drawObject(ctx, resources.player, dx, dy)

const drawBullet = (ctx, resources) => (dx, dy) =>
  drawObject(ctx, resources.bullet, dx, dy)

const drawEnemy = (ctx, resources) => (dx, dy) =>
  drawObject(ctx, resources.enemy, dx, dy)

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
  bullet: buildBullet(),
})

// buildImports :: CanvasRenderingContext2D -> Object
// the functions to import into the wasm
const buildImports = (ctx, resources) => ({
  clear_stage: buildClearStage(ctx),
  draw_player: drawPlayer(ctx, resources),
  draw_bullet: drawBullet(ctx, resources),
  draw_enemy: drawEnemy(ctx, resources),
  rand: Math.random,
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

const runGame = (shooter) => {
  const ctx = document.getElementById('canvas').getContext('2d')
  ctx.fillStyle = 'black'

  ctx.canvas.width = 800;
  ctx.canvas.height = 600;

  const resources = buildResources()
  const imports = buildImports(ctx, resources)
  bindImports(imports)

  shooter.init_game();
  buildKeyBindings(shooter)

  var currentTimestamp = new Date()
  const update = () => {
    window.requestAnimationFrame(update)

    const oldTimestamp = currentTimestamp
    currentTimestamp = new Date()

    shooter.update_state((currentTimestamp - oldTimestamp) / 1000)
    shooter.render()
  }

  window.requestAnimationFrame(update);
}

const bindImports = (imports) =>
  Object
  .entries(imports)
  .forEach(([name, fn]) => window[name] = fn)

const shooter = import('./sh00t3r')
shooter.then(runGame)
