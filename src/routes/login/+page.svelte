<script lang="ts">
	import Twitch from "$lib/components/twitch.svelte";
	import Button from "$lib/components/ui/button/button.svelte";
	import { commands } from "$lib/bindings";
	import ThemeToggle from "$lib/components/theme_toggle.svelte";
	import Hamster from "$lib/components/hamster.svelte";
	import { goto } from "$app/navigation";
	import { resolve } from "$app/paths";
	import store from "$lib/store.svelte";

	let loading = $state(false);
	let error = $state(false);

	function login(): void {
		loading = true;
		error = false;
		commands.login().then((res) => {
			loading = false;
			if (res.status === "ok") {
				console.log("logged in", res);
				store.username = "DEBUG"; // TODO
				goto(resolve("/bot"));
			} else {
				console.error("fuck", res.error);
				error = true;
			}
		});
	}
</script>

<main class="flex flex-col items-center justify-center w-[100vw] h-[100vh]">
	<ThemeToggle class="absolute top-4 right-4" />

	{#if !loading}
		<Button class="bg-[#a970ff] h-16 w-50 text-foreground" onclick={login}>
			<Twitch /> Login with Twitch
		</Button>
		{#if error}
			<p class="text-red-500 mt-3">
				<strong>Error during login, please try again</strong>
			</p>
		{/if}
	{:else}
		<Hamster />
	{/if}
</main>
