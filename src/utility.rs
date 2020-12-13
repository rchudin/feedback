use crate::error::Error;
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

pub(crate) async fn reqwest_file_part(part: Part) -> Result<reqwest::multipart::Part, Error> {
    if let (Some(ty), Some(name)) = (part.content_type(), part.filename()) {
        let name = String::from(name);
        let ty = String::from(ty);
        let data = stream_data(part).await?;
        Ok(reqwest::multipart::Part::bytes(data)
            .file_name(name)
            .mime_str(&ty)?)
    } else {
        Err(Error::StaticMessage("file upload error"))
    }
}
