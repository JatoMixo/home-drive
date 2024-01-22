<script lang="ts">
    export let actualPath = "";
    export let serverIp;
    export let directoriesInCurrentPath;
    let addingFolder = false;
    let newFolderName = "";

    const switchAddFolderMenu = () => {
        addingFolder = !addingFolder;
    };

    const createDirectory = () => {
        fetch("http://" + serverIp + ":8080" + "/directory?path=" + actualPath + "/" + newFolderName + "/");
        directoriesInCurrentPath.push(newFolderName);
        directoriesInCurrentPath = directoriesInCurrentPath;

        addingFolder = false;
        newFolderName = "";
    }
</script>

<style lang="scss">
    @import "../styles.scss";

    #create-folder-button,
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

    #new-folder-button,
    #new-folder-text {
        font-size: 20px;
        font-family: "Hack Nerd Font";

        margin: 0;
    }

    #create-folder-button {
        padding: 4px;
    }

    #new-folder-menu {
        display: flex;
        align-items: center;
        justify-content: center;

        padding: 4px;
        padding-left: 10px;
        gap: 15px;
    }
</style>

{#if addingFolder}
    <div class="box" id="new-folder-menu">
        <input type="text" bind:value={newFolderName} class="text-input" />
        <button id="create-folder-button" class="box" on:click={createDirectory}>
            <img src="/AddFolderSymbol.png" alt="" id="icon"/>
        </button>
    </div>
{/if}

<button id="new-folder-button" class="box" on:click={switchAddFolderMenu}>
    
    <img src="/AddFolderSymbol.png" alt="" id="icon"/>
    <p id="new-folder-text">New Folder</p>
</button>
