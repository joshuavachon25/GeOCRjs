import {writable} from "svelte/store";

export const appContext = writable({
	state: 0,
	error: "",
	archive: undefined
})