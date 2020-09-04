use serde::{Serialize,Deserialize};
use actix_web::{HttpResponse,Result,web,error};
use crate::auth;
use crate::connect;


use diesel::dsl::*;
// use crate::schema::users::dsl::*;
use uuid::Uuid;

#[derive(Serialize,Deserialize,Debug)]
pub struct MyError<T:Clone>{
    err:T
}
#[derive(Serialize,Deserialize,Debug)]
pub struct Login{
    username:String,
    email:String
}
#[derive(Serialize,Deserialize,Debug)]
pub struct AuthKey{
    auth_token:String
}
impl AuthKey{
    pub fn new(key:String)->Self{
        Self{
            auth_token:key
        }
    }
}
pub async fn login_user(info:web::Json<Login>)->Result<HttpResponse,error::Error>{

   

    let found=user_exists(&info);
    if found{
        let k=auth::Claim::new(String::from(info.email.to_owned())).encode().unwrap();
        let f=AuthKey::new(k.to_owned());
        Ok(HttpResponse::Ok().json(f))
       

    }
   else{
    //    Ok(HttpResponse::NotFound())
       Err(error::ErrorNotFound("the user  does not exists.please provide correct credentials".to_owned()))
     
    //    Ok(HttpResponse::Ok().json("user does not exists"))
       
   }
  
 
}

pub async fn failure_handler ()->Result<String>{

    return Err(error::ErrorForbidden("Authentication is required"))
}
pub fn is_logged_in(item:String)->Result<bool>{
    let i=AuthKey::new(item);
    // test_login(i);
    let b=auth::Claim::decode(&i.auth_token);
    if let Ok(val)=b{
        Ok(true)

    }
    else{
      Err(error::ErrorBadGateway(false))
    }

   

    

}
pub async fn test_login(info:web::Query<AuthKey>)->Result<String>{

    let b=auth::Claim::decode(&info.auth_token);
    match  b {
        Ok(val)=> Ok(format!("Successfully logged in{:?}",val)),
        Err(e)=>Err(error::ErrorNotAcceptable("the auth details are invalid".to_owned()))

    }
    // println!("the token {:?}",b);

    // Ok(format!("Hello everying is fine {:?}",b))

   
}


pub  fn user_exists(info:&Login)->bool{
    use crate::diesel::prelude::*;
    use crate::schema::users ;
    use crate::schema::users::dsl::*;
    let conn=connect::establish_connection();

   
    let j:bool=select(exists(users::table.filter(username.eq(info.username.to_owned())).filter(email.eq(info.email.to_owned()))))
    .get_result(&conn).expect("error while trying ro query database");

    // needs to handle above appropriately
  
    j
    // users::table.filter(username.eq("ASdsa")).filter(email.eq("email")).get_result(&conn);

}
pub async fn test_uuid()->Result<HttpResponse>{
    let user_uuid=Uuid::new_v4().to_string();

    Ok(HttpResponse::Ok().json(user_uuid))
    

}