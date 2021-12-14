# Tamagotchi

The traditional, Japanese, handheld digital pet. Recreated using smart contracts.

## Contracts

| Name                         | Description                                    |
| ---------------------------- | ---------------------------------------------- |
| [`Market`](contracts/Market) | Used as a marketplace to buy FOOD tokens       |
| [`Food`](packages/Food)      | Snip-20 contract, used to create the token     |
| [`Pet`](contracts/Pet)       | Tamagotchi like interface through the contract |

## Building the contracts

To start building run:

```
cargo build --release --target wasm32-unknown-unknown && cargo schema
```

This will build all the binaries and generate the schema.

For a production ready (optimized & compressed) build, run the following from the root of the repo:

```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer:0.12.3
```

Or use the Secret Network's optimizer for each contract manually. (if the above does not work)

The optimized contracts are generated in the artifacts/ directory.

## Using the contracts

**Note: section is a WIP**
_Make sure to upload all contracts first_

1. Create an instance of the Food contract using the following init message:

```javascript
{
   "name":"Food",
   "symbol":"FDT",
   "decimals":2, // for a conversion of 1/100
   "prng_seed":<random_string>,
   "config":{
      "enable_mint":true, //to be used from the market
      "enable_burn":true
   }
}
```

2. Create an instance of the Market contract using the following init message:
   _TODO_
3. Create an instance of the Pet contract
    _TODO_

## Playing rules

_TODO_
