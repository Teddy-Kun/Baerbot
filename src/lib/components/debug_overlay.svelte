<script lang="ts">
	import { resolve } from "$app/paths";
	import store from "$lib/store.svelte";
</script>

{#if store.debug}
	<div
		class="absolute top-2 mx-2 w-full h-[70vh] bg-black/60 backdrop-blur-lg rounded-md"
	>
		<div class="flex flex-col gap-1">
			<a href={resolve("/bot")}>bot</a>
			<a href={resolve("/login")}>login</a>
		</div>
	</div>
{/if}
