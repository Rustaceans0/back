use chatgpt_rs::prelude::*;
use validator::Validate;
#[derive(Debug, Deserialize, Validate, Serialize)]
pub struct auth_data {
    #[validate(length(min = 8, max = 64))] // right now we don't know how to log in
    pub password: String,
    #[validate(length(min = 3, max = 16))]
    pub username: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct SomeResponse {
    pub json: auth_data,
}
pub async fn step_x(data: SomeData, client: &Client) -> actix_web::Result<SomeData> {
    data.validate().map_err(ErrorBadRequest)?;
    let mut res = client
        .post("")
        .send_json(&data)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let mut body = BytesMut::new();
    while let Some(chunk) = res.next().await {
        body.extend_from_slice(&chunk?);
    }

    let body: HttpBinResponse = serde_json::from_slice(&body).unwrap();

    println!("{body:?}");

    Ok(body.json)
}

pub async fn create_something(
    some_data: web::Json<SomeData>,
    client: web::Data<Client>,
) -> Result<HttpResponse, Error> {
    let some_data_2 = step_x(some_data.into_inner(), &client).await?;
    let some_data_3 = step_x(some_data_2, &client).await?;
    let d = step_x(some_data_3, &client).await?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&d).unwrap()))
}
pub async fn get_letter(req: web::json::Json) -> Result<HttpResponse, Error> {
    let cl = ChatGPT::new(key);
    let response: CompletionResponse = cl.send_message(req.into_inner()).await?;

    HttpResponse::Ok.json("{:?}", response)
}
