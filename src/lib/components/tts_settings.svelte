<script lang="ts">
	import { commands } from "$lib/bindings";
	import Hamster from "./hamster.svelte";
	import * as Sidebar from "./ui/sidebar/index";
	import * as Select from "./ui/select/index";

	let loading: boolean = $state(false);
	let voices: { [str: string]: string[] } = $state({});

	let languages: string[] = $derived(Object.keys(voices));

	let selected_language: string = $state("");

	function update_voices(): void {
		loading = true;
		commands
			.getTtsVoices()
			.then((res) => {
				for (const data of res) {
					if (voices[data.language])
						voices[data.language].push(data.name);
					else voices[data.language] = [data.name];
				}

				selected_language = languages[0];
			})
			.finally(() => (loading = false));
	}

	update_voices();
</script>

<Sidebar.Trigger />

<div>
	{#if loading}
		<Hamster />
	{:else}
		{#if languages.length}
			<Select.Root type="single" bind:value={selected_language}>
				<Select.Trigger>
					{selected_language}
				</Select.Trigger>

				<Select.Content>
					{#each languages as lang (lang)}
						<Select.Item value={lang}>
							{lang}
						</Select.Item>
					{/each}
				</Select.Content>
			</Select.Root>
		{/if}
		{#each voices[selected_language] as voice, i (i)}
			Name: {voice}<br />
		{/each}
	{/if}
</div>
