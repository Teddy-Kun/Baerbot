<script lang="ts">
	import Hamster from "$lib/components/hamster.svelte";
	import { commands } from "$lib/bindings";
	import { goto } from "$app/navigation";
	import { resolve } from "$app/paths";
	import { onMount } from "svelte";
	import store from "$lib/store.svelte";

	onMount(() => {
		commands.isLoggedIn().then((res) => {
			console.debug("isLoggedIn", res);
			if (res) store.register_login(res);
			else goto(resolve("/login"));
		});
	});
</script>

<main class="w-screen h-screen flex items-center justify-center">
	<Hamster />
</main>
