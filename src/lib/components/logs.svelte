<script lang="ts">
	import Button from "./ui/button/button.svelte";
	import FolderOpenIcon from "@lucide/svelte/icons/folder-open";
	import RefreshIcon from "@lucide/svelte/icons/refresh-cw";
	import * as Sidebar from "./ui/sidebar/index";
	import { commands } from "../bindings";
	import { onMount } from "svelte";
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
			for (let i = res.length - 1; i >= 0; i--) {
				logBuf.push({
					id: i,
					text: res[i],
				});
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

	onMount(() => {
		get_logs();
		setInterval(() => {
			refresh_timer--;
			if (refresh_timer == 0) {
				refresh_timer = 5;
				get_logs();
			}
		}, 1000);
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
	<code class="flex flex-col flex-1 rounded-md bg-muted p-4 overflow-auto">
		<VirtualScroll
			data={logs}
			key="id"
			let:data
			estimateSize={24}
			keeps={75}
		>
			<p
				class="mb-4 pl-10 p-2 -indent-8 hover:bg-muted-foreground/15 rounded-md"
			>
				{@html ansi_converter.toHtml(data.text)}
			</p>
		</VirtualScroll>
	</code>
</div>
