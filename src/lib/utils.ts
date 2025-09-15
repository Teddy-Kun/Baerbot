import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";
import type { ErrorMsg } from "./bindings";
import { toast } from "svelte-sonner";

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type WithoutChild<T> = T extends { child?: any } ? Omit<T, "child"> : T;
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type WithoutChildren<T> = T extends { children?: any } ? Omit<T, "children"> : T;
export type WithoutChildrenOrChild<T> = WithoutChildren<WithoutChild<T>>;
export type WithElementRef<T, U extends HTMLElement = HTMLElement> = T & { ref?: U | null };

export function set_accent(color: string): void {
	const root = document.getElementsByTagName("html")[0];
	root.style.setProperty("--primary", color)
	root.style.setProperty("--ring", color)
	root.style.setProperty("--sidebar-primary", color)
	root.style.setProperty("--sidebar-ring", color)
	console.log(color, root)
}

export function toast_error(err: ErrorMsg): void {
	let level: 'warning' | 'error' = 'error';
	let msg: string
	switch (err) {
		case "TokenLoad":
			level = "warning";
			msg = "Error loading the token from storage. Please log in again"
			break;
		case "TokenSave":
			level = 'warning'
			msg = "Error saving the login. You will have to log in again next time"
			break;
		case "TwitchAuth":
			msg = "Error logging into Twitch"
			break;
		default:
			msg = "An unknown error occured. Go slap Teddy"
			break;
	}
	toast[level](msg, { duration: 3000 });
}
