<script lang="ts">
	import type { Exec } from "$lib/bindings";
	import ExecSelect from "../exec_select.svelte";
	import Input from "../ui/input/input.svelte";
	import * as Select from "../ui/select/index";
	import Separator from "../ui/separator/separator.svelte";

	let { update }: { update: (e: Exec, valid: boolean) => void } = $props();

	let chance_input: "Percentage" | "Fraction" = $state("Percentage");
	let percentage: number = $state(50);
	let fraction: { one: number; some: number } = $state({ one: 1, some: 2 });
	let option_one: { exec: Exec; valid: boolean } = $state({
		exec: { ChatMsg: "" },
		valid: false,
	});
	let option_two: { exec: Exec; valid: boolean } = $state({
		exec: { ChatMsg: "" },
		valid: false,
	});

	let chance: number = $derived.by((): number => {
		if (chance_input === "Percentage")
			return Math.max(Math.min(percentage, 100), 0) / 100;
		else
			return Math.min(
				Math.max(fraction.one, 1) / Math.max(fraction.some, 1),
				1,
			);
	});

	$effect((): void => {
		update(
			{ Chance: [chance, option_one.exec, option_two.exec] },
			option_one.valid && option_two.valid,
		);
	});
</script>

<span></span>
<Select.Root type="single" bind:value={chance_input}>
	<Select.Trigger class="w-full">
		as {chance_input}
	</Select.Trigger>

	<Select.Content>
		<Select.Item value="Percentage">as Percentage</Select.Item>
		<Select.Item value="Fraction">as Fraction</Select.Item>
	</Select.Content>
</Select.Root>
{#if chance_input === "Fraction"}
	<span class="flex items-center gap-2">
		<Input bind:value={fraction.one} type="number" min={1} />
		in
		<Input bind:value={fraction.some} type="number" min={1} />
	</span>
{:else}
	<span class="flex items-center gap-1">
		<Input
			bind:value={percentage}
			class="w-fit"
			type="number"
			min={0}
			max={100}
		/>%
	</span>
{/if}

<Separator class="col-span-2" />
<ExecSelect
	update_exec={(e, v): void => {
		option_one = { exec: e, valid: v };
	}}
/>
<Separator class="col-span-2" />
<ExecSelect
	update_exec={(e, v): void => {
		option_two = { exec: e, valid: v };
	}}
/>
