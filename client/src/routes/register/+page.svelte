<script lang="ts">
	import { goto } from '$app/navigation';
	import { userData, user } from '../stores';
	import { Login, MeDoc, Register, Role } from '../../codegen';
	import { browser } from '$app/environment';
	import { ApolloError } from '@apollo/client/core';

	$: authorizedToRegister = $user?.role === 'ADMIN' || $user?.role === 'ENGINEER';
	$: if ($user && !authorizedToRegister) {
		if (browser) {
			goto('/');
		}
	}

	const login = () => {
		console.log('running login');

		Login({
			variables: { loginUserInput: { email, password } },
			refetchQueries: [{ query: MeDoc }]
		})
			.then(({ data }) => {
				userData.set(data?.login);
				goto('/');
			})
			.catch((e) => {
				if (e instanceof ApolloError) {
					console.log('login error message', e.message);
				}
			});
		email = '';
		password = '';
	};

	const register = () => {
		Register({
			variables: {
				registerUserInput: { firstName, lastName, email, password, role }
			}
		})
			.then(({ data }) => {
				console.log('register data', data?.register);
				goto('/');
			})
			.catch((e) => {
				if (e instanceof ApolloError) {
					console.log('register error message', e.message);
				}
			});
		email = '';
		password = '';
		firstName = '';
		lastName = '';
		role = null;
	};

	$: email = 'dsucic@bonterraenergy.com';
	$: password = 'everythinghastostartsomewhere';
	$: firstName = '';
	$: lastName = '';
	$: role = null;
</script>

<form on:submit|preventDefault={authorizedToRegister ? register : login} method="POST">
	{#if authorizedToRegister}
		<input name="firstName" type="text" bind:value={firstName} />
		<input name="lastName" type="text" bind:value={lastName} />
	{/if}
	<input name="email" type="email" bind:value={email} />
	<input name="password" type="password" bind:value={password} />
	{#if authorizedToRegister}
		<select bind:value={role}>
			{#each Object.values(Role) as role}
				<option value={role}>
					{role}
				</option>
			{/each}
		</select>
	{/if}
	<button disabled={email.length === 0} type="submit"
		>{authorizedToRegister ? 'Register' : 'Login'}</button
	>
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
