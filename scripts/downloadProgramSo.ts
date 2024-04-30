// const fs = require("fs");
import * as fs from "fs";
import * as web3 from "@solana/web3.js";
// const web3 = require("@solana/web3.js");

// Initialize connection
const connection = new web3.Connection(web3.clusterApiUrl("mainnet-beta"));

// Public key of the account where the .so file is stored
const publicKey = new web3.PublicKey(
  "36CvGf4Db6N8ApGkh1F6PLGXK1SYFFBUiEDkJjPEvDL7"
);

async function fetchAndSaveSOFile() {
  try {
    const accountInfo = await connection.getAccountInfo(publicKey);
    if (accountInfo === null) {
      console.log("No account info found.");
      return;
    }
    console.log(accountInfo);

    // The account data is a Buffer containing the bytes of the .so file
    // Start at 45 because that's the number of bytes in the state
    // https://github.com/solana-labs/solana/blob/27eff8408b7223bb3c4ab70523f8a8dca3ca6645/cli/src/program_v4.rs#L785C5-L785C85
    // also just use `solana program dump` instead, it's better than this
    fs.writeFileSync("./output.so", accountInfo.data.subarray(45));
    console.log("File saved as output.so");
  } catch (error) {
    console.error("Failed to fetch or save the file:", error);
  }
}

fetchAndSaveSOFile();
