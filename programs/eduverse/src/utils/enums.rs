use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Eq, PartialEq, Debug, Copy)]
pub enum LessonState {
    #[default]
    Offer,
    Rejected,
    Approved,
    Started,
    Ended,
}
