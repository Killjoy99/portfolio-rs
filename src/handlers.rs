use actix_web::{get, post, web, HttpResponse};
use serde::{Serialize, Deserialize};
use tera::Tera;
use sqlx::{SqlitePool};
use chrono::{DateTime, Utc};
use web::Data;
// Remove the query! macro and use query instead
use sqlx::{query};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct ContactMessage {
    id: i64,
    name: String,
    email: String,
    message: String,
    created_at: DateTime<Utc>, // Automatically set by the database
}

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

#[derive(Deserialize)]
struct ContactForm{
    name: String,
    email: String,
    message: String,
}

// In your contact_form handler, use query with bind parameters
#[post("/contact")]
pub async fn contact_form(
    form: web::Form<ContactForm>,
    pool: Data<SqlitePool>,
) -> HttpResponse {
    // Use query() instead of query!() for runtime query preparation
    let result = query(
        "INSERT INTO contact_messages (name, email, message) VALUES (?, ?, ?)",
    )
    .bind(&form.name)
    .bind(&form.email)
    .bind(&form.message)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Thank you for your message! I'll get back to you soon."),
        Err(e) => {
            eprintln!("Failed to save message to database: {}", e);
            HttpResponse::InternalServerError().body("Failed to save your message. Please try again later.")
        }
    }
}

// In your dashboard handler, use query_as with manual binding
#[get("/dashboard")]
pub async fn dashboard(
    pool: Data<SqlitePool>,
    tmpl: Data<Tera>,
) -> HttpResponse {
    // Use query_as with explicit SQL
    let result = sqlx::query_as::<_, ContactMessage>(
        "SELECT id, name, email, message, created_at FROM contact_messages ORDER BY created_at DESC"
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(messages) => {
            let mut ctx = tera::Context::new();
            ctx.insert("messages", &messages);

            match tmpl.render("dashboard.html.tera", &ctx) {
                Ok(rendered) => HttpResponse::Ok().body(rendered),
                Err(e) => {
                    eprintln!("Template error: {}", e);
                    HttpResponse::InternalServerError().body("Error rendering template")
                }
            }
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().body("Error fetching messages")
        }
    }
}

#[get("/")]
pub async fn index(tmpl: web::Data<Tera>) -> HttpResponse {
    let portfolio_data = PortfolioData {
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
        contact_email: "philani.dlamini@outlook.com".to_string(),
    };

    let mut ctx = tera::Context::new();
    ctx.insert("portfolio", &portfolio_data);

    match tmpl.render("index.html.tera", &ctx) {
        Ok(rendered) => HttpResponse::Ok().body(rendered),
        Err(_) => HttpResponse::InternalServerError().body("Error rendering template"),
    }
}
