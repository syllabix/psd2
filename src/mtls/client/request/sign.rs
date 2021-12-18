use std::fs::{self, File};
use std::time::SystemTime;

use detached_jws::SerializeJwsWriter;
use hyper::header::HeaderValue;
use openssl::pkey::PKey;
use openssl::{hash::MessageDigest, sign::Signer};
use serde::Serialize;
use serde_json::{json, Map, Value};

use crate::mtls;

//  example:
//  {
//     "alg": "PS256",
//     "kid": "/C=GB/O=Crush Only B.V/OU=001580000103UAvAAM/CN=2kiXQyo0tedjW2somjSgH7/2.5.4.97=PSDUK-REVCA-22dc5285-6bb0-437b-8102-3773bfd7f6e6",
//     "crit": [
//       "http://openbanking.org.uk/iat",
//       "http://openbanking.org.uk/tan",
//       "http://openbanking.org.uk/iss",
//     ],
//     "http://openbanking.org.uk/tan": "openbanking.ork.uk",
//
//   }
//
// once you have a signed token - split on the '.' and concat with '..'

type JwsHeader = Map<String, Value>;

pub fn sign_request<T: Serialize>(req: &mut mtls::client::Request<T>) {
    let mut header = Map::new();
    header.insert("alg".to_string(), Value::String("PS256".to_string()));
    header.insert("kid".to_string(), Value::String("/C=GB/O=Crush Only B.V/OU=001580000103UAvAAM/CN=2kiXQyo0tedjW2somjSgH7/2.5.4.97=PSDUK-REVCA-22dc5285-6bb0-437b-8102-3773bfd7f6e6".to_string()));
    header.insert(
        "crit".to_string(),
        Value::Array(vec![
            Value::String("http://openbanking.org.uk/iat".to_string()),
            Value::String("http://openbanking.org.uk/tan".to_string()),
            Value::String("http://openbanking.org.uk/iss".to_string()),
        ]),
    );
    header.insert(
        "http://openbanking.org.uk/tan".to_string(),
        Value::String("openbanking.ork.uk".to_string()),
    );
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    header.insert(
        "http://openbanking.org.uk/iat".to_string(),
        Value::Number(serde_json::Number::from(now)),
    );
    header.insert(
        "http://openbanking.org.uk/iss".to_string(),
        Value::String("<client id>".to_string()),
    );

    let pkey = {
        let file = fs::read(".certificates/revolut/private.key").unwrap();
        PKey::private_key_from_pem(&file).unwrap()
    };

    let payload = serde_json::to_vec(&req.data).unwrap();

    let signer = Signer::new(MessageDigest::sha256(), &pkey).unwrap();

    let jws = detached_jws::serialize("PS256".to_string(), header, &mut payload.as_slice(), signer)
        .unwrap();

    let jws_header = HeaderValue::from_bytes(&jws[..]).unwrap();

    req.headers.insert("x-jws-signature", jws_header);
}
