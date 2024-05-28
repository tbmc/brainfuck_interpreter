<script lang="ts" context="module">
	import hljs from 'highlight.js/lib/core';
	import brainfuck from '$lib/brainfuckLanguageDefinition';

	hljs.registerLanguage('brainfuck', brainfuck);

	const highlight = (code: string, syntax: string | undefined) => {
		if (syntax === undefined) throw new Error('You should select a syntax');
		return hljs.highlight(code, { language: syntax }).value;
	};
</script>

<script lang="ts">
	import { Pane, Splitpanes } from 'svelte-splitpanes';
	import ListScriptsComponent from '$lib/ListScriptsComponent.svelte';
	import { CodeJar } from '@novacbn/svelte-codejar';
	import { connectWebsocket } from '$lib/websocket';
	import { onMount } from 'svelte';

	let sendDataFn: null | ((data: string) => void) = null;
	const onExecuteClick = () => {
		sendDataFn = connectWebsocket(
			currentCodeText,
			(data: string) => {
				outputText.value += data;
			},
			() => {
				outputText.value += '\n\nProgram ended.\n\n';
			}
		);
	};

	const onScriptChange = (value: string) => {
		currentCodeText = value;
	};

	let currentCodeText: string = '';
	let inputText: CodeJar;
	let outputText: HTMLTextAreaElement;

	onMount(() => {
		// @ts-expect-error event is InputEvent at runtime
		outputText.addEventListener('input', (event: InputEvent) => {
			if (sendDataFn === null) throw new Error('This should not happen');
			sendDataFn(event.data === null ? '\n' : event.data);
		});
	});
</script>

<nav>
	<span>
		<div class="title">Brainfuck interpreter</div>
		<ListScriptsComponent onChange={onScriptChange} />
		<button type="button" on:click={onExecuteClick}>Execute</button>
	</span>
</nav>

<div class="root-container">
	<Splitpanes style="width: 100vw; height: 90vh;">
		<Pane>
			<CodeJar
				bind:this={inputText}
				class="hljs code-jar"
				syntax="brainfuck"
				{highlight}
				value={currentCodeText}
				withLineNumbers={true}
				style="width: 100%; height: 100%;"
			/>
		</Pane>
		<Pane>
			<textarea bind:this={outputText} />
		</Pane>
	</Splitpanes>
</div>

<style>
	nav {
		height: 10vh;
		padding-bottom: 10px;
		padding-top: 10px;
		text-align: center;
	}

	nav span {
		margin-left: auto;
		margin-right: auto;
		display: grid;
		grid-template-columns: 1fr 1fr 1fr;
		gap: 10px;
	}

	nav span .title {
			font-weight: bold;
			font-size: 1.5em;
			height: 100%;
			display: flex;
			align-items: center;
			justify-content: center;
	}

	.root-container {
		display: grid;
	}

	textarea {
		resize: none;

		width: 100%;
		height: 100%;
	}
</style>
