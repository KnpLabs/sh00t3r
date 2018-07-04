import curry from 'lodash.curry'

const MAX_FRAMERATE = 60

const buildClearStage = ctx => () => {
  ctx.fillStyle = '#030303'
  ctx.fillRect(0, 0, ctx.canvas.width, ctx.canvas.height)
}

const centerPosToTopLeft = (x, y, resource) => [
  x - Math.floor(resource.width / 2),
  y - Math.floor(resource.height / 2),
]

const drawCanvas = (ctx, resource, dx, dy) => {
  const [x, y] = centerPosToTopLeft(dx, dy, resource.canvas)
  ctx.drawImage(resource.canvas, x, y)
}

const drawPlayer = (ctx, resources) => (dx, dy) => {
  const [x, y] = centerPosToTopLeft(dx, dy, resources.player)
  ctx.drawImage(resources.player, x, y)
}

const drawBullet = (ctx, resources) => (dx, dy) =>
  drawCanvas(ctx, resources.bullet, dx, dy)

const drawEnemy = (ctx, resources) => (dx, dy, radius) => {
  const enemy = resources.enemy[`type${radius}`]
  const [x, y] = centerPosToTopLeft(dx, dy, enemy)
  const scale = radius / 20; // 20 as the 1 scale
  ctx.drawImage(enemy, x, y, enemy.width * scale, enemy.height * scale)
}

const drawLifepack = (ctx, resources) => (dx, dy) =>
  drawCanvas(ctx, resources.lifepack, dx, dy)


const createContext = (w, h) => {
  const canvas = document.createElement('canvas');
  canvas.width = w;
  canvas.height = h;

  return canvas.getContext('2d');
}

const buildPlayer = () => document.getElementById('sprite-ship')

const buildEnemy = () => {
  return {
    type20: document.getElementById('sprite-invader-a'),
    type30: document.getElementById('sprite-invader-b'),
    type40: document.getElementById('sprite-invader-c'),
  }
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

let keys = {
  movingUp: false,
  movingDown: false,
  movingLeft: false,
  movingRight: false,
  shooting: false
};

const buildKeyBindings = exports => {
  const handleKey = (key, enabled) => {
    switch (key) {
      case 'ArrowUp':
        keys.movingUp = enabled
        break
      case 'ArrowDown':
        keys.movingDown = enabled
        break
      case 'ArrowLeft':
        keys.movingLeft = enabled
        break
      case 'ArrowRight':
        keys.movingRight = enabled
        break
      case ' ':
        keys.shooting = enabled
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
    const oldTimestamp = currentTimestamp
    const newTimestamp = new Date()

    const timeDelta = (newTimestamp - oldTimestamp)

    if (timeDelta < (1.0 / MAX_FRAMERATE)) {
      return
    }

    currentTimestamp = newTimestamp

    shooter.update_state(
      (timeDelta) / 1000,
      keys.movingUp,
      keys.movingDown,
      keys.movingLeft,
      keys.movingRight,
      keys.shooting
    )

    if (shooter.render()) {
      window.requestAnimationFrame(update)
    }
  }

  window.requestAnimationFrame(update);
}

const bindImports = (imports) =>
  Object
  .entries(imports)
  .forEach(([name, fn]) => window[name] = fn)

const shooter = import('./sh00t3r')
shooter.then(runGame)
