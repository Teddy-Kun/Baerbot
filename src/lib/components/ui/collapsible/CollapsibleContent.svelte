<script lang="ts">
	import { cn } from "@/lib/utils";
	import { Collapsible as Collapsible, type WithoutChild } from "bits-ui";

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithoutChild<Collapsible.ContentProps> = $props();

</script>

<Collapsible.Content
	bind:ref
	class={cn(
		"data-[state=closed]:animate-collapsible-up data-[state=open]:animate-collapsible-down overflow-hidden transition-all",
		className
	)}
	{...restProps}
>
	{@render children?.()}
</Collapsible.Content>