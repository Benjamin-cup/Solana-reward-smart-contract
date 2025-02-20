import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaRewards } from "../target/types/rewards";
import assert from 'assert';

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const program = anchor.workspace.SolanaRewards as Program<SolanaRewards>;

let rewardState;
let userAccounts = [];

before(async () => {
    rewardState = anchor.web3.Keypair.generate();
    await program.rpc.initialize({
        accounts: {
            rewardState: rewardState.publicKey,
            user: provider.wallet.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
        },
        signers: [rewardState],
    });
});

describe("Solana Rewards Smart Contract Tests", () => {

    async function completeTask(user, activityType) {
        return await program.rpc.completeTask(activityType, {
            accounts: {
                userAccount: user.publicKey,
                rewardState: rewardState.publicKey,
                user: provider.wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [user],
        });
    }

    it("More Tasks Than Users - High Demand, Low Supply", async () => {
        let user = anchor.web3.Keypair.generate();
        userAccounts.push(user);
        let activityType = "Check-in";

        await completeTask(user, activityType);
        let userAccount = await program.account.userAccount.fetch(user.publicKey);
        let expectedReward = 10_000_000 * 1.2; // 20% increase

        assert.strictEqual(userAccount.rewards[0].amount, expectedReward, "Reward should be increased by 20%");
    });

    it("More Users Than Tasks - Low Demand, High Supply", async () => {
        let user1 = anchor.web3.Keypair.generate();
        let user2 = anchor.web3.Keypair.generate();
        userAccounts.push(user1, user2);
        let activityType = "Check-in";

        await completeTask(user1, activityType);
        await completeTask(user2, activityType);

        let userAccount1 = await program.account.userAccount.fetch(user1.publicKey);
        let userAccount2 = await program.account.userAccount.fetch(user2.publicKey);
        let expectedReward = 10_000_000 * 0.9; // 10% decrease

        assert.strictEqual(userAccount1.rewards[0].amount, expectedReward, "Reward should be decreased by 10%");
        assert.strictEqual(userAccount2.rewards[0].amount, expectedReward, "Reward should be decreased by 10%");
    });

    it("Balanced Demand-Supply - No Change in Reward", async () => {
        let user = anchor.web3.Keypair.generate();
        userAccounts.push(user);
        let activityType = "Check-in";

        await completeTask(user, activityType);
        let userAccount = await program.account.userAccount.fetch(user.publicKey);
        let expectedReward = 10_000_000; // No change

        assert.strictEqual(userAccount.rewards[0].amount, expectedReward, "Reward should remain unchanged");
    });

    it("Progressive Farming Penalty", async () => {
        let user = anchor.web3.Keypair.generate();
        userAccounts.push(user);
        let activityType = "Vote in a Poll";

        await completeTask(user, activityType);
        await completeTask(user, activityType);
        await completeTask(user, activityType);

        let userAccount = await program.account.userAccount.fetch(user.publicKey);
        let expectedPenaltyReward = 50_000_000 / 2; // 50% reduction after third repetition

        assert.strictEqual(userAccount.rewards[2].amount, expectedPenaltyReward, "Reward should be reduced by 50% on third repetition");
    });

    it("RNG-Based Task Availability and Dynamic User Entries", async () => {
        let user = anchor.web3.Keypair.generate();
        userAccounts.push(user);
        let activityType = "Refer a User";
        
        await completeTask(user, activityType);
        let userAccount = await program.account.userAccount.fetch(user.publicKey);
        let rewardBefore = userAccount.rewards[0].amount;
        
        await new Promise(resolve => setTimeout(resolve, 10000)); // Simulate task availability change

        await completeTask(user, activityType);
        let updatedUserAccount = await program.account.userAccount.fetch(user.publicKey);
        let rewardAfter = updatedUserAccount.rewards[1].amount;
        
        assert.notStrictEqual(rewardBefore, rewardAfter, "Reward should change based on RNG task availability");
    });

});
