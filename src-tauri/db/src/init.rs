use std::sync::Arc;

use tauri::Manager;
use tokio::sync::Mutex;

use crate::models::DbConnection;  

use surrealdb::engine::any;

pub async fn init_db(
    app_handle: Option<&tauri::AppHandle>,
    is_test: bool,
) -> Result<DbConnection, String> {
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

    let db = any::connect(endpoint).await.map_err(|e| e.to_string())?;

    let ns = if is_test {
        "test_namespace"
    } else {
        "namespace"
    };
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
       
        
        "CREATE Statement SET 
            body_part = 'thorax',
            indication = '',
            statement = 'Zwerchfell glatt konturiert, laterale Randwinkel frei. Lunge seitengleich belüftet mit unauffälliger Gefäßzeichnung.
    
            Kein Nachweis pneumonischer Infiltrate, keine Ergussbildung.
    
            Herz von normaler Größe und Konfiguration. Mediastinum mittelständig, nicht verbreitert.
    
            Trachea mittelständig, nicht eingeengt. Knöcherner Thorax unauffällig.',
            assessment = ''",
    
        "CREATE Statement SET 
            body_part = 'shoulder',
            indication = '',
            statement = 'Die am Glenohumeralgelenk und am AC-Gelenk beteiligten Skelettabschnitte sind normal konfiguriert. Mineralgehalt und Knochenstruktur regelrecht.
            Regelrechte Artikulation im Glenohumeralgelenk und im AC-Gelenk. Der Gelenkspalt ist allseits normal weit.
    
            Die mitabgebildeten knöchernen Strukturen des Schultergürtels und des Thorax sind unauffällig.
            Keine periartikulären Verkalkungen.',
            assessment = ''",
    
      
        "CREATE Statement SET 
            body_part = 'thorax',
            indication = '',
            statement = 'Harmonische Lordose. Alle LWK nach Anzahl, Form und Größe normal konfiguriert. Keine Gefügestörung.
            Regelrechter Mineralgehalt. Kortikale Randstrukturen einschließlich der Grund- und Deckplatten stellen sich glatt begrenzt dar.
    
            Regelrechte Abbildung der Abgänge der Bogenwurzeln. Unauffällige Konfiguration der Dorn-, Quer- und Gelenkfortsätze.
    
            Keine Höhenminderung der ZWR. Der Spinalkanal weist knöchern keine Stenosierung auf. In den paravertebralen Weichteilen kein Nachweis pathologischer Verkalkungen.',
            assessment = ''",
    
      
        "CREATE Statement SET 
            body_part = 'knee',
            indication = '',
            statement = 'Normale Form des Kniegelenkes ohne Achsfehlstellung. Regelrechte Artikulation mit unauffälliger Darstellung der artikulierenden Gelenkflächen. Normale Gelenkspaltweite.
            Mineralgehalt und Knochenstruktur regelrecht. Glatte Kortikalisbegrenzung von Femur und Tibia. Regelrechte Form der Patella mit unauffälliger Artikulation und glatter Gelenkfläche. Kein Anhalt für intra- und periartikuläre Verkalkungen. Unauffällige Weichteile.',
            assessment = ''",
    
        
        "CREATE Statement SET 
            body_part = 'hip',
            indication = '',
            statement = 'Regelrechte Artikulation mit unauffälliger Darstellung der artikulierenden Gelenkflächen. Normale Gelenkspaltweite.
            Mineralgehalt und Knochenstruktur regelrecht. Pfannendach unauffällig abgebildet. Glatte Hüftkopfkonturen. Regelrechte Trabekulierung im Schenkelhals.
    
            Unauffälliges Trochantermassiv. Kein Anhalt für intra- und periartikuläre Verkalkungen.
    
            Die ossären Strukturen des mitabgebildeten knöchernen Beckens stellen sich regelrecht dar. Unauffällige Weichteile.',
            assessment = ''",
    
        
        "CREATE Statement SET 
            body_part = 'wrist',
            indication = '',
            statement = 'Am Handgelenk _____ sind keine Frakturen oder andere knöcherne Anomalien zu erkennen. Die radiokarpalen und interkarpalen Gelenke sehen normal aus und es gibt keine Verschiebung des Pronator-Fettpolsters.
            Es sind keine Weichteilschwellungen zu sehen.',
            assessment = ''",
    
        
        "CREATE Statement SET 
            body_part = 'hand',
            indication = '',
            statement = 'Keine Voraufnahmen zum Vergleich. Normaler Kalksalzgehalt und reguläre Knochenstruktur des Handskeletts. Achsengerechte Stellung und normal breite Gelenkspalte an allen abgebildeten Gelenken.
            Kein Hinweis für entzündlich bedingte ossäre Läsionen. Keine wesentliche Weichteilschwellung.',
            assessment = ''",
    
        
        "CREATE Statement SET 
            body_part = 'foot',
            indication = '',
            statement = 'Regelrechter Kalksalzgehalt. Glatte Gelenkflächen ohne Stufenbildung, keine Gelenkfehlstellung.
            Gelenkspalt normal breit. Kein Nachweis vorzeitiger degenerativer Veränderungen.
    
            Fußgewölbe regelrecht. Unauffällige Weichteile.',
            assessment = ''",
    
        
        "CREATE Statement SET 
            body_part = 'heel',
            indication = '',
            statement = 'Die abgebildeten Knochen weisen eine normale Ausrichtung und Architektur auf.
            Keine offensichtliche lytische oder sklerotische knöcherne Läsion.
            Es ist keine offensichtliche Fraktur zu erkennen.
            Die Gelenkspalte und Gelenkränder sind intakt.
            Die Weichteile zeigen ein normales Erscheinungsbild.',
            assessment = ''",
    
        
        "CREATE Statement SET 
            body_part = 'elbow',
            indication = '',
            statement = 'Am Ellenbogen ____ sind keine Frakturen oder andere knöcherne Anomalien zu erkennen.
            Der Gelenkspalt zeigt ein normales Erscheinungsbild ohne Verschiebung der vorderen oder hinteren Fettpolster, die auf einen Erguss hindeuten.
            Die anterioren humeralen und radiokapitellaren Linien sind normal.
            Schlussfolgerung:
            Normaler Gleitfilm ____ Ellenbogen.',
            assessment = ''",
    
        
        "CREATE Statement SET 
            body_part = 'spine',
            indication = '',
            statement = 'Harmonische Lordose. Alle LWK nach Anzahl, Form und Größe normal konfiguriert. Keine Gefügestörung.
            Regelrechter Mineralgehalt. Kortikale Randstrukturen einschließlich der Grund- und Deckplatten stellen sich glatt begrenzt dar.
    
            Regelrechte Abbildung der Abgänge der Bogenwurzeln. Unauffällige Konfiguration der Dorn-, Quer- und Gelenkfortsätze.
    
            Keine Höhenminderung der ZWR. Der Spinalkanal weist knöchern keine Stenosierung auf. In den paravertebralen Weichteilen kein Nachweis pathologischer Verkalkungen.',
            assessment = ''",

        
        "CREATE Statement SET 
            body_part = 'angio',
            indication = '',
            statement = 'Selektive Darstellung der linken und rechten Koronararterie in mehreren Projektionen.
            Linkskoronarsystem: Hauptstamm unauffällig. RIVA und RCX regelrecht angelegt, ohne relevante Stenosierungen.
            Rechtskoronarsystem: Regelrechter Verlauf der RCA ohne relevante Stenosierungen.
            
            Gute linksventrikuläre Funktion ohne regionale Wandbewegungsstörungen.
            Normale Ejektionsfraktion.
            
            Keine interventionspflichtige KHK.',
            assessment = ''",


        "CREATE Statement SET 
            body_part = 'angio',
            indication = '',
            statement = 'DSA der unteren Extremität in mehreren Serien.
            Regelrechte Darstellung der A. femoralis communis, profunda und superficialis.
            A. poplitea und Unterschenkelarterien durchgängig mit regulärem Lumen.
            
            Unauffälliger Abstrom in die Fußarterien.
            Keine hämodynamisch relevanten Stenosen oder Verschlüsse.
            Regelrechte Kontrastmittelpassage.',
            assessment = ''",


    "CREATE Statement SET 
            body_part = 'angio',
            indication = '',
            statement = 'Selektive Darstellung der hirnversorgenden Gefäße beidseits.
            ACC, ACI und ACE regelrecht kontrastiert ohne Stenosen.
            Intrakraniell regelrechte Gefäßaufzweigungen des vorderen und hinteren Stromgebiets.
            
            Basilarissystem unauffällig.
            Zeitgerechte arterielle und venöse Drainage.
            Keine Hinweise auf arteriovenöse Malformationen oder Aneurysmen.
            
            Regelrechte Perfusion aller Gefäßterritorien.',
            assessment = ''",
    
        
    "CREATE User SET 
            email = 'dr.test@med.com',
            password = '911medical',
            name = 'Dr. Testo',
            role = 'user'",

    "CREATE Patient SET 
            name = 'John Doe',
            date_of_birth = '1990-01-01',
            gender = 'male',
            contact_number = '1234567890',
            address = '1234 Test St, Test City, Test Country'",
    
    "CREATE Patient SET 
            name = 'Jane Doelly',
            date_of_birth = '1990-01-01',
            gender = 'female',
            contact_number = '1234567890',
            address = '1234 Test St, Test City, Test Country'",
            
    ",
            
    ];
    

    for statement in initial_statements {
        let db = db_connection.get().lock().await;
        db.query(statement).await.map_err(|e| e.to_string())?;
    }

    Ok(())
}

pub async fn setup_database(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let db_connection = init_db(Some(&app.app_handle()), true).await?;
    define_db_on_startup(db_connection.clone()).await?;
    app.manage(db_connection);
    println!("Database setup completed successfully");
    Ok(())
}