use serde::Serialize;

pub const DEFAULT_RESPONSE: &'static str = "{}";

#[derive(Serialize)]
pub struct Response<T> {
    pub code: i16,
    pub msg: String,
    pub data: T,
}

pub fn success(data: impl Serialize) -> Response<impl Serialize> {
    return Response {
        code: 200,
        msg: String::from(""),
        data,
    };
}