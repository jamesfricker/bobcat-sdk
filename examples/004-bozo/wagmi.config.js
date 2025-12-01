// @ts-check

import IBozo from './out/IBozo.sol/IBozo.json';

/** @type {import('@wagmi/cli').Config} */
export default {
  out: 'src/generated.js',
  contracts: [{
    address: '0xd2c5cdeceaa85e9e44edcc63b207febaab8dcaf4',
    abi: IBozo.abi,
    name: 'Bozo'
  }],
  plugins: [],
}
