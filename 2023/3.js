const { readLines } = require("./util");

const rows = readLines("3.txt").map((line) => line.split(""));

let numbers = [];
let currentIndex = 0;
rows.forEach((row, rowIndex) => {
  row.forEach((char, colIndex) => {
    if (!isNaN(parseInt(char))) {
      if (!numbers[currentIndex]) numbers[currentIndex] = [];
      numbers[currentIndex].push({ char, rowIndex, colIndex });
    } else {
      currentIndex++;
    }
  });
});

numbers = numbers.filter((n) => n);

let result = 0;
numbers.forEach((number) => {
  let isPartNumber = false;
  number.forEach((char) => {
    const positions = [];
    positions.push(rows[char.rowIndex - 1]?.[char.colIndex - 1]);
    positions.push(rows[char.rowIndex - 1]?.[char.colIndex]);
    positions.push(rows[char.rowIndex - 1]?.[char.colIndex + 1]);
    positions.push(rows[char.rowIndex]?.[char.colIndex + 1]);
    positions.push(rows[char.rowIndex + 1]?.[char.colIndex + 1]);
    positions.push(rows[char.rowIndex + 1]?.[char.colIndex]);
    positions.push(rows[char.rowIndex + 1]?.[char.colIndex - 1]);
    positions.push(rows[char.rowIndex]?.[char.colIndex - 1]);
    for (const pos of positions) {
      if (pos !== "." && pos !== undefined && isNaN(parseInt(pos))) {
        isPartNumber = true;
        break;
      }
    }
  });
  if (isPartNumber) {
    result += parseInt(
      number.map((n) => n.char).reduce((acc, curr) => acc + curr, "")
    );
  }
});

console.log(result);
