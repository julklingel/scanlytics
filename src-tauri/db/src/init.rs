use std::sync::Arc;

use tokio::sync::Mutex;
use tauri::Manager;

use crate::models::DbConnection;  



use surrealdb::engine::any;

pub async fn init_db(app_handle: Option<&tauri::AppHandle> ,is_test: bool) -> Result<DbConnection, String> {
    let endpoint = if is_test {
        "memory".to_string()
    } else {
        let app_handle = app_handle.ok_or("App handle is None")?;
        let app_local_data_dir = app_handle
            .path()
            .app_local_data_dir()
            .map_err(|e| format!("Failed to get app local data directory: {}", e))?;
        
        let db_path = app_local_data_dir.join("database.db");
        format!("rocksdb:{}", db_path.display())
    };

    let db = any::connect(endpoint)
        .await
        .map_err(|e| e.to_string())?;

    let ns = if is_test { "test_namespace" } else { "namespace" };
    let db_name = if is_test { "test_database" } else { "database" };

    db.use_ns(ns)
        .use_db(db_name)
        .await
        .map_err(|e| e.to_string())?;

    Ok(DbConnection(Arc::new(Mutex::new(db))))
}



pub async fn define_db_on_startup(db_connection: DbConnection) -> Result<(), String> {
    let define_statements: Vec<&str> = vec![
        "DEFINE TABLE Organization SCHEMAFULL;",
        "DEFINE FIELD name ON Organization TYPE string;",
        "DEFINE FIELD address ON Organization TYPE string;",
        "DEFINE FIELD email ON Organization TYPE string ASSERT string::is::email($value);",
        "DEFINE FIELD created_at ON Organization TYPE datetime DEFAULT time::now();",
        "DEFINE FIELD updated_at ON Organization TYPE datetime DEFAULT time::now() VALUE time::now();",
        "DEFINE FIELD user ON TABLE Organization TYPE option<array<record<User>>>;",
        "DEFINE FIELD user.* ON Organization TYPE option<record<User>>;",

        "DEFINE TABLE User SCHEMAFULL;",
        "DEFINE FIELD name ON User TYPE string;",
        "DEFINE FIELD email ON User TYPE string ASSERT string::is::email($value);",
        "DEFINE FIELD role ON User TYPE string;",
        "DEFINE FIELD created_at ON User TYPE datetime DEFAULT time::now();",
        "DEFINE FIELD updated_at ON User TYPE datetime DEFAULT time::now() VALUE time::now();",
        "DEFINE FIELD notes ON TABLE User TYPE option<array<record<PatientNote>>>;",
        "DEFINE FIELD notes.* ON User TYPE option<record<PatientNote>>;",
        "DEFINE FIELD Statement ON TABLE User TYPE option<array<record<Statement>>>;",
        "DEFINE FIELD Statement.* ON User TYPE option<record<Statement>>;",
        "DEFINE FIELD Image ON TABLE User TYPE option<array<record<Image>>>;",
        "DEFINE FIELD Image.* ON User TYPE option<record<Image>>;",
        "DEFINE FIELD organization ON User TYPE option<record<Organization>>;",
        "DEFINE FIELD in ON TABLE Write_Reports TYPE record<User>;",
        "DEFINE FIELD out ON TABLE Write_Reports TYPE record<Report>;",
        "DEFINE FIELD in ON TABLE Access_Statements TYPE record<User>;",
        "DEFINE FIELD out ON TABLE Access_Statements TYPE record<Statement>;",

        "DEFINE TABLE Patient SCHEMAFULL;",
        "DEFINE FIELD name ON Patient TYPE string;",
        "DEFINE FIELD date_of_birth ON Patient TYPE datetime;",
        "DEFINE FIELD gender ON Patient TYPE string;",
        "DEFINE FIELD contact_number ON Patient TYPE string;",
        "DEFINE FIELD address ON Patient TYPE string;",
        "DEFINE FIELD created_at ON Patient TYPE datetime DEFAULT time::now();",
        "DEFINE FIELD updated_at ON Patient TYPE datetime DEFAULT time::now() VALUE time::now();",
        "DEFINE FIELD notes ON TABLE Patient TYPE option<array<record<PatientNote>>>;",
        "DEFINE FIELD notes.* ON Patient TYPE option<record<PatientNote>>;",
        "DEFINE FIELD report ON TABLE Patient TYPE option<array<record<Report>>>;",
        "DEFINE FIELD report.* ON Patient TYPE option<record<Report>>;",
        "DEFINE FIELD image ON TABLE Patient TYPE option<array<record<Image>>>;",
        "DEFINE FIELD image.* ON Patient TYPE option<record<Image>>;",
        "DEFINE FIELD out ON TABLE Treated_By TYPE record<User>;",
        "DEFINE FIELD in ON TABLE Treated_By TYPE record<Patient>;",

        "DEFINE TABLE PatientNote SCHEMAFULL;",
        "DEFINE FIELD symptoms ON PatientNote TYPE string;",
        "DEFINE FIELD diagnosis ON PatientNote TYPE string;",
        "DEFINE FIELD treatment ON PatientNote TYPE string;",
        "DEFINE FIELD created_at ON PatientNote TYPE datetime DEFAULT time::now() VALUE time::now();",
        "DEFINE FIELD updated_at ON PatientNote TYPE datetime DEFAULT time::now() VALUE time::now();",
        "DEFINE FIELD severity ON PatientNote TYPE string ASSERT $value IN ['low', 'medium', 'high'];",
        "DEFINE FIELD is_urgent ON PatientNote TYPE bool;",
        "DEFINE FIELD patient ON PatientNote TYPE record<Patient>;",
        "DEFINE FIELD user_owner ON PatientNote TYPE record<User>;",
        "DEFINE FIELD out ON TABLE PatientNotes_Reports_Join TYPE record<User>;",
        "DEFINE FIELD in ON TABLE PatientNotes_Reports_Join TYPE record<PatientNote>;",

        "DEFINE TABLE Statement SCHEMAFULL;",
        "DEFINE FIELD body_part ON Statement TYPE string;",
        "DEFINE FIELD indication ON Statement TYPE string;",
        "DEFINE FIELD statement ON Statement TYPE string;",
        "DEFINE FIELD assessment ON Statement TYPE string;",
        "DEFINE FIELD created_at ON Statement TYPE datetime DEFAULT time::now();",
        "DEFINE FIELD updated_at ON Statement TYPE datetime DEFAULT time::now() VALUE time::now();",
        "DEFINE FIELD user_owner ON Statement TYPE option<record<User>>;",

        "DEFINE TABLE Report SCHEMAFULL;",
        "DEFINE FIELD body_part ON Report TYPE option<string>;",
        "DEFINE FIELD condition ON Report TYPE option<string>;",
        "DEFINE FIELD report_text ON Report TYPE string;",
        "DEFINE FIELD created_at ON Report TYPE datetime DEFAULT time::now();",
        "DEFINE FIELD updated_at ON Report TYPE datetime DEFAULT time::now() VALUE time::now();",
        "DEFINE FIELD in ON TABLE Statements_Reports_Join TYPE record<Statement>;",
        "DEFINE FIELD out ON TABLE Statements_Reports_Join TYPE record<Report>;",
        "DEFINE FIELD patient ON Report TYPE record<Patient>;",
        "DEFINE FIELD user_owner ON Report TYPE record<User>;",

        "DEFINE TABLE Image SCHEMAFULL;",
        "DEFINE FIELD name ON Image TYPE string;",
        "DEFINE FIELD path ON Image TYPE string;",
        "DEFINE FIELD body_type ON Image TYPE option<string>;",
        "DEFINE FIELD modal_type ON Image TYPE string ASSERT $value IN ['xray', 'mri', 'ct'];",
        "DEFINE FIELD file_type ON Image TYPE string;",
        "DEFINE FIELD created_at ON Image TYPE datetime DEFAULT time::now();",
        "DEFINE FIELD updated_at ON Image TYPE datetime DEFAULT time::now() VALUE time::now();",
        "DEFINE FIELD patient ON Image TYPE record<Patient>;",
        "DEFINE FIELD user ON Image TYPE record<User>;",
        "DEFINE FIELD in ON TABLE Images_Reports_Join TYPE record<Image>;",
        "DEFINE FIELD out ON TABLE Images_Reports_Join TYPE record<Report>;",
        "DEFINE INDEX Treated_By ON TABLE Treated_By COLUMNS in, out UNIQUE;",
        "DEFINE INDEX Access_Statements ON TABLE Access_Statements COLUMNS in, out UNIQUE;",
        "DEFINE INDEX PatientNotes_Reports_Join ON TABLE PatientNotes_Reports_Join COLUMNS in, out UNIQUE;",
        "DEFINE INDEX Statements_Reports_Join ON TABLE Statements_Reports_Join COLUMNS in, out UNIQUE;",
        "DEFINE INDEX Images_Reports_Join ON TABLE Images_Reports_Join COLUMNS in, out UNIQUE;",
        "DEFINE INDEX Write_Reports ON TABLE Write_Reports COLUMNS in, out UNIQUE;",
        "DEFINE INDEX Email ON TABLE User COLUMNS email UNIQUE;",

        "DEFINE TABLE Treated_By SCHEMAFULL;",
        "DEFINE TABLE Access_Statements SCHEMAFULL;",
        "DEFINE TABLE PatientNotes_Reports_Join SCHEMAFULL;",
        "DEFINE TABLE Statements_Reports_Join SCHEMAFULL;",
        "DEFINE TABLE Images_Reports_Join SCHEMAFULL;",
        "DEFINE TABLE Write_Reports SCHEMAFULL;",

        "DEFINE TABLE Models SCHEMAFULL;",
        "DEFINE FIELD name ON TABLE Models TYPE string;",
        "DEFINE FIELD version ON TABLE Models TYPE string;",
        "DEFINE FIELD category ON TABLE Models TYPE string;",
        "DEFINE FIELD link ON TABLE Models TYPE string;",
        "DEFINE FIELD created_at ON TABLE Models TYPE datetime DEFAULT time::now();",
    ];

    for statement in define_statements {
        let db = db_connection.get().lock().await;
        db.query(statement).await.map_err(|e| e.to_string())?;
    }

    let initial_statements = vec![

        "CREATE Statement SET \
            body_part = 'knee', \
            indication = '', \
            statement = 'Herr Dr. Norbert Munz\nFÄ/FA für Orthopädie und Unfallchirurgie\nOranienstraße 158\n10969 Berlin\n\nBetrifft: Möller, Uwe , * 16.12.1986\n\nHRDr. Norbert Munz,\n\ndie Untersuchung ergab folgenden Befund:\n\n', \
            assessment = ''",
        
        "CREATE Statement SET \
            body_part = 'knee', \
            indication = '', \
            statement = 'Rechtes Kniegelenk:\nDeutliche Gelenkspaltverschmälerung des femorotibialen Gelenkspaltes im medialen Kompartiment mit mäßiggradigen osteophytären Randausziehungen der Gelenkflächen. Keine Geröllzysten. Deutliche Gelenkspaltverschmälerung im femoropatellaren Gelenk mit Kantenanbauten am Oberpol und Unterpol der Patella sowie am femoralen Gleitlager. Keine Frakturlinien. Röntgenologisch kein Gelenkerguss.\n', \
            assessment = ''",
        
        "CREATE Statement SET \
            body_part = 'knee', \
            indication = '', \
            statement = 'Beurteilung:\nRechtes Kniegelenk:\nMittelgradige Arthrose im femorotibialen Gelenk, medial betont.', \
            assessment = ''",

    "CREATE User SET \
        email = 'dr.test@med.com', \
        password = '911medical', \
        name = 'Dr. Testo', \
        role = 'user' \
        "

    ];

    for statement in initial_statements {
        let db = db_connection.get().lock().await;
        db.query(statement).await.map_err(|e| e.to_string())?;
    }

    Ok(())
}

