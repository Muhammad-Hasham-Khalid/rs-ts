// import fs from "fs";

/*
function _basics() {
  const foo = [1, 2, 3].map((n) => n + 1);

  console.log(foo);
}
*/

/*
async function readFileAndPrint() {
  fs.readFileSync("lines", "utf-8")
    .toString()
    .split("\n")
    .filter((_, idx) => idx % 2 == 0)
    .filter((_, idx) => idx > 1 && idx < 4)
    .forEach((line) => console.log(line));
}
*/

enum Color {
  Red,
  Green,
  Blue,
  Yellow,
}

function printColor(color: Color) {
  switch (color) {
    case Color.Red:
      console.log("red");
      break;

    case Color.Green:
      console.log("green");
      break;

    case Color.Blue:
      console.log("blue");
      break;
  }
}

function practice(n: number | undefined): number {
  if (typeof n === "undefined") return 0;
  return n * 5;
}

printColor(Color.Yellow); // won't print anything
