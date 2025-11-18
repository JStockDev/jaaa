use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

use crate::error::CodeError;

impl JWT {
    pub fn now(user_id: Uuid) -> Self {
        JWT {
            header: JWTHeader::default(),
            body: JWTBody {
                sub: user_id,
                exp: OffsetDateTime::now_utc() + Duration::minutes(15),
            },
        }
    }

    pub fn encode(&self) -> String {
        let mut jwt = String::new();

        let header = serde_json::to_string(&self.header).unwrap();
        let body = serde_json::to_string(&self.body).unwrap();

        let header_b64 = BASE64_URL_SAFE_NO_PAD.encode(header);
        let body_b64 = BASE64_URL_SAFE_NO_PAD.encode(body);

        jwt.push_str(&header_b64);
        jwt.push('.');
        jwt.push_str(&body_b64);

        jwt
    }

    pub fn encode_and_sign(&self, key: &[u8]) -> String {
        let mut jwt = self.encode();

        let mut signer = Hmac::<Sha256>::new_from_slice(key).unwrap();
        signer.update(jwt.as_bytes());
        jwt.push('.');
        jwt.push_str(&BASE64_URL_SAFE_NO_PAD.encode(signer.finalize().into_bytes()));

        jwt
    }

    pub fn decode(raw: String) -> Result<(Self, Vec<u8>), CodeError> {
        let parts: Vec<String> = raw.split('.').map(|d| d.to_string()).collect();

        if parts.len() != 3 {
            return Err(CodeError::DecodeError);
        }

        let raw_header = BASE64_URL_SAFE_NO_PAD
            .decode(&parts[0])
            .map_err(|_| CodeError::DecodeError)?;
        let raw_body = BASE64_URL_SAFE_NO_PAD
            .decode(&parts[1])
            .map_err(|_| CodeError::DecodeError)?;
        let signature = BASE64_URL_SAFE_NO_PAD
            .decode(&parts[2])
            .map_err(|_| CodeError::DecodeError)?;

        let header: JWTHeader = serde_json::from_str(
            &String::from_utf8(raw_header).map_err(|_| CodeError::DecodeError)?,
        )
        .map_err(|_| CodeError::DecodeError)?;
        let body: JWTBody =
            serde_json::from_str(&String::from_utf8(raw_body).map_err(|_| CodeError::DecodeError)?)
                .map_err(|_| CodeError::DecodeError)?;

        Ok((Self { header, body }, signature))
    }

    pub fn verify_signature(&self, signature: Vec<u8>, key: &[u8]) -> bool {
        if signature.len() != 32 {
            return false
        }
        
        let mut signer = Hmac::<Sha256>::new_from_slice(key).unwrap();
        signer.update(self.encode().as_bytes());

        signer.verify(signature.as_slice().into()).is_ok()
    }

    pub fn get_user_id(&self) -> Uuid {
        self.body.sub
    }

    pub fn get_expiry(&self) -> OffsetDateTime {
        self.body.exp
    }
}

pub(super) struct JWT {
    header: JWTHeader,
    body: JWTBody,
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct JWTBody {
    sub: Uuid,
    #[serde(with = "numeric_datetime")]
    exp: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub(super) struct JWTHeader {
    typ: JWTType,
    alg: Algorithm,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub(super) enum Algorithm {
    #[default]
    HS256,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub(super) enum JWTType {
    #[default]
    JWT,
}

mod numeric_datetime {
    use serde::{self, Deserialize, Deserializer, Serializer};
    use time::OffsetDateTime;

    pub fn serialize<S>(date: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.unix_timestamp());
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let s = s.parse::<i64>().map_err(serde::de::Error::custom)?;
        let dt = OffsetDateTime::from_unix_timestamp(s).map_err(serde::de::Error::custom)?;
        Ok(dt)
    }
}
