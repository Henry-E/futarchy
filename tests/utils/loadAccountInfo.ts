import { writeFileSync, readFileSync } from "fs";
import { AccountInfo, PublicKey } from "@solana/web3.js";

export function saveAccountInfo(
  accountInfo: AccountInfo<Buffer>,
  accountName: string
) {
  accountName = "/root/projects/henrye-futarchy/tests/accounts/" + accountName;
  // Construct filenames based on the provided account name
  const metadataFilename = `${accountName}.json`;
  const binaryFilename = `${accountName}.bin`;

  // Serialize the metadata except for binary data
  const metadata = {
    executable: accountInfo.executable,
    lamports: accountInfo.lamports,
    owner: accountInfo.owner.toBase58(),
    rentEpoch: accountInfo.rentEpoch,
  };
  const jsonString = JSON.stringify(metadata, null, 2);

  // Save the metadata to a JSON file
  writeFileSync(metadataFilename, jsonString);

  // Save the binary data directly to a binary file
  writeFileSync(binaryFilename, accountInfo.data);
}

export function loadAccountInfo(accountName: string): AccountInfo<Buffer> {
  accountName = "/root/projects/henrye-futarchy/tests/accounts/" + accountName;
  // Construct filenames based on the provided account name
  const metadataFilename = `${accountName}.json`;
  const binaryFilename = `${accountName}.bin`;

  // Read and parse the metadata JSON file
  const metadataString = readFileSync(metadataFilename, "utf-8");
  const metadata = JSON.parse(metadataString);

  // Load the binary data into a Buffer
  const binaryData = readFileSync(binaryFilename);

  // Reconstruct the AccountInfo object
  return {
    data: binaryData,
    executable: metadata.executable,
    owner: new PublicKey(metadata.owner),
    rentEpoch: metadata.rentEpoch,
    lamports: metadata.lamports,
  };
}
