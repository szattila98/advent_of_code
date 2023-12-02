const { readLines } = require("./util");

const data = readLines("2.txt");

const maxPerColor = { red: 12, green: 13, blue: 14 };

let power = 0;
let result = 0;

for (const line of data) {
  const [game, ballStr] = line.split(": ");
  const gameId = parseInt(game.split(" ")[1]);
  const sets = ballStr.split("; ");

  const balls = {
    red: 0,
    green: 0,
    blue: 0,
  };

  for (const set of sets) {
    const setStats = set.split(", ");
    for (const ball of setStats) {
      const [count, color] = ball.split(" ");
      balls[color] =
        parseInt(count) > balls[color] ? parseInt(count) : balls[color];
    }
  }

  if (
    balls.red <= maxPerColor.red &&
    balls.green <= maxPerColor.green &&
    balls.blue <= maxPerColor.blue
  ) {
    result += gameId;
  }

  power += balls.red * balls.green * balls.blue;
}

console.log({ result, power });
