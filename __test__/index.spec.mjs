import test from 'ava'

import { sum, sumWithLua } from '../index.js'

test('sum from native', (t) => {
  t.is(sum(1, 2), 3)
})


test('sum from native with lua', (t) => {
  t.is(sumWithLua(1, 2), 3)
})
