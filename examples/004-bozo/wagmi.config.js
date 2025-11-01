// @ts-check

import IBozo from './out/IBozo.sol/IBozo.json';

/** @type {import('@wagmi/cli').Config} */
export default {
  out: 'src/generated.js',
  contracts: [{
    address: '0x3421264e413489b1e69ae84ace8c33c6cb7809ff',
    abi: IBozo.abi,
    name: 'Bozo'
  }],
  plugins: [],
}
