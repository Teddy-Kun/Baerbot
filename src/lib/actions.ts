export interface Trigger {
	trigger: "command" | "redeem",
	name: string
}

export interface Action {
	trigger: Trigger,
	action: unknown,
	params: unknown
}
