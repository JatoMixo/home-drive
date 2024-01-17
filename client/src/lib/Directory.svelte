<script lang="ts">
    export let directory;
    export let serverIp;
    export let path;

    let gotRemoved = false; 

    const remove = () => {
        fetch("http://" + serverIp + ":8080" + "/delete?path=" + path + "/" + directory + "/");
        gotRemoved = true;
    };

    const enter = () => {
        path += directory + "/";
    };
</script>

<style lang="scss">
    @import "../styles.scss";

    #main-container {
        display: flex;
        align-items: center;

        background-color: $normal-background-color;

        gap: 5px;

        transition: background-color 100ms;
        padding: 5px;

        border-radius: 15px;
        border-width: 0px;

        margin-right: 0px;

        width: 100%;
    }

    #main-container:hover {
        background-color: $dark-background-color;
    }

    #main-container:active {
        background-color: #252525;
    }

    #icon {
        width: 50px;
    }

    #name {
        font-size: 20px;

        color: white;
        text-shadow: 0 0 3px white;
    }

    .action-button-icon {
        width: 40px;
    }

    #trash-icon {
        width: 25px;
    }

    .action-button {
        background-color: $normal-background-color;

        width: 60px;
        height: 60px;

        border-width: 0px;
        border-radius: 15px;
    }

    #delete {
        margin-left: auto;
    }
</style>

{#if !gotRemoved}
    <button id="main-container" on:click={enter}>
        <img src="/FolderSymbol.png" alt="[DIR]" id="icon"/>
        <p id="name">{directory}</p>

        <button id="delete" class="action-button" on:click={remove}>
            <img src="/TrashSymbol.png" alt="[DELETE]" class="action-button-icon" id="trash-icon"/>
        </button>
    </button>
{/if}
