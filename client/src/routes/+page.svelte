<script lang="ts">
	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import { fade } from 'svelte/transition';
	import { AllFacilities } from '../codegen';
	import { user, pageMargin } from './stores';

	$: if (!$user) {
		if (browser) {
			goto('/login');
		}
	}
	$: facilities = AllFacilities({});
</script>

<article transition:fade style="margin-left: {$pageMargin}px;">
	{#if $facilities.loading}
		<div style="grid-column: 1/3;">Loading</div>
	{/if}
	<h2 class="c1" style="grid-row: 1;">Row</h2>
	<h2 class="c2" style="grid-row: 1;">ID</h2>
	<h2 class="c3" style="grid-row: 1;">IDPA</h2>
	<h2 class="c4" style="grid-row: 1;">Name</h2>
	<h2 class="c5" style="grid-row: 1;">Type</h2>
	<h2 class="c6" style="grid-row: 1;">Date Created</h2>
	{#each $facilities.data?.allFacilities || [] as { id, idpa, name, type, createdAt, updatedAt }, i}
		<div class="c1" style="grid-row: {i + 2};">
			{i + 1}
		</div>
		<div class="c2" style="grid-row: {i + 2};">
			{id}
		</div>
		<div class="c3" style="grid-row: {i + 2};">
			{idpa}
		</div>
		<div class="c4" style="grid-row: {i + 2};">
			{name}
		</div>
		<div class="c5" style="grid-row: {i + 2};">
			{type}
		</div>
		<div class="c6" style="grid-row: {i + 2};">
			{createdAt.split('T')[0]}
		</div>
	{/each}
</article>

<style>
	article {
		display: grid;
		grid-template-columns: 30px 300px 100px max-content 40px 100px;
		font-family: 'Comic Sans MS', cursive;
		font-size: 2em;
	}
	div,
	h2 {
		font-size: small;
		border: 1px solid black;
	}
	.c1 {
		grid-column: 1;
	}
	.c2 {
		grid-column: 2;
	}
	.c3 {
		grid-column: 3;
	}
	.c4 {
		grid-column: 4;
	}
	.c5 {
		grid-column: 5;
	}
	.c6 {
		grid-column: 6;
	}
</style>
