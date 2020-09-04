use actix_web::{web,guard,error,HttpResponse};
use actix_service::{Service};
use crate::controller::user_controller::{self};
use crate::controller::user2_controller;
use crate::controller::auth;
use crate::middleware::SayHiMiddleware;



pub fn scoped_config(cfg:&mut web::ServiceConfig){
    cfg.service(
        
       web::scope("/user")
       
       .route("/test",web::get()
       .to(user_controller::test_user))
      

       .wrap_fn(|req, srv| {
        println!("Hi from start. You requested: {}", req.path());
        let res=srv.call(req);
       res 

       })
     
      
       
       .wrap_fn(|req,srv|{
           println!("i am in the middleware");
           
           let v=req.query_string().to_string();
           
        
           println!("this is it {:?}",v);
           let y=String::from(v);
           let length=y.len();
         

       
        
    
           let fut=srv.call(req)
           ;

           let t=4;
           let mut l=async {
               if 4
               >3{
                   
                   Ok(fut.await?)
               }
               else{
                return Err(error::ErrorForbidden("no authentication"));

               }
           };

          l
        //    async {
        //        let mut res=fut.await?;
        //     Ok(res)
        //    }
          

          
          
         
        //    async{
           
              
               
               
        //        let mut res=fut.await?;
        //        if (4>1){
        //            return Err(error::ErrorForbidden("no authentication"));
        //        }
        //       Ok(res)

            
        //    }
       }) 
       
        .route("/createUser",web::post().to(user_controller::create_user))
        .route("/getUsers",web::get().to(user_controller::get_users))
        .route("/getUser",web::get().to(user_controller::get_user))
        .route("/update",web::post().to(user_controller::update_user))
        .route("/login",web::post().to(auth::login_user))
        .route("/testLogin",web::get().to(auth::test_login))
        .route("/getUid",web::get().to(auth::test_uuid))
      
        
        

    );
    cfg.service(
        web::scope("/user2")
        .route("/getUser", web::get().to(user2_controller::get_user))
        .route("/createUser",web::post().to(user2_controller::new_user))

    );
}
pub fn testing(){
    // let v:i32="3".parse().unwrap();
}