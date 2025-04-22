<script lang="ts">
	import Button from "@/lib/components/ui/button/button.svelte";
	import { commands } from "@/bindings";
	import { toast } from "svelte-sonner";
	import { Input } from "@/lib/components/ui/input";
	import store from "@/store.svelte";

	let name = $state("");

	async function greet(event: Event): Promise<void> {
		event.preventDefault();
		// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
		toast(await commands.greet(name));
	}
</script>

<div class="flex flex-col gap-4">
	<h1>Current Tab: {store.currentTab}</h1>
	<Input
		id="greet-input"
		placeholder="Enter a name..."
		bind:value={name}
	/>
	<Button type="submit" onclick={greet}>Greet</Button>
</div>