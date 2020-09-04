use chrono::{Duration,Utc};
use serde::{Serialize,Deserialize};
use jsonwebtoken::{encode,decode,Header,Validation,EncodingKey,DecodingKey,TokenData,errors::Error};
use dotenv::dotenv;

use std::env;
#[derive(Debug,Serialize,Deserialize)]
pub struct Claim{
    pub sub:String,
    pub company:String,
    pub exp:usize
}

impl Claim{
    fn get_secret_key<'a>()->Result<String,&'a str>{
        dotenv().ok();
        let key=env::var("SECRET_KEY").expect("key is required");
        Ok(key)
    }
  
    pub fn new(value_to_enocode:String)->Self{
        let k=Utc::now().naive_local()+ Duration::seconds(55);
        let uv=k.timestamp() as usize; 
        let g=Claim{
            sub:value_to_enocode.to_owned(),
            company:"dsfsdf".to_owned(),
            exp:uv
        };
        g
    }

    pub fn encode(&self)->Result<String,&str>{
        dotenv().ok();

        let key=Self::get_secret_key().unwrap();
      
        let token=encode(&Header::default(), &self, &
        EncodingKey::from_secret(key.as_ref())).expect("Error while trying to encode");
        Ok(token)


    }

    pub fn decode(token:&String)->Result<TokenData<Self>,Error>{
        let key=Self::get_secret_key().unwrap();
        let deserialized=decode::<Self>(token,
        &DecodingKey::from_secret(key.as_ref()),&Validation::default())?;
        Ok(deserialized)
    }


}