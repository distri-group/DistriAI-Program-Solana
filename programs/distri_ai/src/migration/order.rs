use anchor_lang::prelude::*;
use crate::state::order::*;

pub fn migrate_order_new(ctx: Context<MigrationOrderNew>) -> Result<()> {
    let order_before = &mut ctx.accounts.order_before;
    let order_after = &mut ctx.accounts.order_after;
    order_after.order_id = order_before.order_id;
    order_after.buyer = order_before.buyer;
    order_after.seller = order_before.seller;
    order_after.machine_id = order_before.machine_id;
    order_after.price = order_before.price;
    order_after.duration = order_before.duration;
    order_after.total = order_before.total;
    order_after.metadata = order_before.metadata.clone();
    order_after.status = order_before.status.clone();
    order_after.order_time = order_before.order_time;
    order_after.start_time = order_before.start_time;
    order_after.refund_time = order_before.refund_time;

    Ok(())
}

pub fn migrate_order_rename(ctx: Context<MigrationOrderRename>) -> Result<()> {
    let order_before = &mut ctx.accounts.order_before;
    let order_after = &mut ctx.accounts.order_after;
    order_after.order_id = order_before.order_id;
    order_after.buyer = order_before.buyer;
    order_after.seller = order_before.seller;
    order_after.machine_id = order_before.machine_id;
    order_after.price = order_before.price;
    order_after.duration = order_before.duration;
    order_after.total = order_before.total;
    order_after.metadata = order_before.metadata.clone();
    order_after.status = order_before.status.clone();
    order_after.order_time = order_before.order_time;
    order_after.start_time = order_before.start_time;
    order_after.refund_time = order_before.refund_time;
    order_after.model1_owner = order_before.model1_owner;
    order_after.model1_name = order_before.model1_name.clone();
    order_after.model2_owner = order_before.model2_owner;
    order_after.model2_name = order_before.model2_name.clone();
    order_after.model3_owner = order_before.model3_owner;
    order_after.model3_name = order_before.model3_name.clone();
    order_after.model4_owner = order_before.model4_owner;
    order_after.model4_name = order_before.model4_name.clone();
    order_after.model5_owner = order_before.model5_owner;
    order_after.model5_name = order_before.model5_name.clone();

    Ok(())
}

#[derive(Accounts)]
pub struct MigrationOrderNew<'info> {
    #[account(
        mut,
        close = signer
    )]
    pub order_before: Box<Account<'info, Order>>,

    #[account(
        init,
        seeds = [b"order-new", order_before.buyer.as_ref(), order_before.order_id.as_ref()],
        bump,
        payer = signer,
        space = 8 + OrderNew::INIT_SPACE
    )]
    pub order_after: Box<Account<'info, OrderNew>>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MigrationOrderRename<'info> {
    #[account(
        mut,
        close = signer
    )]
    pub order_before: Box<Account<'info, OrderNew>>,

    #[account(
        init,
        seeds = [b"order", order_before.buyer.as_ref(), order_before.order_id.as_ref()],
        bump,
        payer = signer,
        space = 8 + Order::INIT_SPACE
    )]
    pub order_after: Box<Account<'info, Order>>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
