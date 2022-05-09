<script lang="ts">import UAParser from 'ua-parser-js'
const ua = new UAParser()
// eslint-disable-next-line no-unused-vars
const detectedOS = ua.getOS().name
</script>