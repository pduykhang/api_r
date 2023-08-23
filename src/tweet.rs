
use actix_web::HttpResponse;
use actix_web::web::{Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::constants::APPLICATION_JSON;
use crate::response::Response;
use crate::like;


pub type Tweets = Response<Tweet>;

#[derive(Deserialize,Serialize,Debug)]
pub struct Tweet {
    pub id:String,
    pub created_at : DateTime<Utc>,
    pub message:String,
    pub likes: Vec<like::Like>
}

impl Tweet {
    pub fn new(message:String) -> Self{
        Self{
            id:Uuid::new_v4().to_string(),
            created_at:Utc::now(),
            message,
            likes:vec![]
        }
    }
}
#[derive(Debug,Deserialize,Serialize)]
pub struct TweetRequest {
    pub message :Option<String>
}

impl TweetRequest{
    pub fn to_tweet(&self) -> Option<Tweet>{
        match &self.message {
            Some(message) => Some(Tweet::new(message.to_string())),
            _ => None
        }
    }
}
#[get("/tweets")]
pub async fn list() ->HttpResponse{
    let tweets = Tweets{results:vec![]};
    HttpResponse::Ok().content_type(crate::constants::APPLICATION_JSON).json(tweets)
}
#[post("/tweets")]
pub async fn create(tweet_req:Json<TweetRequest>)->HttpResponse {
    HttpResponse::Created().content_type(APPLICATION_JSON).json(tweet_req.to_tweet())
}

#[get("/tweets/{id}")]
pub async fn get ()-> HttpResponse{
    let found_tweet:Option<Tweet> =None;
    match found_tweet {
        Some(tweet)=> HttpResponse::Ok().content_type(APPLICATION_JSON).json(tweet),
        None=> HttpResponse::NoContent().content_type(APPLICATION_JSON).await.unwrap()
    }
}

/// delete a tweet by its id `/tweets/{id}`
#[delete("/tweets/{id}")]
pub async fn delete() -> HttpResponse {
    // TODO delete tweet by ID
    // in any case return status 204

    HttpResponse::NoContent()
        .content_type(APPLICATION_JSON)
        .await
        .unwrap()
}

