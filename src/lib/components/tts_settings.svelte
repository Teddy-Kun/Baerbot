<script lang="ts">
	import { commands } from "$lib/bindings";
	import Hamster from "./hamster.svelte";
	import * as Sidebar from "./ui/sidebar/index";
	import * as Select from "./ui/select/index";
	import Button from "./ui/button/button.svelte";
	import Input from "./ui/input/input.svelte";
	import { toast } from "svelte-sonner";

	let loading: boolean = $state(false);
	let save_loading: boolean = $state(false);
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
				for (const data of res.voices) {
					if (voices[data.language])
						voices[data.language].push(data.name);
					else voices[data.language] = [data.name];
				}

				if (res.active) {
					selected_language = res.active.language;
					selected_voice = res.active.name;
				} else select_language(languages[0]);
			})
			.finally(() => (loading = false));
	}

	function test_tts(): void {
		commands.testTts(test_text, {
			language: selected_language,
			name: selected_voice,
		});
	}

	function save_voice(): void {
		commands
			.setTtsVoice({
				language: selected_language,
				name: selected_voice,
			})
			.then(() => {
				toast.success("Voice selection saved");
			});
	}

	update_voices();
</script>

<Sidebar.Trigger />

<div>
	{#if loading}
		<div class="flex flex-1 items-center justify-center">
			<Hamster />
		</div>
	{:else}
		<div class="flex w-full gap-2 mb-2">
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

			<Button disabled={save_loading} onclick={save_voice}>Save</Button>
		</div>

		<div class="flex w-full gap-2">
			<Input bind:value={test_text} />
			<Button onclick={test_tts}>Test</Button>
		</div>
	{/if}
</div>
