<script lang="ts">
	import scripts from './brain_fuck_scripts';

	export let onChange: (newValue: string) => void;

	let selectOnChange = () => {
		if (selectValue === '') {
			onChange('');
		} else {
			const text = scripts[selectValue].trimStart();
			onChange(text);
		}
	};

	let selectValue: string = '';
</script>

<div>
	<p>Select script</p>
	<select
		name="script"
		aria-label="select brain fuck script"
		bind:value={selectValue}
		on:change={selectOnChange}
	>
		<option value="">None</option>
		{#each Object.keys(scripts) as script}
			<option value={script}>{script}</option>
		{/each}
	</select>
</div>

<style>
	div {
		display: grid;
		grid-template-columns: 1fr 1fr;
	}

	p {
		vertical-align: center;
		justify-content: center;
		align-items: center;
		display: flex;
		height: 100%;
	}
</style>
