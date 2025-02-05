use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run <role>");
        return;
    }

    let role = args[1].as_str();
    let pool = PgPoolOptions::new().max_connections(5)
        .connect("postgres://username:password@localhost/globacom").await.expect("Failed to connect to database");

    match role {
        "Administrator" => show_database_structure(&pool).await,
        "ProjectManager" => show_table_structure(&pool, "project").await,
        "Employee" => show_table_structure(&pool, "staff").await,
        "Customer" => show_table_structure(&pool, "customer").await,
        "Vendor" => show_table_structure(&pool, "dataplan").await,
        _ => eprintln!("Invalid role"),
    }
}

async fn show_database_structure(pool: &Pool<Postgres>) {
    let rows = sqlx::query!("SELECT tablename FROM pg_tables WHERE schemaname = 'public'")
        .fetch_all(pool).await.expect("Error fetching database structure");
    println!("Database Structure:");
    for row in rows {
        println!("- {}", row.tablename);
    }
}

async fn show_table_structure(pool: &Pool<Postgres>, table: &str) {
    let query = format!("SELECT column_name, data_type FROM information_schema.columns WHERE table_name = '{}'", table);
    let rows = sqlx::query(&query).fetch_all(pool).await.expect("Error fetching table structure");
    println!("Table Structure for {}:", table);
    for row in rows {
        println!("- {} ({})", row.get::<String, _>("column_name"), row.get::<String, _>("data_type"));
    }
}
