use futures::Future;
use seed::fetch;
use seed::prelude::*;
use serde::Deserialize;

pub const TITLE: &str = "Example B";
pub const DESCRIPTION: &str =
    "Click button 'Try to Fetch JSON' to send request to non-existent endpoint.
    Server will return 404 with empty body and Serde then fail to decode body into predefined JSON.";

fn get_request_url() -> String {
    "/api/non-existent-endpoint".into()
}

// Model

#[derive(Default)]
pub struct Model {
    pub response_with_data_result: Option<fetch::ResponseWithDataResult<ExpectedResponseData>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ExpectedResponseData {
    something: String,
}

// Update

#[derive(Clone)]
pub enum Msg {
    SendRequest,
    Fetched(fetch::FetchResult<ExpectedResponseData>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut Orders<Msg>) {
    match msg {
        Msg::SendRequest => {
            orders.skip().perform_cmd(send_request());
        }

        Msg::Fetched(Ok(response_with_data_result)) => {
            model.response_with_data_result = Some(response_with_data_result);
        }

        Msg::Fetched(Err(request_error)) => {
            log!("Example_B error:", request_error);
            orders.skip();
        }
    }
}

fn send_request() -> impl Future<Item = Msg, Error = Msg> {
    fetch::Request::new(get_request_url())
        .fetch_json(|fetch_object| Msg::Fetched(fetch_object.result))
}

// View

pub fn view(model: &Model) -> impl View<Msg> {
    vec![
        match &model.response_with_data_result {
            None => empty![],
            Some(fetch::ResponseWithDataResult { status, data, .. }) => div![
                div![format!("Status code: {}", status.code)],
                div![format!(r#"Status text: "{}""#, status.text)],
                div![format!(r#"Data: "{:#?}""#, data)]
            ],
        },
        button![simple_ev(Ev::Click, Msg::SendRequest), "Try to Fetch JSON"],
    ]
}
