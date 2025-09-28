<script lang="ts">
	import Button from "./ui/button/button.svelte";
	import FolderOpenIcon from "@lucide/svelte/icons/folder-open";
	import RefreshIcon from "@lucide/svelte/icons/refresh-cw";
	import * as Sidebar from "./ui/sidebar/index";
	import { commands } from "../bindings";
	import { onDestroy, onMount } from "svelte";
	import VirtualScroll from "svelte-virtual-scroll-list";
	import Convert from "ansi-to-html";

	interface Log {
		id: number;
		text: string;
	}

	let logs: Log[] = $state([]);
	let refresh_timer: number = $state(5);

	let ansi_converter = new Convert({
		colors: {
			0: "#000",
			1: "#A00",
			2: "#0A0",
			3: "#A50",
			4: "#1347dc", // <- only actually changed value, the rest is default; 'DEBUG' was hard to see in darkmode
			5: "#A0A",
			6: "#0AA",
			7: "#AAA",
			8: "#555",
			9: "#F55",
			10: "#5F5",
			11: "#FF5",
			12: "#55F",
			13: "#F5F",
			14: "#5FF",
			15: "#FFF",
		},
	});

	function get_logs(): void {
		commands.getCurrentLogs().then((res) => {
			// if the length is identical, there is no need to update the UI, we only ever append logs, never remove them
			if (res.length === logs.length) return;

			const logBuf: Log[] = [];
			let i = -1;
			for (const log of res) {
				console.log(log.charAt(0), log);
				if (log.startsWith("\u001b[2m2")) {
					i++;
					logBuf.push({
						id: i,
						text: log,
					});
				} else {
					logBuf[i].text += "\n" + log;
				}
			}
			logs = logBuf;
		});
	}

	function manual_refresh(): void {
		refresh_timer = 5;
		get_logs();
	}

	function open_logs(): void {
		commands.openLogDir();
	}

	let interval: number | undefined;

	onMount(() => {
		get_logs();
		interval = setInterval(() => {
			refresh_timer--;
			if (refresh_timer == 0) {
				refresh_timer = 5;
				get_logs();
			}
		}, 1000);
	});

	onDestroy(() => {
		if (interval !== undefined) clearInterval(interval);
	});
</script>

<div class="flex flex-col flex-1 h-screen">
	<header class="flex justify-between mb-2">
		<Sidebar.Trigger />
		<span class="flex items-center gap-1">
			<Button
				variant="secondary"
				class="font-mono"
				onclick={manual_refresh}
			>
				<RefreshIcon />
				{refresh_timer}
			</Button>
			<Button variant="secondary" onclick={open_logs}>
				<FolderOpenIcon />
				Open Logs
			</Button>
		</span>
	</header>
	<code
		class="flex flex-reverse flex-1 rounded-md bg-muted p-4 overflow-auto [&>div]:w-full"
	>
		<VirtualScroll
			data={logs}
			key="id"
			let:data
			estimateSize={24}
			keeps={75}
		>
			<p
				class="mb-4 pl-10 p-2 -indent-8 hover:bg-muted-foreground/15 rounded-md whitespace-pre-line"
			>
				{@html ansi_converter.toHtml(data.text)}
			</p>
		</VirtualScroll>
	</code>
</div>
