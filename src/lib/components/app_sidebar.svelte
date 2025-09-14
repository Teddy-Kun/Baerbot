<script lang="ts">
	import * as Sidebar from "$lib/components/ui/sidebar/index.js";
	import Button from "./ui/button/button.svelte";
	import SunIcon from "@lucide/svelte/icons/sun";
	import MoonIcon from "@lucide/svelte/icons/moon";
	import { toggleMode } from "mode-watcher";
</script>

<Sidebar.Root>
	<Sidebar.Header />
	<Sidebar.Content>
		<Sidebar.Group />
		<Sidebar.Group />
	</Sidebar.Content>
	<Sidebar.Footer>
		<Button
			class="w-full px-2"
			onclick={toggleMode}
			variant="outline"
			size="icon"
		>
			<span
				class="h-full w-[1.5rem] absolute flex items-center justify-center left-4"
			>
				<SunIcon
					class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 !transition-all dark:-rotate-90 dark:scale-0"
				/>
				<MoonIcon
					class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 !transition-all dark:rotate-0 dark:scale-100"
				/>
			</span>
			<span class="w-full">Toggle theme</span>
		</Button>
	</Sidebar.Footer>
</Sidebar.Root>
