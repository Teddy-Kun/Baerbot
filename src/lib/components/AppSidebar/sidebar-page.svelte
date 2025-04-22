<script lang="ts">
	import AppSidebar from "@/lib/components/AppSidebar/app-sidebar.svelte";
	import { Separator } from "@/lib/components/ui/separator/index.js";
	import * as Sidebar from "@/lib/components/ui/sidebar/index.js";
	import Button from "../ui/button/button.svelte";
	import Sun from "@lucide/svelte/icons/sun";
	import Moon from "@lucide/svelte/icons/moon";
	import { toggleMode } from "mode-watcher";

	let { children } = $props();
</script>

<Sidebar.Provider>
	<AppSidebar />
	<Sidebar.Inset class="px-4">
		<header
			class="flex h-16 shrink-0 items-center gap-2 transition-[width,height] ease-linear group-has-[[data-collapsible=icon]]/sidebar-wrapper:h-12"
		>
			<div class="flex items-center gap-2 w-full">
				<Sidebar.Trigger class="-ml-1" />
				<Separator orientation="vertical" class="mr-2 h-4" />

				<div class="flex-1"></div>
				<Button onclick={toggleMode} variant="outline" size="icon">
					<Sun
						class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"
					/>
					<Moon
						class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
					/>
					<span class="sr-only">Toggle theme</span>
				</Button>
			</div>
		</header>

		{@render children?.()}
	</Sidebar.Inset>
</Sidebar.Provider>
