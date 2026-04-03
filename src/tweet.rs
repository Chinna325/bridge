use crate::request::Tweet;
use crate::{Context, response};
use crate::{
    backend, request,
    response::PublicMetrics,
    service_request::{self, ServiceRequest},
    service_response::{self},
    tweet,
};
use prost::Message;

impl service_request::Tweet {
    pub async fn new(
        &self,
        type_of_add: service_request::TweetAdd,
        tweet_data: Vec<u8>,
    ) -> Result<(), ()> {
        let req = ServiceRequest {
            operation: Some(service_request::service_request::Operation::AddTweet(
                service_request::AddTweet {
                    tweet: Some(self.clone()),
                    tweet_add,
                    tweet_data,
                },
            )),
        };
        let mut conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(Err(()))?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::AddTweet(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Err(());
            }
        }
        Ok(())
    }
    pub async fn from_uuid(uuid: Vec<u8>) -> Result<Self, ()> {
        let req = ServiceRequest {
            operation: Some(service_request::service_request::Operation::GetTweet(
                service_request::GetTweet { tweet_id: uuid },
            )),
        };
        let mut conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(Err(()))?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::GetTweet(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Err(());
            }
            let tweet_data = resp.tweet_data.clone();
            let tweet = service_request::Tweet::decode(tweet_data.as_slice());
            if tweet.is_err() {
                return Err(());
            }
            return Ok(tweet.unwrap());
        }
        Err(())
    }
    pub async fn remove(&self, tweet_remove: service_request::TweetRemove) -> Result<(), ()> {
        let req = ServiceRequest {
            operation: Some(service_request::service_request::Operation::RemoveTweet(
                service_request::RemoveTweet {
                    tweet_remove: tweet_remove as i32,
                },
            )),
        };
        let mut conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(Err(()))?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::RemoveTweet(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Err(());
            }
        }
        Ok(())
    }
    pub async fn update(&mut self) -> Result<(), ()> {
        let req = ServiceRequest {
            operation: Some(service_request::service_request::Operation::UpdateTweet(
                service_request::UpdateTweet {
                    tweet_data: self.clone().encode_to_vec(),
                    tweet_id: self.tweet_id.clone(),
                },
            )),
        };
        let mut conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(Err(()))?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::UpdateTweet(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Err(());
            }
        }
        Ok(())
    }
    pub async fn list() -> Result<Vec<Vec<u8>>, ()> {
        let req = ServiceRequest {
            operation: Some(service_request::service_request::Operation::ListTweets(
                service_request::ListTweets {},
            )),
        };
        let mut conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(Err(()))?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::ListTweets(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Err(());
            }
            return Ok(resp.tweet_ids);
        }
        Err(())
    }

    pub async fn add_like(&mut self) -> Result<(), ()> {
        todo!()
    }
    pub async fn remove_like(&mut self) -> Result<(), ()> {
        todo!()
    }

    pub async fn add_dislike(&mut self) -> Result<(), ()> {
        todo!()
    }
    pub async fn remove_dislike(&mut self) -> Result<(), ()> {
        todo!()
    }

    pub async fn add_love(&mut self) -> Result<(), ()> {
        todo!()
    }
    pub async fn remove_love(&mut self) -> Result<(), ()> {
        todo!()
    }
    pub fn form(tweet: request::Tweet) -> Self {
        Self {
            tweet_id: tweet.tweet_id.clone(),
            text: tweet.text.clone(),
            created_at: Utc::now().timestamp() as u64,
            owner: tweet.owner.clone(),
            public_metrics: Some(service_request::PublicMetrics::default()),
            hashtags: tweet.hashtags.clone(),
            user_names: tweet.user_names.clone(),
        }
    }
}

impl request::AddTweet {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(errors::form_response("AddTweet", response::Status::BackendError).await);
        }
        let tweet = self.tweet.clone();
        if tweet.is_none() {
            return Some(errors::form_response("AddTweet", response::Status::BackendError).await);
        }
        let tweet = tweet.unwrap();
        let mut tweet_id = tweet.tweet_id.clone();
        let mut tweet_add = service_request::TweetAdd::Add;
        if tweet_id.is_empty() {
            let uuid = Uuid::new_v4();
            let mut bytes = uuid.as_bytes().to_vec();
            let millis = chrono::Utc::now().timestamp_millis() as u64;
            bytes.extend_from_slice(&millis.to_be_bytes());
            tweet_id = bytes;
            let tweet = service_request::Tweet::form(tweet.clone());
            let resp = tweet.new(tweet_add, tweet.encode_to_vec()).await;
            if resp.is_err() {
                return Some(
                    errors::form_response("AddTweet", response::Status::BackendError).await,
                );
            }
        } else {
            let tweet = service_request::Tweet::from_uuid(tweet_id.clone()).await;
            if tweet.is_err() {
                return Some(
                    errors::form_response("AddTweet", response::Status::BackendError).await,
                );
            }
            let tweet = tweet.unwrap();
            tweet_add = service_request::TweetAdd::Repost;
            let resp = tweet.new(tweet_add, Vec::new()).await;
            if resp.is_err() {
                return Some(
                    errors::form_response("AddTweet", response::Status::BackendError).await,
                );
            }
        }
        Some(response::Response {
            operation: Some(response::response::Operation::AddTweet(
                response::AddTweet {
                    status: response::Status::Success as i32,
                    message: None,
                    tweet_id: tweet_id.clone(),
                },
            )),
        })
    }
}

impl request::RemoveTweet {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(
                errors::form_response("RemoveTweet", response::Status::BackendError).await,
            );
        }
        let tweet = service_request::Tweet::from_uuid(self.tweet_id.clone()).await;
        if tweet.is_err() {
            return Some(
                errors::form_response("RemoveTweet", response::Status::BackendError).await,
            );
        }
        let tweet = tweet.unwrap();
        let mut tweet_remove = service_request::TweetRemove::OwnTweet;
        if ctx.email != tweet.owner {
            tweet_remove = service_request::TweetRemove::RepostedTweet;
        }
        let resp = tweet.remove(tweet_remove).await;
        if resp.is_err() {
            return Some(
                errors::form_response("RemoveTweet", response::Status::BackendError).await,
            );
        }
        Some(response::Response {
            operation: Some(response::response::Operation::RemoveTweet(
                response::RemoveTweet {
                    status: response::Status::Success as i32,
                    message: None,
                },
            )),
        })
    }
}

impl request::GetTweet {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(errors::form_response("GetTweet", response::Status::BackendError).await);
        }
        let tweet = service_request::Tweet::from_uuid(self.tweet_id.clone()).await;
        if tweet.is_err() {
            return Some(errors::form_response("GetTweet", response::Status::BackendError).await);
        }
        let tweet = tweet.unwrap();
        let metrics = tweet.public_metrics.clone().unwrap();
        let public_metrics = response::PublicMetrics {
            retweet_count: metrics.retweet_count,
            reply_count: metrics.reply_count,
            like_count: metrics.like_count,
            quote_count: metrics.quote_count,
            bookmark_count: metrics.bookmark_count,
            impression_count: metrics.impression_count,
        };
        let tweet = response::Tweet {
            tweet_id: self.tweet_id.clone(),
            text: tweet.text.clone(),
            created_at: tweet.created_at,
            owner: tweet.owner.clone(),
            hashtags: tweet.hashtags.clone(),
            user_names: tweet.user_names.clone(),
            public_metrics: Some(public_metrics),
        };
        Some(response::Response {
            operation: Some(response::response::Operation::GetTweet(
                response::GetTweet {
                    status: response::Status::Success as i32,
                    message: None,
                    tweet: Some(tweet),
                },
            )),
        })
    }
}

impl request::ListTweets {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(errors::form_response("ListTweets", response::Status::BackendError).await);
        }
        let tweets = service_request::Tweet::list().await;
        if tweets.is_err() {
            return Some(errors::form_response("ListTweets", response::Status::BackendError).await);
        }
        let tweets = tweets.unwrap();
        Some(response::Response {
            operation: Some(response::response::Operation::ListTweets(
                response::ListTweets {
                    status: response::Status::Success as i32,
                    message: None,
                    tweets: tweets,
                },
            )),
        })
    }
}

impl request::UpdateTweet {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(
                errors::form_response("UpdateTweet", response::Status::BackendError).await,
            );
        }
        let tweet = self.tweet.clone();
        if tweet.is_none() {
            return Some(
                errors::form_response("UpdateTweet", response::Status::BackendError).await,
            );
        }
        let tweet = tweet.unwrap();
        let tweet = service_request::Tweet::from_uuid(self.tweet_id.clone()).await;
        if tweet.is_err() {
            return Some(
                errors::form_response("UpdateTweet", response::Status::BackendError).await,
            );
        }
        let mut tweet = tweet.unwrap();
        tweet.hashtags = self.hash_tags.clone();
        tweet.user_names = self.user_names.clone();
        tweet.text = self.text.clone();
        let resp = tweet.update().await;
        if resp.is_err() {
            return Some(
                errors::form_response("UpdateTweet", response::Status::BackendError).await,
            );
        }
        Some(response::Response {
            operation: Some(response::response::Operation::UpdateTweet(
                response::UpdateTweet {
                    status: response::Status::Success as i32,
                    message: None,
                },
            )),
        })
    }
}
