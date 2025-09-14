<script lang="ts">
	import Button from "$lib/components/ui/button/button.svelte";
	import * as Resizable from "$lib/components/ui/resizable/index.js";
	import { set_accent } from "$lib/utils";
</script>

<div class="flex-1">
	<Resizable.PaneGroup direction="horizontal">
		<Resizable.Pane>
			<Button onclick={(): void => set_accent("#ff0000")}>TEST</Button>
		</Resizable.Pane>
		<Resizable.Handle />
		<Resizable.Pane>Two</Resizable.Pane>
	</Resizable.PaneGroup>
</div>
