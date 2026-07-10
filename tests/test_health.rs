use std::process::Stdio;

use irmars::IrmaClient;
use serial_test::serial;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};

#[test]
#[serial]
fn test_health() {
    if option_env!("RUN_INTEGRATION_TESTS").is_some() {
        tokio_test::block_on(async {
            let mut irmaserver = Command::new("irma")
                .arg("server")
                .stderr(Stdio::piped())
                .spawn()
                .expect("Could not start irma server");

            let irmaserver_stderr = irmaserver
                .stderr
                .take()
                .expect("No stderr available from irma server");
            let mut irmaserver_lines = BufReader::new(irmaserver_stderr).lines();
            loop {
                let line = irmaserver_lines
                    .next_line()
                    .await
                    .expect("Error reading from irma server stderr")
                    .expect("No line recieved");
                if line.contains("Server listening") {
                    break;
                }
            }

            println!("Server started");

            // Create an irma client
            let client = IrmaClient::new("http://localhost:8088/").unwrap();

            // A running server should report itself as healthy.
            client.health().await.expect("Health check failed");

            irmaserver.kill().await.expect("Error killing irma server");
        });
    }
}
