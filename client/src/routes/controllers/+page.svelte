<script lang="ts">
	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import { fly } from 'svelte/transition';
	import { ControllersBy } from '../../codegen';
	import { user, pageMargin, facilityId } from '../stores';

	$: if (!$user) {
		if (browser) {
			goto('/login');
		}
	}
	$: controllers = ControllersBy({
		variables: { by: { facilityId: $facilityId } }
	});
</script>

<article transition:fly style="margin-left: {$pageMargin}px;">
	<ul>
		{#each $controllers?.data?.controllersBy || [] as controller, i}
			<li>
				{controller.manufacturer?.manufacturer} - {controller.function
					?.function} {i}
			</li>
		{/each}
	</ul>
</article>
