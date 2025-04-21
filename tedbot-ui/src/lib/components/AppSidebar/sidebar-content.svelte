<script lang="ts">
	import * as Sidebar from "@/lib/components/ui/sidebar/index.js";
	import type { Component } from "svelte";

	let {
		items,
	}: {
		items: {
			title: string;
			// this should be `Component` after @lucide/svelte updates types
			icon: Component;
			isActive?: boolean;
			switch: () => void;
		}[];
	} = $props();
</script>

<Sidebar.Group>
	<Sidebar.Menu>
		{#each items as item (item.title)}
			<Sidebar.MenuItem>
				<Sidebar.MenuButton isActive={item.isActive} onclick={item.switch}>
					<item.icon />
					{item.title}
				</Sidebar.MenuButton>
			</Sidebar.MenuItem>
		{/each}
	</Sidebar.Menu>
</Sidebar.Group>
