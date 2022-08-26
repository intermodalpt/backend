import {writable} from "svelte/store";

export const api_server = "https://api.intermodal.pt"

export const mode = writable(localStorage.getItem("mode"));