use candid::Principal;
use canister_types::models::{
    api_error::ApiError,
    profile::{PostProfile, ProfileResponse},
};

use crate::utils::{call, Context};

pub fn add_profile(
    ctx: &Context,
    sender: Principal,
    input: (PostProfile, Principal),
) -> Result<ProfileResponse, ApiError> {
    call::<(PostProfile, Principal), (Result<ProfileResponse, ApiError>,)>(
        ctx,
        sender,
        "add_profile",
        input,
    )
    .0
}
