import test from 'ava'

import { PuzzleCore } from '../index.js'

test('Puzzle', (t) => {
  let pz = new PuzzleCore(4)
  pz.moveSequence('LLLUUU')
  t.is(pz.getPuzzle()[0][0], 0)
})
