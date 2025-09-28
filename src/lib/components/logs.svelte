<script lang="ts">
	import Button from "./ui/button/button.svelte";
	import FolderOpenIcon from "@lucide/svelte/icons/folder-open";
	import * as Sidebar from "./ui/sidebar/index";
	import { commands } from "../bindings";
	import { onMount } from "svelte";
	import VirtualScroll from "svelte-virtual-scroll-list";

	interface Log {
		id: number;
		text: string;
	}

	let logs: Log[] = $state([]);

	function get_logs(): void {
		commands.getCurrentLogs().then((res) => {
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

	function open_logs(): void {
		commands.openLogDir();
	}

	onMount(get_logs);
</script>

<div class="flex flex-col flex-1 h-screen">
	<header class="flex justify-between mb-2">
		<Sidebar.Trigger />
		<Button variant="secondary" onclick={open_logs}>
			<FolderOpenIcon />
			Open Logs
		</Button>
	</header>
	<code
		class="flex flex-col-reverse flex-1 rounded-md bg-muted p-4 overflow-auto"
	>
		<VirtualScroll
			data={logs}
			key="id"
			let:data
			estimateSize={24}
			keeps={75}
		>
			<p>{data.text}</p>
		</VirtualScroll>
	</code>
</div>
