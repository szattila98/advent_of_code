const { readLines } = require("./util");

const numFinder = (data) => {
  const numbers = [];
  for (const item of data) {
    let sanitized = item;

    sanitized = sanitized.replaceAll("one", "o1ne");
    sanitized = sanitized.replaceAll("two", "t2wo");
    sanitized = sanitized.replaceAll("three", "t3hree");
    sanitized = sanitized.replaceAll("four", "f4our");
    sanitized = sanitized.replaceAll("five", "f5ive");
    sanitized = sanitized.replaceAll("six", "s6ix");
    sanitized = sanitized.replaceAll("seven", "s7even");
    sanitized = sanitized.replaceAll("eight", "e8ight");
    sanitized = sanitized.replaceAll("nine", "n9ine");

    let first;
    for (const char of sanitized) {
      if (parseInt(char)) {
        first = char;
        break;
      }
    }
    let last;
    for (const char of sanitized.split("").reverse()) {
      if (parseInt(char)) {
        last = char;
        break;
      }
    }

    numbers.push(parseInt(first + last));
  }

  return numbers.reduce((acc, curr) => acc + curr, 0);
};

const data1 = readLines("1.txt");
const data2 = readLines("1.txt");

//console.log(numFinder(data1));
console.log(numFinder(data2));
