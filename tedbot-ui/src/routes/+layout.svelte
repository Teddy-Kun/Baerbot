<script lang="ts">
	import { Toaster } from "@/lib/components/ui/sonner";
    import { ModeWatcher } from "mode-watcher";

	let { children } = $props();
</script>

<ModeWatcher />
<Toaster />
{@render children?.()}
