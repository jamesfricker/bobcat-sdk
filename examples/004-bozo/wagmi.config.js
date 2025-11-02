// @ts-check

import IBozo from './out/IBozo.sol/IBozo.json';

/** @type {import('@wagmi/cli').Config} */
export default {
  out: 'src/generated.js',
  contracts: [{
    address: '0xb22100180b2062b5fb3dfe741788b2a8b4d8cd97',
    abi: IBozo.abi,
    name: 'Bozo'
  }],
  plugins: [],
}
