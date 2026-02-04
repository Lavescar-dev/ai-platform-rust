use axum::response::Html;

pub async fn index() -> Html<&'static str> {
    Html(include_str!("../../templates/index.html"))
}

pub async fn chat_page() -> Html<&'static str> {
    Html(include_str!("../../templates/index.html"))
}

pub async fn content_page() -> Html<&'static str> {
    Html(include_str!("../../templates/index.html"))
}

pub async fn code_page() -> Html<&'static str> {
    Html(include_str!("../../templates/index.html"))
}

pub async fn email_page() -> Html<&'static str> {
    Html(include_str!("../../templates/index.html"))
}

pub async fn video_page() -> Html<&'static str> {
    Html(include_str!("../../templates/index.html"))
}

pub async fn seo_page() -> Html<&'static str> {
    Html(include_str!("../../templates/index.html"))
}

pub async fn image_page() -> Html<&'static str> {
    Html(include_str!("../../templates/index.html"))
}

pub async fn voice_page() -> Html<&'static str> {
    Html(include_str!("../../templates/index.html"))
}

pub async fn resume_page() -> Html<&'static str> {
    Html(include_str!("../../templates/index.html"))
}

pub async fn bot_page() -> Html<&'static str> {
    Html(include_str!("../../templates/index.html"))
}
