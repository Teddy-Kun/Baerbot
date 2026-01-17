<script lang="ts">
	import { goto } from "$app/navigation";
	import { resolve } from "$app/paths";
	import { commands } from "$lib/bindings";
	import BotActions from "$lib/components/bot_actions.svelte";
	import Logs from "$lib/components/logs.svelte";
	import Obs from "$lib/components/obs.svelte";
	import TtsSettings from "$lib/components/tts_settings.svelte";
	import store from "$lib/store.svelte";
	import { onMount } from "svelte";

	onMount(() => {
		commands.isLoggedIn().then((res) => {
			if (res) store.register_login(res);
			else goto(resolve("/login"));
		});
	});
</script>

<div class="flex-1">
	<!-- <Resizable.PaneGroup direction="horizontal">
		<Resizable.Pane> -->
	<div class="flex flex-col size-full p-4 gap-4">
		{#if store.current_tab === "actions"}
			<BotActions />
		{:else if store.current_tab === "obs"}
			<Obs />
		{:else if store.current_tab === "tts"}
			<TtsSettings />
		{:else if store.current_tab === "logs"}
			<Logs />
		{/if}
	</div>
	<!-- </Resizable.Pane>
		<Resizable.Handle />
		<Resizable.Pane>
			{#if store.username}
				<Chat
					class="size-full bg-background"
					channel_name={store.username}
					size={1}
					stroke={1}
				/>
			{:else}
				<Hamster />
			{/if}
		</Resizable.Pane>
	</Resizable.PaneGroup> -->
</div>
