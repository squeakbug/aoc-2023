const fs = require('node:fs');

function* lazySplit(str, delimiter = ',') {
    let start = 0;
    let end = str.indexOf(delimiter);
    
    while (end !== -1) {
        yield str.slice(start, end).trim();
        start = end + delimiter.length;
        end = str.indexOf(delimiter, start);
    }
    
    yield str.slice(start).trim();
}

// 17 is a prime number, so ...?
function hash(arr) {
    return arr.split("").reduce((acc, curr) => {
        acc += curr.charCodeAt(0);
        acc *= 17;
        acc %= 256;
        return acc;
    }, 0)
}

function part1(data) {
    let totalSum = 0;
    for (const len of lazySplit(data)) {
        totalSum += hash(len);
    }
    return totalSum;
}

class Len {
    constructor(label, focal_length) {
        this.label = label;
        this.focal_length = focal_length;
    }
}

function getFocusingPower(boxes) {
    let power = 0;
    for (let [box_indx, box] of boxes.entries()) {
        let box_power = 0;
        for (let [len_indx, len] of box.entries()) {
            box_power += (len_indx + 1) * len.focal_length;
        }
        box_power *= (box_indx + 1);
        power += box_power;
    }
    return power;
}

function parseLenOperation(strLen) {
    let label = "";
    for (let ch in strLen) {
        if (ch == '=')
    }
    [label, focal_length] = strLen.split("");
}

function part2(data) {
    let boxes = Array(256);
    for (const len of lazySplit(data)) {
        let op = parseLenOperation(len);
        if (op == "add") {
            // find place
        } else {
            // find 
        }
    }
}

function main() {
    if (process.argv.length !== 3) {
        console.error('usage: app <input_filename>');
        process.exit(1);
    }

    let filename = process.argv.at(2)
    let data;
    try {
        data = fs.readFileSync(filename, 'utf8').toString();
    } catch (err) {
        console.error(err);
        process.exit(1);
    }

    part1_sln = part1(data)
    part2_sln = part2(data)

    console.log(`part1_sln = ${part1_sln}`)
    console.log(`part2_sln = ${part2_sln}`)
}

main()
