import * as wasm from './pkg';

async function main() {
  let app = document.getElementById("snake-app");
  let scoreContainer = document.getElementById("snake-score");
  app.innerText = wasm.gamePrint();

  let playBtn = document.getElementById("snake-play-btn");
  playBtn.addEventListener("click", (e) => {
    let current_direction = wasm.getSnakeDirection();
    let next_direction = wasm.getSnakeDirection();

    document.getElementById("snake-key-left")
      .addEventListener("click", () => {
        if (current_direction != 3) { next_direction = 1; }
      });
    document.getElementById("snake-key-up")
      .addEventListener("click", () => {
        if (current_direction != 0) { next_direction = 2; }
      });
    document.getElementById("snake-key-right")
      .addEventListener("click", () => {
        if (current_direction != 1) { next_direction = 3; }
      });
    document.getElementById("snake-key-down")
      .addEventListener("click", () => {
        if (current_direction != 2) { next_direction = 0; }
      });

    document.onkeydown = (event) => {
      event = event || window.event;
      let key = event.key || event.keyCode;
      switch (key) {
        case 37:
        case "ArrowLeft":
          event.preventDefault();
          if (current_direction != 3) { next_direction = 1; }
          break;
        case 38:
        case "ArrowUp":
          event.preventDefault();
          if (current_direction != 0) { next_direction = 2; }
          break;
        case 39:
        case "ArrowRight":
          event.preventDefault();
          if (current_direction != 1) { next_direction = 3; }
          break;
        case 40:
        case "ArrowDown":
          event.preventDefault();
          if (current_direction != 2) { next_direction = 0; }
          break;
      }
    };

    let gameSpeed = 300;

    const gameLoop = setInterval(() => {
      wasm.setSnakeDirection(next_direction);

      wasm.gameTick();
      app.innerText = wasm.gamePrint();
      scoreContainer.innerText = wasm.gameScore();

      current_direction = wasm.getSnakeDirection();

      if (wasm.gameIsOver()) {
        scoreContainer.innerText += "   ⛔ Game Over ⛔";
        clearInterval(gameLoop);
      };
    }, gameSpeed);
  });
}

main();
