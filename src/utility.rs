use bytes::BufMut;
use futures::stream::TryStreamExt;
use warp::multipart::Part;

pub(crate) async fn stream_data(part: Part) -> Result<Vec<u8>, warp::Error> {
    part.stream()
        .try_fold(Vec::new(), |mut vec, data| {
            vec.put(data);
            async move { Ok(vec) }
        })
        .await
}
