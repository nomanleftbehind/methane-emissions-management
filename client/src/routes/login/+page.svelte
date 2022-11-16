<script lang="ts">
	import { goto } from '$app/navigation';
	import { userData, user } from '../stores';
	import { Login, MeDoc } from '../../codegen';
	import { browser } from '$app/environment';
	import { ApolloError } from '@apollo/client/core';

	$: if ($user) {
		if (browser) {
			goto('/');
		}
	}

	const login = () => {
		Login({
			variables: { loginUserInput: { email, password } },
			refetchQueries: [{ query: MeDoc }]
		})
			.then(({ data }) => {
				userData.set(data?.login);
			})
			.catch((e) => {
				if (e instanceof ApolloError) {
					console.log('e message', e.message);
				}
			});
		email = '';
		password = '';
	};

	$: email = 'dsucic@bonterraenergy.com';
	$: password = 'everythinghastostartsomewhere';
</script>

<form method="POST">
	<input name="email" type="email" bind:value={email} />
	<input name="password" type="password" bind:value={password} />
	<button disabled={email.length === 0} on:click={login}>Log in</button>
</form>

<style>
	form {
		display: flex;
		flex-direction: column;
		width: 15%;
		padding: 10px;
		border: 1px solid black;
		border-radius: 10px;
		background-color: rgb(173, 196, 178);
		margin: 20px;
		box-shadow: 3px 3px 3px #a5a5a5;
	}

	button {
		width: 50%;
		margin: 10px;
		justify-content: center;
	}
</style>
