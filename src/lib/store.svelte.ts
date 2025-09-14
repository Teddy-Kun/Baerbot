class Store {
	username: string | null = $state(null);
	logged_in: boolean = $derived(this.username !== null);
}

const store = new Store();
export default store;