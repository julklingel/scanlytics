use super::models::{
    PatientNoteRecord, PatientNoteRequest, PatientNoteResponse, PatientNoteWithPatientResponse,
    PatientResponse, UserResponse,
};

use scanlytics_db::Error as SurrealError;
use scanlytics_db::{Any, Surreal, Thing};

/// Creates a new patient note with associated relationships.
///
/// # Arguments
///
/// * `db` - Database connection
/// * `data` - Note creation request data
///
/// # Returns
///
/// Returns a `Result` containing either:
/// * `Ok(PatientNoteResponse)` - Successfully created note
/// * `Err(String)` - Error message if creation fails
///
/// # Errors
///
/// This function will return an error if:
/// * Referenced patient doesn't exist
/// * Referenced user doesn't exist
/// * Note creation fails
/// * Relationship updates fail

pub async fn create_patient_note_service(
    db: &Surreal<Any>,
    data: PatientNoteRequest,
) -> Result<PatientNoteResponse, String> {
    let patient: Option<PatientResponse> = db
        .select(("Patient", &data.patient_id))
        .await
        .map_err(|e| e.to_string())?;
    let patient = patient.ok_or_else(|| "Patient not found".to_string())?;

    let user_owner: Option<UserResponse> = db
        .select(("User", &data.user_owner))
        .await
        .map_err(|e| e.to_string())?;
    let user_owner = user_owner.ok_or_else(|| "User not found".to_string())?;

    let patient_note_record = PatientNoteRecord {
        patient: patient.id.clone(),
        symptoms: data.symptoms,
        diagnosis: data.diagnosis,
        treatment: data.treatment,
        severity: data.severity,
        is_urgent: data.is_urgent,
        user_owner: user_owner.id.clone(),

    };

    let note: PatientNoteResponse = db
        .create("PatientNote")
        .content(patient_note_record)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Failed to create patient note".to_string())?;

    let patient_id = patient.id.clone();
    let note_id = note.id.clone();
    let user_owner_id = data.user_owner.clone();

    db.query("UPDATE type::thing($table, $id) SET notes += $note")
        .bind(("table", "Patient"))
        .bind(("id", patient_id))
        .bind(("note", note_id.clone()))
        .await
        .map_err(|e| e.to_string())?;

    db.query("UPDATE type::thing($table, $id) SET notes += $note")
        .bind(("table", "User"))
        .bind(("id", user_owner_id))
        .bind(("note", note_id))
        .await
        .map_err(|e| e.to_string())?;

    Ok(note)
}

/// Retrieves all patient notes with associated patient information.
///
/// # Arguments
///
/// * `db` - Database connection
///
/// # Returns
///
/// Returns a `Result` containing either:
/// * `Ok(Vec<PatientNoteWithPatientResponse>)` - List of notes with patient details
/// * `Err(SurrealError)` - Database error if query fails

pub async fn get_patient_notes_service(
    db: &Surreal<Any>,
) -> Result<Vec<PatientNoteWithPatientResponse>, SurrealError> {
    let query = "
        SELECT
            id,
            symptoms,
            diagnosis,
            treatment,    
            severity,
            is_urgent,
            { id: patient.id, name: patient.name } as patient,
            { id: user_owner.id, name: user_owner.name } as user_owner,
            created_at,
            updated_at
        FROM PatientNote
        FETCH patient, user_owner;
    ";
    let result: Vec<PatientNoteWithPatientResponse> = db.query(query).await?.take(0)?;
    Ok(result)
}

/// Updates an existing patient note.
///
/// # Arguments
///
/// * `db` - Database connection
/// * `id` - Unique identifier of the note
/// * `data` - Updated note data
///
/// # Returns
///
/// Returns a `Result` containing either:
/// * `Ok(Option<PatientNoteResponse>)` - Updated note if found
/// * `Err(String)` - Error message if update fails
///
/// # Errors
///
/// This function will return an error if:
/// * Referenced patient doesn't exist
/// * Referenced user doesn't exist
/// * Note update fails


pub async fn update_patient_note_service(
    db: &Surreal<Any>,
    id: String,
    data: PatientNoteRequest,
) -> Result<Option<PatientNoteResponse>, String> {
    let patient: Option<Thing> = db
        .select(("Patient", &data.patient_id))
        .await
        .map_err(|e| e.to_string())?;
    let patient = patient.ok_or_else(|| "Patient not found".to_string())?;

    let user_owner: Option<Thing> = db
        .select(("User", &data.user_owner))
        .await
        .map_err(|e| e.to_string())?;
    let user_owner = user_owner.ok_or_else(|| "Patient not found".to_string())?;

    let updated_note = PatientNoteRecord {
        patient: patient,
        symptoms: data.symptoms,
        diagnosis: data.diagnosis,
        treatment: data.treatment,
        severity: data.severity,
        is_urgent: data.is_urgent,
        user_owner: user_owner,
      

    };

    let updated: Option<PatientNoteResponse> = db
        .update(("PatientNote", id))
        .merge(updated_note)
        .await
        .map_err(|e| e.to_string())?;

    Ok(updated)
}

/// Deletes a patient note from the system.
///
/// # Arguments
///
/// * `db` - Database connection
/// * `id` - Unique identifier of the note to delete
///
/// # Returns
///
/// Returns a `Result` containing either:
/// * `Ok(Option<PatientNoteResponse>)` - Deleted note if found
/// * `Err(String)` - Error message if deletion fails

pub async fn delete_patient_note_service(
    db: &Surreal<Any>,
    id: String,
) -> Result<Option<PatientNoteResponse>, String> {
    let deleted: Option<PatientNoteResponse> = db
        .delete(("PatientNote", id))
        .await
        .map_err(|e| e.to_string())?;
    Ok(deleted)
}


#[cfg(test)]
mod tests {
    use super::*;
    use scanlytics_db::{Datetime, Thing};
    use serde_json::json;

    async fn setup_test_db() -> Surreal<Any> {
        let db_conn = scanlytics_db::init_db(None, true).await.unwrap();
        let db = db_conn.get().lock().await;
        db.clone()
    }

    async fn create_test_user(db: &Surreal<Any>) -> (Thing, String) {
        let user_data = json!({
            "name": "Test Doctor",
            "email": "doctor@test.com",
            "role": "user",
            "organization": null,
            "patients": [],
            "notes": [],
            "statements": [],
            "images": [],
            "reports": [],
            "created_at": Datetime::default(),
            "updated_at": Datetime::default()
        });

        let created: UserResponse = db
            .create("User")
            .content(user_data.clone())
            .await
            .unwrap()
            .unwrap();


        let id_str = created.id.to_string();
        let id_only = id_str.split(':').nth(1).unwrap_or("").to_string();

        (created.id.clone(), id_only)
    }

    async fn create_test_patient(db: &Surreal<Any>) -> (Thing, String) {
        let patient_data = json!({
            "name": "Test Patient",
            "date_of_birth": Datetime::default(),
            "gender": "M",
            "contact_number": "1234567890",
            "address": "Test Address",
            "notes": [],
            "reports": [],
            "images": [],
            "created_at": Datetime::default(),
            "updated_at": Datetime::default()
        });

        let created: PatientResponse = db
            .create("Patient")
            .content(patient_data.clone())
            .await
            .unwrap()
            .unwrap();


        let id_str = created.id.to_string();
        let id_only = id_str.split(':').nth(1).unwrap_or("").to_string();

        (created.id.clone(), id_only)
    }
   

    #[tokio::test]
    async fn test_delete_patient_note() {
        let db = setup_test_db().await;
        let (_, user_id) = create_test_user(&db).await;
        let (_, patient_id) = create_test_patient(&db).await;

    
        let note_request = PatientNoteRequest {
            patient_id,
            symptoms: "Test symptoms".to_string(),
            diagnosis: "Test diagnosis".to_string(),
            treatment: "Test treatment".to_string(),
            severity: "Mild".to_string(),
            is_urgent: false,
            user_owner: user_id,
        };

        let created_note = create_patient_note_service(&db, note_request).await.unwrap();
        let note_id = created_note.id.to_string().split(':').nth(1).unwrap_or("").to_string();

     
        let result = delete_patient_note_service(&db, note_id).await;
        assert!(result.is_ok());

  
        let notes = get_patient_notes_service(&db).await.unwrap();
        assert!(notes.iter().all(|note| note.id != created_note.id));
    }

    #[tokio::test]
    async fn test_create_patient_note_invalid_patient() {
        let db = setup_test_db().await;
        let (_, user_id) = create_test_user(&db).await;

        let note_request = PatientNoteRequest {
            patient_id: "nonexistent_patient".to_string(),
            symptoms: "Test symptoms".to_string(),
            diagnosis: "Test diagnosis".to_string(),
            treatment: "Test treatment".to_string(),
            severity: "Mild".to_string(),
            is_urgent: false,
            user_owner: user_id,
        };

        let result = create_patient_note_service(&db, note_request).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Patient not found");
    }

    #[tokio::test]
    async fn test_create_patient_note_invalid_user() {
        let db = setup_test_db().await;
        let (_, patient_id) = create_test_patient(&db).await;

        let note_request = PatientNoteRequest {
            patient_id,
            symptoms: "Test symptoms".to_string(),
            diagnosis: "Test diagnosis".to_string(),
            treatment: "Test treatment".to_string(),
            severity: "Mild".to_string(),
            is_urgent: false,
            user_owner: "nonexistent_user".to_string(),
        };

        let result = create_patient_note_service(&db, note_request).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "User not found");
    }

    #[tokio::test]
    async fn test_delete_nonexistent_note() {
        let db = setup_test_db().await;
        
        let result = delete_patient_note_service(&db, "nonexistent_note".to_string()).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_get_patient_notes_empty() {
        let db = setup_test_db().await;
        
        let result = get_patient_notes_service(&db).await;
        assert!(result.is_ok());
        
        let notes = result.unwrap();
        assert!(notes.is_empty());
    }

    #[tokio::test]
    async fn test_multiple_notes_for_patient() {
        let db = setup_test_db().await;
        let (_, user_id) = create_test_user(&db).await;
        let (_, patient_id) = create_test_patient(&db).await;

     
        let note_request1 = PatientNoteRequest {
            patient_id: patient_id.clone(),
            symptoms: "First symptoms".to_string(),
            diagnosis: "First diagnosis".to_string(),
            treatment: "First treatment".to_string(),
            severity: "Mild".to_string(),
            is_urgent: false,
            user_owner: user_id.clone(),
        };

        let note_request2 = PatientNoteRequest {
            patient_id,
            symptoms: "Second symptoms".to_string(),
            diagnosis: "Second diagnosis".to_string(),
            treatment: "Second treatment".to_string(),
            severity: "Severe".to_string(),
            is_urgent: true,
            user_owner: user_id,
        };

        create_patient_note_service(&db, note_request1).await.unwrap();
        create_patient_note_service(&db, note_request2).await.unwrap();

        let result = get_patient_notes_service(&db).await;
        assert!(result.is_ok());

        let notes = result.unwrap();
        assert_eq!(notes.len(), 2);
        
  
        let has_mild_note = notes.iter().any(|note| note.severity == "Mild");
        let has_severe_note = notes.iter().any(|note| note.severity == "Severe");
        assert!(has_mild_note);
        assert!(has_severe_note);
    }

}
