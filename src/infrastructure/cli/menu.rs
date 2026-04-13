use colored::Colorize;
use dialoguer::{Confirm, Input, Password, Select};
use std::str::FromStr;
use std::sync::Arc;

use crate::application::auth::login::LoginUseCase;
use crate::application::auth::register::RegisterUsecase;
use crate::application::employee::create_employee::{CreateEmployeeInput, CreateEmployeeUseCase};
use crate::application::employee::delete_employee::DeleteEmployeeUsecase;
use crate::application::employee::list_employee::ListEmployeeUsecase;
use crate::domain::entities::employee::{Departament, Employee};
use crate::domain::errors::DomainError;
use crate::domain::ports::employee_repository::EmployeeRepository;
use crate::infrastructure::persistence::in_memory_employee_repository::InMemoryEmployeeRepository;
use crate::infrastructure::persistence::postgres_employee_repository::PostgressEmployeeRepository;

const DEPARTMENTS: [&str; 5] = ["Engineering", "Sale", "RRHH", "Finance", "Operations"];

enum MainMenuOption {
    Login,
    Register,
    Exit,
}

impl MainMenuOption {
    fn from_index(i: usize) -> Result<Self, DomainError> {
        match i {
            0 => Ok(Self::Login),
            1 => Ok(Self::Register),
            2 => Ok(Self::Exit),
            _ => Err(DomainError::MenuError("Invalid option".to_string())),
        }
    }
}

enum AuthMenuOption {
    ListEmployees,
    CreateEmployee,
    DeleteEmployee,
    Logout,
}

impl AuthMenuOption {
    fn from_index(i: usize) -> Result<Self, DomainError> {
        match i {
            0 => Ok(Self::ListEmployees),
            1 => Ok(Self::CreateEmployee),
            2 => Ok(Self::DeleteEmployee),
            3 => Ok(Self::Logout),
            _ => Err(DomainError::MenuError("Invalid option".to_string())),
        }
    }
}

pub struct Menu;

impl Menu {
    pub fn new() -> Self {
        Self
    }

    pub async fn main(&self) -> Result<(), DomainError> {
        dotenvy::dotenv().ok();

        let repo: Arc<dyn EmployeeRepository> = match std::env::var("DATABASE_URL") {
            Ok(url) => match PostgressEmployeeRepository::new(&url).await {
                Ok(pg) => Arc::new(pg),
                Err(e) => {
                    println!(
                        "{}",
                        format!(
                            "[WARNING] Could not connect to Postgres: {}. Using in-memory storage — data will not persist between sessions.",
                            e
                        )
                        .yellow()
                        .bold()
                    );
                    Arc::new(InMemoryEmployeeRepository::new())
                }
            },
            Err(_) => {
                println!(
                    "{}",
                    "[WARNING] DATABASE_URL not set. Using in-memory storage — data will not persist between sessions."
                        .yellow()
                        .bold()
                );
                Arc::new(InMemoryEmployeeRepository::new())
            }
        };

        loop {
            println!("\n{}", "=== Management System ===".bold().cyan());

            let items = ["Login", "Register", "Exit"];
            let selection = Select::new()
                .with_prompt("Select an option")
                .items(&items)
                .default(0)
                .interact()
                .map_err(|e| DomainError::MenuError(e.to_string()))?;

            match MainMenuOption::from_index(selection)? {
                MainMenuOption::Login => {
                    match self.handle_login(Arc::clone(&repo)).await {
                        Ok(employee) => {
                            println!("{}", format!("Welcome, {}!", employee.name).green().bold());
                            self.authenticated_menu(Arc::clone(&repo)).await?;
                        }
                        Err(e) => println!("{}", format!("Error: {}", e).red()),
                    }
                }
                MainMenuOption::Register => {
                    match self.handle_register(Arc::clone(&repo)).await {
                        Ok(employee) => {
                            println!(
                                "{}",
                                format!("Employee '{}' registered successfully!", employee.name)
                                    .green()
                            );
                        }
                        Err(e) => println!("{}", format!("Error: {}", e).red()),
                    }
                }
                MainMenuOption::Exit => {
                    println!("{}", "Goodbye!".yellow());
                    break;
                }
            }
        }

        Ok(())
    }

    async fn handle_login(&self, repo: Arc<dyn EmployeeRepository>) -> Result<Employee, DomainError> {
        println!("\n{}", "--- Login ---".bold());

        let email: String = Input::new()
            .with_prompt("Email")
            .interact_text()
            .map_err(|e| DomainError::MenuError(e.to_string()))?;

        let password: String = Password::new()
            .with_prompt("Password")
            .interact()
            .map_err(|e| DomainError::MenuError(e.to_string()))?;

        LoginUseCase::new(repo).execute(&email, &password).await
    }

    async fn handle_register(
        &self,
        repo: Arc<dyn EmployeeRepository>,
    ) -> Result<Employee, DomainError> {
        println!("\n{}", "--- Register ---".bold());

        let input = self.employee_form_input()?;
        RegisterUsecase::new(repo).execute(input).await
    }

    async fn authenticated_menu(&self, repo: Arc<dyn EmployeeRepository>) -> Result<(), DomainError> {
        loop {
            println!("\n{}", "--- Employee Panel ---".bold().cyan());

            let items = ["List Employees", "Create Employee", "Delete Employee", "Logout"];
            let selection = Select::new()
                .with_prompt("Select an option")
                .items(&items)
                .default(0)
                .interact()
                .map_err(|e| DomainError::MenuError(e.to_string()))?;

            match AuthMenuOption::from_index(selection)? {
                AuthMenuOption::ListEmployees => {
                    if let Err(e) = self.handle_list(Arc::clone(&repo)).await {
                        println!("{}", format!("Error: {}", e).red());
                    }
                }
                AuthMenuOption::CreateEmployee => {
                    match self.handle_create(Arc::clone(&repo)).await {
                        Ok(emp) => {
                            println!("{}", format!("Employee '{}' created.", emp.name).green())
                        }
                        Err(e) => println!("{}", format!("Error: {}", e).red()),
                    }
                }
                AuthMenuOption::DeleteEmployee => {
                    match self.handle_delete(Arc::clone(&repo)).await {
                        Ok(_) => println!("{}", "Employee deleted successfully.".green()),
                        Err(e) => println!("{}", format!("Error: {}", e).red()),
                    }
                }
                AuthMenuOption::Logout => {
                    println!("{}", "Logged out.".yellow());
                    break;
                }
            }
        }

        Ok(())
    }

    async fn handle_list(&self, repo: Arc<dyn EmployeeRepository>) -> Result<(), DomainError> {
        let employees = ListEmployeeUsecase::new(repo).execute().await?;

        if employees.is_empty() {
            println!("{}", "No employees found.".yellow());
            return Ok(());
        }

        println!("\n{}", "--- Employees ---".bold());
        for emp in &employees {
            println!(
                "  {} | {} | {} | {} | Active: {}",
                emp.id.to_string().dimmed(),
                emp.name.bold(),
                emp.departament,
                emp.email.value(),
                if emp.active {
                    "Yes".green().to_string()
                } else {
                    "No".red().to_string()
                },
            );
        }

        Ok(())
    }

    async fn handle_create(&self, repo: Arc<dyn EmployeeRepository>) -> Result<Employee, DomainError> {
        println!("\n{}", "--- Create Employee ---".bold());

        let input = self.employee_form_input()?;
        CreateEmployeeUseCase::new(repo).execute(input).await
    }

    async fn handle_delete(&self, repo: Arc<dyn EmployeeRepository>) -> Result<(), DomainError> {
        self.handle_list(Arc::clone(&repo)).await?;

        let id_str: String = Input::new()
            .with_prompt("Employee ID to delete")
            .interact_text()
            .map_err(|e| DomainError::MenuError(e.to_string()))?;

        let id = uuid::Uuid::parse_str(&id_str)
            .map_err(|_| DomainError::EmployeeNotFound(id_str))?;

        let confirmed = Confirm::new()
            .with_prompt("Are you sure you want to delete this employee?")
            .default(false)
            .interact()
            .map_err(|e| DomainError::MenuError(e.to_string()))?;

        if !confirmed {
            println!("{}", "Cancelled.".yellow());
            return Ok(());
        }

        DeleteEmployeeUsecase::new(repo).execute(id).await
    }

    fn employee_form_input(&self) -> Result<CreateEmployeeInput, DomainError> {
        let name: String = Input::new()
            .with_prompt("Name")
            .interact_text()
            .map_err(|e| DomainError::MenuError(e.to_string()))?;

        let email: String = Input::new()
            .with_prompt("Email")
            .interact_text()
            .map_err(|e| DomainError::MenuError(e.to_string()))?;

        let password: String = Password::new()
            .with_prompt("Password")
            .with_confirmation("Confirm password", "Passwords do not match")
            .interact()
            .map_err(|e| DomainError::MenuError(e.to_string()))?;

        let dept_index = Select::new()
            .with_prompt("Department")
            .items(&DEPARTMENTS)
            .default(0)
            .interact()
            .map_err(|e| DomainError::MenuError(e.to_string()))?;

        let departament = Departament::from_str(DEPARTMENTS[dept_index])?;

        let salary_str: String = Input::new()
            .with_prompt("Salary")
            .interact_text()
            .map_err(|e| DomainError::MenuError(e.to_string()))?;

        let salary = salary_str
            .parse::<rust_decimal::Decimal>()
            .map_err(|_| DomainError::InvalidSalary(salary_str))?;

        Ok(CreateEmployeeInput {
            name,
            departament,
            email,
            password_hash: password,
            salary,
        })
    }
}
