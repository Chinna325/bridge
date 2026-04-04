// use crate::request::Tweet;
use crate::errors;
use crate::response::Response;
use crate::{Context, response};
use crate::{
    backend,
    request,
    // response::PublicMetrics,
    service_request::{self, ServiceRequest},
    service_response::{self},
    // tweet,
};
use chrono::Utc;
use prost::Message;
use uuid::Uuid;

impl service_request::Tweet {
    pub async fn new(
        &self,
        type_of_add: service_request::TweetAdd,
        tweet_data: Vec<u8>,
    ) -> Result<(), ()> {
        let req = ServiceRequest {
            operation: Some(service_request::service_request::Operation::AddTweet(
                service_request::AddTweet {
                    user_name: self.owner.clone(),
                    tweet_data: tweet_data,
                    tweet_add: type_of_add as i32,
                    tweet_id: self.tweet_id.clone(),
                },
            )),
        };
        let conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(())?;
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
        let conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(())?;
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
                    tweet_id: self.tweet_id.clone(),
                },
            )),
        };
        let conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(())?;
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
        let conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(())?;
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
    pub async fn list(user_name: String) -> Result<Vec<Vec<u8>>, ()> {
        let req = ServiceRequest {
            operation: Some(service_request::service_request::Operation::ListTweets(
                service_request::ListTweets { user_name },
            )),
        };
        let conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(())?;
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

    pub async fn react_to_tweet(&mut self) -> Result<(), ()> {
        let req = ServiceRequest {
            operation: Some(service_request::service_request::Operation::TweetReact(
                service_request::TweetReact {
                    tweet_id: self.tweet_id.clone(),
                    tweet_data: self.clone().encode_to_vec(),
                },
            )),
        };
        let conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(())?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::TweetReact(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Err(());
            }
        }
        Ok(())
    }

    pub async fn list_replies(
        &self,
        parent_id: Vec<u8>,
    ) -> Result<Vec<service_response::Reply>, ()> {
        let req = ServiceRequest {
            operation: Some(service_request::service_request::Operation::ListReplies(
                service_request::ListReplies {
                    tweet_id: self.tweet_id.clone(),
                    parent_id: parent_id.clone(),
                },
            )),
        };
        let conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(())?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::ListReplies(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Err(());
            }
            let replies = resp.replies.clone();
            return Ok(replies);
        }
        Err(())
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

impl service_request::Reply {
    pub async fn new(&self) -> Result<(), ()> {
        let req = ServiceRequest {
            operation: Some(service_request::service_request::Operation::AddReply(
                service_request::AddReply {
                    reply: Some(self.clone()),
                },
            )),
        };
        let conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(())?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::AddReply(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Err(());
            }
        }
        Ok(())
    }

    pub async fn remove(&self) -> Result<(), ()> {
        let req = ServiceRequest {
            operation: Some(service_request::service_request::Operation::RemoveReply(
                service_request::RemoveReply {
                    reply_id: self.reply_id.clone(),
                    parent_id: self.parent_id.clone(),
                    tweet_id: self.tweet_id.clone(),
                },
            )),
        };
        let conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(())?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::RemoveReply(resp)),
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
            operation: Some(service_request::service_request::Operation::UpdateReply(
                service_request::UpdateReply {
                    reply_id: self.reply_id.clone(),
                    parent_id: self.parent_id.clone(),
                    text: self.text.clone(),
                    hash_tags: self.hash_tags.clone(),
                    user_name: self.user_names.clone(),
                    tweet_id: self.tweet_id.clone(),
                },
            )),
        };
        let conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(())?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::UpdateReply(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Err(());
            }
        }
        Ok(())
    }

    pub async fn from_uuid(
        tweet_id: Vec<u8>,
        reply_id: Vec<u8>,
        parent_id: Vec<u8>,
    ) -> Result<Self, ()> {
        let req = ServiceRequest {
            operation: Some(service_request::service_request::Operation::GetReply(
                service_request::GetReply {
                    reply_id: reply_id.clone(),
                    parent_id: parent_id.clone(),
                    tweet_id: tweet_id.clone(),
                },
            )),
        };
        let conn = backend::ceate_grpc_connection().await;
        let resp = req.execute(conn).await.ok_or(())?;
        if let service_response::ServiceResponse {
            operation: Some(service_response::service_response::Operation::GetReply(resp)),
        } = resp
        {
            if resp.status != service_response::Status::Success as i32 {
                return Err(());
            }
            let reply = resp.reply.clone();
            if reply.is_none() {
                return Err(());
            }
            let reply = reply.unwrap();
            return Ok(Self {
                tweet_id,
                user_name: reply.user_name.clone(),
                text: reply.text.clone(),
                reply_id,
                parent_id,
                likes: reply.likes,
                dislikes: reply.dislikes,
                created_at: reply.created_at,
                hash_tags: reply.hash_tags.clone(),
                user_names: reply.user_names.clone(),
            });
        }
        Err(())
    }
    pub async fn react_reply(&self) -> Result<(), ()> {
        Ok(())
    }

    pub fn from(reply: request::Reply) -> Self {
        Self {
            tweet_id: reply.tweet_id.clone(),
            user_name: reply.user_name.clone(),
            text: reply.text.clone(),
            reply_id: reply.reply_id.clone(),
            parent_id: reply.parent_id.clone(),
            likes: reply.likes,
            dislikes: reply.dislikes,
            created_at: 0_u64,
            hash_tags: Vec::new(),
            user_names: Vec::new(),
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
            let mut tweet = service_request::Tweet::form(tweet.clone());
            tweet.tweet_id = tweet_id.clone();
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
        let tweets = service_request::Tweet::list(self.user_name.clone()).await;
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
        let tweet = service_request::Tweet::from_uuid(self.tweet_id.clone()).await;
        if tweet.is_err() {
            return Some(
                errors::form_response("UpdateTweet", response::Status::BackendError).await,
            );
        }
        let mut tweet = tweet.unwrap();
        if tweet.owner != ctx.email {
            return Some(
                errors::form_response("UpdateTweet", response::Status::BackendError).await,
            );
        }
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

impl request::TweetReact {
    pub fn from(number: i32) -> Self {
        match number {
            0 => Self::Like,
            1 => Self::Love,
            2 => Self::DisLike,
            _ => Self::Like,
        }
    }
}

impl request::UndoTweetReact {
    pub fn from(number: i32) -> Self {
        match number {
            0 => Self::UndoLike,
            1 => Self::UndoDisLike,
            2 => Self::UndoLove,
            _ => Self::UndoLike,
        }
    }
}
impl request::ReactToTweet {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(
                errors::form_response("ReactToTweet", response::Status::BackendError).await,
            );
        }
        let tweet = service_request::Tweet::from_uuid(self.tweet_id.clone()).await;
        if tweet.is_err() {
            return Some(
                errors::form_response("ReactToTweet", response::Status::BackendError).await,
            );
        }
        let mut tweet = tweet.unwrap();
        let metrics = tweet.public_metrics.clone();
        let mut metrics = metrics.unwrap();
        let operation = request::TweetReact::from(self.tweet_react);
        let resp = match operation {
            request::TweetReact::Like => {
                metrics.like_count += 1;
                tweet.public_metrics = Some(metrics.clone());
                tweet.react_to_tweet().await
            }
            request::TweetReact::Love => {
                metrics.love_count += 1;
                tweet.public_metrics = Some(metrics.clone());
                tweet.react_to_tweet().await
            }
            request::TweetReact::DisLike => {
                metrics.dislike_count += 1;
                tweet.public_metrics = Some(metrics.clone());
                tweet.react_to_tweet().await
            }
        };
        if resp.is_err() {
            return Some(
                errors::form_response("ReactToTweet", response::Status::BackendError).await,
            );
        }
        Some(response::Response {
            operation: Some(response::response::Operation::ReactToTweet(
                response::ReactToTweet {
                    status: response::Status::Success as i32,
                    message: None,
                },
            )),
        })
    }
}

impl request::UndoReactToTweet {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(
                errors::form_response("UndoReactToTweet", response::Status::BackendError).await,
            );
        }
        let tweet = service_request::Tweet::from_uuid(self.tweet_id.clone()).await;
        if tweet.is_err() {
            return Some(
                errors::form_response("UndoReactToTweet", response::Status::BackendError).await,
            );
        }
        let mut tweet = tweet.unwrap();
        let metrics = tweet.public_metrics.clone();
        let mut metrics = metrics.unwrap();
        let operation = request::UndoTweetReact::from(self.tweet_react);
        let resp = match operation {
            request::UndoTweetReact::UndoDisLike => {
                metrics.dislike_count -= 1;
                tweet.public_metrics = Some(metrics.clone());
                tweet.react_to_tweet().await
            }
            request::UndoTweetReact::UndoLike => {
                metrics.like_count -= 1;
                tweet.public_metrics = Some(metrics.clone());
                tweet.react_to_tweet().await
            }
            request::UndoTweetReact::UndoLove => {
                metrics.love_count -= 1;
                tweet.public_metrics = Some(metrics.clone());
                tweet.react_to_tweet().await
            }
        };
        if resp.is_err() {
            return Some(
                errors::form_response("UndoReactToTweet", response::Status::BackendError).await,
            );
        }
        Some(response::Response {
            operation: Some(response::response::Operation::UndoReactToTweet(
                response::UndoReactToTweet {
                    status: response::Status::Success as i32,
                    message: None,
                },
            )),
        })
    }
}

impl request::RepostTweet {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(
                errors::form_response("RepostTweet", response::Status::BackendError).await,
            );
        }
        let tweet = service_request::Tweet::from_uuid(self.tweet_id.clone()).await;
        if tweet.is_err() {
            return Some(
                errors::form_response("RepostTweet", response::Status::BackendError).await,
            );
        }
        let mut tweet = tweet.unwrap();
        if let Some(metrics) = tweet.public_metrics.as_mut() {
            metrics.retweet_count += 1;
        }
        let resp = tweet
            .new(service_request::TweetAdd::Repost, tweet.encode_to_vec())
            .await;
        if resp.is_err() {
            return Some(
                errors::form_response("RepostTweet", response::Status::BackendError).await,
            );
        }
        Some(response::Response {
            operation: Some(response::response::Operation::RepostTweet(
                response::RepostTweet {
                    status: response::Status::Success as i32,
                    message: None,
                },
            )),
        })
    }
}

impl request::ReplyToTweet {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(
                errors::form_response("ReplyToTweet", response::Status::BackendError).await,
            );
        }
        if self.reply.clone().is_none() {
            return Some(
                errors::form_response("ReplyToTweet", response::Status::BackendError).await,
            );
        }
        let reply = self.reply.clone().unwrap();
        let mut reply = service_request::Reply::from(reply);
        reply.created_at = chrono::Utc::now().timestamp() as u64;
        let uuid = Uuid::new_v4();
        let mut bytes = uuid.as_bytes().to_vec();
        let millis = chrono::Utc::now().timestamp_millis() as u64;
        bytes.extend_from_slice(&millis.to_be_bytes());
        reply.reply_id = bytes.clone();
        let resp = reply.new().await;
        if resp.is_err() {
            return Some(
                errors::form_response("ReplyToTweet", response::Status::BackendError).await,
            );
        }
        Some(response::Response {
            operation: Some(response::response::Operation::ReplyToTweet(
                response::ReplyToTweet {
                    status: response::Status::Success as i32,
                    message: None,
                    reply_id: bytes,
                },
            )),
        })
    }
}

impl request::EditReply {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(errors::form_response("EditReply", response::Status::BackendError).await);
        }
        let tweet = service_request::Tweet::from_uuid(self.tweet_id.clone()).await;
        if tweet.is_err() {
            return Some(errors::form_response("EditReply", response::Status::BackendError).await);
        }
        let tweet = tweet.unwrap();
        let reply = service_request::Reply::from_uuid(
            tweet.tweet_id.clone(),
            self.reply_id.clone(),
            self.parent_id.clone(),
        )
        .await;
        if reply.is_err() {
            return Some(errors::form_response("EditReply", response::Status::BackendError).await);
        }
        let mut reply = reply.unwrap();
        if reply.user_name != ctx.user_name {
            return Some(errors::form_response("EditReply", response::Status::BackendError).await);
        }
        reply.text = self.text.clone();
        reply.hash_tags = self.hash_tags.clone();
        reply.user_names = self.user_names.clone();
        let resp = reply.update().await;
        if resp.is_err() {
            return Some(errors::form_response("EditReply", response::Status::BackendError).await);
        }
        Some(response::Response {
            operation: Some(response::response::Operation::EditReply(
                response::EditReply {
                    status: response::Status::Success as i32,
                    message: None,
                },
            )),
        })
    }
}

impl response::Reply {
    pub fn from(reply: service_response::Reply) -> Self {
        Self {
            tweet_id: reply.tweet_id.clone(),
            user_name: reply.user_name.clone(),
            text: reply.text.clone(),
            reply_id: reply.reply_id.clone(),
            parent_id: reply.parent_id.clone(),
            likes: reply.likes,
            dislikes: reply.dislikes,
        }
    }
    pub fn from_request(reply: service_request::Reply) -> Self {
        Self {
            tweet_id: reply.tweet_id.clone(),
            user_name: reply.user_name.clone(),
            text: reply.text.clone(),
            reply_id: reply.reply_id.clone(),
            parent_id: reply.parent_id.clone(),
            likes: reply.likes,
            dislikes: reply.dislikes,
        }
    }
}
impl request::GetReply {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(errors::form_response("GetReply", response::Status::BackendError).await);
        }
        let tweet = service_request::Tweet::from_uuid(self.tweet_id.clone()).await;
        if tweet.is_err() {
            return Some(errors::form_response("GetReply", response::Status::BackendError).await);
        }
        let tweet = tweet.unwrap();
        let reply = service_request::Reply::from_uuid(
            tweet.tweet_id.clone(),
            self.reply_id.clone(),
            self.parent_id.clone(),
        )
        .await;
        if reply.is_err() {
            return Some(errors::form_response("GetReply", response::Status::BackendError).await);
        }
        let reply = reply.unwrap();
        let reply = response::Reply::from_request(reply);
        Some(response::Response {
            operation: Some(response::response::Operation::GetReply(
                response::GetReply {
                    status: response::Status::Success as i32,
                    message: None,
                    reply: Some(reply),
                },
            )),
        })
    }
}

impl request::ListReplies {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(
                errors::form_response("ListReplies", response::Status::BackendError).await,
            );
        }
        let tweet = service_request::Tweet::from_uuid(self.tweet_id.clone()).await;
        if tweet.is_err() {
            return Some(
                errors::form_response("ListReplies", response::Status::BackendError).await,
            );
        }
        let tweet = tweet.unwrap();
        let replies = tweet.list_replies(self.parent_id.clone()).await;
        if replies.is_err() {
            return Some(
                errors::form_response("ListReplies", response::Status::BackendError).await,
            );
        }
        let replies = replies.unwrap();
        let mut objects = Vec::new();
        for reply in replies {
            objects.push(response::Reply::from(reply));
        }
        Some(response::Response {
            operation: Some(response::response::Operation::ListReplies(
                response::ListReplies {
                    status: response::Status::Success as i32,
                    message: None,
                    replies: Vec::new(),
                },
            )),
        })
    }
}

impl request::RemoveReply {
    pub async fn handle(&self, ctx: &mut Context) -> Option<Response> {
        if !ctx.is_acuthenticated {
            return Some(
                errors::form_response("RemoveReply", response::Status::BackendError).await,
            );
        }
        let tweet = service_request::Tweet::from_uuid(self.tweet_id.clone()).await;
        if tweet.is_err() {
            return Some(
                errors::form_response("RemoveReply", response::Status::BackendError).await,
            );
        }
        let tweet = tweet.unwrap();
        let reply = service_request::Reply::from_uuid(
            tweet.tweet_id.clone(),
            self.reply_id.clone(),
            self.parent_id.clone(),
        )
        .await;
        if reply.is_err() {
            return Some(
                errors::form_response("RemoveReply", response::Status::BackendError).await,
            );
        }
        let reply = reply.unwrap();
        if reply.user_name != ctx.user_name.clone() {
            return Some(
                errors::form_response("RemoveReply", response::Status::BackendError).await,
            );
        }
        let resp = reply.remove().await;
        if resp.is_err() {
            return Some(
                errors::form_response("RemoveReply", response::Status::BackendError).await,
            );
        }
        Some(response::Response {
            operation: Some(response::response::Operation::RemoveReply(
                response::RemoveReply {
                    status: response::Status::Success as i32,
                    message: None,
                },
            )),
        })
    }
}
