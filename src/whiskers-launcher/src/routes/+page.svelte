<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { appWindow } from '@tauri-apps/api/window';

	// ===========================
	// UI Events
	// ===========================
	appWindow.setResizable(false);

	window.addEventListener('keydown', (event) => {
		switch (event.key) {
			case 'Escape': {
				appWindow.close();
				break;
			}
		}

		if (event.ctrlKey && event.key === 's') {
            openSettings();
			appWindow.close();
        }
	});

	function openSettings(){
		invoke("open_settings_window");
	}
</script>

<!-- 
======================================================================================
 UI
======================================================================================
-->

<div class="bg-blue-400 h-screen overflow-hidden flex justify-center">
	<div class=" w-[800px] bg-red-500">
		<button on:click={openSettings}>Open Settings</button>
	</div>
</div>
