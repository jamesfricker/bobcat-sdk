// @ts-check

import IBozo from './out/IBozo.sol/IBozo.json';

/** @type {import('@wagmi/cli').Config} */
export default {
  out: 'src/generated.js',
  contracts: [{
    address: '0x0b8f1939481a337488aae1146063ecacd03462a1',
    abi: IBozo.abi,
    name: 'Bozo'
  }],
  plugins: [],
}
