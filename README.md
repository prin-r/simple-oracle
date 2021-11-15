# Simple Oracle

This repo consist of two components:

### Simple Oracle Contract

The oracle contract itself will expose two main functions:

set_price(symbol,price): saves the price of the given symbol to state
get_price(symbol): returns the latest saved price for a given symbol

### Off-Chain Microservices

##### Price Server
This server will fetch and aggregate prices of the above assets from Binance and CoinGecko API every 30 seconds, medianize the values, and expose it through a REST API endpoint (e.g. http://localhost:8888/latest endpoint).
```json
{
  "prices":{
    "BTC":"63004.000000",
    "ETH":"4559.250000"
  }
}
```

##### Price Feeder
This is the service that will be responsible for fetching prices from the price server and asynchronously sending price update transactions to the oracle smart contract on the Terra blockchain.
However, price update transactions will only be sent if the new price fetched from Binance differs from the price value available on the contract by more than 0.1%.

![img](https://user-images.githubusercontent.com/12705423/141727453-3778d96c-527f-4bfc-b213-c56f098fc955.png)