use crate::api::*;
use crate::error::*;

pub enum Event {
    CreatePledge(Pledge),
    UpdatePledge(Pledge),
    DeletePledge(Pledge),
    CreateMember(Member),
    UpdateMember(Member),
    DeleteMember(Member),
    CreateMemberPledge(Member),
    UpdateMemberPledge(Member),
    DeleteMemberPledge(Member),
}

#[derive(Debug, Default)]
pub struct Webhook {
    pub webhook_secret: String,
}

impl Webhook {
    pub fn check_signature(&self, body: &[u8], signature: &str) -> PatreonResult<bool> {
        use hmac::{Hmac, Mac};
        use md5::Md5;
        type HmacMd5 = Hmac<Md5>;
        let mut mac = HmacMd5::new_from_slice(self.webhook_secret.as_bytes())
            .map_err(|_| PatreonError::Message("Invalid hmac key length".to_string()))?;
        mac.update(body);
        let local = hex::encode(mac.finalize().into_bytes().as_slice());
        Ok(local.eq(signature))
    }

    pub fn parse_event(&self, body: &[u8], trigger: &str) -> PatreonResult<Event> {
        match trigger {
            "pledges:create" => Ok(Event::CreatePledge(DocResponse::parse(body)?)),
            "pledges:update" => Ok(Event::UpdatePledge(DocResponse::parse(body)?)),
            "pledges:delete" => Ok(Event::DeletePledge(DocResponse::parse(body)?)),
            "members:create" => Ok(Event::CreateMember(DocResponse::parse(body)?)),
            "members:update" => Ok(Event::UpdateMember(DocResponse::parse(body)?)),
            "members:delete" => Ok(Event::DeleteMember(DocResponse::parse(body)?)),
            "members:pledge:create" => Ok(Event::CreateMemberPledge(DocResponse::parse(body)?)),
            "members:pledge:update" => Ok(Event::UpdateMemberPledge(DocResponse::parse(body)?)),
            "members:pledge:delete" => Ok(Event::DeleteMemberPledge(DocResponse::parse(body)?)),
            _ => Err(PatreonError::Message(format!("unknown trigger: {trigger}"))),
        }
    }
}
