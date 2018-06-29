import curry from 'lodash.curry'

const MAX_FRAMERATE = 50

const buildClearStage = ctx => () => {
  ctx.fillStyle = '#030303'
  ctx.fillRect(0, 0, ctx.canvas.width, ctx.canvas.height)
}

const centerPosToTopLeft = (x, resource) => x - resource.canvas.width / 2

const drawObject = (ctx, resource, dx, dy) =>
  ctx.drawImage(resource.canvas, centerPosToTopLeft(dx, resource), centerPosToTopLeft(dy, resource))

const drawPlayer = (ctx, resources) => (dx, dy) =>
  drawObject(ctx, resources.player, dx, dy)

const drawBullet = (ctx, resources) => (dx, dy) =>
  drawObject(ctx, resources.bullet, dx, dy)

const drawEnemy = (ctx, resources) => (dx, dy, radius) => {
  drawObject(ctx, resources.enemy['type' + radius], dx, dy)
}

const drawLifepack = (ctx, resources) => (dx, dy) =>
  drawObject(ctx, resources.lifepack, dx, dy)


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
  let enemies = {}

  for (let radius = 10 ; radius <= 30 ; radius += 10) {
    const ctx = createContext(radius * 2, radius * 2)

    ctx.fillStyle = '#ff00' + (radius * 3);
    ctx.beginPath();
    ctx.arc(
     ctx.canvas.width / 2,
     ctx.canvas.height / 2,
     radius,
     0,
     2 * Math.PI
    );
    ctx.fill();

    enemies['type' + radius] = ctx
  }

  return enemies
}

const buildBullet = () => {
    const ctx = createContext(2, 5)

    ctx.fillStyle = 'yellow';
    ctx.fillRect(0, 0, ctx.canvas.width, ctx.canvas.height)

    return ctx
}

const buildLifepack = () => {
    const ctx = createContext(10, 10)

    ctx.fillStyle = 'blue';
    ctx.fillRect(0, 0, ctx.canvas.width, ctx.canvas.height)

    return ctx
}

const buildResources = () => ({
  player: buildPlayer(),
  enemy: buildEnemy(),
  bullet: buildBullet(),
  lifepack: buildLifepack()
})

// buildImports :: CanvasRenderingContext2D -> Object
// the functions to import into the wasm
const buildImports = (ctx, resources) => ({
  clear_stage: buildClearStage(ctx),
  draw_player: drawPlayer(ctx, resources),
  draw_bullet: drawBullet(ctx, resources),
  draw_enemy: drawEnemy(ctx, resources),
  draw_hud: drawHud(ctx)(resources),
  draw_lifepack: drawLifepack(ctx, resources),
  draw_game_over: drawGameOver(ctx),
  rand: Math.random
})

const createTextContext = curry((width, height, font, color, align) => {
  const context = createContext(width, height)
  context.font = font
  context.fillStyle = color
  context.textAlign = align
  context.textBaseline = 'top'
  return context
})

const createHudContext = curry((width, height, align) =>
  createTextContext(width, height, '18pt Calibri', 'blue', align)
)

const buildHud = () => {
  return {
    life: createHudContext(800, 100, 'right'),
    score: createHudContext(800, 100, 'left'),
  }
}

const drawHud = curry((ctx, resources, remainingLifes, currentScore) => {
  const {life: lifeComponent, score: scoreComponent} = buildHud()

  scoreComponent.fillText(`Score ${currentScore}`, 20, 20)
  ctx.drawImage(scoreComponent.canvas, 0, 0)

  lifeComponent.fillText(`Lifes: ${remainingLifes}`, 750, 50)
  ctx.drawImage(lifeComponent.canvas, 0, 500)
})

const drawGameOver = curry((ctx, score) => {
  const screen = createTextContext(800, 600, '32pt Calibri', 'orange', 'center')
  screen.fillText('GAME OVER!', 400, 0)
  screen.fillText(`Your score: ${score}`, 400, 60)

  ctx.drawImage(screen.canvas, 0, 220)
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
  console.log(imports);
  bindImports(imports)

  shooter.init_game();
  buildKeyBindings(shooter)

  var currentTimestamp = new Date()
  const update = () => {
    const oldTimestamp = currentTimestamp
    currentTimestamp = new Date()

    const timeDelta = (currentTimestamp - oldTimestamp)
    const framerate = parseInt(1000 / timeDelta)
    const delay = framerate > MAX_FRAMERATE
      ? 1000 / MAX_FRAMERATE * (framerate - MAX_FRAMERATE)
      : 1000 / MAX_FRAMERATE

    setTimeout(() => {
      shooter.update_state(timeDelta / 1000)

      if (shooter.render()) {
        window.requestAnimationFrame(update)
      }
    }, delay)
  }

  window.requestAnimationFrame(update);
}

const bindImports = (imports) =>
  Object
  .entries(imports)
  .forEach(([name, fn]) => window[name] = fn)

const shooter = import('./sh00t3r')
shooter.then(runGame)
