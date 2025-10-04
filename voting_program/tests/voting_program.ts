import * as anchor from "@coral-xyz/anchor";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js"
import { Program, BN } from "@coral-xyz/anchor";
import { VotingProgram } from "../target/types/voting_program";
import { expect } from "chai";

describe("voting_program", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.votingProgram as Program<VotingProgram>;
  const provider = anchor.getProvider();

  it("initialises poll", async () => {
    const signer = Keypair.generate()

    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(signer.publicKey, 1_000_000_000)
    );

    const pollId = new BN(1);
    const description = "First poll";
    const pollStart = new BN(Math.floor(Date.now() / 1000));
    const pollEnd = new BN(pollStart.toNumber() + 3600);

    const [pollPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("poll"), pollId.toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    await program.methods
      .initializePoll(pollId, description, pollStart, pollEnd)
      .accountsPartial({
        signer: signer.publicKey,
        poll: pollPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([signer])
      .rpc();

    const pollAccount = await program.account.poll.fetch(pollPda);

    expect(pollAccount.pollId.toString()).to.equal(pollId.toString());
    expect(pollAccount.description).to.equal(description);
    expect(pollAccount.pollStart.toString()).to.equal(pollStart.toString());
    expect(pollAccount.pollEnd.toString()).to.equal(pollEnd.toString());
    expect(pollAccount.candidateAmount.toNumber()).to.equal(0);
  });

  it("initialises candidate", async () => {
    const user = Keypair.generate();

    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(user.publicKey, 1_000_000_000)
    );

    const pollID = new BN(1);
    const candidateName = 'JohnDoe';

    const [pollPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("poll"), pollID.toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    const [candidatePda] = PublicKey.findProgramAddressSync(
      [Buffer.from(candidateName), pollID.toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    await program.methods
      .initializeCandidate(candidateName, pollID)
      .accountsPartial({
        user: user.publicKey,
        candidate: candidatePda,
        poll: pollPda,
        systemProgram: SystemProgram.programId
      })
      .signers([user])
      .rpc();


    const candidateAccount = await program.account.candidate.fetch(candidatePda);

    expect(candidateAccount.candidateName).to.equal(candidateName);
    expect(candidateAccount.candidateVote.toNumber()).to.equal(0);
  });
  
  it("allows votes", async () => {
    const user = Keypair.generate();
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(user.publicKey, 1_000_000_000)
    );

    const pollID = new BN(5);
    const candidateName = "JohnDoe";
    const description = "First poll";
    const pollStart = new BN(Math.floor(Date.now() / 1000));
    const pollEnd = new BN(pollStart.toNumber() + 3600);

    const [pollPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("poll"), pollID.toArrayLike(Buffer, "le", 8)],
      program.programId
    );
    const [candidatePda] = PublicKey.findProgramAddressSync(
      [Buffer.from(candidateName), pollID.toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    await program.methods
      .initializePoll(pollID, description, pollStart, pollEnd)
      .accountsPartial({
        signer: user.publicKey,
        poll: pollPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    await program.methods
      .initializeCandidate(candidateName, pollID)
      .accountsPartial({
        user: user.publicKey,
        candidate: candidatePda,
        poll: pollPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    await program.methods
      .vote(candidateName, pollID)
      .accountsPartial({
        user: user.publicKey,
        candidate: candidatePda,
        poll: pollPda,
      })
      .signers([user])
      .rpc();

    const candidateAccount = await program.account.candidate.fetch(candidatePda);

    expect(candidateAccount.candidateName).to.equal(candidateName);
    expect(candidateAccount.candidateVote.toNumber()).to.equal(1);
  });

});
