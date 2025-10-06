<script lang="ts">
	import type { Trigger } from "$lib/bindings";
	import type { RedeemMap } from "$lib/utils";
	import Input from "./ui/input/input.svelte";
	import * as Select from "./ui/select/index";

	let {
		redeems,
		num_redeems,
		update_redeems,
		update_trigger,
	}: {
		redeems: RedeemMap;
		num_redeems: number;
		update_redeems: () => void;
		update_trigger: (trigger: Trigger) => void;
	} = $props();

	// some massaging to convince typescript that `Trigger` has keys
	type KeysOfUnion<T> = T extends unknown ? keyof T : never;
	type TriggerKey = KeysOfUnion<Trigger>;

	let trigger_type: TriggerKey = $state("Command");
	let identifier: string = $state("");

	$effect(() => {
		let ident: string;
		if (trigger_type === "Command") ident = identifier.toLowerCase();
		else ident = identifier;
		update_trigger({ [trigger_type]: ident } as Trigger);
	});

	function trigger_type_selected(value: string): void {
		if (value === "Redeem") update_redeems();
		identifier = "";
	}
</script>

<Select.Root
	type="single"
	bind:value={trigger_type}
	onValueChange={trigger_type_selected}
>
	<Select.Trigger class="w-full">
		{trigger_type === "Command" ? "Chat-Command" : trigger_type}
	</Select.Trigger>
	<Select.Content>
		<Select.Item value="Command">Chat-Command</Select.Item>
		<Select.Item value="Redeem">Redeem</Select.Item>
	</Select.Content>
</Select.Root>

{#if trigger_type === "Command"}
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
