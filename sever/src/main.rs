use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Serialize)]
struct SystemInfo {
    os: String,
    arch: String,
    uptime: String,
}

#[derive(Deserialize)]
struct DockerCommand {
    command: String,
    args: Vec<String>,
}

#[get("/api/system-info")]
async fn get_system_info() -> impl Responder {
    let os = std::env::consts::OS.to_string();
    let arch = std::env::consts::ARCH.to_string();
    let uptime_output = Command::new("uptime").output();
    let uptime = match uptime_output {
        Ok(output) => String::from_utf8_lossy(&output.stdout).trim().to_string(),
        Err(_) => "N/A".to_string(),
    };

    let info = SystemInfo { os, arch, uptime };
    HttpResponse::Ok().json(info)
}

#[post("/api/docker")]
async fn handle_docker_command(req_body: web::Json<DockerCommand>) -> impl Responder {
    let mut cmd = Command::new("docker");
    cmd.arg(&req_body.command);
    for arg in &req_body.args {
        cmd.arg(arg);
    }

    let output = cmd.output().expect("Failed to execute Docker command");

    if output.status.success() {
        HttpResponse::Ok().body(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        HttpResponse::InternalServerError().body(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_system_info)
            .service(handle_docker_command)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
