<script lang="ts">
	import * as Sidebar from "@/lib/components/ui/sidebar/index.js";
	import { Bot } from "lucide-svelte";

// let { teams }: { teams: { name: string; logo: any; plan: string }[] } = $props();
</script>

<Sidebar.Menu>
	<Sidebar.MenuItem class="flex items-center gap-2">
		<div
			class="bg-sidebar-primary text-sidebar-primary-foreground flex aspect-square size-8 items-center justify-center rounded-lg"
		>
			<Bot class="size-4" />
		</div>
		<div class="text-left text-sm leading-tight truncate font-semibold">
			Tedbot
		</div>
	</Sidebar.MenuItem>
</Sidebar.Menu>
