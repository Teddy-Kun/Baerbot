<script lang="ts">
	import * as Sidebar from "$lib/components/ui/sidebar/index.js";
	import Button from "./ui/button/button.svelte";
	import SunIcon from "@lucide/svelte/icons/sun";
	import MoonIcon from "@lucide/svelte/icons/moon";
	import * as Tabs from "./ui/tabs";
	import store from "$lib/store.svelte";
	import { commands } from "$lib/bindings";
	import { toggle_theme } from "$lib/utils";
	import { goto } from "$app/navigation";
	import { resolve } from "$app/paths";

	function logout(): void {
		commands.logout().then(() => {
			store.username = null;
			goto(resolve("/login"));
		});
	}
</script>

<Sidebar.Root>
	<Sidebar.Header />
	<Sidebar.Content>
		<Tabs.Root
			bind:value={store.current_tab}
			class="w-full px-2"
			orientation="vertical"
		>
			<Tabs.List class="flex flex-col h-fit w-full">
				<Tabs.Trigger class="w-full" value="actions">
					Actions
				</Tabs.Trigger>
				<Tabs.Trigger class="w-full" value="obs">OBS</Tabs.Trigger>
				<Tabs.Trigger class="w-full" value="logs">Logs</Tabs.Trigger>
			</Tabs.List>
		</Tabs.Root>
	</Sidebar.Content>
	<Sidebar.Footer>
		<Button onclick={logout}>Logout</Button>
		<Button
			class="w-full px-2"
			onclick={toggle_theme}
			variant="outline"
			size="icon"
		>
			<span
				class="h-full w-6 absolute flex items-center justify-center left-4"
			>
				<SunIcon
					class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all! dark:-rotate-90 dark:scale-0"
				/>
				<MoonIcon
					class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all! dark:rotate-0 dark:scale-100"
				/>
			</span>
			<span class="w-full">Toggle theme</span>
		</Button>
	</Sidebar.Footer>
</Sidebar.Root>
