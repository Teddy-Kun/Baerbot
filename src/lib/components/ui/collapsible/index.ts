import { Collapsible as CollapsiblePrimitive } from "bits-ui";
import CollapsibleContent from "./CollapsibleContent.svelte";

const Root = CollapsiblePrimitive.Root;
const Trigger = CollapsiblePrimitive.Trigger;

export {
	Root,
	CollapsibleContent as Content,
	Trigger,
	//
	Root as Collapsible,
	CollapsibleContent,
	Trigger as CollapsibleTrigger,
};
