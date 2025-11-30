// @ts-check

import IBozo from './out/IBozo.sol/IBozo.json';

/** @type {import('@wagmi/cli').Config} */
export default {
  out: 'src/generated.js',
  contracts: [{
    address: '0x4f48b57abf53180da5c825a4a22bb6ba91c388d5',
    abi: IBozo.abi,
    name: 'Bozo'
  }],
  plugins: [],
}
