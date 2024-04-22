import { Keypair, PublicKey } from "@solana/web3.js";
import { AmmClient } from "../../AmmClient";
import { InstructionHandler } from "../../InstructionHandler";
import { getATA, getAmmAddr, getAmmLpMintAddr } from "../../utils";
import BN from "bn.js";
import { MethodsBuilder } from "@coral-xyz/anchor/dist/cjs/program/namespace/methods";
import { Amm } from "../../types/amm";

export const createAmmHandler = (
  client: AmmClient,
  baseMint: PublicKey,
  quoteMint: PublicKey,
  twapInitialObservation: BN,
  twapMaxObservationChangePerUpdate: BN,
  proposal: PublicKey
): MethodsBuilder<Amm, any> => {
  let [ammAddr] = getAmmAddr(
    client.program.programId,
    baseMint,
    quoteMint,
    proposal
  );
  let [lpMint] = getAmmLpMintAddr(client.program.programId, ammAddr);

  let [vaultAtaBase] = getATA(baseMint, ammAddr);
  let [vaultAtaQuote] = getATA(quoteMint, ammAddr);

  return client.program.methods
    .createAmm({
      twapInitialObservation,
      twapMaxObservationChangePerUpdate,
      proposal,
    })
    .accounts({
      user: client.provider.publicKey,
      amm: ammAddr,
      lpMint,
      baseMint,
      quoteMint,
      vaultAtaBase,
      vaultAtaQuote,
    });
};
