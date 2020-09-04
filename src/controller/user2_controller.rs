
use crate::models::users2::{NewUser2,User2};
use serde::{Deserialize,Serialize};
use actix_web::{web,Result,HttpResponse};
use uuid::Uuid;
use crate::diesel;

use crate::diesel::prelude::*;
use crate::schema::users2;
use crate::connect;
#[derive(Deserialize,Serialize,Debug)]
pub struct Details{
    pub username:String,
    pub email:String,
}
pub async fn new_user(info:web::Json<Details>)->String{
    let public_id=Uuid::new_v4().to_string();
    let user_1=NewUser2{
        publicId:public_id,
        username:info.email.to_owned(),
        email:info.username.to_owned()    
    };
    let conn=connect::establish_connection();
    let results=diesel::insert_into(users2::table).values(&user_1).execute(&conn).expect("error while tr
    ying to save");


    format!("success inserted user {}",results)
    // let h=self::get_user(results as i32 )
}
#[derive(Deserialize,Serialize,Debug)]
pub struct UserId{
    id:i32
}
#[derive(Deserialize,Serialize,Debug)]
pub struct PublicId{
    uuid:String,
    username:String,
}



pub async fn get_user(info:web::Query<UserId>)->Result<HttpResponse>{
    use crate::schema::users2::dsl::*;
    let conn=connect::establish_connection();
    let user_found=users2.find(info.id).limit(1).get_results::<User2>(&conn)
    .expect("An error occurred");
    if user_found.len()>0{
       
       let yt=&user_found[0].publicId;
       let un=&user_found[0].username;
        Ok(HttpResponse::Ok().json(PublicId{uuid:yt.to_string(),username:un.to_string()}))

    }
    else{
        Ok(HttpResponse::Ok().json("No user exist with id {}"))
    }
  
}