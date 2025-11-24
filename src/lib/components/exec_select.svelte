<script lang="ts">
	import type { Exec } from "$lib/bindings";
	import * as Select from "./ui/select/index";
	import ExecUser from "./exec_detailed/exec_user.svelte";
	import {
		AllKeys,
		KeyTranslations,
		type ExecKey,
	} from "./exec_detailed/exec_utils";
	import ExecTimeout from "./exec_detailed/exec_timeout.svelte";
	import TextInput from "./exec_detailed/text_input.svelte";
	import ExecWip from "./exec_detailed/exec_wip.svelte";
	import ExecRandom from "./exec_detailed/exec_random.svelte";
	import ExecCounter from "./exec_detailed/exec_counter.svelte";

	let { update_exec }: { update_exec: (e: Exec, valid: boolean) => void } =
		$props();

	let exec_type: ExecKey = $state("ChatMsg");
	let child_exec: Exec = $state({ ChatMsg: "" });
	let valid: boolean = $state(false);

	$effect(() => update_exec(child_exec, valid));
</script>

<Select.Root
	type="single"
	bind:value={exec_type}
	onValueChange={(): void => {
		valid = false;
	}}
>
	<Select.Trigger class="w-full">
		{KeyTranslations[exec_type] ?? exec_type}
	</Select.Trigger>
	<Select.Content>
		{#each AllKeys as key (key)}
			<Select.Item value={key}>
				{KeyTranslations[key] ?? key}
			</Select.Item>
		{/each}
	</Select.Content>
</Select.Root>

{#if exec_type === "Ban"}
	<ExecUser
		update_target={(t): void => {
			child_exec = { Ban: t };
			valid = true;
		}}
	/>
{:else if exec_type === "Timeout"}
	<ExecTimeout
		update_exec={(e): void => {
			child_exec = e;
			valid = true;
		}}
	/>
{:else if exec_type === "ChatMsg"}
	<TextInput
		class="col-span-2"
		update={(text, v): void => {
			child_exec = { ChatMsg: text };
			valid = v;
		}}
	/>
{:else if exec_type === "Chance"}
	<ExecRandom
		update={(e, v): void => {
			child_exec = e;
			valid = v;
		}}
	/>
{:else if exec_type === "Counter"}
	<ExecCounter
		class="col-span-2"
		update={(e, v): void => {
			child_exec = { Counter: { counter: 0, template: e } };
			valid = v;
		}}
	/>
{:else}
	<ExecWip
		update={(v): void => {
			child_exec = { [exec_type]: "" } as Exec;
			valid = v;
		}}
	/>
{/if}
