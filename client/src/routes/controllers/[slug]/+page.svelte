<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<a href="/">Home</a>
<h1>{data.post.title}</h1>
<div>{@html data.post.content}</div>
