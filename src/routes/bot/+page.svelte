<script lang="ts">
	import { goto } from "$app/navigation";
	import { resolve } from "$app/paths";
	import { commands } from "$lib/bindings";
	import Chat from "$lib/components/chat.svelte";
	import Hamster from "$lib/components/hamster.svelte";
	import Button from "$lib/components/ui/button/button.svelte";
	import * as Resizable from "$lib/components/ui/resizable/index.js";
	import store from "$lib/store.svelte";
	import { set_accent } from "$lib/utils";
	import { onMount } from "svelte";

	onMount(() => {
		commands.isLoggedIn().then((res) => {
			if (res) store.register_login(res);
			else goto(resolve("/login"));
		});
	});
</script>

<div class="flex-1">
	<Resizable.PaneGroup direction="horizontal">
		<Resizable.Pane>
			<Button onclick={(): void => set_accent("#ff0000")}>TEST</Button>
		</Resizable.Pane>
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
	</Resizable.PaneGroup>
</div>
