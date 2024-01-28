<script lang="ts">
    export let file;
    export let path;
    export let server;

    let gotRemoved = false;

    const remove = () => {
        fetch("http://" + server.ip + ":" + server.port + "/delete?path=" + path + "/" + file);
        gotRemoved = true;
    };
</script>

<style lang="scss">
    @import "../scss/styles.scss";

    #main-container {
        display: flex;
        align-items: center;

        background-color: $normal-background-color;

        gap: 5px;

        padding: 5px;

        border-radius: 15px;
        border-width: 0px;

        margin-right: 0px;
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

        transition: background-color 100ms;
    }

    .action-button:hover {
        background-color: #111111;
    }

    .action-button:active {
        background-color: #000000;
    }

    #download {
        margin-left: auto;

        display: flex;
        justify-content: center;
        align-items: center;
    }
</style>

{#if !gotRemoved}
    <div id="main-container">
        <img src="/FileSymbol.png" alt="[FILE]" id="icon"/>
        <p id="name">{file}</p>

        <a id="download" class="action-button" href={"http://" + server.ip + ":" + server.port + "/download/" + path + "/" + file}>
            <img src="/DownloadSymbol.png" alt="[DOWNLOAD]" class="action-button-icon"/>
        </a>
        <button id="delete" class="action-button" on:click={remove}>
            <img src="/TrashSymbol.png" alt="[DELETE]" class="action-button-icon" id="trash-icon"/>
        </button>
    </div>
{/if}
