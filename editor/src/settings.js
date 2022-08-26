import { writable } from "svelte/store";

export const api_server = "https://api.intermodal.pt"

export const token = writable(null);
