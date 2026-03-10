use crate::models::invitation::{InvitationDTO, InvitationOutputDTO};

pub fn invitation_to_invitation_output(invitation: InvitationDTO) -> InvitationOutputDTO {
    let ends_at_seconds = invitation
        .ends_at
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    InvitationOutputDTO {
        guid: invitation.guid,
        is_active: invitation.is_active,
        invited_username: invitation.invited_username,
        ends_at: ends_at_seconds,
    }
}
