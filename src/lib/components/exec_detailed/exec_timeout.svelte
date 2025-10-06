<script lang="ts">
	import { Label } from "bits-ui";
	import ExecUser from "./exec_user.svelte";
	import Input from "../ui/input/input.svelte";
	import type { Exec, ExecTarget } from "$lib/bindings";
	import { onMount } from "svelte";

	let { update_exec }: { update_exec: (e: Exec) => void } = $props();

	let target: ExecTarget = $state("Other");
	let timeout: number = $state(60);
	let timeout_converted: string = $derived.by((): string => {
		if (timeout < 60) return "";
		const minutes = timeout / 60;
		if (minutes < 60) return `~${minutes.toFixed(2)} Minute(s)`;
		const hours = minutes / 60;
		if (hours < 24) return `~${hours.toFixed(2)} Hour(s)`;
		const days = hours / 24;
		return `~${days.toFixed(2)} Day(s)`;
	});

	function update(): void {
		update_exec({ Timeout: [target, timeout < 1 ? 1 : timeout] });
	}

	$effect(update);
	onMount(update);
</script>

<ExecUser
	update_target={(t): void => {
		target = t;
	}}
/>
<div>
	<Label.Root for="timout_sec">Timout in Seconds</Label.Root>
	<Input
		bind:value={timeout}
		id="timout_sec"
		type="number"
		min={1}
		max={Number.MAX_SAFE_INTEGER}
	/>
</div>
<i class="mt-7.5 text-muted-foreground">{timeout_converted}</i>
