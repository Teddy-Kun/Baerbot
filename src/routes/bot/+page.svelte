<script lang="ts">
	import * as Resizable from "$lib/components/ui/resizable/index.js";
</script>

<main class="border border-red-600">
	<Resizable.PaneGroup direction="horizontal">
		<Resizable.Pane>One</Resizable.Pane>
		<Resizable.Handle />
		<Resizable.Pane>Two</Resizable.Pane>
	</Resizable.PaneGroup>
</main>
