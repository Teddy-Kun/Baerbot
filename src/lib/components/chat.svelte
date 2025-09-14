<script lang="ts">

	interface ChatProps {
		channel_name: string
		size: 1 | 2 | 3
		stroke: 1 | 2 | 3 | 4
		shadow: 1 | 2 | 3 
	}
	let { channel_name, size, stroke, shadow }: ChatProps = $props();
</script>

<iframe src={`https://www.giambaj.it/twitch/jchat/v2/?channel=${channel_name}&size=${size}&font=1&animate=true&bots=true&fade=30&stroke=${stroke}&shadow=${shadow}`} title="Chat Preview">

</iframe>
