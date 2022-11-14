<script lang="ts">
	import { onMount } from 'svelte';
	import { userData, user } from './stores';
	import { Me, Logout } from '../codegen';
	import { ApolloError } from '@apollo/client/core';

	onMount(async () => {
		Me({}).subscribe(({ data: { me } }) => userData.set(me));
	});

	const logout = () => {
		Logout({})
			.then(({ data }) => {
				if (data) {
					userData.set(null);
				}
			})
			.catch((e) => {
				if (e instanceof ApolloError) {
					console.log('e message', e.message);
				}
			});
	};
</script>

<nav>
	<a href="/">Home</a>
	<a href="/about">About</a>
	<a href="/controllers/hello-world">Controllers</a>
	<a href="/settings">Settings</a>
	{#if !$user}
		<a href="/login">Login</a>
	{:else}
		<button on:click={logout}>
			{$user.email}
		</button>
	{/if}
</nav>

<slot />

<style>
	nav {
		background-color: rgb(226, 223, 43);
		padding: 10px;
		/* margin: -10px; */
	}
	a {
		margin: 4px;
	}
</style>
