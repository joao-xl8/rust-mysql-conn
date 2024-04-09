use dotenv::dotenv;
use mysql::prelude::*;
use mysql::*;

#[derive(Debug, PartialEq, Eq)]
struct Resume {
    id: i32,
    name: String,
    email_address: String,
    education: String,
    experience: String,
}

struct Database {
    pool: Pool,
}

impl Database {
    fn new() -> Result<Self> {
        let user_env = std::env::var("DATABASE_USER").expect("DATABASE_USER must be set.");
        let pass_env = std::env::var("DATABASE_PASS").expect("DATABASE_PASS must be set.");
        let db_name_env = std::env::var("DATABASE_NAME").expect("DATABASE_NAME must be set.");
        let port_env = std::env::var("DATABASE_PORT").expect("DATABASE_PORT must be set.");

        let opts = OptsBuilder::new()
            .user(Some(user_env))
            .pass(Some(pass_env))
            .tcp_port(port_env.parse::<u16>().unwrap())
            .db_name(Some(db_name_env));

        let pool = Pool::new(opts)?;
        Ok(Self { pool })
    }

    fn create_table(&mut self) -> Result<()> {
        let mut conn = self.pool.get_conn()?;
        conn.query_drop(
            r"CREATE TABLE resumes (
                id INT NOT NULL AUTO_INCREMENT,
                name TEXT NOT NULL,
                email_address TEXT NOT NULL,
                education TEXT NOT NULL,
                experience TEXT NOT NULL,
                PRIMARY KEY (id)
            )",
        )?;
        Ok(())
    }

    fn insert_resume(&mut self, resume: Resume) -> Result<()> {
        let mut conn = self.pool.get_conn()?;
        conn.exec_drop(
            r"INSERT INTO resumes (name, email_address, education, experience)
            VALUES (:name, :email_address, :education, :experience)",
            params! {
                "name" => resume.name,
                "email_address" => resume.email_address,
                "education" => resume.education,
                "experience" => resume.experience,
            },
        )?;
        Ok(())
    }

    fn delete_resume(&mut self, id: i32) -> Result<()> {
        let mut conn = self.pool.get_conn()?;
        conn.exec_drop(
            r"DELETE FROM resumes WHERE id = :id",
            params! {
                "id" => id,
            },
        )?;
        Ok(())
    }

    fn get_all_resumes(&mut self) -> Result<Vec<Resume>> {
        let mut conn = self.pool.get_conn().expect("Failed to get connection.");
        let all_resumes = conn
            .query_map(
                "SELECT id, name, email_address, education, experience from resumes",
                |(id, name, email_address, education, experience)| Resume {
                    id,
                    name,
                    email_address,
                    education,
                    experience,
                },
            )
            .expect("Failed to query resumes.");
        Ok(all_resumes)
    }
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Load .env file

    let mut db = Database::new()?;
    let all_resumes = db.get_all_resumes()?;
    println!("{:?}", all_resumes);

    // populate resumes
    let resume1 = Resume {
        id: 1,
        name: "John Doe".to_string(),
        email_address: "teste@teste".to_string(),
        education: "Computer Science".to_string(),
        experience: "Software Engineer".to_string(),
    };
    let resume2 = Resume {
        id: 2,
        name: "Jane Doe".to_string(),
        email_address: "teste@teste".to_string(),
        education: "Computer Science".to_string(),
        experience: "Software Engineer".to_string(),
    };
    let resume3 = Resume {
        id: 3,
        name: "John Smith".to_string(),
        email_address: "teste@teste".to_string(),
        education: "Computer Science".to_string(),
        experience: "Software Engineer".to_string(),
    };
    db.insert_resume(resume1)?;
    db.insert_resume(resume2)?;
    db.insert_resume(resume3)?;

    let all_resumes = db.get_all_resumes()?;
    println!("{:?}", all_resumes);

    // delete resume
    db.delete_resume(1)?;

    let all_resumes = db.get_all_resumes()?;
    println!("{:?}", all_resumes);

    Ok(())
}
