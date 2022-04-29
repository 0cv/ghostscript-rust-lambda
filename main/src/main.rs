extern crate ghostscript;
use ghostscript as gs;
use gs::builder::{BuilderResult, GhostscriptBuilder};

use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

/// This is also a made-up example. Requests come into the runtime as unicode
/// strings in json format, which can map to any structure that implements `serde::Deserialize`
/// The runtime pays no attention to the contents of the request payload.
#[derive(Deserialize)]
struct Request {
    command: String,
}

/// This is a made-up example of what a response structure may look like.
/// There is no restriction on what it can be. The runtime requires responses
/// to be serialized into json. The runtime pays no attention
/// to the contents of the response payload.
#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // this needs to be set to false, otherwise ANSI color codes will
        // show up in a confusing manner in CloudWatch logs.
        .with_ansi(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let func = service_fn(my_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

pub(crate) async fn my_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // extract some useful info from the request
    let command = event.payload.command;
    exec_pdf();

    // prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        msg: format!("Command {} executed.", command),
    };

    // return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}


fn exec_pdf() {
    let mut builder = GhostscriptBuilder::new();

    builder.with_init_params(&[
        "-sDEVICE=tiffg4",
        "-dNOPAUSE",
        "-r300x300",
        "-sOutputFile=/tmp/output.tif",
        "--",
        "./input.pdf",
    ]);

    // If we used build() instead of build_simple() we could have passed any data
    // to associate with the new Ghostscript interpreter instance.
    // Such user data can also implement some useful Ghostscript callback traits.
    match builder.build_simple() {
        BuilderResult::Running(instance) => {
            // This is where we could get a running instance for further interpreter work.
            // But our init params above should have made the interpreter immediately quit
            // after rendering the file.
            eprintln!("Unexpected ghostscript instance: {:?}", instance);
            // Our user data can be extracted back, destroying the interpreter instance.
            eprintln!("I'm just a NoCallback: {:?}", instance.into_inner());
            unreachable!("The instance should have quit immediately after initialization.");
        },
        BuilderResult::Quit(user_data) => {
            // Interpreter ran and quit. Execution successfully completed.
            // Our user data is returned back. But we used build_simple() instead of build().
            println!("I'm just a NoCallback: {:?}", user_data);
        },
        BuilderResult::Failed(e) => {
            // Interpreter failed to build or run. The user_data is returned to us still.
            eprintln!("I'm just a NoCallback: {:?}", e.user_data);
            // As well as details about which part of the build process failed.
            panic!("Error building instance: {:?}", e.kind_and_code());
        },
    }

    // The builder can be reused to keep building new instances.
    // All the settings and parameters are preserved.
    // The following repeats the same rendering as above, but the has_quit() shorthand is used
    // to convert BuilderResult into Result in a similar way to the above match.
    builder
        .build_simple()
        .has_quit()
        .expect("Interpreter failed to start or kept running");
}
