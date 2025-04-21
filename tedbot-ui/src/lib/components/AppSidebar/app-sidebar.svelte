<script lang="ts" module>
	import BookOpen from "@lucide/svelte/icons/book-open";
	import Bot from "@lucide/svelte/icons/bot";
	import Settings2 from "@lucide/svelte/icons/settings-2";
	import SquareTerminal from "@lucide/svelte/icons/square-terminal";

	// This is sample data.
	const data = [
		{
			title: "Playground",
			icon: SquareTerminal,
			isActive: true,
			switch: (): void => {toast("Playground");},
		},
		{
			title: "Models",
			url: "#",
			icon: Bot,
			switch: (): void => {toast("Models");},
		},
		{
			title: "Documentation",
			url: "#",
			icon: BookOpen,
			switch: (): void => {toast("Documentation");},
		},
		{
			title: "Settings",
			url: "#",
			icon: Settings2,
			switch: (): void => {toast("Settings");},
		},
	];
</script>

<script lang="ts">
	import SidebarContent from "@/lib/components/AppSidebar/sidebar-content.svelte";
	import SidebarFooter from "@/lib/components/AppSidebar/sidebar-footer.svelte";
	import SidebarHeader from "@/lib/components/AppSidebar/sidebar-header.svelte";
	import * as Sidebar from "@/lib/components/ui/sidebar/index.js";
	import type { ComponentProps } from "svelte";
	import { toast } from "svelte-sonner";

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
