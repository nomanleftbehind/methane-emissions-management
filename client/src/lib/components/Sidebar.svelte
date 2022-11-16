<script>
	import { fly } from 'svelte/transition';
	import { AllFacilities } from '../../codegen';
	import { pageMargin } from '../../routes/stores';

	export let show = false;

	$: facilities = AllFacilities({});
</script>

{#if show}
	<nav transition:fly={{ x: -$pageMargin, opacity: 1 }}>
		<ul>
			{#each $facilities.data?.allFacilities || [] as { id, idpa, name, type, createdAt, updatedAt }, i}
				<li>
					<button class="btn text-slate-300">{idpa}</button>
				</li>
			{/each}
		</ul>
	</nav>
{/if}

<style>
	nav {
		position: fixed;
		top: 47px;
		left: 0;
		height: 100%;
		padding: 2rem 1rem 0.6rem;
		border-right: 1px solid #aaa;
		border-top: 1px solid #aaa;
		background: #fff;
		overflow-y: scroll;
		width: 20rem;
	}
	li {
		list-style-type: none;
	}
	button {
		padding: 4px;
		margin: 4px;
		border-radius: 5px;
		border-right: unset;
		border-left: unset;
		border-top: unset;
		background-color: unset;
		cursor: pointer;
		border-bottom: 1px solid rgb(106, 153, 208);
	}
	button:hover {
		transform: scale(1.1);
		/* background-color: rgb(106,153,208); */
	}
</style>
