<script lang="ts">
	import { goto } from "$app/navigation";
	import { resolve } from "$app/paths";
	import { commands, type Action } from "$lib/bindings";
	import Chat from "$lib/components/chat.svelte";
	import Hamster from "$lib/components/hamster.svelte";
	import Button from "$lib/components/ui/button/button.svelte";
	import Input from "$lib/components/ui/input/input.svelte";
	import * as Resizable from "$lib/components/ui/resizable/index.js";
	import store from "$lib/store.svelte";
	import { onMount } from "svelte";

	let command: string = $state("");
	let response: string = $state("");

	function add_action(): void {
		let action: Action = {
			trigger: { Command: command },
			exec: { ChatMsg: response },
		};
		commands.addAction(action);
	}

	onMount(() => {
		commands.isLoggedIn().then((res) => {
			if (res) store.register_login(res);
			else goto(resolve("/login"));
		});

		commands.getAllActions().then((res) => {
			console.log(res);
		});
	});
</script>

<div class="flex-1">
	<Resizable.PaneGroup direction="horizontal">
		<Resizable.Pane>
			<div class="flex flex-col p-4 gap-4">
				<Input bind:value={command} />
				<Input bind:value={response} />
				<Button onclick={add_action}>Add Action</Button>
			</div>
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
