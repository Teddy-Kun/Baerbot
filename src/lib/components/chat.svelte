<script lang="ts">
	import type { ClassValue } from "svelte/elements";

	interface ChatProps {
		channel_name: string;
		size: 1 | 2 | 3;
		stroke?: 1 | 2 | 3 | 4;
		shadow?: 1 | 2 | 3;
		class?: ClassValue;
	}
	let {
		channel_name,
		size,
		stroke,
		shadow,
		class: cls,
	}: ChatProps = $props();
</script>

<div class={cls}>
	<iframe
		class="size-full"
		src={`http://www.giambaj.it/twitch/jchat/v2/?channel=${channel_name}&size=${size}&background-color=black&font=1&animate=true&bots=true&stroke=${stroke ?? size}&shadow=${shadow ?? size}`}
		title="Chat Preview"
		allowtransparency={true}
		style="background-color: transparent;"
	>
	</iframe>
</div>
