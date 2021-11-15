const {
  LCDClient,
  MnemonicKey,
  MsgExecuteContract,
} = require("@terra-money/terra.js");
const axios = require("axios");

require("dotenv").config();

const CONTRACT_ADDRESS = "terra1zh5an7vrcr923zhvxz6sngd3w6kjj7d89nc22p";
const DEVIATION = 0.1;
const INTERVAL = 30000;

const client = new LCDClient({
  chainID: "bombay-12",
  URL: "https://bombay-lcd.terra.dev",
});

const mk = new MnemonicKey({
  mnemonic: process.env.MNEMONIC,
});

const wallet = client.wallet(mk);

const deviation = (a, b) => Math.abs(a - b) / Math.min(a, b);

const getPrice = async (symbol) => {
  try {
    const result = await client.wasm.contractQuery(CONTRACT_ADDRESS, {
      get_price: {
        symbol: symbol,
      },
    });
    return result.price;
  } catch (err) {
    console.log(`Error: ${err}`);
  }
  return null;
};

const createSetPrice = (symbol, price) => {
  return new MsgExecuteContract(
    wallet.key.accAddress, // sender
    CONTRACT_ADDRESS, // contract account address
    // handle msg
    {
      set_price: {
        symbol: symbol,
        price: price,
      },
    }
  );
};

setInterval(async () => {
  try {
    console.log("Checking");
    const { data } = await axios.get("http://localhost:8888/latest");
    const rates = data.prices;
    let executeMsgs = [];

    await Promise.all(
      Object.keys(rates).map(async (each) => {
        const lastUpdatedPrice = await getPrice(each);
        if (deviation(rates[each], lastUpdatedPrice) > DEVIATION / 100) {
          executeMsgs.push(createSetPrice(each, rates[each]));
        }
      })
    );

    if (executeMsgs.length > 0) {
      const executeTx = await wallet.createAndSignTx({
        msgs: executeMsgs,
      });

      const executeTxResult = await client.tx.broadcastSync(executeTx);
      console.log(executeTxResult);
    }
  } catch (err) {
    console.log(`Something went wrong -> ${err}`);
  }
}, INTERVAL);
