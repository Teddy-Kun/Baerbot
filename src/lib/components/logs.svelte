<script lang="ts">
	import Button from "./ui/button/button.svelte";
	import FolderOpenIcon from "@lucide/svelte/icons/folder-open";
	import * as Sidebar from "./ui/sidebar/index";
</script>

<div class="flex flex-col flex-1">
	<div class="flex justify-between mb-2">
		<Sidebar.Trigger />
		<Button variant="secondary">
			<FolderOpenIcon />
			Open Logs
		</Button>
	</div>
	<code class="flex-1 rounded-md bg-muted overflow-auto p-4">
		There will be logs here
	</code>
</div>
