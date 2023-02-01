import { Player, Reversi, State } from "../wasm/wasm.js";

const windowWidth = () => window.innerWidth;

const windowHeight = () => window.innerHeight - 32;

type Position = {
  x: number;
  y: number;
};

interface Drawable {
  draw(ctx: CanvasRenderingContext2D): void;
}

class Board implements Drawable {
  readonly x: number;
  readonly y: number;
  readonly w: number;
  readonly h: number;
  readonly tileW: number;
  readonly tileH: number;

  public focus: Position | null = null;

  constructor(
    canvas: HTMLCanvasElement,
  ) {
    this.x = 10;
    this.y = 10;
    this.w = canvas.width - 20;
    this.h = canvas.height - 20;
    this.tileW = this.w / 8;
    this.tileH = this.h / 8;
  }

  position(offsetX: number, offsetY: number): Position {
    return {
      x: Math.floor(offsetX / this.tileW),
      y: Math.floor(offsetY / this.tileH),
    };
  }

  draw(ctx: CanvasRenderingContext2D) {
    if (this.focus !== null) {
      ctx.fillStyle = "grey";
      ctx.fillRect(
        this.x + this.focus.x * this.tileW,
        this.y + this.focus.y * this.tileH,
        this.tileW,
        this.tileH,
      );
      ctx.fillStyle = "black";
    }

    ctx.strokeRect(this.x, this.y, this.w, this.h);

    ctx.beginPath();
    for (let n = 1; n < 8; n++) {
      ctx.moveTo(this.x + this.tileW * n, this.y);
      ctx.lineTo(this.x + this.tileW * n, this.y + this.h);
      ctx.moveTo(this.x, this.y + this.tileH * n);
      ctx.lineTo(this.x + this.w, this.y + this.tileH * n);
    }
    ctx.stroke();
  }
}

class Tile implements Drawable {
  constructor(
    private readonly board: Board,
    private readonly state: number,
    private readonly x: number,
    private readonly y: number,
  ) {}

  draw(ctx: CanvasRenderingContext2D) {
    const x = this.board.x + this.board.tileW * this.x + this.board.tileW / 2;
    const y = this.board.y + this.board.tileH * this.y + this.board.tileH / 2;
    const radius = this.board.tileW * 0.4;

    switch (this.state) {
      case State.Player1:
        ctx.beginPath();
        ctx.arc(x, y, radius, 0, Math.PI * 2);
        ctx.fill();
        break;
      case State.Player2:
        ctx.beginPath();
        ctx.arc(x, y, radius, 0, Math.PI * 2);
        ctx.stroke();
        break;
    }
  }
}

function resizeElement(elem: HTMLCanvasElement) {
  const w = windowWidth();
  const h = windowHeight();
  const size = w < h ? w : h;
  elem.width = size;
  elem.height = size;
}

function draw(board: Board, ctx: CanvasRenderingContext2D, game: Reversi) {
  const draw: [Drawable] = [board];

  for (let y = 0; y < game.y(); y++) {
    for (let x = 0; x < game.x(); x++) {
      const state = game.state(x, y);
      if (state === undefined) {
        continue;
      }
      draw.push(new Tile(board, state, x, y));
    }
  }

  for (const obj of draw) {
    obj.draw(ctx);
  }
}

function app() {
  let game = Reversi.new_game();
  const canvas = document.createElement("canvas");

  resizeElement(canvas);
  let board = new Board(canvas);
  const ctx = canvas.getContext("2d") as CanvasRenderingContext2D;
  draw(board, ctx, game);

  addEventListener("resize", (_) => {
    resizeElement(canvas);
    board = new Board(canvas);
    draw(board, ctx, game);
  });

  let currentPosition: Position = { x: 0, y: 0 };
  canvas.addEventListener("mousemove", (e: MouseEvent) => {
    const newPosition = board.position(e.offsetX, e.offsetY);
    if (
      currentPosition.x === newPosition.x && currentPosition.y === newPosition.y
    ) {
      return;
    }
    currentPosition = newPosition;
    if (
      game.is_act(game.current_player(), currentPosition.x, currentPosition.y)
    ) {
      board.focus = currentPosition;
    } else {
      board.focus = null;
    }

    ctx.clearRect(0, 0, canvas.width, canvas.height);
    draw(board, ctx, game);
  });

  canvas.addEventListener("mouseup", (e: MouseEvent) => {
    const position = board.position(e.offsetX, e.offsetY);
    const newGame = game.action(game.current_player(), position.x, position.y);
    if (newGame === undefined) {
      return;
    }
    game = newGame;

    ctx.clearRect(0, 0, canvas.width, canvas.height);
    draw(board, ctx, game);
  });

  document.getElementById("app")?.appendChild(canvas);
}

export default app;
