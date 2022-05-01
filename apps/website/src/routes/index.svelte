<!-- TODO: use tailwind/unocss -->
<div class="index-page" flex="~ direction=column">
	<div class="top-section">
		<!-- TODO: Find a background image for this section to make it look nice, then find a font colour for it -->
		<h1>Glowsquid launcher</h1>
	</div>
		<div class="point" />
	<div class="middle-section">
			<p>Built on rust and svelte.</p>
			<h1>Fast</h1>
			<p>Helpful defaults and easy modpack installation</p>
			<h1>Useful</h1>
		<div class="point" />
		<div class="seperator" />
			<p>Free, forever.</p>
			<h1>Open source</h1>
		<div class="seperator" />
		<div class="point" />
	</div>
</div>

<style>
	.index-page {
		font-family: 'Varela', sans-serif;
	}
	.top-section h1 {
		font-size: 5em;
		font-weight: bold;
		margin-bottom: 0;
	}
	.top-section {
		color: black;
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		padding: 10px;
		margin-bottom: 10px;
		height: calc(100vh / 3 * 2);
		text-align: center;
	}
	.middle-section {
		color: rgb(186, 186, 186);
		background-color: rgb(61, 61, 61);
		display: flex;
		flex-direction: row;
		justify-content: space-around;
		align-items: center;
		flex-wrap: wrap;
		padding: 10px;
		height: calc(100vh / 3);
	}
	.middle-section .point {
		/* Align all text to the centre */
		text-align: center;
		width: 15vw;
		height: 20vh;
		border-radius: 10px;
		margin: 10px;
		padding: 10px;
	}
	.middle-section .point h1 {
		font-size: 2em;
		margin-bottom: 10px;
	}
	.middle-section .point p {
		font-size: 1em;
	}
</style>