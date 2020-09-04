extern crate serde;
extern crate serde_json;
use crate::schema::users as urs;
use crate::models::user::{NewUser,User,ChangeUser};
use crate::diesel;
use crate::connect;
use crate::diesel::prelude::*;
use actix_web::{web,Result,HttpResponse};
use serde::{Deserialize,Serialize};


#[derive(Deserialize)]

pub struct UserId{
    pub id:i32
}

#[derive(Deserialize,Serialize)]
pub struct AllUsers{
    users:Vec<User>
}
#[derive(Deserialize,Serialize)]
pub struct  Create {
    pub username:String,
    pub email:String
}

pub async fn test_user()->String{
    format!("Hello loud and clear")
}

pub async fn create_user(info:web::Json<Create>)->String{
    let conn=connect::establish_connection();
    let inst_user=NewUser{
        username:info.username.clone(),
        email:info.email.clone()
    };
    let _=diesel::insert_into(urs::table).values(&inst_user).execute(&conn).expect("Error occurred");
    format!("Dfsdf")
}
// implement only by only admin
pub async fn get_users()->Result<HttpResponse>{
    use crate::schema::users::dsl::*;
    let conn=connect::establish_connection();
    let people=users.load::<User>(&conn).expect("Error loading post");
    // let all=AllUsers{
    //     users:people
    // };
   
//    let b=serde_json::to_string(&f).unwrap();
   Ok(HttpResponse::Ok().json(people))

}

pub async fn get_user(user_id:web::Query<UserId>)->Result<HttpResponse>{
    println!("the user id is {:?}",user_id.id);
    use crate::schema::users::dsl::*;
    let conn=connect::establish_connection();
    let user_found=users.find(user_id.id).limit(1).get_results::<User>(&conn).expect("An error occurred");
    Ok(HttpResponse::Ok().json(user_found))
   
  

}
pub async fn delete_user(user_id:web::Query<UserId>){
    use crate::schema::users::dsl::*;
 
    let conn=connect::establish_connection();
    let _=  diesel::delete(urs::table.filter(id.eq(user_id.id))).execute(&conn).expect("Error occurred") ;
 
}
// info:web::Json<ChangeUser>
#[derive(Deserialize,Debug)]
pub struct HolderChange{
    username:String,
    email:String,
}
pub async fn update_user(info:web::Json<HolderChange>,user_id:web::Query<UserId>)->String{
    use crate::schema::users::dsl::*;
    use diesel::*;
    let conn=connect::establish_connection();
    let changes=ChangeUser{
        email:sanitize(info.email.clone()),
        username:sanitize(info.username.clone())
    };
    // let emails=if info.email!=""{
    //     Some(info.email.to_owned())
    // }else{
    //     None
    // };
    // let usernames=if info.username!=""{
    //     Some(info.username.to_owned())
    // }else{
    //     None
    // };
    // let changes=ChangeUser{
    //     email:emails,
    //     username:usernames
    // };
    
 
    diesel::update(users.filter(id.eq(user_id.id))).set(&changes).execute(&conn).expect("an error occurred");
   

    format!("it was a success")

}

pub fn sanitize(item:String)->Option<String>{
    if item!=""{
        Some(item)
    }
    else{
        None
    }
  

}