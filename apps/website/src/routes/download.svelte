<script>
	// Find what operating system the user is on
	import UAParser from "ua-parser-js";
	let ua = new UAParser();
	let detectedOS = ua.getOS().name;
</script>