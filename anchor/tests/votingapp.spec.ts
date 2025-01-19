import * as anchor from '@coral-xyz/anchor'
import {Program} from '@coral-xyz/anchor'
import {Keypair, PublicKey} from '@solana/web3.js'
import {Votingapp} from '../target/types/votingapp'
import { BankrunProvider, startAnchor } from "anchor-bankrun";
import { IconVectorTriangleOff } from '@tabler/icons-react';


const IDL = require("../target/idl/votingapp.json")

const votingAddress = new PublicKey("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF")

describe('votingapp', () => {
  
  it('Initialize Poll', async () => {
    const context = await startAnchor("", [{name: "votingapp", programId:votingAddress}], []);
    const provider = new BankrunProvider(context);

    const votingProgram = new Program<Votingapp>(
      IDL,
      provider,
    );

    await votingProgram.methods
  .initializePoll(
    new anchor.BN(1), // pollId
    "What is your favorite color?", // description
    new anchor.BN(0), // pollStart
    new anchor.BN(1837263852) // pollEnd
  )
  .rpc();
    
  });
});
