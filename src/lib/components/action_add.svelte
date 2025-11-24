<script lang="ts">
	import {
		type Trigger,
		type Action,
		commands,
		type Exec,
	} from "$lib/bindings";
	import { toast } from "svelte-sonner";
	import Button, { buttonVariants } from "./ui/button/button.svelte";
	import * as Dialog from "./ui/dialog/index";
	import type { RedeemMap } from "$lib/utils";
	import TriggerSelect from "./trigger_select.svelte";
	import ExecSelect from "./exec_select.svelte";
	import { Descriptions, type ExecKey } from "./exec_detailed/exec_utils";

	let open: boolean = $state(false);
	let trigger: Trigger = $state({ Command: "" });
	let exec: Exec = $state({ ChatMsg: "" });
	let exec_key: ExecKey = $derived(Object.keys(exec)[0] as ExecKey);
	let valid: boolean = $state(false);

	let num_redeems: number = $state(0);
	let redeems: RedeemMap = {};

	let {
		update,
	}: {
		update: () => void;
	} = $props();

	function add_action(): void {
		let action: Action = {
			trigger,
			exec,
			disabled: false,
		};
		commands.addAction(action).then(update);
		open = false;
	}

	function get_redeems(): void {
		commands.getRedeems().then((res) => {
			console.debug("redeems", res);
			if (res.status === "ok") {
				num_redeems = res.data.length;
				for (const redeem of res.data) redeems[redeem.id] = redeem;
			} else toast.error("Couldn't fetch redeems");
		});
	}

	function set_trigger(trig: Trigger): void {
		trigger = trig;
	}

	function set_exec(e: Exec, v: boolean): void {
		valid = v;
		exec = e;
	}

	function reset(): void {
		get_redeems();
		trigger = { Command: "" };
		exec = { ChatMsg: "" };
		valid = false;
	}
</script>

<Dialog.Root
	bind:open
	onOpenChange={(open): void => {
		if (open) reset();
	}}
>
	<Dialog.Trigger class={buttonVariants({ variant: "default" })}>
		Add Action
	</Dialog.Trigger>

	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>Add Action</Dialog.Title>
			<Dialog.Description class={{ hidden: !Descriptions[exec_key] }}>
				{Descriptions[exec_key]}
			</Dialog.Description>
		</Dialog.Header>

		<!-- Main Content -->
		<div class="grid grid-cols-[13rem_1fr] gap-2">
			<TriggerSelect
				{redeems}
				{num_redeems}
				update_redeems={get_redeems}
				update_trigger={set_trigger}
			/>
			<ExecSelect update_exec={set_exec} />
		</div>
		<!-- End Main Content -->

		<Dialog.Footer>
			<Dialog.Close class={buttonVariants({ variant: "destructive" })}>
				Cancel
			</Dialog.Close>

			<Button disabled={!valid} onclick={add_action}>Save</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
