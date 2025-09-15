<script lang="ts">
	import { ModeWatcher } from "mode-watcher";
	import "../app.css";
	import store from "$lib/store.svelte";
	import DebugOverlay from "$lib/components/debug_overlay.svelte";
	import { Toaster } from "$lib/components/ui/sonner/index.js";
	import { get_accent_color } from "$lib/utils";

	let { children } = $props();

	function toggel_debug_mode(): void {
		store.debug = !store.debug;
	}

	addEventListener("keypress", (event) => {
		if (event.altKey && event.shiftKey && event.key === "D")
			toggel_debug_mode();
	});

	get_accent_color();
</script>

<DebugOverlay />
<ModeWatcher />
<Toaster />
{@render children()}
