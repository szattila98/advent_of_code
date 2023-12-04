const { readLines } = require("./util");

let data = readLines("4.txt");

let result = 0;
const gameCounter = data.map(() => 1);
for (let index = 0; index < data.length; index++) {
  const line = data[index];

  const [_cardNum, winnerPipeActual] = line.split(": ");
  const [winnerNumberLine, actualNumberLine] = winnerPipeActual.split(" | ");
  const winnerNumbers = winnerNumberLine.split(/ |  /).filter((n) => n);
  const actualNumbers = actualNumberLine.split(/ |  /).filter((n) => n);

  let lineResult = 0;
  let winCount = 0;

  for (const actualNumber of actualNumbers) {
    if (winnerNumbers.includes(actualNumber)) {
      lineResult = lineResult === 0 ? 1 : lineResult * 2;
      winCount++;
    }
  }

  for (let j = 0; j < winCount; j++) {
    gameCounter[index + 1 + j] += gameCounter[index];
  }

  result += lineResult;
}

console.log(result);
console.log(gameCounter.reduce((acc, curr) => acc + curr, 0));
