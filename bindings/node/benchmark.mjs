import {Klotsk as Puzzle} from './puzzle.mjs';

import {PuzzleCore} from "./index.js";

// system windows 11 64bit
// nodejs v20.8.0
// CPU I5-14600KF
// RAM 32GB 4800MHz
// puzzleJs Time: 16931ms
// puzzleRs Time: 12421ms
function puzzleJs() {
    let pzjs = new Puzzle(10)
    let start_time = new Date().getTime()
    for (let i = 0; i < 10 ** 7; i++) {
        pzjs.move_sqnc('UUUUUUUUUUDDDDDDDDDD')
    }
    let end_time = new Date().getTime()
    let delta = end_time - start_time
    console.log(`puzzleJs Time: ${delta}ms`) // puzzleJs Time: 16931ms
}

function puzzleRs() {
    let pzrs = new PuzzleCore(10)
    let start_time = new Date().getTime()
    for (let i = 0; i < 10 ** 7; i++) {
        pzrs.moveSequence('UUUUUUUUUUDDDDDDDDDD')
    }
    let end_time = new Date().getTime()
    let delta = end_time - start_time
    console.log(`puzzleRs Time: ${delta}ms`) // puzzleRs Time: 12421ms
}

puzzleRs()
puzzleJs()
