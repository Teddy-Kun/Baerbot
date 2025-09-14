<script lang="ts">
	import AppSidebar from "$lib/components/app_sidebar.svelte";
	import * as Sidebar from "$lib/components/ui/sidebar/index.js";

	let { children } = $props();
</script>

<Sidebar.Provider class="size-full">
	<AppSidebar />
	<main class="w-full h-[100vh] flex flex-col">
		<Sidebar.Trigger />
		{@render children?.()}
	</main>
</Sidebar.Provider>
