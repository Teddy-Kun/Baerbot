<script lang="ts">
	import * as Sidebar from "$lib/components/ui/sidebar/index.js";
	import AppSidebar from "../lib/components/sidebar.svelte";
	import { Toaster } from "@/lib/components/ui/sonner";
	import { ModeWatcher } from "mode-watcher";

	let { children } = $props();
</script>

<ModeWatcher />
<Toaster />

<Sidebar.Provider>
	<AppSidebar />
	<main>
		<Sidebar.Trigger />
		{@render children?.()}
	</main>
</Sidebar.Provider>
