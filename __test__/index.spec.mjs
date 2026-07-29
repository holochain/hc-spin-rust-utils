import test from 'ava'
import { unpackAndSaveWebhapp, ZomeCallSigner } from '../index.js'

test('native binding loads and exposes the public API', (t) => {
  t.is(typeof unpackAndSaveWebhapp, 'function')
  t.is(typeof ZomeCallSigner, 'function')
  t.is(typeof ZomeCallSigner.connect, 'function')
  t.is(typeof ZomeCallSigner.prototype.signZomeCall, 'function')
})
