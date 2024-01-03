<script lang="ts">
    export let actualPath = "";

    const addFolder = () => {

    };

</script>

<style lang="scss">
    @import "../styles.scss";

    #new-folder-button {
        display: flex;
        align-items: center;
        gap: 10px;

        padding: 10px;
    }

    #icon {
        width: 30px;

        margin: 0;
    }

    #new-folder-text {
        font-size: 20px;
        font-family: "Hack Nerd Font";

        margin: 0;
    }
</style>

<button id="new-folder-button" class="box" on:click={addFolder}>
    <img src="/AddFolderSymbol.png" alt="" id="icon"/>
    <p id="new-folder-text">New Folder</p>
</button>
