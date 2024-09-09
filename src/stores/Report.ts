import { writable } from "svelte/store";

type Report = {
  id: {
    [x: string]: any; String: string 
};
  reportText: string;
  bodyPart: string;
  condition: string;
  patient: {
    id: { String: string };
    name: string;
  };
  userOwner: {
    id: { String: string };
    name: string;
  };
  createdAt: string;
  updatedAt: string;
};

export const ReportStore = writable<Report[]>([]);
