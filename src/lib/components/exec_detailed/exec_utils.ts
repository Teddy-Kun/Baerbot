import type { Exec } from "$lib/bindings";

// some massaging to convince typescript that `Trigger` has keys
type KeysOfUnion<T> = T extends unknown ? keyof T : never;
export type ExecKey = KeysOfUnion<Exec>;

// key_translations breaks eslint indentation here for some reason
export const KeyTranslations = Object.freeze({
	Ban: "Ban User",
	Chance: "Random",
	ChatMsg: "Chat-Message",
	Counter: "Count something",
	Timeout: "Timeout User",
} satisfies { [k in ExecKey]: string | null });

export const AllKeys = Object.freeze(Object.keys(KeyTranslations) as ExecKey[]);

export const TEDDY_WIP: string = "Teddy is still working on it 🧸⚙️";

export const Descriptions = Object.freeze({
	Ban: "Other: A user specified by the one activating the action. User: The one activating the action",
	Chance:
		"Randomly executes one of the two options with the chance given. You can technically chain them into multiple randoms, but the UI is really not made for it.",
	ChatMsg:
		"Say a message in chat. You can use {a..b} to generate a random number, in the range of `a` up to and including `b`. For example {1..6} would be like rolling a 6-sided dice.",
	Counter: null,
	Timeout:
		"Other: A user specified by the one activating the action. User: The one activating the action",
} satisfies { [k in ExecKey]: string | null });
