<script lang="ts">
    export let displayingSettings = false;

    const switchSettings = () => {
        displayingSettings = !displayingSettings;
    }
</script>

<style lang="scss">
    @import "../scss/styles.scss";
    @import "../scss/box.scss";
    @import "../scss/button.scss";

    #settings-button {
        padding: 5px;
    }

    #icon {
        height: 30px;
    }
</style>

<button on:click={switchSettings} id="settings-button" class="box">
    <img src="/SettingsSymbol.png" id="icon" alt="VSCode Shut Up"/>
</button>