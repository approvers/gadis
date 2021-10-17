import isEven from "is-even";

function is_even(number: number): boolean {
  return isEven(number);
}

const number: number = 7123478;

console.log(`${number} is ${is_even(number) ? "even" : "odd"}`)
