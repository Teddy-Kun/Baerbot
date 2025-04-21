<script lang="ts">
	import * as Sidebar from "@/lib/components/ui/sidebar/index.js";
	import { useSidebar } from "@/lib/components/ui/sidebar/index.js";
	import Button from "../ui/button/button.svelte";
	import { Cog } from "lucide-svelte";
	import * as Dialog from "../ui/dialog";

	let iconClass = $state("");

	$effect(() => {
		const sidebar = useSidebar();
		console.log("updating");
		if (sidebar.open) {
			iconClass = "";
		} else {
			iconClass = "ml-2";
		}
	});
</script>

<Sidebar.Menu>
	<Dialog.Root>
		<Dialog.Trigger>
			<Sidebar.MenuItem>
				<Button class="w-full" variant="outline">
					<Cog class={iconClass} />
					<p class="truncate">Settings</p>
				</Button>
			</Sidebar.MenuItem>
		</Dialog.Trigger>

		<Dialog.Content>
			<Dialog.Header>
				<Dialog.Title>Settings</Dialog.Title>
			</Dialog.Header>

			WIP
		</Dialog.Content>
	</Dialog.Root>
</Sidebar.Menu>
