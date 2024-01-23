<script lang="ts">
    export let ip = "";
</script>

<style lang="scss">
    @import "../scss/variables.scss";
    @import "../scss/box.scss";
    @import "../scss/text_input.scss";

    #main-container {
        padding: 12px;

        width: 290px;
    }

    #title {
        margin: 0;
        font-size: 30px;

        margin-bottom: 15px;
    }

    .property {
        margin: 0;

        font-size: 18px;
    }

    #ip-section {
        display: flex;
        align-items: center;
        gap: 15px;
    }
</style>

<div id="main-container" class="box">
    <h1 id="title"><u>Settings</u></h1>

    <div id="ip-section">
        <p class="property">IP: </p>
        <input type="text" class="text-input" bind:value={ip}/>
    </div>
</div>
