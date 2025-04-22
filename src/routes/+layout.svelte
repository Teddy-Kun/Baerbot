<script lang="ts">
	import SidebarPage from "@/lib/components/AppSidebar/sidebar-page.svelte";
	import { Toaster } from "@/lib/components/ui/sonner";
	import { ModeWatcher } from "mode-watcher";

	let { children } = $props();
</script>

<ModeWatcher />
<Toaster />
<SidebarPage>
	{@render children?.()}
</SidebarPage>