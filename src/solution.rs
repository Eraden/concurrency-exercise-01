use crate::statement::*;
use async_trait::async_trait;
use futures::prelude::*;

#[async_trait]
pub trait Solution {
    async fn solve(repositories: Vec<ServerName>) -> Option<Binary>;
}

pub struct Solution0;

#[async_trait]
impl Solution for Solution0 {
    async fn solve(repositories: Vec<ServerName>) -> Option<Binary> {
        use futures::stream::StreamExt;
        let len = repositories.len();

        let (sender, receiver) = futures::channel::mpsc::channel::<Option<Binary>>(len);

        for name in repositories {
            let mut sender = sender.clone();
            tokio::spawn(async move {
                let res = download(name).await;
                sender.send(res.ok()).await.ok();
            });
        }

        let mut rx = receiver.take(len);
        while let Some(res) = rx.next().await {
            if res.is_some() {
                return res;
            }
        }
        None
    }
}
