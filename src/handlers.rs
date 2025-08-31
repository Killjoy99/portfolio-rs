use actix_web::{get, web, HttpResponse};
use serde::Serialize;
use tera::Tera;

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

#[get("/")]
pub async fn index(tmpl: web::Data<Tera>) -> HttpResponse {
    let portfolio_data = PortfolioData {
        name: "Philani Dlamini".to_string(),
        title: "Full Stack Developer".to_string(),
        about: "I'm a passionate developer with experience in Python, a lil bit of C and Rust, and modern web technologies. I love building efficient and scalable applications.".to_string(),
        skills: vec![
            "Python".to_string(),
            "FastAPI".to_string(),
            "Rust".to_string(),
            "sqlx".to_string(),
            "JavaScript".to_string(),
            "ReactJs".to_string(),
            "PostgreSQL".to_string(),
            "MySql".to_string(),
            "Sage 200".to_string(),
            "PHP Laravel".to_string(),
        ],
        projects: vec![
            Project {
                name: "Project 1".to_string(),
                description: "A web application built with Actix Web and React".to_string(),
                technologies: vec!["Rust".to_string(), "React".to_string(), "PostgreSQL".to_string()],
                github_url: Some("https://github.com/Killjoy99/project1".to_string()),
                live_url: Some("https://project1.demo".to_string()),
            },
            Project {
                name: "Project 2".to_string(),
                description: "A CLI tool for developers".to_string(),
                technologies: vec!["Rust".to_string(), "CLAP".to_string()],
                github_url: Some("https://github.com/Killjoy99/project2".to_string()),
                live_url: None,
            },
        ],
        contact_email: "philani.dlamini@outlook.com".to_string(),
    };

    let mut ctx = tera::Context::new();
    ctx.insert("portfolio", &portfolio_data);

    let rendered = tmpl.render("index.html.tera", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/projects")]
pub async fn projects() -> HttpResponse {
    HttpResponse::Ok().body("Projects page - coming soon!")
}

#[get("/contact")]
pub async fn contact() -> HttpResponse {
    HttpResponse::Ok().body("Contact page - coming soon!")
}