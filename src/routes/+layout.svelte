<script lang="ts">
	import { ModeWatcher } from "mode-watcher";
	import "../app.css";
	import store from "$lib/store.svelte";
	import DebugOverlay from "$lib/components/debug_overlay.svelte";

	let { children } = $props();

	function toggel_debug_mode(): void {
		store.debug = !store.debug;
	}

	addEventListener("keypress", (event) => {
		if (event.altKey && event.shiftKey && event.key === "D")
			toggel_debug_mode();
	});
</script>

<DebugOverlay />
<ModeWatcher />
{@render children()}
