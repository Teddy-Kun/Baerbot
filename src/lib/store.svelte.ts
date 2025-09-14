class Store {
	username: string | null = $state(null);
	logged_in: boolean = $derived(this.username !== null);
	debug = $state(false);
}

const store = new Store();
export default store;