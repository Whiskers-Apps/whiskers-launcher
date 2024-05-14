<script lang="ts">
	import Input from '$lib/components/input.svelte';
	import Toggle from '$lib/components/toggle.svelte';
	import type { Settings } from '$lib/settings/settings';
	import { createEventDispatcher } from 'svelte';

	export let settings: Settings;

	let text = '';
    let toggled = true;

	let dispatch = createEventDispatcher();

	function onInput(event: CustomEvent<string>) {
        text = event.detail;
    }

    function onToggle(event: CustomEvent<boolean>) {
        toggled = event.detail;
    }
</script>

<Input text={text} on:input={onInput} placeholder="Chupapi monhenho" />
<Toggle toggled={toggled} on:toggle={onToggle} />

{toggled}