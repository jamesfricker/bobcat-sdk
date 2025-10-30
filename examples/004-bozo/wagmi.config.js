// @ts-check

import IBozo from './out/IBozo.sol/IBozo.json';

/** @type {import('@wagmi/cli').Config} */
export default {
  out: 'src/generated.js',
  contracts: [{
    address: '0x6221a9c005f6e47eb398fd867784cacfdcfff4e7',
    abi: IBozo.abi,
    name: 'Bozo'
  }],
  plugins: [],
}
