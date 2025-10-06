<script lang="ts">
	import Button from "./ui/button/button.svelte";
	import * as Table from "$lib/components/ui/table/index";
	import BotIcon from "@lucide/svelte/icons/bot";
	import BotOffIcon from "@lucide/svelte/icons/bot-off";
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
	import ActionAdd from "./action_add.svelte";
	import type { ExecKey } from "./exec_detailed/exec_utils";
	import * as Tooltip from "./ui/tooltip/index";

	let loading: boolean = $state(true);

	let actions: Action[] = $state([]);

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
		let key = Object.keys(trigger)[0] as ExecKey;
		// @ts-expect-error works guaranteed, typescript is just stupid
		let res: unknown = trigger[key];
		if (typeof res === "object") return "Display WIP";
		return `${res}`;
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

	function toggle_action(action: Action): void {
		const trigger_string = get_trigger_inner(action.trigger, false);
		commands.toggleDisableAction(trigger_string).then((disabled) => {
			if (disabled === null) return;

			let action = actions.find(
				(a) => get_trigger_inner(a.trigger, false) === trigger_string,
			);
			if (action) action.disabled = disabled;
		});
	}

	onMount(update_actions);
</script>

<Sidebar.Trigger />
{#if loading}
	<Hamster />
{:else}
	<ActionAdd update={update_actions} />

	<div>
		<h3>Current Commands</h3>
		<i class="text-muted-foreground text-sm">
			You can click commands to disable them.
		</i>
	</div>

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
				<Table.Cell
					class={{
						"text-muted-foreground line-through": action.disabled,
					}}
				>
					<p>{get_trigger_type(action.trigger)}</p>
					<p>{get_trigger_inner(action.trigger)}</p>
				</Table.Cell>
				<Table.Cell
					class={{
						"text-muted-foreground line-through": action.disabled,
					}}
				>
					<p>{get_exec_type(action.exec)}</p>
					<p>{get_exec_inner(action.exec)}</p>
				</Table.Cell>
				<Table.Cell>
					<Tooltip.Root>
						<Tooltip.Trigger>
							<Button
								variant="secondary"
								onclick={(): void => toggle_action(action)}
							>
								{#if action.disabled}
									<BotIcon />
								{:else}
									<BotOffIcon />
								{/if}
							</Button>
						</Tooltip.Trigger>
						<Tooltip.Content>
							{action.disabled ? "Enable" : "Disable"}
						</Tooltip.Content>
					</Tooltip.Root>

					<Tooltip.Root>
						<Tooltip.Trigger>
							<Button
								variant="destructive"
								onclick={(): void => remove_action(action)}
							>
								<TrashIcon />
							</Button>
						</Tooltip.Trigger>
						<Tooltip.Content>Delete</Tooltip.Content>
					</Tooltip.Root>
				</Table.Cell>
			</Table.Row>
		{/each}
	</Table.Root>
{/if}
