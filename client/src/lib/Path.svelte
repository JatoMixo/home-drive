<script lang="ts">
    export let path = "";
</script>

<style lang="scss">

    // Import the color variables
    @import "../styles.scss";

    #path-container {
        padding: 10px;
        width: 100%;
    }

    #path-text {
        margin: 0px;

        font-size: 20px;
    }
</style>

<div id="path-container" class="box">
    <p id="path-text">{path}</p>
</div>
