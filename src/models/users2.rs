use crate::schema::users2;
use serde::{Deserialize,Serialize};

// upgraded  user2

#[derive(Insertable,Deserialize,Serialize)]
#[table_name="users2"]
pub struct NewUser2{
    pub publicId:String,
    pub username:String,
    pub email:String
} 
#[derive(Insertable,Queryable,Serialize,Deserialize,Debug)]
#[table_name="users2"]
pub struct  User2{
    pub id:i32,
    pub publicId:String,
    pub username:String,
    pub email:String
}

#[derive(AsChangeset,Deserialize,Debug)]
#[table_name="users2"]
pub struct ChangeUser2{
    pub username:Option<String>,
    pub email:Option<String>
}