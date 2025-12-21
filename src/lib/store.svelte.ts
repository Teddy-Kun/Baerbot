import { goto } from "$app/navigation";
import { resolve } from "$app/paths";

class Store {
	username: string | null = $state(null);
	logged_in: boolean = $derived(this.username !== null);
	current_tab: "actions" | "obs" | "logs" = $state("actions");
	debug = $state(false);

	register_login(username: string): void {
		this.username = username;
		goto(resolve("/bot"));
	}
}

const store = new Store();
export default store;
