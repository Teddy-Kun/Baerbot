<script lang="ts">
	import Button from "./ui/button/button.svelte";
	import Input from "./ui/input/input.svelte";
	import * as Table from "$lib/components/ui/table/index";
	import TrashIcon from "@lucide/svelte/icons/trash";
	import {
		commands,
		type Action,
		type Exec,
		type Trigger,
	} from "$lib/bindings";
	import { onMount } from "svelte";
	import Hamster from "./hamster.svelte";
	import * as Sidebar from "./ui/sidebar/index";

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
		commands
			.removeAction(get_trigger_inner(action.trigger))
			.then(update_actions);
	}

	function update_actions(): void {
		commands.getAllActions().then((res) => {
			console.debug("actions", res);
			actions = res;
			loading = false;
		});
	}

	onMount(update_actions);
</script>

<Sidebar.Trigger />
{#if loading}
	<Hamster />
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
						onclick={(): void => remove_action(action)}
					>
						<TrashIcon />
					</Button>
				</Table.Cell>
			</Table.Row>
		{/each}
	</Table.Root>
{/if}
