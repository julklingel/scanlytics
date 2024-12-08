
export interface Model {
    id: number;
    label: string;
    type: string;
    variant: "default" | "secondary";
    selected: boolean;
  }
  
  export interface ModelImageResult {
    filename: string;
    image_type: string;
    confidence: number;
  }
  
  export interface ModelStatement {
    indication: string;
    statement: string;
    assessment: string;
  }
  
  export interface ModelResponse {
    results: ModelImageResult[];
    statements: ModelStatement[];
  }
  
  export interface Suggestion {
    id: number;
    text: string;
  }
  
  export interface FileData {
    filename: string;
    extension: string;
    data: number[];
  }
  
  export interface ReportData {
    patient_id: string;
    user_owner: string;
    body_part: string;
    report_text: string;
    files: FileData[];
  }
  
  export interface ReportComponentProps {
    active_user: string;
    patient_id: string;
  }
  
  export interface User {
    user_email: string;
  }
  
  export interface Patient {
    id: string;
  }
  
  export interface Doctor {
    id: string;
  }
  
  export interface CarouselApi {
    scrollTo: (index: number) => void;
  }
  