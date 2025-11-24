<script lang="ts">
	import { cn } from "$lib/utils";
	import type { ClassValue } from "clsx";

	let {
		update,
		placeholder,
		class: className,
	}: {
		update: (t: string, valid: boolean) => void;
		placeholder?: string;
		class?: ClassValue | null;
	} = $props();
	let text: string = $state("");
	let valid = $derived(text.includes("{}"));

	$effect(() => update(text, text.length > 0 && valid));
</script>

<span></span>
<textarea
	bind:value={text}
	class={cn(
		"selection:bg-primary dark:bg-input/30 selection:text-primary-foreground border-input ring-offset-background placeholder:text-muted-foreground shadow-xs flex h-9 w-full min-w-0 rounded-md border bg-transparent px-3 pt-1.5 text-sm font-medium outline-none transition-[color,box-shadow] disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
		"focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]",
		"aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive",
		className,
	)}
	{placeholder}
></textarea>
