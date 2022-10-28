use crate::statement::*;
use async_trait::async_trait;
use futures::prelude::*;

#[async_trait]
pub trait Solution {
    async fn solve(repositories: Vec<ServerName>) -> Option<Binary>;
}

pub struct Solution0;

impl Solution0 {
    async fn download_one(
        name: ServerName,
        mut sender: futures::channel::mpsc::Sender<Option<Binary>>,
    ) {
        sender.send(download(name).await.ok()).await.ok();
    }
}

#[async_trait]
impl Solution for Solution0 {
    async fn solve(repositories: Vec<ServerName>) -> Option<Binary> {
        use futures::stream::StreamExt;
        let len = repositories.len();

        let (sender, receiver) = futures::channel::mpsc::channel::<Option<Binary>>(len);

        let download_all = futures::stream::iter(repositories.into_iter())
            .for_each_concurrent(len, move |name| Self::download_one(name, sender.clone()));

        let wait_loaded = async move {
            let mut rx = receiver.take(len);
            while let Some(res) = rx.next().await {
                if res.is_some() {
                    return res;
                }
            }
            None
        };

        let (res, _) = futures::join!(wait_loaded, download_all);
        res
    }
}
