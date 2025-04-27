use sea_orm::{ ConnectionTrait, Database, DatabaseConnection, Statement };
use dotenvy::dotenv;
use std::env;


pub struct TestDatabase {
    pub db: DatabaseConnection,
}

impl TestDatabase {
    pub async fn new (rebuild: bool) -> Result<Self, sea_orm::DbErr> {
        dotenv().ok();
        let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db: DatabaseConnection = Database::connect(database_url).await.expect("Database connection failed");
        if rebuild {
            Self::drop_table(&db).await.expect("Drop table failed");
        }
        Self::create_table(&db).await.expect("Create table failed");
        Ok(Self { db })
    }

    async fn create_table(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
        Self::create_ships_table(db).await?;
        Self::create_roles_table(db).await?;
        Self::create_users_table(db).await?;
        Ok(())
    }

    async fn drop_table(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
        db.execute(Self::sql(db, "DROP TABLE IF EXISTS users;")).await?;
        db.execute(Self::sql(db, "DROP TABLE IF EXISTS roles;")).await?;
        db.execute(Self::sql(db, "DROP TABLE IF EXISTS ships;")).await?;
        Ok(())
    }

    fn sql<S: Into<String>>(db: &DatabaseConnection, sql: S) -> Statement {
        Statement::from_string(db.get_database_backend(), sql.into())
    }

    async fn create_users_table (db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
        db.execute(Self::sql(db, r#"
            CREATE TABLE IF NOT EXISTS users (
                id INT AUTO_INCREMENT PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                account VARCHAR(255) NOT NULL UNIQUE,
                password VARCHAR(255) NOT NULL,
                email VARCHAR(255),
                phone VARCHAR(255),
                description VARCHAR(255),
                role_id INT NOT NULL DEFAULT 1,
                ship_id INT NOT NULL DEFAULT 1,
                created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                deleted_at DATETIME,
                FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE RESTRICT ON UPDATE CASCADE,
                FOREIGN KEY (ship_id) REFERENCES ships(id) ON DELETE RESTRICT ON UPDATE CASCADE
            );
        "#)).await?;

        db.execute(Self::sql(db, r#"
            INSERT INTO users (name, account, password, email, phone, description, role_id, ship_id) VALUES
                ('南宮柳信', 'nangong5421', '1234', 'nangong5421@gmail.com', '(+886) 0123-456-789', '柳白', 5, 5),
                ('小白', 'littlewhite', '1234', 'littlewhite@gmail.com', '(+886) 0123-456-790', '小白', 4, 5),
                ('小黑', 'littleblack', '1234', 'littleblack@gmail.com', '(+886) 0123-456-791', '小黑', 4, 5),
                ('小綠', 'littlegreen', '1234', 'littlegreen@gmail.com', '(+886) 0123-456-792', '小綠', 3, 4),
                ('小紅', 'littlered', '1234', 'littlered@gmail.com', '(+886) 0123-456-793', '小紅', 3, 4),
                ('小藍', 'littleblue', '1234', 'littleblue@gmail.com', '(+886) 0123-456-794', '小藍', 3, 4),
                ('小黃', 'littleyellow', '1234', 'littleyellow@gmail.com', '(+886) 0123-456-795', '小黃', 2, 4),
                ('孔子', 'congzi', '1234', 'congzi@gmail.com', '(+886) 0123-456-796', '三十個人才能讓我站起來打人', 1, 3),
                ('孫子', 'sunzi', '1234', 'sunzi@gmail.com', '(+886) 0123-456-797', '俺老孫速速趕來', 1, 3),
                ('荀子', 'xunzi', '1234', 'xunzi@gmail.com', '(+886) 0123-456-798', '君子曰：學不可以己', 1, 2),
                ('老子', 'laizi', '1234', 'laizi@gmail.com', '(+886) 0123-456-799', '道可道，非常道', 1, 1),
                ('墨子', 'mozi', '1234', 'mozi@gmail.com', '(+886) 0123-456-800', '勇，志之所以敢也', 1, 1);
        "#)).await?;
        Ok(())
    }

    async fn create_roles_table (db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
        db.execute(Self::sql(db, r#"
            CREATE TABLE IF NOT EXISTS roles (
                id INT AUTO_INCREMENT PRIMARY KEY,
                name VARCHAR(255) NOT NULL UNIQUE
            );
        "#)).await?;

        db.execute(Self::sql(db, r#"
            INSERT INTO roles (name) VALUES
                ('使用者'),
                ('測試者'),
                ('開發者'),
                ('管理者'),
                ('擁有者');
        "#)).await?;
        Ok(())
    }

    async fn create_ships_table (db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
        db.execute(Self::sql(db, r#"
            CREATE TABLE IF NOT EXISTS ships (
                id INT AUTO_INCREMENT PRIMARY KEY,
                name VARCHAR(255) NOT NULL UNIQUE
            )
        "#)).await?;

        db.execute(Self::sql(db, r#"
            INSERT INTO ships (name) VALUES
                ('免費方案'),
                ('標準方案'),
                ('優質方案'),
                ('豪華方案'),
                ('終身方案');
        "#)).await?;
        Ok(())
    }
}
