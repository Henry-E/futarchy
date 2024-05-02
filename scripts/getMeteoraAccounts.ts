import { AccountInfo, PublicKey, Connection } from "@solana/web3.js";
import { writeFileSync, readFileSync } from "fs";
import { saveAccountInfo } from "../tests/utils/loadAccountInfo"; // Adjust path as needed

async function fetchAccountInfo(connection, pubkey) {
  const accountInfo = await connection.getAccountInfo(new PublicKey(pubkey));
  return accountInfo;
}

function getConnectionFromConfig() {
  const config = JSON.parse(
    readFileSync(
      "/root/projects/henrye-futarchy/scripts/account_configs/solana_config.json",
      "utf-8"
    )
  );
  const rpcUrl = config.json_rpc_url;
  return new Connection(rpcUrl);
}

async function main() {
  const config = JSON.parse(
    readFileSync(
      "/root/projects/henrye-futarchy/scripts/account_configs/meteora_accounts.json",
      "utf-8"
    )
  );
  const connection = getConnectionFromConfig();

  for (const account of config.accounts) {
    const accountInfo: AccountInfo<Buffer> = await fetchAccountInfo(
      connection,
      account.pubkey
    );

    if (accountInfo) {
      const accountName = `${account.subfolder}/${account.name}`;
      saveAccountInfo(accountInfo, accountName);
      console.log(`Saved account ${account.name} to ${accountName}`);
    } else {
      console.error(`Failed to fetch account info for ${account.name}`);
    }
  }
}

main().catch((err) => console.error("An error occurred:", err));
