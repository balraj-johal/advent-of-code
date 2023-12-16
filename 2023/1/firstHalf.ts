import { INPUT } from "./input";

console.log("fuck")

const isNumber = (str: string) => {
    if (typeof str != "string") return false // we only process strings!  
    return !isNaN(Number(str)) && // use type coercion to parse the _entirety_ of the string (`parseFloat` alone does not do this)...
           !isNaN(parseFloat(str)) // ...and ensure strings of whitespace fail
}

const INPUT_ARRAY = INPUT.split(/\r?\n/);

const getFirstNumber = (string: string) => {
    let num, index = 0;
    while (!num) {
        if (isNumber(string.charAt(index))) {
            num = Number(string.charAt(index));
        }
        index++;
    }
    return num;
}

const getLastNumber = (string: string) => {
    let num = 0;
    let index = string.length - 1;
    while (!num) {
        if (isNumber(string.charAt(index))) {
            num = Number(string.charAt(index));
        }
        index--;
    }
    return num;
}

let result = 0;

for (const string of INPUT_ARRAY) {
    result += Number(`${getFirstNumber(string)}${getLastNumber(string)}`);
}

console.log(result);