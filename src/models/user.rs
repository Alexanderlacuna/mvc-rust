use crate::schema::users;
use serde::{Deserialize,Serialize};


#[derive(Insertable,Deserialize,Serialize)]
#[table_name="users"]
pub struct NewUser{
    pub username:String,
    pub email:String
}

#[derive(Insertable,Queryable,Serialize,Deserialize,Debug)]
pub struct  User{
    pub id:Option<i32>,
    pub username:String,
    pub email:String
}

#[derive(AsChangeset,Deserialize,Debug)]
#[table_name="users"]
pub struct ChangeUser{
    pub username:Option<String>,
    pub email:Option<String>
}
