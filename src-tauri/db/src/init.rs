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
            body_part = 'thorax', \
            indication = 'Therapieresistenter Husten (8 Wochen), Nichtraucherin', \
            statement = 'Zwerchfell glatt konturiert, Lunge seitengleich belüftet, Keine Infiltrate/Ergüsse, Herz normal, Mediastinum unauffällig', \
            assessment = 'Unauffälliger Lungen- und Herzbefund'",

        "CREATE Statement SET \
            body_part = 'shoulder', \
            indication = 'Sturz, Bewegungseinschränkung', \
            statement = 'Normale Konfiguration, Regelrechte Artikulation, Normale Gelenkspalten, Keine Verkalkungen', \
            assessment = 'Regelrechter Befund, keine Fraktur'",

        "CREATE Statement SET \
            body_part = 'lws', \
            indication = 'Chronische Lumbalgie (3 Monate)', \
            statement = 'Harmonische Lordose, Normale Wirbelkörper, Regelrechter Mineralgehalt, Keine Höhenminderung ZWR', \
            assessment = 'Unauffällige LWS'",

        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Therapieresistente Schmerzen (3 Wochen)', \
            statement = 'Normale Form, Regelrechte Artikulation, Normale Gelenkspaltweite, Keine Verkalkungen', \
            assessment = 'Altersentsprechender Befund, keine Arthrose'",

        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Z.n. Sportverletzung', \
            statement = 'Keine Fraktur, Regelrechte Patella-Position, Normale Gelenkspaltweite, Keine Ergussbildung', \
            assessment = 'Unauffälliger Befund'",

        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'V.a. Gonarthrose', \
            statement = 'Mediale Gelenkspaltverschmälerung, Osteophytäre Anbauten, Subchondrale Sklerosierung, Regelrechte Patella-Position', \
            assessment = 'Mäßiggradige mediale Gonarthrose bds.'",

        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Akutes Trauma', \
            statement = 'Keine Fraktur, Suprapatellarer Erguss, Normale Achsstellung, Intakte Gelenkflächen', \
            assessment = 'Kein Frakturnachweis, Ergussbildung'",

        "CREATE Statement SET \
            body_part = 'hip', \
            indication = 'Z.n. Trauma', \
            statement = 'Regelrechte Artikulation, Normale Gelenkspaltweite, Unauffälliges Pfannendach, Glatte Hüftkopfkonturen', \
            assessment = 'Keine Fraktur/Luxation, altersentsprechend'",

        "CREATE Statement SET \
            body_part = 'hand', \
            indication = 'Schwellung PIP II, V.a. Psoriasisarthritis', \
            statement = 'Normaler Kalksalzgehalt, Normale Gelenkspalten, Keine ossären Läsionen, Keine Weichteilschwellung', \
            assessment = 'Keine Hinweise auf Psoriasisarthritis'",

        "CREATE Statement SET \
            body_part = 'foot', \
            indication = 'Supinationstrauma', \
            statement = 'Regelrechter Kalksalzgehalt, Glatte Gelenkflächen, Normale Gelenkspalten, Normales Fußgewölbe', \
            assessment = 'Kein Frakturnachweis'",

        "CREATE Statement SET \
            body_part = 'foot', \
            indication = 'Nicht spezifiziert', \
            statement = 'Normale Ausrichtung, Keine Läsionen, Intakte Gelenkspalten, Normale Weichteile', \
            assessment = 'Unauffälliger Befund'",

        "CREATE Statement SET \
            body_part = 'elbow', \
            indication = 'Nicht spezifiziert', \
            statement = 'Keine Frakturen, Normaler Gelenkspalt, Normale Fettpolster', \
            assessment = 'Unauffälliger Befund'",

            "CREATE Statement SET \
            body_part = 'thorax', \
            indication = 'Akute Dyspnoe, Fieber', \
            statement = 'Basale Verschattungen beidseits, Kleine Ergüsse, Herz gering verbreitert, Keine Stauungszeichen', \
            assessment = 'Bilaterale Pneumonie mit kleinen Ergüssen'",
    
        "CREATE Statement SET \
            body_part = 'thorax', \
            indication = 'COPD-Exazerbation', \
            statement = 'Überblähte Lunge, Abgeflachtes Zwerchfell, Verstärkte Bronchialzeichnung, Keine Infiltrate', \
            assessment = 'Typische COPD-Zeichen, kein Infiltrat'",
    
        "CREATE Statement SET \
            body_part = 'shoulder', \
            indication = 'Chronische Schmerzen, Impingement-Syndrom', \
            statement = 'Subakromiale Sklerose, Verschmälerter Subakromialraum, Kalkdepot in der Supraspinatussehne, AC-Gelenksarthrose', \
            assessment = 'Tendinosis calcarea, AC-Arthrose'",
    
        "CREATE Statement SET \
            body_part = 'lws', \
            indication = 'Ischialgie links', \
            statement = 'Höhenminderung L5/S1, Spondylarthrose, Facettengelenksarthrose L4-S1, Skoliotische Fehlhaltung', \
            assessment = 'Degenerative LWS-Veränderungen mit Bandscheibendegeneration L5/S1'",
    
        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Chronische Instabilität', \
            statement = 'Laterale Gelenkspaltverschmälerung, Osteophyten tibial, Patella alta, Kleine Gelenkmäuse', \
            assessment = 'Fortgeschrittene laterale Gonarthrose'",
    
        "CREATE Statement SET \
            body_part = 'hip', \
            indication = 'Chronische Coxalgie', \
            statement = 'Konzentrische Gelenkspaltverschmälerung, Subchondrale Sklerosierung, Osteophytäre Anbauten, Acetabuläre Protrusion', \
            assessment = 'Fortgeschrittene Coxarthrose'",
    
        "CREATE Statement SET \
            body_part = 'hand', \
            indication = 'Rheumatoide Arthritis', \
            statement = 'Gelenkspaltdestruktionen MCP II-IV, Ulnardeviation, Subluxationsstellung, Periartikuläre Osteopenie', \
            assessment = 'Typische Zeichen einer rheumatoiden Arthritis'",
    
        "CREATE Statement SET \
            body_part = 'foot', \
            indication = 'Chronische Fußschmerzen, Hallux valgus', \
            statement = 'Hallux valgus beidseits, Spreizfußdeformität, Hammerzehen II-IV, Arthrose MTP I', \
            assessment = 'Fortgeschrittener Hallux valgus mit Spreizfußdeformität'",
    
        "CREATE Statement SET \
            body_part = 'elbow', \
            indication = 'Epicondylitis lateralis', \
            statement = 'Kalkeinlagerungen am lateralen Epicondylus, Normale Gelenkstellung, Keine Arthrose, Intakte Fettpolster', \
            assessment = 'Radiologische Zeichen einer Epicondylitis lateralis'",
    
        "CREATE Statement SET \
            body_part = 'thorax', \
            indication = 'Routinekontrolle nach COVID', \
            statement = 'Diskrete streifige Zeichnungsvermehrung basal, Normales Herz, Keine Ergüsse, Zwerchfell regelrecht', \
            assessment = 'Minimale Restbefunde nach COVID-19'",
    
        "CREATE Statement SET \
            body_part = 'shoulder', \
            indication = 'Rotatorenmanschettenruptur', \
            statement = 'Humeruskopfhochstand, Acromionsporn, Degenerative AC-Gelenksveränderungen, Subakromiale Sklerose', \
            assessment = 'Radiologische Zeichen einer Rotatorenmanschettenruptur'",
    
        "CREATE Statement SET \
            body_part = 'lws', \
            indication = 'Posturale Instabilität', \
            statement = 'Retrolisthesis L4/L5, Facettengelenksarthrose, Normale Mineralisierung, Erhaltene Bandscheibenhöhen', \
            assessment = 'Degenerative Veränderungen mit Retrolisthesis L4/L5'",

            "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Chronische Instabilität nach Meniskektomie', \
            statement = 'Varusfehlstellung, Ausgeprägte mediale Gelenkspaltverschmälerung, Osteophyten femoral/tibial, Subchondrale Zysten', \
            assessment = 'Schwere mediale Gonarthrose bei Varusfehlstellung'",
    
        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Z.n. VKB-Plastik', \
            statement = 'Regelrechte Bohrkanalposition, Normaler Gelenkspalt, Keine Ergussbildung, Intakte Patellahöhe', \
            assessment = 'Regelrechter postoperativer Befund nach VKB-Plastik'",
    
        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Patellofemorale Schmerzen', \
            statement = 'Lateralisierte Patella, Verschmälerung lateraler Gelenkspalt, Osteophyten retropatellar, Normale Beinachse', \
            assessment = 'Patellofemorale Arthrose mit Lateralisation'",
    
        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Baker-Zyste', \
            statement = 'Weichteilschatten dorsal, Diskrete Gelenkspaltverschmälerung medial, Keine freien Gelenkkörper, Regelrechte Patella', \
            assessment = 'Baker-Zyste bei beginnender medialer Gonarthrose'",
    
        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Hämarthros nach Trauma', \
            statement = 'Suprapatellarer Erguss, Impression lateraler Femurkondylus, Regelrechte Patella, Normale Beinachse', \
            assessment = 'Hämarthros bei V.a. laterale Flake-Fraktur'",
    
        "CREATE Statement SET \
            body_part = 'angio', \
            indication = 'pAVK Stadium IIb', \
            statement = 'Hochgradige AFS-Stenose links, Mehretagenverschluss A. poplitea, Kollateralisation über Profunda-Äste, Regelrechter Run-off', \
            assessment = 'Relevante pAVK vom Oberschenkeltyp links'",
    
        "CREATE Statement SET \
            body_part = 'angio', \
            indication = 'Akute Beinischämie rechts', \
            statement = 'Kompletter Verschluss A. poplitea, Keine Kollateralisation, Fehlender Run-off, Regelrechte Beckengefäße', \
            assessment = 'Akuter thrombotischer Verschluss A. poplitea rechts'",
    
        "CREATE Statement SET \
            body_part = 'angio', \
            indication = 'Nierenarterienstenose', \
            statement = 'Hochgradige Abgangsstenose A. renalis rechts, Poststenotische Dilatation, Normale linke Nierenarterie, Unauffällige Aorta', \
            assessment = 'Hämodynamisch relevante Nierenarterienstenose rechts'",
    
        "CREATE Statement SET \
            body_part = 'angio', \
            indication = 'Mesenteriale Ischämie', \
            statement = 'Hochgradige Stenose Truncus coeliacus, Verschluss A. mesenterica superior, Kollateralisation über A. mesenterica inferior', \
            assessment = 'Chronische mesenteriale Ischämie'",
    
        "CREATE Statement SET \
            body_part = 'angio', \
            indication = 'Aortendissektion', \
            statement = 'Stanford Typ B Dissektion ab linker A. subclavia, True/False Lumen, Regelrechte Perfusion Viszeralarterien, Keine Ruptur', \
            assessment = 'Akute Aortendissektion Stanford B'",
    
        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Chondrokalzinose', \
            statement = 'Punktförmige Verkalkungen im medialen und lateralen Meniskus, Dezente Gelenkspaltverschmälerung, Intakte Kortikalis', \
            assessment = 'Typische Zeichen einer Chondrokalzinose'",
    
        "CREATE Statement SET \
            body_part = 'angio', \
            indication = 'Dialyseshuntdysfunktion', \
            statement = 'Hochgradige Stenose der venösen Anastomose, Regelrechter arterieller Zufluss, Kollateralen über V. cephalica', \
            assessment = 'Interventionspflichtige Shuntstenose'",
    
        "CREATE Statement SET \
            body_part = 'angio', \
            indication = 'Lebertransplantation', \
            statement = 'Regelrechte arterielle Anastomose, Gute Perfusion intrahepatischer Äste, Keine Stenosen/Verschlüsse, Normaler Fluss', \
            assessment = 'Unauffällige hepatische Gefäßsituation nach LTX'",
    
        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Osteochondrosis dissecans', \
            statement = 'Dissektat medialer Femurkondylus, Intakte Gelenkflächen, Keine freien Gelenkkörper, Regelrechte Beinachse', \
            assessment = 'Osteochondrosis dissecans Stadium III medialer Femurkondylus'",
    
        "CREATE Statement SET \
            body_part = 'angio', \
            indication = 'Pulmonalarterienembolie', \
            statement = 'Zentrale Embolie A. pulmonalis rechts, Subtotaler Verschluss Unterlappenarterie links, Normale Aorta', \
            assessment = 'Bilaterale Pulmonalarterienembolie'",
    
        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Rheumatoide Arthritis', \
            statement = 'Gelenkspaltdestruktion, Usuren femoral/tibial, Synovialitis, Bandinstabilität', \
            assessment = 'Fortgeschrittene rheumatoide Gonarthritis'",

            "CREATE Statement SET \
            body_part = 'thorax', \
            indication = 'Akute respiratorische Insuffizienz', \
            statement = 'Diffuse alveoläre Verschattungen beidseits, Kerley-B-Linien, Verbreitertes Herz, Pleuraergüsse', \
            assessment = 'Kardiales Lungenödem'",
    
        "CREATE Statement SET \
            body_part = 'thorax', \
            indication = 'V.a. Pneumothorax nach ZVK-Anlage', \
            statement = 'Apikal rechts Pleuragrenzlamelle sichtbar, Mediastinalshift nach links, Lunge links normal belüftet, ZVK-Spitze korrekt', \
            assessment = 'Rechtsseitiger Pneumothorax nach ZVK-Anlage'",
    
        "CREATE Statement SET \
            body_part = 'thorax', \
            indication = 'COVID-19 Verlaufskontrolle', \
            statement = 'Rückläufige bilaterale Infiltrate, Keine Ergüsse mehr, Herz normal groß, Zwerchfell frei', \
            assessment = 'Regredienter COVID-19 Befund'",
    
        "CREATE Statement SET \
            body_part = 'thorax', \
            indication = 'Tuberkulose-Screening', \
            statement = 'Apikale Verschattungen beidseits, Kavernenbildung rechts, Verplumpte Hili, Keine Ergüsse', \
            assessment = 'Radiologisch aktive Lungentuberkulose'",
    
        "CREATE Statement SET \
            body_part = 'thorax', \
            indication = 'Kontrolle nach Portimplantation', \
            statement = 'Portkatheter regelrecht über V. subclavia rechts, Katheterspitze in V. cava superior, Keine Komplikationen', \
            assessment = 'Regelrechte Portanlage'",
    
    
        "CREATE Statement SET \
            body_part = 'breast', \
            indication = 'Screening-Mammographie', \
            statement = 'ACR Typ c, Gruppierte Mikroverkalkungen im oberen äußeren Quadranten rechts, BIRADS 4', \
            assessment = 'Suspekte Mikroverkalkungen rechts, Biopsie empfohlen'",
    
        "CREATE Statement SET \
            body_part = 'breast', \
            indication = 'Tastbefund links', \
            statement = 'Sternförmige Verdichtung retromamillär links, Hauteinziehung, Axilläre Lymphknoten vergrößert, BIRADS 5', \
            assessment = 'Hochgradig malignitätsverdächtiger Befund links'",
    
        "CREATE Statement SET \
            body_part = 'breast', \
            indication = 'Verlaufskontrolle nach BET rechts', \
            statement = 'Postoperative Veränderungen lateral rechts, Keine neuen Herdbefunde, Symmetrische Axilla, BIRADS 2', \
            assessment = 'Unauffällige Verlaufskontrolle nach BET'",
    
        "CREATE Statement SET \
            body_part = 'breast', \
            indication = 'Mastodynie beidseits', \
            statement = 'ACR Typ d, Multiple Zysten beidseits, Keine suspekten Herde, BIRADS 2', \
            assessment = 'Fibrozystische Mastopathie'",
    
        "CREATE Statement SET \
            body_part = 'breast', \
            indication = 'Sekretion links', \
            statement = 'Intraduktale Verkalkungen retromamillär links, Duktektasie, Keine Raumforderung, BIRADS 3', \
            assessment = 'V.a. intraduktales Papillom links'",
    
        
        "CREATE Statement SET \
            body_part = 'abdomen', \
            indication = 'Akutes Abdomen', \
            statement = 'Multiple Dünndarmspiegelbildungen, Aufgeweitete Darmschlingen, Freie Luft subphrenisch, Keine Verkalkungen', \
            assessment = 'Ileus mit Perforationszeichen'",
    
        "CREATE Statement SET \
            body_part = 'abdomen', \
            indication = 'Chronische Obstipation', \
            statement = 'Massiv erweitertes Kolon, Koprostase, Keine freie Luft, Normales Verteilungsmuster', \
            assessment = 'Koprostase mit Kolondilatation'",
    
        "CREATE Statement SET \
            body_part = 'abdomen', \
            indication = 'Steinkolik', \
            statement = 'Multiple Verkalkungen im Nierenlager links, Ureterstein links distal, Keine Harnstauung, Normale Darmgasverteilung', \
            assessment = 'Nephrolithiasis und distaler Ureterstein links'",
    
        "CREATE Statement SET \
            body_part = 'abdomen', \
            indication = 'Cholezystitis', \
            statement = 'Multiple Konkremente im rechten Oberbauch, Aerobilie, Freie Flüssigkeit perihepatisch, Keine freie Luft', \
            assessment = 'Cholezystolithiasis mit Zeichen der Cholezystitis'",
    
        "CREATE Statement SET \
            body_part = 'thorax', \
            indication = 'Aspirationspneumonie', \
            statement = 'Infiltrat rechter Unterlappen, Kleine Ergussbildung rechts, Herz altersentsprechend, Keine Stauung', \
            assessment = 'Pneumonie rechter Unterlappen mit Begleiterguss'",
    
        "CREATE Statement SET \
            body_part = 'breast', \
            indication = 'Kontrolle nach Mammakarzinom', \
            statement = 'Z.n. Mastektomie rechts, Narbengewebe ohne Rezidiv, Linke Brust unauffällig, BIRADS 2', \
            assessment = 'Kein Anhalt für Rezidiv nach Mastektomie'",
    
        "CREATE Statement SET \
            body_part = 'abdomen', \
            indication = 'Appendizitis', \
            statement = 'Appendikolith, Verdichtung im rechten Unterbauch, Keine freie Luft, Normales Darmgas', \
            assessment = 'Radiologische Zeichen einer Appendizitis'",
    
        "CREATE Statement SET \
            body_part = 'thorax', \
            indication = 'Pleuraerguss', \
            statement = 'Massive Verschattung rechts basal, Mediastinalshift nach links, Kompressionsatelektase, Herz mittelständig', \
            assessment = 'Ausgeprägter Pleuraerguss rechts mit Mediastinalshift'",
    
        "CREATE Statement SET \
            body_part = 'breast', \
            indication = 'Präoperative Lokalisation', \
            statement = 'Drahtmarkierter Herdbefund im oberen inneren Quadranten links, Korrekte Drahtlage, BIRADS 5', \
            assessment = 'Regelrechte präoperative Drahtmarkierung'",
    
        "CREATE Statement SET \
            body_part = 'abdomen', \
            indication = 'Subileus', \
            statement = 'Einzelne Dünndarmspiegelbildungen, Kein kompletter Stillstand, Normale Kolonrahmen, Keine freie Luft', \
            assessment = 'Radiologische Zeichen eines Subileus'",
        
        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Seronegative Arthritis', \
            statement = 'Symmetrische Gelenkspaltverschmälerung, Ausgeprägte Erosionen femoral/tibial, Periartikuläre Osteopenie, Gelenkerguss', \
            assessment = 'Fortgeschrittene seronegative Arthritis'",

        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Psoriasis-Arthritis', \
            statement = 'Asymmetrische Gelenkdestruktion, Pencil-in-cup Deformität, Periostale Knochenneubildung, Weichteilschwellung', \
            assessment = 'Typische radiologische Zeichen einer Psoriasis-Arthritis'",

        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Juvenile idiopathische Arthritis', \
            statement = 'Wachstumsstörung, Epiphysäre Verbreiterung, Frühzeitige Gelenkdestruktion, Periartikuläre Osteoporose', \
            assessment = 'Aktive juvenile idiopathische Arthritis'",

        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Kristallarthropathie', \
            statement = 'Chondrocalcinose der Menisci, Punktförmige Verkalkungen im Gelenkknorpel, Degenerative Veränderungen, Osteophyten', \
            assessment = 'CPPD-Arthropathie mit sekundärer Arthrose'",

        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Gichtarthritis', \
            statement = 'Multiple Gichttophi, Punched-out Läsionen, Asymmetrische Gelenkbeteiligung, Erhaltene Gelenkspaltweite', \
            assessment = 'Chronische Gichtarthritis mit typischen Tophi'",

        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Rheumatoide Arthritis Frühstadium', \
            statement = 'Diskrete Gelenkspaltver­schmälerung, Beginnende juxtaartikuläre Osteoporose, Keine Erosionen, Synovitis', \
            assessment = 'Frühe radiologische Zeichen einer RA'",

        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Fortgeschrittene RA', \
            statement = 'Massive Gelenkdestruktion, Subluxationsstellung, Mutilierende Arthritis, Ausgeprägte periartikuläre Osteoporose', \
            assessment = 'Schwere destruierende rheumatoide Arthritis'",

        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Reaktive Arthritis', \
            statement = 'Asymmetrische Synovitis, Periostale Reaktion, Enthesiopathie der Patellasehne, Normaler Mineralgehalt', \
            assessment = 'Befund vereinbar mit reaktiver Arthritis'",

        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Hämophile Arthropathie', \
            statement = 'Verbreiterte Epiphysen, Unregelmäßige Gelenkflächen, Zystische Veränderungen, Hämosiderinablagerungen', \
            assessment = 'Typische Zeichen einer hämophilen Arthropathie'",

        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Spondyloarthritis', \
            statement = 'Enthesitische Veränderungen, Asymmetrische Gelenkbeteiligung, Periartikuläre Knochenneubildung, Erguss', \
            assessment = 'Radiologische Zeichen einer Spondyloarthritis'",

        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Overlap-Syndrom', \
            statement = 'Gemischtes Erosionsmuster, Arthritis-Arthrose, Subluxationstendenz, Periartikuläre Osteopenie', \
            assessment = 'Komplexes arthritisches Geschehen bei Overlap-Syndrom'",

        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Septische Arthritis', \
            statement = 'Rasche Gelenkdestruktion, Massive Weichteilschwellung, Gelenkerguss, Periartikuläre Osteolyse', \
            assessment = 'Radiologische Zeichen einer septischen Arthritis'",

        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Arthritis bei Sarkoidose', \
            statement = 'Symmetrische Gelenkbeteiligung, Zystenartige Läsionen, Erhaltene Gelenkspaltweite, Periartikuläre Verdichtungen', \
            assessment = 'Arthritische Veränderungen bei Sarkoidose'",

        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Neuropathische Arthropathie', \
            statement = 'Massive Destruktion, Knochenfragmentierung, Deformierung, Subluxation ohne adäquates Trauma', \
            assessment = 'Charcot-Gelenk bei neuropathischer Arthropathie'",
        
            "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Seronegative Arthritis', \
            statement = 'Symmetrische Gelenkspaltverschmälerung, Ausgeprägte Erosionen femoral/tibial, Periartikuläre Osteopenie, Gelenkerguss', \
            assessment = 'Fortgeschrittene seronegative Arthritis'",

        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Psoriasis-Arthritis', \
            statement = 'Asymmetrische Gelenkdestruktion, Pencil-in-cup Deformität, Periostale Knochenneubildung, Weichteilschwellung', \
            assessment = 'Typische radiologische Zeichen einer Psoriasis-Arthritis'",

        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Juvenile idiopathische Arthritis', \
            statement = 'Wachstumsstörung, Epiphysäre Verbreiterung, Frühzeitige Gelenkdestruktion, Periartikuläre Osteoporose', \
            assessment = 'Aktive juvenile idiopathische Arthritis'",

        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Kristallarthropathie', \
            statement = 'Chondrocalcinose der Menisci, Punktförmige Verkalkungen im Gelenkknorpel, Degenerative Veränderungen, Osteophyten', \
            assessment = 'CPPD-Arthropathie mit sekundärer Arthrose'",

        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Gichtarthritis', \
            statement = 'Multiple Gichttophi, Punched-out Läsionen, Asymmetrische Gelenkbeteiligung, Erhaltene Gelenkspaltweite', \
            assessment = 'Chronische Gichtarthritis mit typischen Tophi'",

        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Rheumatoide Arthritis Frühstadium', \
            statement = 'Diskrete Gelenkspaltver­schmälerung, Beginnende juxtaartikuläre Osteoporose, Keine Erosionen, Synovitis', \
            assessment = 'Frühe radiologische Zeichen einer RA'",

        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Fortgeschrittene RA', \
            statement = 'Massive Gelenkdestruktion, Subluxationsstellung, Mutilierende Arthritis, Ausgeprägte periartikuläre Osteoporose', \
            assessment = 'Schwere destruierende rheumatoide Arthritis'",

        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Reaktive Arthritis', \
            statement = 'Asymmetrische Synovitis, Periostale Reaktion, Enthesiopathie der Patellasehne, Normaler Mineralgehalt', \
            assessment = 'Befund vereinbar mit reaktiver Arthritis'",

        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Hämophile Arthropathie', \
            statement = 'Verbreiterte Epiphysen, Unregelmäßige Gelenkflächen, Zystische Veränderungen, Hämosiderinablagerungen', \
            assessment = 'Typische Zeichen einer hämophilen Arthropathie'",

        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Spondyloarthritis', \
            statement = 'Enthesitische Veränderungen, Asymmetrische Gelenkbeteiligung, Periartikuläre Knochenneubildung, Erguss', \
            assessment = 'Radiologische Zeichen einer Spondyloarthritis'",

        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Overlap-Syndrom', \
            statement = 'Gemischtes Erosionsmuster, Arthritis-Arthrose, Subluxationstendenz, Periartikuläre Osteopenie', \
            assessment = 'Komplexes arthritisches Geschehen bei Overlap-Syndrom'",


        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Septische Arthritis', \
            statement = 'Rasche Gelenkdestruktion, Massive Weichteilschwellung, Gelenkerguss, Periartikuläre Osteolyse', \
            assessment = 'Radiologische Zeichen einer septischen Arthritis'",

        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Arthritis bei Sarkoidose', \
            statement = 'Symmetrische Gelenkbeteiligung, Zystenartige Läsionen, Erhaltene Gelenkspaltweite, Periartikuläre Verdichtungen', \
            assessment = 'Arthritische Veränderungen bei Sarkoidose'",

        "CREATE Statement SET \
            body_part = 'knee', \
            indication = 'Neuropathische Arthropathie', \
            statement = 'Massive Destruktion, Knochenfragmentierung, Deformierung, Subluxation ohne adäquates Trauma', \
            assessment = 'Charcot-Gelenk bei neuropathischer Arthropathie'",

            "CREATE Statement SET \
            body_part = 'head', \
            indication = 'Akuter Schlaganfall', \
            statement = 'Hypodenses Areal im Media-Stromgebiet links, Keine Blutung, Diskrete Mittellinienverlagerung, Dense-Media-Sign', \
            assessment = 'Akuter ischämischer Mediainfarkt links'",
    
        "CREATE Statement SET \
            body_part = 'head', \
            indication = 'Kopfschmerz, Erbrechen', \
            statement = 'Hyperdense Blutung temporal rechts, Ödem, Mittellinienverlagerung 8mm, Kompression Seitenventrikel', \
            assessment = 'Akute intrazerebrale Blutung temporal rechts'",
    
        "CREATE Statement SET \
            body_part = 'head', \
            indication = 'Schädel-Hirn-Trauma', \
            statement = 'Epiduralhämatom frontal links, Schädelfraktur temporal, Hirnödem, Keine Mittellinienverlagerung', \
            assessment = 'Epiduralhämatom bei Schädelfraktur'",
    
        "CREATE Statement SET \
            body_part = 'head', \
            indication = 'V.a. SAB', \
            statement = 'Hyperdensität in den basalen Zisternen, Ventrikeleinbruchsblutung, Fisher Grad 4, Hydrocephalus', \
            assessment = 'Subarachnoidalblutung mit Ventrikeleinbruch'",
    
        "CREATE Statement SET \
            body_part = 'head', \
            indication = 'Chronische Kopfschmerzen', \
            statement = 'Multiple supratentorielle Mikroblutungen, Leukenzephalopathie, Normale Weite der Liquorräume', \
            assessment = 'Zeichen einer chronischen mikroangiopathischen Enzephalopathie'",
    
        "CREATE Statement SET \
            body_part = 'head', \
            indication = 'Akute Verwirrtheit', \
            statement = 'Chronisch-subdurales Hämatom beidseits, Hygrome frontal, Altersentsprechende Hirnatrophie', \
            assessment = 'Chronisch-subdurales Hämatom bilateral'",
    
        "CREATE Statement SET \
            body_part = 'head', \
            indication = 'Metastasensuche', \
            statement = 'Multiple kontrastmittelaufnehmende Raumforderungen supra- und infratentoriell, Perifokales Ödem, Mittellinienshift', \
            assessment = 'Multiple zerebrale Metastasen'",
    
        "CREATE Statement SET \
            body_part = 'head', \
            indication = 'Krampfanfall', \
            statement = 'Hypodense Raumforderung frontal rechts mit irregulärem Enhancement, Ödem, Kompression Vorderhorn', \
            assessment = 'V.a. hochgradiges Gliom frontal rechts'",
    
        "CREATE Statement SET \
            body_part = 'head', \
            indication = 'Hydrocephalus', \
            statement = 'Deutlich erweiterte innere und äußere Liquorräume, Transependymales Ödem, Aquäduktstenose', \
            assessment = 'Obstruktiver Hydrocephalus bei Aquäduktstenose'",
    
        "CREATE Statement SET \
            body_part = 'head', \
            indication = 'Sinusitis', \
            statement = 'Verschattung Sinus maxillaris und frontalis beidseits, Flüssigkeitsspiegel, Mukosaschwellung', \
            assessment = 'Pansinusitis mit Flüssigkeitsretention'",
    
        "CREATE Statement SET \
            body_part = 'head', \
            indication = 'V.a. Hirnabszess', \
            statement = 'Ringförmig KM-aufnehmende Läsion temporal links, Kapselbildung, Ödem, Masseneffekt', \
            assessment = 'Hirnabszess temporal links'",
    
        "CREATE Statement SET \
            body_part = 'head', \
            indication = 'Schwindel, Ataxie', \
            statement = 'Hypodense Läsion im Kleinhirn links, Kompression 4. Ventrikel, Beginnende Herniation', \
            assessment = 'Zerebellärer Infarkt links mit drohender Herniation'",
    
        "CREATE Statement SET \
            body_part = 'head', \
            indication = 'Demenz-Abklärung', \
            statement = 'Ausgeprägte kortikale Atrophie, Erweiterung Sulci, Hippocampusatrophie bds., Mikroangiopathie', \
            assessment = 'Hirnvolumenminderung vereinbar mit Alzheimer-Demenz'",
    
        "CREATE Statement SET \
            body_part = 'head', \
            indication = 'Meningitis', \
            statement = 'Verstärktes meningeales Enhancement, Hirnödem, Beginnender Hydrocephalus, Mastoiditis links', \
            assessment = 'CT-morphologische Zeichen einer Meningitis'",
    
        "CREATE Statement SET \
            body_part = 'head', \
            indication = 'Shuntkontrolle', \
            statement = 'Regelrechte Ventrikuloperitoneal-Shuntlage, Normalisierte Ventrikelweite, Keine Überdrainage', \
            assessment = 'Regelrechter VP-Shunt-Befund'",
    
        "CREATE Statement SET \
            body_part = 'head', \
            indication = 'Multiple Sklerose', \
            statement = 'Multiple periventrikuläre hypodense Läsionen, Dawson-Finger, Corpus callosum betroffen', \
            assessment = 'MS-typische Läsionen'",
    
        "CREATE Statement SET \
            body_part = 'head', \
            indication = 'Z.n. Schädel-OP', \
            statement = 'Postoperative Veränderungen frontal rechts, Regelrechte Knochenlappenlage, Kein Hygrom/Hämatom', \
            assessment = 'Regelrechter postoperativer Befund'",
    
        "CREATE Statement SET \
            body_part = 'head', \
            indication = 'Intrakranielle Drucksteigerung', \
            statement = 'Abgeflachte Sulci, Komprimierte Ventrikel, Aufgehobene Mark-Rinden-Differenzierung, Tentoriumherniation', \
            assessment = 'Kritisch erhöhter Hirndruck mit Einklemmungszeichen'",

        "CREATE Statement SET \
        body_part = 'angio', \
        indication = 'Carotisstenose links', \
        statement = 'Hochgradige ACI-Stenose links (NASCET 85%), Ulzerierte Plaque, Regelrechter intrakranieller Flow, Kompletter Willisischer Kreis', \
        assessment = 'Interventionspflichtige Carotisstenose links'",

    "CREATE Statement SET \
        body_part = 'angio', \
        indication = 'Basilaristhrombose', \
        statement = 'Kompletter Verschluss A. basilaris, Fehlende Kollateralisation, PICA beidseits offen, VA links dominant', \
        assessment = 'Akute Basilaristhrombose'",

    "CREATE Statement SET \
        body_part = 'angio', \
        indication = 'SAB', \
        statement = 'Aneurysma ACom 7mm, Tochteraneurysma, Vasospasmus A1-Segment links, Regelrechter Flow ACI bds.', \
        assessment = 'Rupturiertes ACom-Aneurysma mit Vasospasmus'",

    "CREATE Statement SET \
        body_part = 'angio', \
        indication = 'AV-Malformation', \
        statement = 'Parietooccipitale AVM rechts (Spetzler-Martin Grad III), Multiple Feeder aus MCA/PCA, Oberflächlicher Venendrain', \
        assessment = 'Komplexe AVM rechts parietooccipital'",

    "CREATE Statement SET \
        body_part = 'angio', \
        indication = 'Becken-Bein-DVT', \
        statement = 'Thrombose V. iliaca communis/externa rechts, Kollateralisation über V. epigastrica, V. femoralis frei', \
        assessment = 'Iliofemorale DVT rechts'",

    "CREATE Statement SET \
        body_part = 'angio', \
        indication = 'Lungenembolie', \
        statement = 'Reitende Embolie Pulmonalishauptstamm, Subtotale Perfusionsausfälle beidseits, RV-Belastung', \
        assessment = 'Zentrale Lungenembolie mit Rechtsherzbelastung'",

    "CREATE Statement SET \
        body_part = 'angio', \
        indication = 'TACE-Planung HCC', \
        statement = 'Hypervaskularisierter Tumor Segment 7, Aberrante A. hepatica aus A. mesenterica, Pfortader offen', \
        assessment = 'TACE-geeignetes HCC mit varianter Gefäßanatomie'",

    "CREATE Statement SET \
        body_part = 'angio', \
        indication = 'Nierentransplantatkontrolle', \
        statement = 'Regelrechte Perfusion Transplantat, Keine Stenosen der Anastomosen, Gute venöse Drainage, Normale Parenchymphase', \
        assessment = 'Unauffällige Transplantatperfusion'",

    "CREATE Statement SET \
        body_part = 'angio', \
        indication = 'Chronische Mesenterialischämie', \
        statement = 'Verschluss Truncus coeliacus, Hochgradige AMS-Stenose, Kollateralisation über AMI, Kaliberstarke Riolan-Anastomose', \
        assessment = 'Relevante Mehrgefäßerkrankung viszeraler Arterien'",

    "CREATE Statement SET \
        body_part = 'angio', \
        indication = 'CCF links', \
        statement = 'High-flow CCF links, Drainage in Sinus cavernosus, Kortikale venöse Stauung, Proptosis', \
        assessment = 'Carotis-Cavernosus-Fistel links'",

    "CREATE Statement SET \
        body_part = 'angio', \
        indication = 'Lebertransplantation', \
        statement = 'Regelrechte arterielle/portale Perfusion, Normale Transitzeit, Keine Anastomosenstenosen, Hepatische Venen offen', \
        assessment = 'Normalbefund nach Lebertransplantation'",

    "CREATE Statement SET \
        body_part = 'angio', \
        indication = 'Prostatakarzinom-Embolisation', \
        statement = 'Erfolgreiche Embolisation beider Aa. prostaticae, Keine Fehlembolisation, Regelrechter Flow A. iliaca interna', \
        assessment = 'Technisch erfolgreiche Prostataembolisation'",

    "CREATE Statement SET \
        body_part = 'angio', \
        indication = 'Myomembolisation', \
        statement = 'Selektive Embolisation dominanter Myomgefäße links, Stase im Zielgebiet, Uterusarterie rechts ohne Pathologie', \
        assessment = 'Erfolgreiche unilaterale Myomembolisation'",

    "CREATE Statement SET \
        body_part = 'angio', \
        indication = 'Dissektionskontrolle', \
        statement = 'Stanford B Dissektion ab LSA, True/False Lumen perfundiert, Viszeralarterien aus True Lumen, Keine Malperfusion', \
        assessment = 'Stabile Stanford B Dissektion'",

    "CREATE Statement SET \
        body_part = 'angio', \
        indication = 'TIPS-Kontrolle', \
        statement = 'TIPS-Stent durchgängig, Normaler Flow, Portosystemischer Gradient 8mmHg, Keine Kollateralen', \
        assessment = 'Regelrechte TIPS-Funktion'",

    "CREATE Statement SET \
        body_part = 'angio', \
        indication = 'Varikosis', \
        statement = 'Insuffiziente V. saphena magna rechts, Multiple Perforansvenen, Regelrechter tiefer Abfluss, Keine TVT', \
        assessment = 'Stammvarikosis VSM rechts'",

    "CREATE Statement SET \
        body_part = 'angio', \
        indication = 'Tumorblutung', \
        statement = 'Aktive Blutung aus A. gastrica sinistra bei Magenkarzinom, Erfolgreiche Coil-Embolisation, Hämostase', \
        assessment = 'Erfolgreiche Embolisation Tumorblutung'",

    "CREATE Statement SET \
        body_part = 'angio', \
        indication = 'Dialyseshunt-Dysfunktion', \
        statement = 'Höchstgradige Anastomosenstenose, Elongierte Shuntvene, Kollateralen, Thrombosierte Outflow-Stenose', \
        assessment = 'Komplexe Shuntdysfunktion mit Thrombose'" ,

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


pub async fn setup_database(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let db_connection = init_db(Some(&app.app_handle()), true).await?;
    define_db_on_startup(db_connection.clone()).await?;
    app.manage(db_connection);
    println!("Database setup completed successfully");
    Ok(())
}