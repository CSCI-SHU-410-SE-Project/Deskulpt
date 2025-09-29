use path_clean::PathClean;
use tauri::http::header::{InvalidHeaderValue, ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE, ORIGIN};
use tauri::http::{HeaderValue, Request, Response, StatusCode};
use tauri::{AppHandle, Manager, Runtime, UriSchemeContext, UriSchemeResponder};

use crate::states::WidgetConfigMapStateExt;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Invalid protocol: {0}; must start with widgets://localhost/")]
    InvalidProtocol(String),
    #[error("Invalid resource URI: {0}; must be in the form $WIDGET_ID/$PATH")]
    InvalidResourceUri(String),
    #[error("Invalid widget ID: {0}")]
    InvalidWidgetId(String),
    #[error("Invalid header value when constructing response")]
    InvalidHeaderValue(#[from] InvalidHeaderValue),
    #[error("HTTP error: {0}")]
    Http(#[from] tauri::http::Error),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Tauri error: {0}")]
    Tauri(#[from] tauri::Error),
}

impl Error {
    fn to_response(&self) -> Response<Vec<u8>> {
        let status = match self {
            Error::InvalidProtocol(_) | Error::InvalidResourceUri(_) => StatusCode::BAD_REQUEST,
            Error::InvalidWidgetId(_) => StatusCode::NOT_FOUND,
            Error::InvalidHeaderValue(_) | Error::Http(_) | Error::Tauri(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            },
            Error::Io(e) => match e.kind() {
                std::io::ErrorKind::NotFound => StatusCode::NOT_FOUND,
                std::io::ErrorKind::PermissionDenied => StatusCode::FORBIDDEN,
                std::io::ErrorKind::InvalidInput => StatusCode::BAD_REQUEST,
                std::io::ErrorKind::IsADirectory => StatusCode::BAD_REQUEST,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
        };

        Response::builder()
            .status(status)
            .header(CONTENT_TYPE, "text/plain; charset=utf-8")
            .body(format!("{self:?}").into())
            .unwrap()
    }
}

async fn widgets_protocol_handler_inner<R: Runtime>(
    app_handle: &AppHandle<R>,
    req: &Request<Vec<u8>>,
) -> Result<Response<Vec<u8>>, Error> {
    let url = req.uri().to_string();
    let url = url
        .strip_prefix("widgets://localhost/")
        .ok_or_else(|| Error::InvalidProtocol(url.clone()))?;

    let mut parts = url.splitn(2, '/');
    let id = parts
        .next()
        .ok_or_else(|| Error::InvalidResourceUri(url.into()))?
        .to_string();
    let path = parts
        .next()
        .ok_or_else(|| Error::InvalidResourceUri(url.into()))?
        .to_string();

    if app_handle.with_widget_config_map(|widgets| !widgets.contains_key(&id)) {
        return Err(Error::InvalidWidgetId(id));
    }

    let base = app_handle
        .path()
        .app_local_data_dir()?
        .join("widgets")
        .join(id);
    let resource_path = base.join(path).clean();
    if !resource_path.starts_with(&base) {
        return Err(Error::InvalidResourceUri(url.into()));
    }
    let data = tokio::fs::read(&resource_path).await?;

    let mime = mime_guess::from_path(&resource_path).first_or_octet_stream();
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, HeaderValue::from_str(mime.as_ref())?)
        .body(data)?;
    Ok(response)
}

fn corsify_response(response: &mut Response<Vec<u8>>, req: &Request<Vec<u8>>) {
    let allow_origin = req
        .headers()
        .get(ORIGIN)
        .map(|v| v.as_bytes())
        .unwrap_or(b"*");
    response.headers_mut().insert(
        ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_bytes(allow_origin).unwrap_or_else(|_| HeaderValue::from_static("*")),
    );
}

pub fn widgets_protocol_handler<R: Runtime>(
    ctx: UriSchemeContext<R>,
    req: Request<Vec<u8>>,
    responder: UriSchemeResponder,
) {
    let app_handle = ctx.app_handle().clone();
    tauri::async_runtime::spawn(async move {
        let mut response = widgets_protocol_handler_inner(&app_handle, &req)
            .await
            .unwrap_or_else(|e| e.to_response());
        corsify_response(&mut response, &req);
        responder.respond(response);
    });
}
