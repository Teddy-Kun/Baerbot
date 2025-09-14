<script lang="ts">
	import { ModeWatcher } from "mode-watcher";
	import "../app.css";
	import store from "$lib/store.svelte";
	import { goto } from "$app/navigation";
	import { resolve } from "$app/paths";
	
	let { children } = $props();

	function check_logged_in_status(): void {
		if (!store.logged_in)
			goto(resolve("/login"));
		else 
			goto(resolve("/bot"));
	}

	$inspect(store.logged_in).with((event) => {
		// TODO: replace with navigation on login
		// TODO: check logged in status in backend
		console.debug(`${event} logged_in:`, store.logged_in);
		check_logged_in_status();
	});

	// simulate initial load
	setTimeout(check_logged_in_status, 2000);
</script>

<ModeWatcher />
{@render children()}
