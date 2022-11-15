<script lang="ts">
	import { onMount } from 'svelte';
	import { userData, user } from './stores';
	import { Me } from '../codegen';
	import Dropdown from '../lib/components/Dropdown.svelte';

	onMount(async () => {
		Me({}).subscribe(({ data: { me } }) => userData.set(me));
	});
</script>

<nav>
	<a href="/">Home</a>
	<a href="/about">About</a>
	<a href="/controllers/hello-world">Controllers</a>
	<a href="/settings">Settings</a>
	{#if !$user}
		<a href="/login">Login</a>
	{:else}
		<Dropdown user={$user} />
	{/if}
</nav>

<slot />

<style>
	nav {
		background-color: rgb(226, 223, 43);
		padding: 10px;
		display: flex;
		position: sticky;
		top: 0;
		margin: -8px -8px 0px -8px;
	}
	a {
		margin: 4px;
	}
</style>
