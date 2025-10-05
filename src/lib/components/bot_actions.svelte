<script lang="ts">
	import Button from "./ui/button/button.svelte";
	import Input from "./ui/input/input.svelte";
	import * as Table from "$lib/components/ui/table/index";
	import TrashIcon from "@lucide/svelte/icons/trash";
	import {
		commands,
		type Action,
		type Exec,
		type FrontendRedeem,
		type Trigger,
	} from "$lib/bindings";
	import { onMount } from "svelte";
	import Hamster from "./hamster.svelte";
	import * as Sidebar from "./ui/sidebar/index";
	import * as Select from "./ui/select/index";
	import { toast } from "svelte-sonner";

	let loading: boolean = $state(true);

	let actions: Action[] = $state([]);

	let trigger: "Command" | "Redeem" = $state("Command");
	let identifier: string = $state("");
	let response: string = $state("");

	let num_redeems: number = $state(0);
	let redeems: { [key: string]: FrontendRedeem } = {};

	function add_action(): void {
		// @ts-expect-error the following two lines in combination are safe, typescript just can't check it
		let trig: Trigger = {};
		// @ts-expect-error the following two lines in combination are safe, typescript just can't check it
		trig[trigger] = identifier.toLowerCase();

		let action: Action = {
			trigger: trig,
			exec: { ChatMsg: response },
		};
		commands.addAction(action).then(update_actions);
	}

	function get_trigger_type(trigger: Trigger): string {
		return Object.keys(trigger)[0];
	}

	function get_trigger_inner(
		trigger: Trigger,
		add_cmd_exclamation = true,
	): string {
		let key = Object.keys(trigger)[0] as keyof Trigger;

		if (add_cmd_exclamation && key == "Command") return `!${trigger[key]}`;

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
			.removeAction(get_trigger_inner(action.trigger, false))
			.then(update_actions);
	}

	function update_actions(): void {
		commands.getAllActions().then((res) => {
			console.debug("actions", res);
			actions = res;
			loading = false;
		});
	}

	function get_redeems(): void {
		commands.getRedeems().then((res) => {
			console.debug("redeems", res);
			if (res.status === "ok") {
				num_redeems = res.data.length;
				for (const redeem of res.data) redeems[redeem.id] = redeem;
			} else toast.error("Couldn't fetch redeems");
		});
	}

	function trigger_type_selected(value: string): void {
		identifier = "";
		if (value === "Redeem") get_redeems();
	}

	onMount(update_actions);
</script>

<Sidebar.Trigger />
{#if loading}
	<Hamster />
{:else}
	<div class="flex gap-2">
		<Select.Root
			type="single"
			bind:value={trigger}
			onValueChange={trigger_type_selected}
		>
			<Select.Trigger class="min-w-40">
				{trigger === "Command" ? "Chat-Command" : trigger}
			</Select.Trigger>
			<Select.Content>
				<Select.Item value="Command">Chat-Command</Select.Item>
				<Select.Item value="Redeem">Redeem</Select.Item>
			</Select.Content>
		</Select.Root>
		{#if trigger === "Command"}
			<Input bind:value={identifier} placeholder="Command" />
		{:else}
			<Select.Root
				type="single"
				bind:value={identifier}
				disabled={num_redeems === 0}
			>
				<Select.Trigger>
					{redeems[identifier]?.name ?? "-"}
				</Select.Trigger>
				<Select.Content>
					{#each Object.keys(redeems) as redeem_id (redeem_id)}
						{@const redeem = redeems[redeem_id]}
						<Select.Item value={redeem.id}>
							{redeem.cost}: {redeem.name}
						</Select.Item>
					{/each}
				</Select.Content>
			</Select.Root>
		{/if}
	</div>
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
