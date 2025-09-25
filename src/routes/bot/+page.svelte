<script lang="ts">
	import { goto } from "$app/navigation";
	import { resolve } from "$app/paths";
	import {
		commands,
		type Action,
		type Exec,
		type Trigger,
	} from "$lib/bindings";
	import Chat from "$lib/components/chat.svelte";
	import Hamster from "$lib/components/hamster.svelte";
	import Button from "$lib/components/ui/button/button.svelte";
	import Input from "$lib/components/ui/input/input.svelte";
	import * as Resizable from "$lib/components/ui/resizable/index.js";
	import store from "$lib/store.svelte";
	import * as Table from "$lib/components/ui/table/index";
	import { onMount } from "svelte";
	import TrashIcon from "@lucide/svelte/icons/trash";

	let loading: boolean = $state(true);

	let actions: Action[] = $state([]);

	let command: string = $state("");
	let response: string = $state("");

	function add_action(): void {
		let action: Action = {
			trigger: { Command: command },
			exec: { ChatMsg: response },
		};
		commands.addAction(action).then(update_actions);
	}

	function get_trigger_type(trigger: Trigger): string {
		return Object.keys(trigger)[0];
	}

	function get_trigger_inner(trigger: Trigger): string {
		let key = Object.keys(trigger)[0] as keyof Trigger;

		if (key == "Command") return `!${trigger[key]}`;

		return trigger[key];
	}

	function get_exec_type(trigger: Exec): string {
		return Object.keys(trigger)[0];
	}

	function get_exec_inner(trigger: Exec): string {
		let key = Object.keys(trigger)[0] as keyof Exec;
		return trigger[key];
	}

	function remove_action(action: Action): void {
		commands.removeAction(get_trigger_inner(action)).then(update_actions);
	}

	function update_actions(): void {
		commands.getAllActions().then((res) => {
			actions = res;
			loading = false;
		});
	}

	onMount(() => {
		commands.isLoggedIn().then((res) => {
			if (res) store.register_login(res);
			else goto(resolve("/login"));
		});

		update_actions();
	});
</script>

<div class="flex-1">
	<Resizable.PaneGroup direction="horizontal">
		<Resizable.Pane>
			<div class="flex flex-col size-full p-4 gap-4">
				{#if loading}
					<div class="flex flex-1 items-center justify-center">
						<Hamster />
					</div>
				{:else}
					<Input bind:value={command} placeholder="Command" />
					<Input bind:value={response} placeholder="Response" />
					<Button onclick={add_action}>Add Action</Button>

					Current Commands
					<Table.Root>
						<Table.Header>
							<Table.Row>
								<Table.Head>Trigger</Table.Head>
								<Table.Head>Action</Table.Head>
								<Table.Head />
							</Table.Row>
						</Table.Header>
						{#each actions as action (action.trigger)}
							<Table.Row>
								<Table.Cell>
									<p>{get_trigger_type(action.trigger)}</p>
									<p>{get_trigger_inner(action.trigger)}</p>
								</Table.Cell>
								<Table.Cell>
									<p>{get_exec_type(action.exec)}</p>
									<p>{get_exec_inner(action.exec)}</p>
								</Table.Cell>
								<Table.Cell>
									<Button
										variant="destructive"
										onclick={(): void =>
											remove_action(action)}
									>
										<TrashIcon />
									</Button>
								</Table.Cell>
							</Table.Row>
						{/each}
					</Table.Root>
				{/if}
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
