<script lang="ts">
	import Button from "@/lib/components/ui/button/button.svelte";
	import { commands } from "@/bindings";
	import { toast } from "svelte-sonner";
	import { Input } from "@/lib/components/ui/input";

	let name = $state("");

	async function greet(event: Event): Promise<void> {
		event.preventDefault();
		// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
		toast(await commands.greet(name));
	}
</script>

<h1>Welcome to Tauri + Svelte</h1>
<form onsubmit={greet}>
	<Input
		id="greet-input"
		placeholder="Enter a name..."
		bind:value={name}
	/>
	<Button type="submit">Greet</Button>
</form>
