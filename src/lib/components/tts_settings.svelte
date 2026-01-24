<script lang="ts">
	import { commands, type TtsBackend } from "$lib/bindings";
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

	let selected_backend: TtsBackend = $state("System");
	let selected_language: string = $state("");
	let selected_voice: string = $state("");
	let test_text: string = $state("This is a test message");

	function get_cfg(): void {
		loading = true;
		commands
			.getTtsCfg()
			.then((res) => {
				selected_backend = res?.backend ?? "System";
				selected_language = res?.voice?.language ?? "";
				if (res?.voice?.name) selected_voice = res.voice.name;
				else if (selected_language)
					selected_voice = voices[selected_language][0] ?? "";
				else selected_voice = "";
				update_voices();
			})
			.finally(() => (loading = false));
	}

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

	function setBackend(backend: string): void {
		commands
			.setTtsBackend(backend as TtsBackend)
			.then(() => {
				selected_backend = backend as TtsBackend;
				voices = {};
				get_cfg();
			})
			.catch(() => toast.error("Error switching TTS Backend"));
	}

	get_cfg();
</script>

<div class="flex justify-between">
	<Sidebar.Trigger />

	<div class="flex gap-2 items-center">
		<label for="backend">Backend:</label>
		<Select.Root type="single" onValueChange={setBackend}>
			<Select.Trigger id="backend">
				{selected_backend}
			</Select.Trigger>

			<Select.Content>
				<Select.Item value="System">System</Select.Item>
				<Select.Item value="Piper">Piper (AI)</Select.Item>
			</Select.Content>
		</Select.Root>
	</div>
</div>

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
