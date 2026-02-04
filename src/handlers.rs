use actix_web::{get, post, web, HttpResponse};
use serde::{Serialize, Deserialize};
use tera::Tera;
use sqlx::sqlite::SqlitePool;
use chrono::Utc;
use actix_session::Session;

// Remove the query! macro and use query instead
#[derive(Serialize)]
struct PortfolioData {
    name: String,
    title: String,
    about: String,
    skills: Vec<String>,
    projects: Vec<Project>,
    contact_email: String,
}

#[derive(Serialize)]
struct Project {
    name: String,
    description: String,
    technologies: Vec<String>,
    github_url: Option<String>,
    live_url: Option<String>,
}


#[derive(Debug, Deserialize)]
struct ContactForm {
    name: String,
    email: String,
    message: String,
}

#[derive(Serialize, Deserialize)]
struct ContactMessage {
    id: i64,
    name: String,
    email: String,
    message: String,
    created_at: String,
}

#[derive(Debug, Deserialize)]
struct LoginForm {
    email: String,
    password: String,
}

// Admin credentials from environment variables or defaults
fn get_admin_credentials() -> (String, String) {
    let admin_email = std::env::var("ADMIN_EMAIL")
        .unwrap_or_else(|_| "".to_string());
    let admin_password = std::env::var("ADMIN_PASSWORD")
        .unwrap_or_else(|_| "".to_string());
    (admin_email, admin_password)
}

#[get("/login")]
pub async fn login_page(tmpl: web::Data<Tera>) -> HttpResponse {
    let ctx = tera::Context::new();
    
    match tmpl.render("login.html.tera", &ctx) {
        Ok(rendered) => HttpResponse::Ok().body(rendered),
        Err(e) => {
            eprintln!("Template error: {}", e);
            HttpResponse::InternalServerError().body("Error rendering login page")
        }
    }
}

#[post("/login")]
pub async fn login(
    form: web::Form<LoginForm>,
    session: Session,
    tmpl: web::Data<Tera>,
) -> HttpResponse {
    let (admin_email, admin_password) = get_admin_credentials();
    
    if form.email == admin_email && form.password == admin_password {
        // Set session
        session.insert("authenticated", true).ok();
        
        HttpResponse::Found()
            .append_header(("Location", "/dashboard"))
            .finish()
    } else {
        let mut ctx = tera::Context::new();
        ctx.insert("error", "Invalid email or password");
        
        match tmpl.render("login.html.tera", &ctx) {
            Ok(rendered) => HttpResponse::Ok().body(rendered),
            Err(e) => {
                eprintln!("Template error: {}", e);
                HttpResponse::InternalServerError().body("Error rendering login page")
            }
        }
    }
}

#[get("/logout")]
pub async fn logout(session: Session) -> HttpResponse {
    // Clear the session
    session.clear();
    
    HttpResponse::Found()
        .append_header(("Location", "/login"))
        .finish()
}

#[get("/")]
pub async fn index(tmpl: web::Data<Tera>) -> HttpResponse {
    let portfolio_data = get_portfolio_data();
    
    let mut ctx = tera::Context::new();
    ctx.insert("portfolio", &portfolio_data);

    match tmpl.render("index.html.tera", &ctx) {
        Ok(rendered) => HttpResponse::Ok().body(rendered),
        Err(_) => HttpResponse::InternalServerError().body("Error rendering template"),
    }
}

#[get("/dashboard")]
pub async fn dashboard(tmpl: web::Data<Tera>, pool: web::Data<SqlitePool>, session: Session) -> HttpResponse {
    // Check if user is authenticated
    let is_authenticated = session
        .get::<bool>("authenticated")
        .ok()
        .flatten()
        .unwrap_or(false);
    
    if !is_authenticated {
        return HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish();
    }
    
    // Fetch all contacts from the database
    let messages = match get_all_contacts(&pool).await {
        Ok(msgs) => msgs,
        Err(e) => {
            eprintln!("Failed to fetch contacts: {}", e);
            Vec::new()
        }
    };
    
    let mut ctx = tera::Context::new();
    ctx.insert("messages", &messages);
    ctx.insert("count", &messages.len());
    
    match tmpl.render("dashboard.html.tera", &ctx) {
        Ok(rendered) => HttpResponse::Ok().body(rendered),
        Err(e) => {
            eprintln!("Template error: {}", e);
            HttpResponse::InternalServerError().body("Error rendering dashboard")
        }
    }
}

async fn get_all_contacts(pool: &SqlitePool) -> Result<Vec<ContactMessage>, sqlx::Error> {
    let rows = sqlx::query_as!(
        ContactMessage,
        r#"
        SELECT id, name, email, message, created_at
        FROM contacts
        ORDER BY id DESC
        "#
    )
    .fetch_all(pool)
    .await?;
    
    Ok(rows)
}

#[post("/contact")]
pub async fn submit_contact(
    form: web::Form<ContactForm>,
    pool: web::Data<SqlitePool>,
    tmpl: web::Data<Tera>,
) -> HttpResponse {
    // Validate form data
    if form.name.is_empty() || form.email.is_empty() || form.message.is_empty() {
        let mut ctx = tera::Context::new();
        ctx.insert("portfolio", &get_portfolio_data());
        ctx.insert("error", "All fields are required");
        
        match tmpl.render("index.html.tera", &ctx) {
            Ok(rendered) => HttpResponse::Ok().body(rendered),
            Err(e) => {
                eprintln!("Template error: {}", e);
                HttpResponse::InternalServerError().body("Error rendering template")
            }
        }
    } else if !is_valid_email(&form.email) {
        let mut ctx = tera::Context::new();
        ctx.insert("portfolio", &get_portfolio_data());
        ctx.insert("error", "Invalid email address");
        
        match tmpl.render("index.html.tera", &ctx) {
            Ok(rendered) => HttpResponse::Ok().body(rendered),
            Err(e) => {
                eprintln!("Template error: {}", e);
                HttpResponse::InternalServerError().body("Error rendering template")
            }
        }
    } else {
        // Save to database
        match save_contact(&pool, &form).await {
            Ok(_) => {
                let mut ctx = tera::Context::new();
                ctx.insert("portfolio", &get_portfolio_data());
                ctx.insert("success", "Thank you for your message! I'll get back to you soon.");
                
                match tmpl.render("index.html.tera", &ctx) {
                    Ok(rendered) => HttpResponse::Ok().body(rendered),
                    Err(e) => {
                        eprintln!("Template error: {}", e);
                        HttpResponse::InternalServerError().body("Error rendering template")
                    }
                }
            },
            Err(e) => {
                eprintln!("Failed to save contact: {}", e);
                let mut ctx = tera::Context::new();
                ctx.insert("portfolio", &get_portfolio_data());
                ctx.insert("error", "Failed to save your message. Please try again later.");
                
                match tmpl.render("index.html.tera", &ctx) {
                    Ok(rendered) => HttpResponse::Ok().body(rendered),
                    Err(e) => {
                        eprintln!("Template error: {}", e);
                        HttpResponse::InternalServerError().body("Error rendering template")
                    }
                }
            }
        }
    }
}

fn get_portfolio_data() -> PortfolioData {
    PortfolioData {
        name: "Philani Dlamini".to_string(),
        title: "Full Stack Developer".to_string(),
        about: "I'm a passionate developer with expertise in cutting-edge technologies. I thrive on solving complex problems and creating innovative solutions that push boundaries.".to_string(),
        skills: vec![
            "Python".to_string(),
            "Rust".to_string(),
            "MongoDB".to_string(),
            "React".to_string(),
            "REDIS".to_string(),
            "PostgreSQL".to_string(),
            "Actix Web".to_string(),
            "Sage 200".to_string(),
            "Docker".to_string(),
        ],
        projects: vec![
            Project {
                name: "Futuristic Portfolio Web App".to_string(),
                description: "A futuristic web application built with Rust and modern frontend technologies.".to_string(),
                technologies: vec!["Rust".to_string(), "Tera".to_string(), "WASM".to_string(), "Actix Web".to_string(), "CSS".to_string(), "JS".to_string()],
                github_url: Some("https://github.com/Killjoy99/portfolio-rs".to_string()),
                live_url: None,
            },
            Project {
                name: "Kivy Lazy Loading Template".to_string(),
                description: "A modern kivymd template with all the Optimisation for beginner Android Application Developers utilising Python 13+, integrated with Cython for Optimisations.".to_string(),
                technologies: vec!["Python".to_string(), "Kivy".to_string(), "KivyMD".to_string(), "Cython".to_string()],
                github_url: Some("https://github.com/Killjoy99/kivymd-lazy-loading-template".to_string()),
                live_url: None,
            },
            Project {
                name: "Sage 200 Python API".to_string(),
                description: "A minimalistic sage SDK integration API using Python FastAPI".to_string(),
                technologies: vec!["Python".to_string(), "pythonnet".to_string(), "FastAPI".to_string(), "SqlAlchemy".to_string()],
                github_url: Some("https://github.com/Killjoy99/sage_integration".to_string()),
                live_url: None,
            },
        ],
        contact_email: "dlamini.philani@outlook.com".to_string(),
    }
}


async fn save_contact(pool: &SqlitePool, form: &ContactForm) -> Result<(), sqlx::Error> {
    let query = r#"
        INSERT INTO contacts (name, email, message, created_at)
        VALUES (?, ?, ?, ?)
    "#;
    
    sqlx::query(query)
        .bind(&form.name)
        .bind(&form.email)
        .bind(&form.message)
        .bind(Utc::now())
        .execute(pool)
        .await?;
    
    Ok(())
}

fn is_valid_email(email: &str) -> bool {
    // Basic email validation
    let re = regex::Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
    re.is_match(email)
}

// Add this function to initialize the database
pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    // Use a relative path for the SQLite database file in the project directory
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://portfolio.db".to_string());
    
    let pool = SqlitePool::connect(&database_url).await?;
    
    // Create contacts table if it doesn't exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS contacts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT NOT NULL,
            message TEXT NOT NULL,
            created_at TEXT NOT NULL
        )
        "#
    )
    .execute(&pool)
    .await?;
    
    Ok(pool)
}
