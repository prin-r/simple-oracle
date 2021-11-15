const express = require("express");
const axios = require("axios");
const ccxt = require("ccxt");

const PORT = 8888;
const MULTIPLIER = 1e9;
const INTERVAL = 30000;

const app = express();

let rates = { BTC: 0, ETH: 0, LUNA: 0 };

async function getPricesFromCoingecko() {
  const { data } = await axios.get(
    "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin,ethereum,terra-luna&vs_currencies=USD"
  );
  return {
    BTC: data["bitcoin"]["usd"] * MULTIPLIER,
    ETH: data["ethereum"]["usd"] * MULTIPLIER,
    LUNA: data["terra-luna"]["usd"] * MULTIPLIER,
  };
}

async function getPriceFromBinance() {
  const binanceInst = new ccxt.binance();
  const data = await binanceInst.fetchTickers([
    "BTC/USDT",
    "ETH/USDT",
    "LUNA/USDT",
  ]);

  return {
    BTC: data["BTC/USDT"]["last"] * MULTIPLIER,
    ETH: data["ETH/USDT"]["last"] * MULTIPLIER,
    LUNA: data["LUNA/USDT"]["last"] * MULTIPLIER,
  };
}

setInterval(async () => {
  try {
    const coingecko = await getPricesFromCoingecko();
    const binance = await getPriceFromBinance();

    Object.keys(coingecko).map((each) => {
      rates[each] = (coingecko[each] + binance[each]) / 2;
    });

    console.log("Update Coingecko Price!");
  } catch (err) {
    console.log(`Something went wrong -> ${err}`);
  }
}, INTERVAL);

app.get("/latest", (req, res) => {
  res.send({ prices: rates });
});

app.listen(PORT, () => console.log(`Listening on port ${PORT}`));
