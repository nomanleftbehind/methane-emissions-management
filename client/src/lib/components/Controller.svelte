<script lang="ts">
	import type { IController } from '../../types';

	export let controller: IController;
	let { id, fdcRecId, function: application, manufacturer, model, serialNumber } = controller;
</script>

<tr>
	<td>{model}</td>
	<td>{fdcRecId}</td>
	<td>{manufacturer?.manufacturer}</td>
	<td>{application?.function}</td>
</tr>

<style>
	td {
		border: 1px solid black;
	}
</style>
