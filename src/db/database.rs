use sea_orm::{ ConnectionTrait, Database, DatabaseConnection, Statement };
use dotenvy::dotenv;
use std::env;


pub async fn init_db () -> DatabaseConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Database::connect(database_url).await.expect("Database connection failed")
}

pub struct TestDatabase {
    pub db: DatabaseConnection,
}

impl TestDatabase {
    pub async fn new (db: DatabaseConnection, rebuild: bool) -> Result<Self, sea_orm::DbErr> {
        
        if rebuild {
            Self::drop_table(&db).await.expect("Drop table failed");
        }
        Self::create_table(&db).await.expect("Create table failed");
        Ok(Self { db })
    }

    async fn create_table(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
        db.execute(Self::sql(db, r#"
            CREATE TABLE IF NOT EXISTS users (
                id INT AUTO_INCREMENT PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                account VARCHAR(255) NOT NULL UNIQUE,
                password VARCHAR(255) NOT NULL,
                email VARCHAR(255),
                phone VARCHAR(255),
                description VARCHAR(255),
                created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                deleted_at DATETIME
            );
        "#)).await?;
        Ok(())
    }

    async fn drop_table(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
        db.execute(Self::sql(db, "DROP TABLE IF EXISTS users;")).await?;
        Ok(())
    }

    fn sql<S: Into<String>>(db: &DatabaseConnection, sql: S) -> Statement {
        Statement::from_string(db.get_database_backend(), sql.into())
    }
}
