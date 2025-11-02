// @ts-check

import IBozo from './out/IBozo.sol/IBozo.json';

/** @type {import('@wagmi/cli').Config} */
export default {
  out: 'src/generated.js',
  contracts: [{
    address: '0x944e82782bb29394939483f3380c69b3b89e6426',
    abi: IBozo.abi,
    name: 'Bozo'
  }],
  plugins: [],
}
