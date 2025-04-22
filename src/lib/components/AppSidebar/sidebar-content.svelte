<script lang="ts">
	import * as Sidebar from "@/lib/components/ui/sidebar/index.js";
	import store from "@/store.svelte";
	import type { Component } from "svelte";

	let {
		items,
	}: {
		items: {
			title: string;
			// this should be `Component` after @lucide/svelte updates types
			icon: Component;
		}[];
	} = $props();
</script>

<Sidebar.Group>
	<Sidebar.Menu>
		{#each items as item (item.title)}
			<Sidebar.MenuItem>
				<Sidebar.MenuButton isActive={store.currentTab === item.title} onclick={(): void => {store.currentTab = item.title;}}>
					<item.icon />
					{item.title}
				</Sidebar.MenuButton>
			</Sidebar.MenuItem>
		{/each}
	</Sidebar.Menu>
</Sidebar.Group>
