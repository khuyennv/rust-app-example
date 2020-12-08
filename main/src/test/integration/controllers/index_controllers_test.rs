#[cfg(test)]
mod test {
    use crate::app::Application;
    use crate::config::CONFIG;
    use actix_web::{test, App};
    use futures_util::TryStreamExt;
    use std::str::from_utf8;

    #[actix_rt::test]
    async fn test_index_get() {
        let config = &CONFIG;
        println!("->>>>>>>>>>>>>>>>>>>>>>>....{:?}", config.env);
        let mut app = test::init_service(App::new().configure(Application::config_app())).await;

        let mut test_request = test::TestRequest::with_header("content-type", "text/plain");
        test_request = test_request.header("x-gapo-role", "user");
        test_request = test_request.header("x-gapo-user-id", "10");

        let req = test_request.to_request();
        let mut response = test::call_service(&mut app, req).await;

        let bytes = response
            .take_body()
            .try_fold(Vec::new(), |mut acc, chunk| async {
                acc.extend(chunk);
                Ok(acc)
            });
        let a = bytes.await.unwrap();
        let b = from_utf8(a.as_slice()).unwrap();

        assert_eq!(b, "Hello world");
        assert!(response.status().is_success());
    }
}
