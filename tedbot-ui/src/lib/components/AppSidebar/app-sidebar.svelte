<script lang="ts" module>
	import BookOpen from "@lucide/svelte/icons/book-open";
	import Settings2 from "@lucide/svelte/icons/settings-2";
	import SquareTerminal from "@lucide/svelte/icons/square-terminal";

	// This is sample data.
	const data = [
		{
			title: "Playground",
			icon: SquareTerminal,
		},
		{
			title: "Models",
			icon: SquareTerminal,
		},
		{
			title: "Documentation",
			icon: BookOpen,
		},
		{
			title: "Settings",
			icon: Settings2,
		},
	];
</script>

<script lang="ts">
	import SidebarContent from "@/lib/components/AppSidebar/sidebar-content.svelte";
	import SidebarFooter from "@/lib/components/AppSidebar/sidebar-footer.svelte";
	import SidebarHeader from "@/lib/components/AppSidebar/sidebar-header.svelte";
	import * as Sidebar from "@/lib/components/ui/sidebar/index.js";
	import type { ComponentProps } from "svelte";

	let {
		ref = $bindable(null),
		collapsible = "icon",
		...restProps
	}: ComponentProps<typeof Sidebar.Root> = $props();
</script>

<Sidebar.Root bind:ref {collapsible} {...restProps}>
	<Sidebar.Header>
		<SidebarHeader />
	</Sidebar.Header>
	<Sidebar.Content>
		<SidebarContent items={data} />
	</Sidebar.Content>
	<Sidebar.Footer>
		<SidebarFooter />
	</Sidebar.Footer>
	<Sidebar.Rail />
</Sidebar.Root>
