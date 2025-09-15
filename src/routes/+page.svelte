<script lang="ts">
	import Hamster from "$lib/components/hamster.svelte";
	import { commands } from "$lib/bindings";
	import { goto } from "$app/navigation";
	import { resolve } from "$app/paths";
	import { onMount } from "svelte";

	onMount(() => {
		commands.isLoggedIn().then((res) => {
			if (res) goto(resolve("/bot"));
			else goto(resolve("/login"));
		});
	});
</script>

<main class="w-[100vw] h-[100vh] flex items-center justify-center">
	<Hamster />
</main>
