import { writable } from "svelte/store";

type Report = {
  id: { String: string };
  reportText: string;
  bodyType: string;
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
