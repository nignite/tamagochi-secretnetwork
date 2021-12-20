# Tamagotchi
The traditional, Japanese, handheld digital pet. Recreated using smart contracts.

## Contracts

| Name                         | Description                                    |
| ---------------------------- | ---------------------------------------------- |
| [`Market`](contracts/Market) | Used as a marketplace to buy FOOD tokens       |
| [`Food`](packages/Food)      | Snip-20 contract, used to create the token     |
| [`Pet`](contracts/Pet)       | Tamagotchi like interface through the contract |

## Building the contracts

To get started, there is a bash script included to build and optimize all contracts. Simply run:

```
chmod +x ./scripts/1_build_optimize_all.sh
./scripts/1_build_optimize_all.sh
```

This is done for compatability reasons. Namely, the optimizer container does not support workspaces and some crates fail to compile on ARM ased CPU's.

The optimized contracts are generated in the artifacts/ directory.

## Using the contracts

_Make sure to upload all contracts first_
You can use a local node or connect to a testnet. These contracts have been tested on the pulsar-1 network.

1. Create an instance of the Food contract using the following init message:

```javascript
{
   "name":"Food",
   "symbol":"FDT",
   "decimals":2,
   "prng_seed":<random_string>,
   "config":{
      "enable_mint":true,
   }
}
```

2. Create an instance of the Market contract using the following init message:

```javascript
{
   "token_contract_address":" <food contract address>",
   "token_contract_hash":"<food contract hash>",
   "token_exchange_rate": "100"
}
```

3. Create an instance of the Pet contract

```javascript
{
   "accepted_token":
      {
         "address": "<food contract address>",
         "hash":"<food contract hash>",
      },
   "allowed_feed_timespan": <time in seconds>,
   "total_saturation_time": <time in secconds>,
   "viewing_key": "<some secret>"
}
```

total_saturation_time - total time a pet can last in seconds
allowed_feed_timespan - time in seconds after which the pet can be fed.

_Feeding timespan should be smaller than saturation time. For example, if saturation time is 4h and allowed feed timespan is 3h, the pet can be fed after 3h but before it dies at the 4h mark._

### Messages
#### Food
This contract is a fork of the official SNIP-20 implementation. All the messages are the same.
#### Market
| Message |
|---------|
|
|
|


