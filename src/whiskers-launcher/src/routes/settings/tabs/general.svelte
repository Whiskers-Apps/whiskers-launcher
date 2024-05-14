<script lang="ts">
	import type { SelectValue } from '$lib/components/classes';
	import Input from '$lib/components/input.svelte';
	import Select from '$lib/components/select.svelte';
	import Toggle from '$lib/components/toggle.svelte';
	import type { Settings } from '$lib/settings/settings';

	export let settings: Settings;

	let text = '';
	let toggled = true;

	let values: SelectValue[] = [
		{
			id: 'bulbasaur',
			value: 'Bulbasaur'
		},
		{
			id: 'charmander',
			value: 'Charmander'
		},
		{
			id: 'squirtle',
			value: 'Squirtle'
		}
	];

	let selectedValue = "charmander";

	function onInput(event: CustomEvent<string>) {
		text = event.detail;
	}

	function onToggle(event: CustomEvent<boolean>) {
		toggled = event.detail;
	}

	function onSelection(value: CustomEvent<SelectValue>){
		selectedValue = value.detail.id;
	}
</script>

<div class=" space-y-4">
	<Input {text} on:input={onInput} placeholder="Type something here" />
	<Toggle {toggled} on:toggle={onToggle} />
	<Select {values} {selectedValue} on:selection={onSelection} />
</div>
