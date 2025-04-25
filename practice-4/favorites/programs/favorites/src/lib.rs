use anchor_lang::prelude::*;
declare_id!("GfwhbDPReQUpdUK6ikBbM9AzkrCE1uVb2dKvr6rqVJQW");

// Anchor programs always use
pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[account]
#[derive(InitSpace)]
pub struct Favorites {
    pub number: u64,

    #[max_len(50)]
    pub color: String,

    pub delegate: Option<Pubkey>,
}

#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE,
        seeds = [b"favorites", user.key().as_ref()],
        bump,
    )]
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,
}



#[derive(Accounts)]
pub struct UpdateFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: This signer is a delegate. Manual validation is done in handler.
    #[account(signer)]
    pub signer: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [b"favorites", user.key().as_ref()],
        bump,
    )]
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,
}

// Our Solana program!
#[program]
pub mod favorites {
    use super::*;

    // Our instruction handler! It sets the user's favorite number and color
    pub fn set_favorites(context: Context<SetFavorites>, number: u64, color: String) -> Result<()> {
        let user_public_key = context.accounts.user.key();
        msg!("Greetings from {}", context.program_id);
        msg!(
            "User {}'s favorite number is {} and favorite color is: {}",
            user_public_key,
            number,
            color
        );

        context
            .accounts
            .favorites
            .set_inner(Favorites { number, color, delegate: None });
        Ok(())
    }

    pub fn update_favorites(
        context: Context<UpdateFavorites>,
        number: u64,
        color: String,
    ) -> Result<()> {
        let current_delegate: Option<Pubkey> = context.accounts.favorites.delegate;
        let signer = context.accounts.user.key;
        
        let is_owner = true;
        let is_delegate = current_delegate == Some(*signer);

        require!(is_owner || is_delegate, CustomError::Unauthorized);

        context
            .accounts
            .favorites
            .set_inner(Favorites { number, color, delegate: current_delegate });
        Ok(())
    }

    pub fn set_authority(
        context: Context<UpdateFavorites>,
        delegate: Option<Pubkey>,
    ) -> Result<()> {
        let favorites = &mut context.accounts.favorites;
            
        match delegate {
            Some(delegate) => {
                favorites.delegate = Some(delegate);
                msg!("✅Delegate set: {}", delegate);
                Ok(())
                }
            None => {
                favorites.delegate = None;
                msg!("✅ Delegate removed.");
                Ok(())
            }
        }
    }
}

#[error_code]
pub enum CustomError {
    #[msg("Only the authority or delegate can update this account.")]
    Unauthorized,
}



