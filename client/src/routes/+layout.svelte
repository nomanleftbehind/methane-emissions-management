<script lang="ts">

	import { user } from './stores';
	import { Me } from '../codegen';

	$: me = Me({});
	$: newUser = $me.data.me && {id: $me.data.me.id, email: $me.data.me.email} || null;
	$: user.set(newUser);
</script>


<nav>
	<a href="/">Home</a>
	<a href="/about">About</a>
	<a href="/controllers/hello-world">Controllers</a>
	<a href="/settings">Settings</a>
	<a href="/login">Login</a>
	<div>{$user?.email || ''}</div>
</nav>

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
  
<slot></slot>