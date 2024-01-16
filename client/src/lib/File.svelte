<script lang="ts">
    export let file;

    const download = () => {

    };

    const remove = () => {

    };
</script>

<style lang="scss">
    @import "../styles.scss";

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
    }
</style>

<div id="main-container">
    <img src="/FileSymbol.png" alt="[FILE]" id="icon"/>
    <p id="name">{file}</p>

    <button id="download" class="action-button" on:click={download}>
        <img src="/DownloadSymbol.png" alt="[DOWNLOAD]" class="action-button-icon"/>
    </button>
    <button id="delete" class="action-button" on:click={remove}>
        <img src="/TrashSymbol.png" alt="[DELETE]" class="action-button-icon" id="trash-icon"/>
    </button>
</div>
