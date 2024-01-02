<script lang="ts">
    export let ip = "";
</script>

<style lang="scss">
    @import "../styles.scss";

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

    .text-input {
        background-color: #171717;
        border: none;
        border-radius: 5px;

        font-family: "Hack Nerd Font";
        font-size: 18px;
        color: $red-color;
        text-shadow: 0 0 3px $red-color;
    }

    .text-input:focus {
        outline: none;
    }
</style>

<div id="main-container" class="box">
    <h1 id="title"><u>Settings</u></h1>

    <div id="ip-section">
        <p class="property">IP: </p>
        <input type="text" class="text-input" bind:value={ip}/>
    </div>
</div>
