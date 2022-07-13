<!-- TODO: use tailwind/unocss -->
<div class="index-page" flex="~ direction=column">
	<!-- TODO: header -->
	<div class="top-section">
		<!-- TODO: Find a background image for this section, find a font colour for it -->
		<h1>Meet the next-gen minecraft launcher.</h1>
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
</style>
