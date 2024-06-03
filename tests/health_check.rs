use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use zero2prod::configuration::{get_configuration, DatabaseSettings};

struct TestApp {
    address: String,
    db_pool: PgPool,
}

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute the request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let body = "name=Mitali%20Madhusmita&email=mitali.kunmun%40gmail.com";
    let response = client
        .post(format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute the request.");

    assert_eq!(200, response.status());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to read form the Database.");
    assert_eq!(saved.email, "mitali.kunmun@gmail.com");
    assert_eq!(saved.name, "Mitali Madhusmita");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("Mitali%20Madhusmita", "Missing the email"),
        ("", "mitali.kunmun%40gmail.com"),
        ("", "Missing the email"),
    ];
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute the request.");
        assert_eq!(
            400,
            response.status(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}",
            error_message
        );
    }
}

/// Spin up an instance of our application
/// and returns its address (i.e. http://localhost:XXXX)
async fn spawn_app() -> TestApp {
    // Get database configurations and prepare for db connection.
    let mut configuration = get_configuration().expect("Unable to read database configuration");
    // Create new database to test
    configuration.database.database_name = uuid::Uuid::new_v4().to_string();
    let connection_pool = configure_database(&configuration.database).await;
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let listner = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a random port");
    let port = listner.local_addr().unwrap().port();
    let server = zero2prod::startup::run(listner, connection_pool.clone())
        .expect("Failed to bind the server");
    let _ = tokio::spawn(server);
    let address = format!("http://localhost:{}", port);
    TestApp {
        address,
        db_pool: connection_pool,
    }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create Database
    let mut connection = PgConnection::connect(&config.connection_string_w_o_db())
        .await
        .expect("Failed to connect to Database");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to a new Database");
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    connection_pool
}
