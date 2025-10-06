<script lang="ts">
	import { onMount } from "svelte";
	import { TEDDY_WIP } from "./exec_utils";

	let { update }: { update: (valid: boolean) => void } = $props();

	onMount(() => update(false));
</script>

<p class="mt-1.5">{TEDDY_WIP}</p>
