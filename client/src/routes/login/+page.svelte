<script lang="ts">

	import { user } from '../stores';
	import {
		Login,
		MeDoc
	} from "../../codegen";

	const login = () => {
		Login({ variables: { loginUserInput: {email, password} }, refetchQueries: [{ query: MeDoc }]})
		.then(({ data }) => {
			const newUser = data?.login && {id: data.login.id, email: data.login.email} || null;
			user.set(newUser);
		})
		.catch((a) => console.log('catch', a));
		email = '';
		password = '';
	}

	$: email = 'dsucic@bonterraenergy.com';
	$: password = 'everythinghastostartsomewhere';
</script>

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

<form method="POST">
  <input name="email" type="email" bind:value={email}>
  <input name="password" type="password" bind:value={password}>
  <button
	disabled={email.length === 0}
	on:click={login}>Log in</button>
</form>

<!-- <br />
<main class="cards">
	<div class="card">
		<h2>Login User</h2>
		<input placeholder="Email..." bind:value={email} />
		<input type='password' placeholder="Password..." bind:value={password} />
		<button
			disabled={email.length === 0}
			on:click={() => {
				Login({ variables: { loginUserInput: {email, password} } });
				// you can "auto refresh queries" adding the code bellow to AddCodegenUser (but here we want to demo the manual refresh button)
				// refetchQueries: [{ query: GetCodegenUsersDoc }],
				email = '';
				password = '';
			}}>Login</button>
	</div>
</main> -->