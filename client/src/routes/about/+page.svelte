<script lang="ts">
	import { goto } from '$app/navigation';
	import { user } from '../stores';
	import { browser } from '$app/environment';

	$: if (!$user) {
		if (browser) {
			goto('/login');
		}
	}
</script>

<h1>About this site</h1>
<p>TODO...</p>
