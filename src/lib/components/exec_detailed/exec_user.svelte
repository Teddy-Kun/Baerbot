<script lang="ts">
	import type { ExecTarget } from "$lib/bindings";
	import { onMount } from "svelte";
	import * as Select from "../ui/select/index";

	let { update_target }: { update_target: (t: ExecTarget) => void } =
		$props();

	let target: ExecTarget = $state("Other");

	$effect(() => update_target(target));
	onMount(() => update_target(target));
</script>

<Select.Root type="single" bind:value={target}>
	<Select.Trigger class="w-full">{target}</Select.Trigger>

	<Select.Content>
		<Select.Item value="Other">Other</Select.Item>
		<Select.Item value="User">User</Select.Item>
	</Select.Content>
</Select.Root>
