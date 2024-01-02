<script lang="ts">
    export let displayingSettings = false;

    const switchSettings = () => {
        displayingSettings = !displayingSettings;
        console.log("SU");
    }
</script>

<style lang="scss">
    @import "../styles.scss";

    #settings-button {
        background-color: $dark-background-color;

        border: solid 4px $red-color;
        border-radius: 15px;
        box-shadow: 0 0 10px $red-color;

        padding: 5px;

        transition: background-color 100ms;
    }

    #settings-button:hover {
        background-color: #151515;
    }

    #settings-button:active {
        background-color: #4D1822;
    }

    #icon {
        height: 30px;
    }
</style>

<button on:click={switchSettings} id="settings-button">
    <img src="/SettingsSymbol.png" id="icon" alt="VSCode Shut Up" />
</button>