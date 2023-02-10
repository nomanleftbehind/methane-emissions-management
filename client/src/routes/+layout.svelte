<script lang="ts">
	import { onMount } from 'svelte';
	import { userData, user, pageMargin } from './stores';
	import { Me } from '../codegen';
	import Dropdown from '../lib/components/Dropdown.svelte';
	import Sidebar from '../lib/components/Sidebar.svelte';
	let sidebar_show = false;

	$: if (!$user) {
		sidebar_show = false;
		pageMargin.set(0);
	}

	onMount(async () => {
		Me({}).subscribe(({ data: { me } }) => userData.set(me));
	});
</script>

<Sidebar bind:show={sidebar_show} />

<nav>
	{#if $user}
		<button
			on:click={() => {
				sidebar_show = !sidebar_show;
				if (sidebar_show) {
					pageMargin.set(350);
				} else {
					pageMargin.set(0);
				}
			}}>Toggle Sidebar</button
		>
	{/if}
	<a href="/">Home</a>
	<a href="/about">About</a>
	<a href="/controllers">Controllers</a>
	{#if !$user || $user.role === 'ADMIN' || $user.role === 'ENGINEER'}
		<a href="/register">{$user ? 'Register' : 'Login'}</a>
	{/if}
	{#if $user}
		<Dropdown user={$user} />
	{/if}
</nav>

<slot />

<style>
	nav {
		position: static;
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
	button {
		border: unset;
		background-color: unset;
		cursor: pointer;
		margin: 4px;
		border-bottom: 1px solid blue;
	}
</style>
