<script lang="ts">
	import { commands } from "$lib/bindings";
	import Hamster from "./hamster.svelte";
	import * as Sidebar from "./ui/sidebar/index";
	import * as Select from "./ui/select/index";
	import Button from "./ui/button/button.svelte";
	import Input from "./ui/input/input.svelte";

	let loading: boolean = $state(false);
	let voices: { [str: string]: string[] } = $state({});

	let languages: string[] = $derived(Object.keys(voices));

	let selected_language: string = $state("");
	let selected_voice: string = $state("");
	let test_text: string = $state("This is a test message");

	function select_language(lang: string): void {
		selected_language = lang;
		selected_voice = voices[lang][0];
	}

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

				select_language(languages[0]);
			})
			.finally(() => (loading = false));
	}

	function testTts(): void {
		commands.testTts(test_text, {
			language: selected_language,
			name: selected_voice,
		});
	}

	update_voices();
</script>

<Sidebar.Trigger />

<div>
	{#if loading}
		<Hamster />
	{:else}
		{#if languages.length}
			<Select.Root type="single" onValueChange={select_language}>
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
		<Select.Root type="single" bind:value={selected_voice}>
			<Select.Trigger>
				{selected_voice}
			</Select.Trigger>
			<Select.Content>
				{#each voices[selected_language] as voice, i (i)}
					<Select.Item value={voice}>
						{voice}
					</Select.Item>
				{/each}
			</Select.Content>
		</Select.Root>

		<Input bind:value={test_text} />
		<Button onclick={testTts}>Test</Button>
	{/if}
</div>
