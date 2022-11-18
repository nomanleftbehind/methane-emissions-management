<script lang="ts">
	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import { fly } from 'svelte/transition';
	import Controller from '../../lib/components/Controller.svelte';
	import { AsyncControllersBy } from '../../codegen';
	import { user, pageMargin, facilityId } from '../stores';

	$: if (!$user) {
		if (browser) {
			goto('/login');
		}
	}
</script>

<article transition:fly style="margin-left: {$pageMargin}px;">
		{#await AsyncControllersBy({ variables: { by: { facilityId: $facilityId } } })}
			Loading...
		{:then { data: { controllersBy } }}
			<table>
				<thead>
					<tr>
						<th>Model</th>
						<th>FDC ID</th>
						<th>Manufacturer</th>
						<th>Application</th>
					</tr>
				</thead>
				<tbody>
					{#each controllersBy || [] as controller}
						<Controller {controller} />
					{/each}
				</tbody>
			</table>
		{/await}
</article>
