extern crate env_logger;
extern crate reqwest;

#[macro_use]
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Credentials {
    access_key_id: String,
    secret_key: String,
    session_token: String,
    expiration: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct KryptonResponse {
    credentials: Credentials,
    identity_id: String,
}

fn main() -> Result<(), Box<std::error::Error>> {
    env_logger::init();
    let http_client = reqwest::Client::new();
    let res = http_client
        .post("https://krypton.soracom.io:8036/v1/provisioning/aws/cognito/credentials")
        .header("Content-Type", "application/json")
        .send()
        .unwrap();

    /* data replied by Krypton
    let json_str = r#"
        {
            "credetials": {
                "accessKeyId": "MyAccessKeyId",
                "expiration": "2019-02-09T08:39:14.701Z",
                "secretKey": "MySecretKey",
                "sessionToken": "MySessionToken"
            },
            "identityId": "MyIdentityID",
            "region": "MyRegion"
        }
    "#;
    */
    let kr: KryptonResponse = match serde_json::from_reader(res) {
        Ok(val) => val,
        Err(err) => {
            println!("{}", err);
            KryptonResponse {
                credentials: Credentials {
                    access_key_id: "".to_string(),
                    expiration: 0,
                    secret_key: "".to_string(),
                    session_token: "".to_string(),
                },
                identity_id: "err".to_string(),
            }
        }
    };
    println!("{:?}", kr);
    Ok(())
}

