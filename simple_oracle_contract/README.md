# Simple Oracle Contract

This contract store prices as mapping from symbol to its price.

### Deployed Contract

| Contract Address      | Link |
| ----------- | ----------- |
| terra1zh5an7vrcr923zhvxz6sngd3w6kjj7d89nc22p| [Link](https://finder.terra.money/testnet/address/terra1zh5an7vrcr923zhvxz6sngd3w6kjj7d89nc22p)       |

### State

| Name      | Type |
| ----------- | ----------- |
|   Owner    |     address   |
|   Rates    |    mapping(string -> int)    |

### Function

| Name      | Type | Params|
| ----------- | ----------- |----------- |
|   GetPrice    |    query    | symbol`String` |
|   SetPrice    |    execute  | symbol`String`, price`u64` |
