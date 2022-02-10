use anchor_lang::prelude::*;

declare_id!("J44wCG1pTuywrLkhZ8cuDmnPPka2ysGWC7BYnLSnw78G");

#[program]
pub mod gifportal {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account; // Get a reference to the account.
        base_account.total_gifs = 0; // Initialize total_gifs.
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        let item = ItemStruct { 	// Build the struct.
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key
        };

        base_account.gif_list.push(item); 	// Add it to the gif_list vector.
        base_account.total_gifs += 1;
        Ok(())
    }
}

// Attach certain variables to the Initialize context.
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>, // Add the signer who calls the AddGif method to the struct so that we can save it
}

// Create a custom struct for us to work with.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
}

// Tell Solana what we want to store on this account.
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStruct>
}
/*
switch to devnet:

1) solana config set --url devnet
2) solana config get
        solana airdrop 2
        solana balance
3) anchor build (save path to .json)
4) solana address -k /home/admina/solana/gifportal/target/deploy/gifportal-keypair.json // Get the new program id.

5) update Anchor.toml and lib.rs w/ new program id.
6) make sure Anchor.toml is on devnet.
7) anchor build // Build again.
IMPORTANT: nothing change in lib.ru file!!! if you do it redeploy contract again
*/
