use sqlx::{postgres::{PgPool, PgPoolOptions}};
use sqlx::Row;
use dotenv::dotenv;
use std::fmt;
use html_escape::encode_text;

use std::{env, time::Duration};
use std::error::Error;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Argon2
};

#[derive(Debug)]
pub enum DatabaseError {
    ConnectionError(String),
    InitializationError(String),
}
impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DatabaseError::ConnectionError(_) => write!(f, "Database connection error occurred"),
            DatabaseError::InitializationError(_) => write!(f, "Database initialization error occurred"),
        }
    }
}
impl std::error::Error for DatabaseError {}

pub async fn init_db() -> Result<PgPool, DatabaseError> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .map_err(|e| DatabaseError::ConnectionError(format!("Environment variable error: {}", e)))?;

    if cfg!(debug_assertions) {
        println!("Connecting to database...");
    }

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .min_connections(1)
        .max_lifetime(Duration::from_secs(30 * 60))
        .acquire_timeout(Duration::from_secs(20))
        .idle_timeout(Duration::from_secs(180))
        .connect(&database_url)
        .await
        .map_err(|e| {
            if cfg!(debug_assertions) {
                println!("Database Error: {}", e);
                DatabaseError::ConnectionError(format!("Failed to connect: {}", e))
            } else {
                DatabaseError::ConnectionError("Connection error".to_string())
            }
        })?;

    if cfg!(debug_assertions) {
        println!("Database connection established!");
    }

    match initialize_tables(&pool).await {
        Ok(_) => {
            if cfg!(debug_assertions) {
                println!("Tables initialized successfully!");
            }
            Ok(pool)
        },
        Err(e) => {
            pool.close().await;
            if cfg!(debug_assertions) {
                println!("Initialization Error: {}", e);
                Err(DatabaseError::InitializationError(format!("Failed to initialize: {}", e)))
            } else {
                Err(DatabaseError::InitializationError("Initialization error".to_string()))
            }
        }
    }
}

fn log_error(error: &str) {
    if cfg!(debug_assertions) {
        println!("Error: {}", error);
    } else {
        // will be added in future versions

    }
}
async fn initialize_tables(pool: &PgPool) -> Result<(), Box<dyn Error>> {
    create_users_table(pool).await?;
    create_posts_table(pool).await?;
    create_categories_table(pool).await?;
    create_categories_relationships_table(pool).await?;
    create_comments_table(pool).await?;
    // Test data creation
    ensure_test_user(pool).await?;
    ensure_test_post(pool).await?;

    Ok(())
}
async fn ensure_test_user(pool: &PgPool) -> Result<(), sqlx::Error> {
    let user_exists = sqlx::query(
        "SELECT EXISTS(SELECT 1 FROM users WHERE role = $1)"
    )
        .bind("admin")
        .fetch_one(pool)
        .await?
        .get::<bool, _>(0);

    if !user_exists {
        println!("Creating test user...");

        // Hash Test User Password
        let password = "admin123".as_bytes();
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password, &salt)
            .unwrap()
            .to_string();

        // Create Test User
        sqlx::query(
            r#"
            INSERT INTO users (
                username,
                profile_image,
                email,
                password_hash,
                full_name,
                role,
                bio
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7
            )
            "#
        )
            .bind("Skyfetch")
            .bind("https://avatars.githubusercontent.com/u/156548013?v=4")
            .bind("senerhalilbaki@gmail.com")
            .bind(password_hash)
            .bind("Halil Baki")
            .bind("admin")
            .bind("Ben Halil, Muş Alparslan Üniversitesi Yazılım Mühendisliği 1.sınıf öğrencisiyim. İlk Defa Rust İle Bir Websitesi Yapmaktayım.")
            .execute(pool)
            .await?;

        println!("Test user created successfully!");
    }

    Ok(())
}



async fn ensure_test_post(pool: &PgPool) -> Result<(), sqlx::Error> {

    let post_exists = sqlx::query(
        "SELECT EXISTS(SELECT 1 FROM posts WHERE id = $1)"
    )
        .bind(1)
        .fetch_one(pool)
        .await?
        .get::<bool, _>(0);

    if !post_exists {
        println!("Creating test post 1...");

        let author_id: i32 = sqlx::query("SELECT id FROM users WHERE role = $1")
            .bind("admin")
            .fetch_one(pool)
            .await?
            .get("id");

        // Create Test Post
        let content = encode_text("<p>Merhaba, ben Halil Baki Şener. <br><br> Bu platform, modern web teknolojileri ve Rust programlama dilinin gücünü bir araya getiren kişisel blog projemdir. <b>Rust</b>'ın güvenli, performanslı ve eşzamanlı programlama özelliklerinden yararlanarak geliştirilmiştir.<br><br> Projede kullanılan teknolojiler: <br>- Rust ve Actix-web framework'ü ile güçlü bir backend altyapısı<br>- PostgreSQL veritabanı (Docker konteyner üzerinde)<br>- Askama template engine ile dinamik içerik yönetimi<br>- SQLx ile type-safe veritabanı işlemleri<br>- Modern güvenlik standartları için Argon2 şifreleme<br>- Tokio runtime ile asenkron işlem yönetimi<br><br> Bu proje, modern web geliştirme pratiklerini ve Rust ekosisteminin sunduğu güçlü araçları keşfetme yolculuğumun bir parçasıdır. Sürekli geliştirilmeye ve yeni özellikler eklenmeye devam edilmektedir.</p>");

        let post1_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO posts (
                title,
                slug,
                content,
                excerpt,
                featured_image,
                author_id,
                status,
                published_at,
                view_count
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9
            )
            RETURNING id
            "#
        )

            .bind("İlk Rust Sitem")
            .bind("ilk-rust-sitem")
            .bind("<p>Merhaba, ben Halil Baki Şener. <br><br> Bu platform, modern web teknolojileri ve Rust programlama dilinin gücünü bir araya getiren kişisel blog projemdir. <b>Rust</b>'ın güvenli, performanslı ve eşzamanlı programlama özelliklerinden yararlanarak geliştirilmiştir.<br><br> Projede kullanılan teknolojiler: <br>- Rust ve Actix-web framework'ü ile güçlü bir backend altyapısı<br>- PostgreSQL veritabanı (Docker konteyner üzerinde)<br>- Askama template engine ile dinamik içerik yönetimi<br>- SQLx ile type-safe veritabanı işlemleri<br>- Modern güvenlik standartları için Argon2 şifreleme<br>- Tokio runtime ile asenkron işlem yönetimi<br><br> Bu proje, modern web geliştirme pratiklerini ve Rust ekosisteminin sunduğu güçlü araçları keşfetme yolculuğumun bir parçasıdır. Sürekli geliştirilmeye ve yeni özellikler eklenmeye devam edilmektedir.</p>")
            .bind("Hakkımda")
            .bind("https://avatars.githubusercontent.com/u/156548013?v=4")
            .bind(author_id)
            .bind("published")
            .bind(chrono::Utc::now())
            .bind(2)
            .fetch_one(pool)
            .await?;

        println!("Test post 1 created with ID: {}", post1_id);

        println!("Creating Test Post 2...");
        // Create Test Post 2
        let post2_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO posts (
                title,
                slug,
                content,
                excerpt,
                featured_image,
                author_id,
                status,
                published_at,
                view_count
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9
            )
            RETURNING id
            "#
        )
            .bind("Rust Programlama Dili")
            .bind("rust-programlama-dili")
            .bind("<p>Rust Programlama Dili: Web Geliştirmede Yeni Bir Çağ<br><br>Rust, Mozilla tarafından geliştirilen ve 2015 yılında 1.0 sürümü yayınlanan modern bir sistem programlama dilidir. Memory safety ve thread safety özelliklerini compile-time'da garanti eden benzersiz ownership sistemi, onu diğer dillerden ayıran en önemli özelliğidir.<br><br><b>Performans Özellikleri:</b><br>- Sıfır maliyetli soyutlamalar (Zero-cost abstractions)<br>- Garbage collector olmaması sayesinde öngörülebilir performans<br>- C/C++ ile karşılaştırılabilir düzeyde düşük seviye kontrol<br>- LLVM tabanlı optimize edilmiş makine kodu üretimi<br>- Thread safety garantisi sayesinde paralel işlemlerde yüksek performans<br><br><b>Web Geliştirme Avantajları:</b><br>- Actix-web framework'ü ile Node.js'den 10 kata kadar daha hızlı HTTP sunucu performansı<br>- Async/await desteği ile etkili concurrent programlama<br>- WebAssembly ile tarayıcıda native hıza yakın performans<br>- Güvenli bellek yönetimi sayesinde runtime hataların minimize edilmesi<br>- Cross-platform derleme ve deployment kolaylığı<br><br><b>Güvenlik Özellikleri:</b><br>- Compile-time memory safety kontrolleri<br>- Race condition'ları önleyen ownership sistemi<br>- Buffer overflow ve null pointer dereference hatalarını engelleyen tip sistemi<br>- Pattern matching ile güvenli hata yönetimi<br><br><b>Ekosistem ve Topluluk:</b><br>- Cargo paket yöneticisi ile kolay dependency yönetimi<br>- Crates.io üzerinde 100,000+ açık kaynak paket<br>- Aktif ve yardımsever geliştirici topluluğu<br>- Detaylı dokümantasyon ve öğrenme kaynakları<br><br>Rust'ın bu özellikleri, onu özellikle yüksek performans ve güvenlik gerektiren web uygulamaları için ideal bir seçim haline getirmektedir. Discord, Dropbox ve Cloudflare gibi büyük teknoloji şirketleri, kritik sistemlerinde Rust'ı tercih etmektedir.</p>")
            .bind("Rust Programlama: Web Programlama ")
            .bind("https://rustacean.net/assets/rustacean-flat-happy.png")
            .bind(author_id)
            .bind("published")
            .bind(chrono::Utc::now())
            .bind(15)
            .fetch_one(pool)
            .await?;

        println!("Test post 2 created with ID: {}", post2_id);

        // Create Category And Take ID
        let category_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO categories (
                name,
                slug,
                description,
                created_at
            ) VALUES (
                $1, $2, $3, $4
            )
            RETURNING id
            "#
        )
            .bind("Yazılım")
            .bind("yazilim")
            .bind("Bu Bir Yazılım Kategorisidir")
            .bind(chrono::Utc::now())
            .fetch_one(pool)
            .await?;

        println!("Test category created with ID: {}", category_id);
        // Create Category And Take ID
        let category_id2: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO categories (
                name,
                slug,
                description,
                created_at
            ) VALUES (
                $1, $2, $3, $4
            )
            RETURNING id
            "#
        )
            .bind("Hakkımda")
            .bind("hakkimda")
            .bind("Bu Bir hakkimda Kategorisidir")
            .bind(chrono::Utc::now())
            .fetch_one(pool)
            .await?;

        println!("Test category created with ID: {}", category_id);

        println!("Creating Test relationships ...");
        sqlx::query(
            r#"
            INSERT INTO categories_relationships (
                term_id,
                post_id
            ) VALUES (
                $1, $2
            )
            "#
        )
            .bind(category_id)
            .bind(post1_id)
            .execute(pool)
            .await?;

        println!("Creating Test relationships ...");
        sqlx::query(
            r#"
            INSERT INTO categories_relationships (
                term_id,
                post_id
            ) VALUES (
                $1, $2
            )
            "#
        )
            .bind(category_id2)
            .bind(post1_id)
            .execute(pool)
            .await?;


        // Post 2 Category Relationship
        sqlx::query(
            r#"
            INSERT INTO categories_relationships (
                term_id,
                post_id
            ) VALUES (
                $1, $2
            )
            "#
        )
            .bind(category_id)
            .bind(post2_id)
            .execute(pool)
            .await?;

        println!("Category relationships created successfully!");
        println!("Creating Test Comments(POST 2)");

        let comment_content = "Rust Dili Harika Bir Dile Benziyor 😊";
        sqlx::query(
            r#"
    INSERT INTO comments (
        post_id,
        user_id,
        content,
        author_name,
        author_email,
        status
    )
    VALUES (
        $1, $2, $3, $4, $5, $6
    )
    "#
        )
            .bind(post2_id)
            .bind(author_id)
            .bind(comment_content)
            .bind("Skyfetch")
            .bind("senerhalilbaki@gmail.com")
            .bind("approved")
            .execute(pool)
            .await?;

    }

    Ok(())
}


async fn create_users_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    let table_exists = check_table_exists(pool, "users").await?;

    if !table_exists {
        println!("Creating users table...");
        sqlx::query(
            r#"
            CREATE TABLE users (
                id SERIAL PRIMARY KEY,
                username VARCHAR(50) UNIQUE NOT NULL,
                email VARCHAR(255) UNIQUE NOT NULL,
                password_hash VARCHAR(255) NOT NULL,
                full_name VARCHAR(100),
                bio TEXT,
                profile_image VARCHAR(255),
                role VARCHAR(20) NOT NULL DEFAULT 'user',
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
            .execute(pool)
            .await?;
        println!("Users table created successfully!");
    }
    Ok(())
}
async fn create_posts_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    let table_exists = check_table_exists(pool, "posts").await?;

    if !table_exists {
        println!("Creating posts table...");
        sqlx::query(
            r#"
            CREATE TABLE posts (
                id SERIAL PRIMARY KEY,
                title VARCHAR(255) NOT NULL,
                slug VARCHAR(255) UNIQUE NOT NULL,
                content TEXT NOT NULL,
                excerpt TEXT,
                featured_image VARCHAR(255),
                author_id INTEGER REFERENCES users(id) NOT NULL,
                status VARCHAR(20) NOT NULL DEFAULT 'draft',
                published_at TIMESTAMPTZ,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                view_count INTEGER DEFAULT 0
            )
            "#,
        )
            .execute(pool)
            .await?;
        println!("Posts table created successfully!");
    }
    Ok(())
}
async fn create_categories_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    let table_exists = check_table_exists(pool, "categories").await?;

    if !table_exists {
        println!("Creating categories table...");
        sqlx::query(
            r#"
            CREATE TABLE categories (
                id SERIAL PRIMARY KEY,
                name VARCHAR(50) NOT NULL,
                slug VARCHAR(50) UNIQUE NOT NULL,
                description TEXT,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
            .execute(pool)
            .await?;
        println!("Categories table created successfully!");
    }
    Ok(())
}

async fn create_categories_relationships_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    let table_exists = check_table_exists(pool, "categories_relationships").await?;

    if !table_exists {
        println!("Creating categories_relationships table...");
        sqlx::query(
            r#"
            CREATE TABLE categories_relationships (
                id SERIAL PRIMARY KEY,
                term_id INTEGER NOT NULL REFERENCES categories(id),
                post_id INTEGER NOT NULL REFERENCES posts(id)
            )
            "#,
        )
            .execute(pool)
            .await?;
        println!("Categories relationships table created successfully!");
    }
    Ok(())
}



async fn create_comments_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    let table_exists = check_table_exists(pool, "comments").await?;

    if !table_exists {
        println!("Creating comments table...");
        sqlx::query(
            r#"
            CREATE TABLE comments (
                id SERIAL PRIMARY KEY,
                post_id INTEGER REFERENCES posts(id) ON DELETE CASCADE,
                user_id INTEGER REFERENCES users(id),
                parent_id INTEGER REFERENCES comments(id),
                content TEXT NOT NULL,
                author_name VARCHAR(100),
                author_email VARCHAR(255),
                status VARCHAR(20) NOT NULL DEFAULT 'pending',
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
            .execute(pool)
            .await?;
        println!("Comments table created successfully!");
    }
    Ok(())
}


async fn check_table_exists(pool: &PgPool, table_name: &str) -> Result<bool, sqlx::Error> {
    let exists = sqlx::query(
        r#"
        SELECT EXISTS (
            SELECT FROM information_schema.tables
            WHERE table_schema = 'public'
            AND table_name = $1
        );
        "#,
    )
        .bind(table_name)
        .fetch_one(pool)
        .await?
        .get::<bool, _>(0);

    Ok(exists)
}