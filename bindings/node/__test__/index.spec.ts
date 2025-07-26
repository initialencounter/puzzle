import test from 'ava'

import { PuzzleCore } from '../index'

test('sync function from native code', (t) => {
  const pz = new PuzzleCore(4)
  t.is(pz.getMode(), 4)
})


test('move', (t) => {
  const pz = new PuzzleCore(4)
  pz.moveSequence('LLLLLUUUUUU')
  const expectedState =  pz.getPuzzle()
  t.is(expectedState[0][0], 0)
})
