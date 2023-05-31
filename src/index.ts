import fs from "fs";

/*
function _basics() {
  const foo = [1, 2, 3].map((n) => n + 1);

  console.log(foo);
}
*/

async function readFileAndPrint() {
  fs.readFileSync("lines", "utf-8")
    .toString()
    .split("\n")
    .filter((_, idx) => idx % 2 == 0)
    .filter((_, idx) => idx > 1 && idx < 4)
    .forEach((line) => console.log(line));
}

readFileAndPrint();
