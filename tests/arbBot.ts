import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import * as token from "@solana/spl-token";
import { BankrunProvider } from "anchor-bankrun";
import { loadAccountInfo } from "./utils/loadAccountInfo";
import { readFileSync } from "fs";

const { PublicKey, Keypair, SystemProgram } = anchor.web3;

import { assert } from "chai";

import { BanksClient, startAnchor } from "solana-bankrun";

const METEORA_POOL_PROGRAM_ID = new anchor.web3.PublicKey(
  "Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB"
);

const AUTOCRAT_MIGRATOR_PROGRAM_ID = new PublicKey(
  "MigRDW6uxyNMDBD8fX2njCRyJC4YZk2Rx9pDUZiAESt"
);

const ARB_BOT_PROGRAM_ID = new PublicKey(
  "A43He8159Wx79j1tzZQqqfKTRNoj5xA3ScKRrgGo8Jb"
);

import { AutocratMigrator } from "../target/types/autocrat_migrator";
const AutocratMigratorIDL: AutocratMigrator = require("../target/idl/autocrat_migrator.json");
import { MeteoraPools } from "../programs/meteora_pools/types";
const MeteoraPoolsIDL: MeteoraPools = require("../programs/meteora_pools/idl.json");
import { ArbBot } from "../target/types/arb_bot";
const ArbBotIDL: ArbBot = require("../target/idl/arb_bot.json");

export type PublicKey = anchor.web3.PublicKey;
export type Signer = anchor.web3.Signer;
export type Keypair = anchor.web3.Keypair;

import {
  createMint,
  createAccount,
  getAccount,
  mintTo,
} from "spl-token-bankrun";

// Function to generate a random PublicKey for demonstration
function randomPublicKey() {
  return anchor.web3.Keypair.generate().publicKey;
}

// Mocking the accounts as per the IDL requirements
const accounts = {
  pool: randomPublicKey(),
  userSourceToken: randomPublicKey(),
  userDestinationToken: randomPublicKey(),
  aVault: randomPublicKey(),
  bVault: randomPublicKey(),
  aTokenVault: randomPublicKey(),
  bTokenVault: randomPublicKey(),
  aVaultLpMint: randomPublicKey(),
  bVaultLpMint: randomPublicKey(),
  aVaultLp: randomPublicKey(),
  bVaultLp: randomPublicKey(),
  adminTokenFee: randomPublicKey(),
  user: anchor.web3.Keypair.generate(), // Normally, this would be the wallet public key of the user
  vaultProgram: SystemProgram.programId, // Placeholder, assuming a custom vault program isn't used
  tokenProgram: token.TOKEN_PROGRAM_ID,
};

async function getAccountsToLoad() {
  const config = JSON.parse(
    readFileSync(
      "/root/projects/henrye-futarchy/scripts/account_configs/meteora_accounts.json",
      "utf-8"
    )
  );
  const accountList = [];

  for (const account of config.accounts) {
    const accountInfo = loadAccountInfo(`${account.subfolder}/${account.name}`);
    accountList.push({
      address: new PublicKey(account.pubkey),
      info: accountInfo,
    });
  }

  return accountList;
}

function loadAccounts() {
  const config = JSON.parse(
    readFileSync(
      "/root/projects/henrye-futarchy/scripts/account_configs/meteora_accounts.json",
      "utf-8"
    )
  );
  const accounts = {};

  for (const account of config.accounts) {
    const varName = account.name; // Use the account's name as the variable name
    accounts[varName] = new PublicKey(account.pubkey);
  }

  return accounts;
}

const meteoraAccounts = loadAccounts();
console.log(meteoraAccounts);

// Transaction arguments
const args = {
  inAmount: new anchor.BN(1000000), // Input amount of token A
  minimumOutAmount: new anchor.BN(900000), // Minimum acceptable output amount of token B
};

describe("autocrat_migrator", async function () {
  console.log("what is going on");
  let provider: BankrunProvider,
    connection,
    migrator,
    meteoraPools: Program<MeteoraPools>,
    arbBot: Program<ArbBot>,
    payer: anchor.web3.Keypair,
    context,
    banksClient: BanksClient,
    META,
    USDC,
    MNDE,
    poolAccountInfoPubkey: anchor.web3.PublicKey,
    BOL;

  before(async function () {
    console.log("before");
    const poolAccountInfo = loadAccountInfo("future-pool");
    poolAccountInfoPubkey = new PublicKey(
      "H5JcH3r77iXRgP37cGZ4XFBJT4wSJmn3XYdzSFMjUqVM"
    );
    console.log(poolAccountInfo);
    context = await startAnchor(
      "./",
      [
        {
          name: "meteora-pool",
          programId: METEORA_POOL_PROGRAM_ID,
        },
      ],
      await getAccountsToLoad()
    );
    banksClient = context.banksClient;
    provider = new BankrunProvider(context);
    anchor.setProvider(provider);
    console.log("after");

    migrator = new anchor.Program<AutocratMigrator>(
      AutocratMigratorIDL,
      AUTOCRAT_MIGRATOR_PROGRAM_ID,
      provider
    );
    //   console.log("migrator", migrator);
    meteoraPools = new anchor.Program<MeteoraPools>(
      MeteoraPoolsIDL,
      METEORA_POOL_PROGRAM_ID,
      provider
    );
    //   console.log("meteoraPools", meteoraPools);
    arbBot = new anchor.Program<ArbBot>(
      ArbBotIDL,
      ARB_BOT_PROGRAM_ID,
      provider
    );

    payer = provider.wallet.payer;

    META = await createMint(
      banksClient,
      payer,
      payer.publicKey,
      payer.publicKey,
      9
    );

    USDC = await createMint(
      banksClient,
      payer,
      payer.publicKey,
      payer.publicKey,
      6
    );

    MNDE = await createMint(
      banksClient,
      payer,
      payer.publicKey,
      payer.publicKey,
      9
    );

    BOL = await createMint(
      banksClient,
      payer,
      payer.publicKey,
      payer.publicKey,
      6
    );
  });
  describe("Fail in the right way", async function () {
    it("fails in the right way", async function () {
      try {
        console.log("ok starting");
        // console.log(meteoraPools);
        const tx = await meteoraPools.methods
          .swap(args.inAmount, args.minimumOutAmount)
          .accounts({
            pool: poolAccountInfoPubkey,
            userSourceToken: accounts.userSourceToken,
            userDestinationToken: accounts.userDestinationToken,
            aVault: accounts.aVault,
            bVault: accounts.bVault,
            aTokenVault: accounts.aTokenVault,
            bTokenVault: accounts.bTokenVault,
            aVaultLpMint: accounts.aVaultLpMint,
            bVaultLpMint: accounts.bVaultLpMint,
            aVaultLp: accounts.aVaultLp,
            bVaultLp: accounts.bVaultLp,
            adminTokenFee: accounts.adminTokenFee,
            user: payer.publicKey,
            vaultProgram: accounts.vaultProgram,
            tokenProgram: accounts.tokenProgram,
          })
          .signers([payer])
          // .signers[accounts.user]//   .instruction();
          //   .simulate();
          .rpc();

        console.log("what's wrong");
        console.log(tx);
      } catch (err) {
        console.log(err);
      }
    });
  });

  describe("Execute Swap", async function () {
    it("should call execute_swap and interact with the Meteora pool correctly", async function () {
      try {
        console.log("Executing autocrat_migrator swap function...");

        // Call the execute_swap function from the autocrat_migrator program
        const tx = await arbBot.methods
          .executeSwap(args.inAmount, args.minimumOutAmount)
          .accounts({
            pool: poolAccountInfoPubkey,
            userSourceToken: accounts.userSourceToken,
            userDestinationToken: accounts.userDestinationToken,
            aVault: accounts.aVault,
            bVault: accounts.bVault,
            aTokenVault: accounts.aTokenVault,
            bTokenVault: accounts.bTokenVault,
            aVaultLpMint: accounts.aVaultLpMint,
            bVaultLpMint: accounts.bVaultLpMint,
            aVaultLp: accounts.aVaultLp,
            bVaultLp: accounts.bVaultLp,
            adminTokenFee: accounts.adminTokenFee,
            user: payer.publicKey,
            vaultProgram: accounts.vaultProgram,
            tokenProgram: accounts.tokenProgram,
            meteoraPoolsProgram: METEORA_POOL_PROGRAM_ID, // This is the ID of the Meteora pool program
          })
          .signers([payer])
          .rpc();

        console.log("Transaction successful: ", tx);
      } catch (err) {
        console.error("Transaction failed: ", err);
      }
    });
  });

  //   describe("#multi_transfer2", async function () {
  //     it("does transfer", async function () {
  //       console.log("what's going on");
  //       let sender = Keypair.generate();
  //       let receiver = Keypair.generate();

  //       let from0 = await createAccount(
  //         banksClient,
  //         payer,
  //         META,
  //         sender.publicKey
  //       );
  //       let to0 = await createAccount(
  //         banksClient,
  //         payer,
  //         META,
  //         receiver.publicKey
  //       );

  //       let from1 = await createAccount(
  //         banksClient,
  //         payer,
  //         USDC,
  //         sender.publicKey
  //       );
  //       let to1 = await createAccount(
  //         banksClient,
  //         payer,
  //         USDC,
  //         receiver.publicKey
  //       );

  //       await mintTo(banksClient, payer, META, from0, payer, 1_000_000);
  //       await mintTo(banksClient, payer, USDC, from1, payer, 10_000);

  //       await migrator.methods
  //         .multiTransfer2()
  //         .accounts({
  //           authority: sender.publicKey,
  //           from0,
  //           to0,
  //           from1,
  //           to1,
  //           lamportReceiver: receiver.publicKey,
  //         })
  //         .preInstructions([
  //           SystemProgram.transfer({
  //             fromPubkey: payer.publicKey,
  //             toPubkey: sender.publicKey,
  //             lamports: 1_000_000_000n,
  //           }),
  //         ])
  //         .signers([sender])
  //         .rpc();

  //       assert((await getAccount(banksClient, from0)).amount == 0n);
  //       assert((await getAccount(banksClient, from1)).amount == 0n);

  //       assert((await getAccount(banksClient, to0)).amount == 1_000_000n);
  //       assert((await getAccount(banksClient, to1)).amount == 10_000n);

  //       assert(
  //         (await banksClient.getAccount(receiver.publicKey)).lamports >
  //           1_000_000_000 * 0.999
  //       );
  //     });
  //   });
});
