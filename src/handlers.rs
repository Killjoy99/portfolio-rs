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
                name: "Futuristic Web App".to_string(),
                description: "A cutting-edge web application built with Rust and modern frontend technologies".to_string(),
                technologies: vec!["Rust".to_string(), "React".to_string(), "WebAssembly".to_string()],
                github_url: Some("https://github.com/Killjoy99/project1".to_string()),
                live_url: Some("https://project1.demo".to_string()),
            },
            Project {
                name: "Blockchain Explorer".to_string(),
                description: "A modern blockchain explorer with real-time analytics and visualization".to_string(),
                technologies: vec!["Rust".to_string(), "Actix Web".to_string(), "PostgreSQL".to_string()],
                github_url: Some("https://github.com/Killjoy99/project2".to_string()),
                live_url: Some("https://blockchain-explorer.demo".to_string()),
            },
            Project {
                name: "AI Assistant".to_string(),
                description: "An intelligent assistant powered by machine learning and natural language processing".to_string(),
                technologies: vec!["Python".to_string(), "TensorFlow".to_string(), "FastAPI".to_string()],
                github_url: Some("https://github.com/Killjoy99/project3".to_string()),
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