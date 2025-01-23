import * as anchor from '@coral-xyz/anchor'
import {Program} from '@coral-xyz/anchor'
import {Keypair, PublicKey} from '@solana/web3.js'
import {Votingapp} from '../target/types/votingapp'
import { BankrunProvider, startAnchor } from "anchor-bankrun";
import { IconVectorTriangleOff } from '@tabler/icons-react';
import { program } from '@coral-xyz/anchor/dist/cjs/native/system';


const IDL = require("../target/idl/votingapp.json")

const votingAddress = new PublicKey("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF")

describe('votingapp', () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Voting as Program<Votingapp>;

  it('InitializePoll', async () => {
    const [pollAddress] = PublicKey.findProgramAddressSync(
      [Buffer.from("poll"), new anchor.BN(1).toArrayLike(Buffer, "le", 8)],
      program.programId
    );
    const tx = await program.methods.initializePoll(
      new anchor.BN(1),
      new anchor.BN(0),
      new anchor.BN(1759508293),
      "test-poll",
      "description",
  )
  .rpc();

  console.log('Your transaction signature', tx);
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
    

  it('initialize candidates', async () => {
    const pollIdBuffer = new anchor.BN(1).toArrayLike(Buffer, "le", 8)

    const [pollAddress] = PublicKey.findProgramAddressSync(
      [Buffer.from("poll"), pollIdBuffer],
      program.programId
    );

    const smoothTx = await program.methods.initializeCandidate(
      new anchor.BN(1), 
      "smooth",
    ).accounts({
      pollAccount: pollAddress
    })
    .rpc();

    const crunchyTx = await program.methods.initializeCandidate(
      new anchor.BN(1), 
      "crunchy",
    ).accounts({
      pollAccount: pollAddress
    })
    .rpc();

    console.log('Your transaction signature', smoothTx);
  });

  it('vote', async () => {

    const tx = await program.methods.vote(
      new anchor.BN(1),
      "smooth",
    )
    .rpc();

    console.log('Your transaction signature', tx);
  
  });
});
