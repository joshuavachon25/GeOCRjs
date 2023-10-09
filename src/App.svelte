<script>
	import { open } from "@tauri-apps/api/dialog";
	import Canvas from "./lib/Canvas.svelte";
	import FAB from "./lib/FAB.svelte";
	import {convertFileSrc} from "@tauri-apps/api/tauri";

	let appContext = {
		state: 0,
		error: 0,
		archive: undefined
	}

	function setAppContextTo(value){
		appContext.state = value
	}

	async function openFile(){
		try {
			const path = await open({
				multiple:false,
				title:"Ouvrir une image d'archive",
				filters: [{
					name: 'Image',
					extensions: ['png', 'jpeg', 'gif', 'tiff', 'bmp']
				}]
			})
			if (!path) {
				appContext.error = "Aucun fichier n'a été choisi"
				return
			}

			appContext.archive = convertFileSrc(path)
			setAppContextTo(1)
		}catch(err){
			appContext.error = "Erreur innatendue"
		}
	}
</script>

<div id="workspace">
		<FAB />
	{#if appContext.state === 0}
		<button class="openBtn" on:click={openFile}>Ouvrir une archive</button>
		{#if appContext.error}
			<p class="openError">Erreur: {appContext.error}</p>
		{/if}
	{:else if appContext.state === 1}
		<Canvas {appContext}/>
	{/if}

</div>
