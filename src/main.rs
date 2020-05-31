use futures_retry::{FutureRetry};
extern crate pretty_env_logger;
#[macro_use] extern crate log;

mod app_errors;

use crate::app_errors::*;

async fn test_async() -> Result<i32, AppError> {
    println!("In test_async");

//    return Ok(42)
    return Err(AppError::General)
}
  
#[tokio::main]
async fn main() -> Result<(), AppError> {
    pretty_env_logger::init();
    info!("such information");
    warn!("o_O");
    error!("much error");

    info!("Event Store Process starting");
    let (number, _attempts) =
        FutureRetry::new(move || test_async(), CustomHandler::new(Some(45)))
        .await?;
    // let (mut number, attempt) =
    //     FutureRetry::new(move || test_async(), CustomHandler::new(3))
    //     .await?;
//        .map_err(|(e, attempt)| eprintln!("Caught an error {} on attempt {}", e, attempt))?;
    info!("Number={}", number);
    Ok(())
}




















// #[tokio::main]
// async fn main() {
//   let addr = //...
//   let mut listener = TcpListener::bind(addr).await.unwrap();
//   let server = listener.incoming()
//     .retry(handle_error) // Magic happens here
//     .and_then(|(stream, _attempt)| {
//       tokio::spawn(serve_connection(stream));
//       ok(())
//     })
//     .try_for_each(|_| ok(()))
//     .map_err(|(e, _attempt)| eprintln!("Caught an error {}", e));
//   server.await
// }
