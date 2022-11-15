<script lang="ts">
	import type { MeQuery } from '../../codegen';
	import { userData } from '../../routes/stores';
	import { Logout } from '../../codegen';
	import { ApolloError } from '@apollo/client/core';

	type IUser = Omit<NonNullable<MeQuery['me']>, '__typename'>;
	let isDropdownOpen = false; // default state (dropdown close)

	export let user: IUser;

	const handleDropdownClick = () => {
		isDropdownOpen = !isDropdownOpen; // togle state on click
	};

	const handleDropdownFocusLoss = ({
		relatedTarget,
		currentTarget
	}: FocusEvent & {
		currentTarget: EventTarget & HTMLDivElement;
	}) => {
		// use "focusout" event to ensure that we can close the dropdown when clicking outside or when we leave the dropdown with the "Tab" button
		if (relatedTarget instanceof HTMLElement && currentTarget.contains(relatedTarget)) return; // check if the new focus target doesn't present in the dropdown tree (exclude ul\li padding area because relatedTarget, in this case, will be null)
		isDropdownOpen = false;
	};

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

<div class="user-menu">
	<div class="dropdown" on:focusout={handleDropdownFocusLoss}>
		<button class="btn m-1" on:click={handleDropdownClick}>
			{user.email}
		</button>
		{#if isDropdownOpen}
			<ul class="dropdown-content menu p-2 shadow bg-base-100 rounded-box w-52">
				<li><button class="btn text-slate-300" on:click={logout}>Logout</button></li>
			</ul>
		{/if}
	</div>
</div>

<style>
	.user-menu {
		display: flex;
		border: 1px black solid;
		width: min-content;
		justify-content: space-between;
		position: absolute; /* This makes expanding dropdown menu not expand the navigation bar */
		right: 1% /* This keeps dropdown component on the right of nav bar */;
	}
</style>
